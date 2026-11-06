#!/usr/bin/env python3
"""Authenticated same-player bridge; Core remains the sole gameplay authority.

Only loopback listeners are allowed. Remote devices use an SSH transport tunnel
and a separate app credential. No shell commands or file paths cross this seam.
"""
import argparse
import hmac
import json
import os
from pathlib import Path
import re
import socket
import socketserver
import stat
import subprocess
import sys
import threading
import time
import uuid

LIMIT = 64 * 1024
SLOT = "shared-autosave"


def read_config(path):
    path = Path(path)
    mode = path.stat().st_mode
    if not stat.S_ISREG(mode) or mode & 0o077:
        raise ValueError("shared-session credentials must be a private file (chmod 600)")
    config = json.loads(path.read_text())
    if config.get("version") != 1:
        raise ValueError("unsupported shared-session configuration")
    for key in ("account_id", "session_id", "device_id"):
        if not re.fullmatch(r"[a-z0-9][a-z0-9.-]{0,127}", config.get(key, "")):
            raise ValueError(f"invalid {key}")
    return config


def envelope(config, kind, revision=0, **payload):
    return dict(protocol_version=1, session_id=config["session_id"],
                request_id="request.bridge." + uuid.uuid4().hex,
                expected_revision=revision, intent=dict(type=kind, **payload))


