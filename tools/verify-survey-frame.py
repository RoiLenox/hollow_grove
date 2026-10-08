#!/usr/bin/env python3
"""Exercise the actual service with private saves; emit replayable case witnesses."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics
import subprocess
import tempfile
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--service", required=True, type=Path)
    parser.add_argument("--world-root", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--cli", type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="hollow-grove-survey-frame."))
    results = []
    for case in ("measurements", "protected-wait", "repaired-rig"):
        session = f"session.frame-witness.{case}"
        saves = work / case
        serial, revision, maximum_bytes = 0, 0, 0
        samples = []
        process = None

        def start():
            return subprocess.Popen(
                [str(args.service.resolve()), "--stdio", "--session", session,
                 "--save-root", str(saves), "--world-root", str(args.world_root.resolve())],
                text=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                stderr=(work / f"{case}.log").open("a"))

        def stop():
            process.stdin.close()
            process.wait(timeout=15)
            assert process.returncode == 0

        def request(kind, accepted=True, **fields):
            nonlocal serial, revision, maximum_bytes
            serial += 1
            packet = {"protocol_version": 1, "session_id": session,
                      "request_id": f"request.frame-witness.{serial}",
                      "expected_revision": revision, "intent": {"type": kind, **fields}}
            before = revision
            began = time.perf_counter_ns()
            process.stdin.write(json.dumps(packet) + "\n")
            process.stdin.flush()
            line = process.stdout.readline()
            assert line, "service stopped without a response"
            samples.append((time.perf_counter_ns() - began) / 1000)
            maximum_bytes = max(maximum_bytes, len(line.encode()))
            assert len(line.encode()) <= 65536, "response exceeds the protocol limit"
            response = json.loads(line)
            assert (response["status"] == "Completed") == accepted, response
            revision = response["revision"]
            if not accepted:
                assert revision == before
            if response.get("view"):
                assert response["view"]["kernel_audit"]["passed"]
            return response.get("view")

        def action(value, accepted=True):
            return request("AdvanceSurveyFrameIntent", accepted=accepted, action=value)

        def offer(scope):
            return action({"OfferUse": {"use_scope": scope}})

        def decide(choice):
            advised = action({"SupportResidentChoice": {"choice": choice}})["survey_frame"]
            count = len(advised["resident_decisions"])
            actual = action("AskResidentToDecide")["survey_frame"]
            assert len(actual["resident_decisions"]) == count + 1
            assert actual["resident_decisions"][-1]["resident_id"] == "being.current-sea.mercy-deep"

        def deliver(disposition):
            return action({"DeliverDisposition": {"disposition": disposition}})["survey_frame"]

        def save(key):
            view = request("SaveIntent", slot_id=f"slot.{key}")
            path = saves / f"{session}.slot.{key}.hgplay.json"
            archive = json.loads(path.read_text())["gameplay_archive"]
            target = args.output / f"{case}-{key}-gameplay-v6.json"
            target.write_text(archive)
            assert json.loads(archive)["schema_version"] == 6
            if args.cli:
                for command in ("lore", "architecture"):
                    output = subprocess.check_output(
                        [str(args.cli.resolve()), command, "--save", str(target),
                         *(["--source-root", str(args.world_root.resolve())] if command == "architecture" else []),
                         "--json"], text=True)
                    report = json.loads(output)
                    if command == "architecture":
                        assert report["source_links_passed"]
                        assert report["verified_binding_count"] == 4
                    (args.output / f"{case}-{key}-{command}.json").write_text(output)
            return view

        try:
            process = start()
            request("EnterHollowGroveIntent")
            request("EstablishHuemanIntent", continuity_id="continuity.frame-witness",
                    participant_id="participant.frame-witness", institutional_being_id="being.frame-witness")
            if case == "repaired-rig":
                for value in ("RequestAssessment", "ConsentToFoundation", "AskClinicianToEstablish", "Rest", "AskClinicianToDischarge"):
                    request("AdvanceSympioteFoundationIntent", action=value)
                # Long clinical history and a full object case share the same view/transport.
                for _ in range(80):
                    for value in ("ReportStrain", "Rest", "AskClinicianToReview", "Rest", "AskClinicianToDischarge"):
                        request("AdvanceSympioteFoundationIntent", action=value)
            foundation = request("SyncIntent")["sympiote_foundation"]
            for value in ("ReceiveNeed", "RecoverParts", "AskStonebendToRegister", "AskReviewerToMeasure"):
                action(value)
            action({"DeliverDisposition": {"disposition": "TemporaryRigDelivered"}}, accepted=False)
            if case == "measurements":
                offer("MeasuringOnly")
                decide("AcceptMeasurements")
                final = deliver("MeasurementsDelivered")
            else:
                action("PrepareRig")
                action("AskReviewerToMeasure")
                offer("TemporaryRig")
                failed = action("AskReviewerToTrialSupport")["survey_frame"]
                assert not failed["support_proven"] and not failed["measurement_proven"]
                assert failed["revisions"][1]["support_receipt"]["claim"] == "FailedSupportTrial"
                decide("WaitSafely")
                if case == "protected-wait":
                    final = deliver("ProtectedDelayDelivered")
                else:
                    action("RepairJoint")
                    action("AskReviewerToTrialSupport", accepted=False)
                    action("AskReviewerToMeasure")
                    action("AskReviewerToTrialSupport")
                    offer("TemporaryRig")
                    decide("AcceptTemporaryRig")
                    final = deliver("TemporaryRigDelivered")
            assert final["complete"]
            before = save("delivered")
            assert before["sympiote_foundation"] == foundation
            stop()
            process = start()
            request("SyncIntent")
            restored = request("LoadIntent", slot_id="slot.delivered")
            assert restored["survey_frame"] == before["survey_frame"]
            assert restored["sympiote_foundation"] == foundation
            if case == "repaired-rig":
                withdrawn = action("AskResidentToWithdraw")["survey_frame"]
                assert not withdrawn["deployed"] and withdrawn["recovery_required"] and not withdrawn["complete"]
                deliver_request = {"DeliverDisposition": {"disposition": "ProtectedDelayDelivered"}}
                action(deliver_request, accepted=False)
                recovered = action("RecoverDeployedRig")["survey_frame"]
                assert not recovered["complete"] and not recovered["recovery_required"]
                final = deliver("ProtectedDelayDelivered")
                assert final["complete"] and len(final["deliveries"]) == 2
                save("recovered")
            stop()
            process = None
            results.append({"case": case, "requests": serial, "save_restart_load": "passed",
                            "complete": final["complete"], "maximum_response_bytes": maximum_bytes,
                            "stdio_round_trip_median_us": statistics.median(samples),
                            "stdio_round_trip_max_us": max(samples), "clinical_care_cycles": 80 if case == "repaired-rig" else 0})
        finally:
            if process is not None and process.poll() is None:
                process.terminate()
                process.wait(timeout=15)
    receipt = {"schema_version": 1, "service": str(args.service.resolve()),
               "service_sha256": hashlib.sha256(args.service.read_bytes()).hexdigest(),
               "private_save_root": str(work), "original_saves_touched": False, "cases": results}
    (args.output / "ACCEPTANCE.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt))


if __name__ == "__main__":
    main()