class Authority:
    def __init__(self, config):
        self.config = config
        self.lock = threading.Lock()
        self.failed = False
        self.process = subprocess.Popen([
            config["service_bin"], "--stdio", "--session", config["session_id"],
            "--save-root", config["save_root"], "--world-root", config["world_root"],
        ], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        try:
            initial = self.exchange(envelope(config, "ObserveIntent"))
            self.revision = initial["revision"]
            save = Path(config["save_root"]) / f'{config["session_id"]}.{SLOT}.hgplay.json'
            backup = save.with_name(save.name + ".backup")
            if save.exists() or backup.exists():
                # Do not silently reverse a Guide transition newer than the checkpoint.
                try:
                    saved = json.loads(save.read_text())
                except (OSError, ValueError):
                    saved = json.loads(backup.read_text())
                if initial["revision"] > saved["service_revision"]:
                    raise RuntimeError("Guide recovery is newer than the shared checkpoint; inspect recovery before resuming")
                loaded = self.exchange(envelope(config, "LoadIntent", self.revision, slot_id=SLOT))
                self.require_completed(loaded)
                self.revision = loaded["revision"]
            self.checkpoint()
        except BaseException:
            self.close()
            raise

    @staticmethod
    def require_completed(response):
        if response.get("status") != "Completed":
            raise RuntimeError(response.get("rejection", {}).get("message", "Core rejected checkpoint"))

    def exchange(self, request):
        encoded = (json.dumps(request, separators=(",", ":")) + "\n").encode()
        if len(encoded) > LIMIT + 1:
            raise ValueError("request exceeds protocol limit")
        self.process.stdin.write(encoded)
        self.process.stdin.flush()
        line = self.process.stdout.readline(LIMIT + 2)
        if not line.endswith(b"\n") or len(line) > LIMIT + 1:
            raise RuntimeError("Core disconnected or exceeded its response limit")
        return json.loads(line)

    def checkpoint(self):
        # Checkpoint is an internal service operation, not a replay-ledger request.
        response = self.exchange(envelope(self.config, "CheckpointIntent", self.revision, slot_id=SLOT))
        self.require_completed(response)

    def handle(self, request):
        with self.lock:
            if self.failed:
                raise RuntimeError("shared authority needs recovery")
            if not isinstance(request, dict) or not isinstance(request.get("intent"), dict):
                raise ValueError("invalid intent envelope")
            kind = request.get("intent", {}).get("type")
            if request.get("session_id") != self.config["session_id"]:
                raise ValueError("credential does not authorize this session")
            if kind == "CheckpointIntent" or (kind in ("LoadIntent", "SaveIntent")
                    and request["intent"].get("slot_id") == SLOT):
                raise ValueError("the shared checkpoint is managed by the host")
            if kind == "EstablishHuemanIntent":
                expected = {"continuity_id": "being-continuity.hueman", "participant_id": "participant.hueman", "institutional_being_id": "being.hueman"}
                if any(request["intent"].get(k) != v for k, v in expected.items()):
                    raise ValueError("credential does not authorize another player identity")
            try:
                response = self.exchange(request)
                if response["revision"] != self.revision or (kind == "LoadIntent" and response["status"] == "Completed"):
                    self.revision = response["revision"]
                    self.checkpoint()
                return response
            except BaseException:
                # No client receives a success for an uncheckpointed mutation.
                self.failed = True
                self.close()
                raise

    def close(self):
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait()
        for stream in (self.process.stdin, self.process.stdout):
            if stream is not None:
                stream.close()


def authenticate(config, hello):
    if not isinstance(hello, dict):
        return False
    token = hello.get("token", "")
    device_id = hello.get("device_id", "")
    if not isinstance(device_id, str) or not isinstance(token, str) or not re.fullmatch(r"[0-9a-f]{64}", token):
        return False
    expected = config.get("devices", {}).get(device_id, "")
    return (hello.get("type") == "SharedSessionHello" and hello.get("version") == 1
            and hello.get("account_id") == config["account_id"]
            and hello.get("session_id") == config["session_id"]
            and isinstance(token, str) and isinstance(expected, str)
            and len(expected) == 64 and hmac.compare_digest(token, expected))


class Handler(socketserver.StreamRequestHandler):
    def read_message(self):
        line = self.rfile.readline(LIMIT + 2)
        if not line:
            return None
        if len(line) > LIMIT + 1 or not line.endswith(b"\n"):
            raise ValueError("invalid frame")
        message = json.loads(line)
        if not isinstance(message, dict):
            raise ValueError("invalid envelope")
        return message

    def write_message(self, message):
        self.wfile.write((json.dumps(message, separators=(",", ":")) + "\n").encode())
        self.wfile.flush()

    def handle(self):
        self.connection.settimeout(10)
        self.connection.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        try:
            if not authenticate(self.server.config, self.read_message()):
                self.write_message(dict(type="SharedSessionWelcome", version=1, accepted=False))
                return
            self.write_message(dict(type="SharedSessionWelcome", version=1, accepted=True,
                                    account_id=self.server.config["account_id"],
                                    session_id=self.server.config["session_id"]))
            self.connection.settimeout(30)
            while (request := self.read_message()) is not None:
                self.write_message(self.server.authority.handle(request))
        except (OSError, ValueError, RuntimeError, KeyError, TypeError):
            # Never log raw requests or credentials.
            return


class Server(socketserver.ThreadingTCPServer):
    allow_reuse_address = True
    daemon_threads = True

    def __init__(self, config, authority):
        self.config, self.authority = config, authority
        self.slots = threading.BoundedSemaphore(16)
        super().__init__(("127.0.0.1", int(config["port"])), Handler)

    def process_request(self, request, client_address):
        if not self.slots.acquire(blocking=False):
            self.shutdown_request(request)
            return
        try:
            super().process_request(request, client_address)
        except BaseException:
            self.slots.release()
            raise

    def process_request_thread(self, request, client_address):
        try:
            super().process_request_thread(request, client_address)
        finally:
            self.slots.release()


def launch(config, extra):
    port = int(config["port"])
    if not 1 <= port <= 65535:
        raise ValueError("invalid port")
    tunnel = None
    if config.get("ssh_host"):
        host = config["ssh_host"]
        if not re.fullmatch(r"[a-zA-Z0-9][a-zA-Z0-9.-]*", host):
            raise ValueError("invalid SSH host alias")
        # Each window owns its forwarding port; closing one cannot strand another.
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        tunnel_command = ["ssh", "-N", "-T", "-o", "BatchMode=yes", "-o", "StrictHostKeyChecking=yes",
            "-o", "ExitOnForwardFailure=yes", "-o", "ConnectTimeout=8", "-o", "ServerAliveInterval=10",
            "-o", "ServerAliveCountMax=3", "-L", f'127.0.0.1:{port}:127.0.0.1:{int(config["remote_port"])}', host]
        tunnel = subprocess.Popen(tunnel_command)
    env = os.environ.copy()
    env.update(HOLLOW_GROVE_SESSION=config["session_id"], HOLLOW_GROVE_HOST="127.0.0.1", HOLLOW_GROVE_PORT=str(port),
               HOLLOW_GROVE_SHARED_CONFIG=str(config["config_path"]))
    try:
        game = subprocess.Popen([config.get("hueman_bin", "hueman"), *extra], env=env)
        while game.poll() is None:
            if tunnel is not None and tunnel.poll() is not None:
                time.sleep(1)
                tunnel = subprocess.Popen(tunnel_command)
            time.sleep(0.1)
        return game.returncode
    finally:
        if tunnel is not None:
            tunnel.terminate()
            tunnel.wait(timeout=10)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("host", "launch"))
    parser.add_argument("--config", required=True)
    args, extra = parser.parse_known_args()
    config = read_config(args.config)
    config["config_path"] = str(Path(args.config).resolve())
    if args.mode == "launch":
        return launch(config, extra)
    if extra:
        parser.error("unknown host arguments")
    authority = Authority(config)
    try:
        with Server(config, authority) as server:
            print(f'Shared Hueman session ready for {config["account_id"]} on loopback port {config["port"]}', flush=True)
            server.serve_forever()
    finally:
        authority.close()
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, RuntimeError) as error:
        print(f"Shared Hueman session: {error}", file=sys.stderr)
        sys.exit(1)
