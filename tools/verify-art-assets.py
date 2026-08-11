#!/usr/bin/env python3
"""Fail-closed validation for the committed Hueman art contract."""

from __future__ import annotations

import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
MANIFEST_PATH = REPO_ROOT / "hueman_godot/assets/manifest.json"
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


class VerificationError(RuntimeError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise VerificationError(f"cannot read JSON {path}: {error}") from error
    if not isinstance(document, dict):
        raise VerificationError(f"expected a JSON object in {path}")
    return document


def repo_file(relative_path: str) -> Path:
    path = (REPO_ROOT / relative_path).resolve()
    try:
        path.relative_to(REPO_ROOT)
    except ValueError as error:
        raise VerificationError(f"path escapes repository: {relative_path}") from error
    if not path.is_file():
        raise VerificationError(f"missing file: {relative_path}")
    return path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def require_digest(relative_path: str, expected: str) -> Path:
    path = repo_file(relative_path)
    actual = sha256(path)
    if actual != expected:
        raise VerificationError(
            f"digest mismatch for {relative_path}: expected {expected}, found {actual}"
        )
    return path


def paeth(left: int, above: int, upper_left: int) -> int:
    estimate = left + above - upper_left
    left_distance = abs(estimate - left)
    above_distance = abs(estimate - above)
    upper_left_distance = abs(estimate - upper_left)
    if left_distance <= above_distance and left_distance <= upper_left_distance:
        return left
    if above_distance <= upper_left_distance:
        return above
    return upper_left


def decode_rgba_png(path: Path) -> tuple[int, int, list[tuple[int, int, int, int]]]:
    payload = path.read_bytes()
    if not payload.startswith(PNG_SIGNATURE):
        raise VerificationError(f"not a PNG: {path}")

    offset = len(PNG_SIGNATURE)
    width = height = 0
    compressed = bytearray()
    saw_end = False
    while offset < len(payload):
        if offset + 12 > len(payload):
            raise VerificationError(f"truncated PNG chunk in {path}")
        length = struct.unpack(">I", payload[offset : offset + 4])[0]
        chunk_type = payload[offset + 4 : offset + 8]
        chunk_start = offset + 8
        chunk_end = chunk_start + length
        if chunk_end + 4 > len(payload):
            raise VerificationError(f"truncated PNG payload in {path}")
        chunk = payload[chunk_start:chunk_end]
        stored_crc = struct.unpack(">I", payload[chunk_end : chunk_end + 4])[0]
        actual_crc = zlib.crc32(chunk_type + chunk) & 0xFFFFFFFF
        if stored_crc != actual_crc:
            raise VerificationError(f"invalid PNG CRC in {path}")
        if chunk_type == b"IHDR":
            width, height, depth, color_type, compression, filtering, interlace = (
                struct.unpack(">IIBBBBB", chunk)
            )
            if (depth, color_type, compression, filtering, interlace) != (8, 6, 0, 0, 0):
                raise VerificationError(
                    f"{path} must be non-interlaced 8-bit RGBA, found "
                    f"depth={depth} color_type={color_type} interlace={interlace}"
                )
        elif chunk_type == b"IDAT":
            compressed.extend(chunk)
        elif chunk_type == b"IEND":
            saw_end = True
            break
        offset = chunk_end + 4

    if width <= 0 or height <= 0 or not compressed or not saw_end:
        raise VerificationError(f"incomplete PNG structure in {path}")
    try:
        scanlines = zlib.decompress(bytes(compressed))
    except zlib.error as error:
        raise VerificationError(f"invalid compressed pixels in {path}: {error}") from error

    bytes_per_pixel = 4
    stride = width * bytes_per_pixel
    if len(scanlines) != height * (stride + 1):
        raise VerificationError(f"unexpected pixel payload size in {path}")

    prior = bytearray(stride)
    pixels: list[tuple[int, int, int, int]] = []
    cursor = 0
    for _ in range(height):
        filter_type = scanlines[cursor]
        cursor += 1
        raw = scanlines[cursor : cursor + stride]
        cursor += stride
        reconstructed = bytearray(stride)
        for index, value in enumerate(raw):
            left = reconstructed[index - bytes_per_pixel] if index >= bytes_per_pixel else 0
            above = prior[index]
            upper_left = prior[index - bytes_per_pixel] if index >= bytes_per_pixel else 0
            if filter_type == 0:
                decoded = value
            elif filter_type == 1:
                decoded = value + left
            elif filter_type == 2:
                decoded = value + above
            elif filter_type == 3:
                decoded = value + ((left + above) // 2)
            elif filter_type == 4:
                decoded = value + paeth(left, above, upper_left)
            else:
                raise VerificationError(f"unsupported PNG filter {filter_type} in {path}")
            reconstructed[index] = decoded & 0xFF
        prior = reconstructed
        pixels.extend(
            tuple(reconstructed[index : index + bytes_per_pixel])
            for index in range(0, stride, bytes_per_pixel)
        )
    return width, height, pixels


def hex_to_rgb(hex_color: str) -> tuple[int, int, int]:
    if len(hex_color) != 7 or not hex_color.startswith("#"):
        raise VerificationError(f"invalid constitutional color: {hex_color}")
    try:
        return tuple(int(hex_color[index : index + 2], 16) for index in (1, 3, 5))
    except ValueError as error:
        raise VerificationError(f"invalid constitutional color: {hex_color}") from error


def verify() -> int:
    manifest = load_json(MANIFEST_PATH)
    if manifest.get("schema_version") != "1.0.0":
        raise VerificationError("unsupported art manifest schema")

    palette_record = manifest.get("palette")
    if not isinstance(palette_record, dict):
        raise VerificationError("art manifest is missing its palette record")
    palette_digest = str(palette_record.get("sha256", ""))
    authority_path = str(palette_record.get("authority_path", ""))
    projection_path = str(palette_record.get("packaged_projection_path", ""))
    palette_path = require_digest(authority_path, palette_digest)
    projection_file = require_digest(projection_path, palette_digest)
    if palette_path.read_bytes() != projection_file.read_bytes():
        raise VerificationError("packaged palette projection is not byte-identical to authority")

    palette = load_json(palette_path)
    colors = palette.get("colors")
    if not isinstance(colors, list):
        raise VerificationError("constitutional palette is missing colors")
    palette_by_id = {
        str(record["id"]): hex_to_rgb(str(record["hex"]))
        for record in colors
        if isinstance(record, dict) and "id" in record and "hex" in record
    }
    if len(palette_by_id) != len(colors):
        raise VerificationError("constitutional palette contains malformed or duplicate colors")

    assets = manifest.get("assets")
    if not isinstance(assets, list) or not assets:
        raise VerificationError("art manifest must contain at least one asset")
    asset_ids: set[str] = set()
    for asset in assets:
        if not isinstance(asset, dict):
            raise VerificationError("art manifest asset must be an object")
        asset_id = str(asset.get("asset_id", ""))
        if not asset_id or asset_id in asset_ids:
            raise VerificationError(f"missing or duplicate asset ID: {asset_id}")
        asset_ids.add(asset_id)

        require_digest(str(asset["source_path"]), str(asset["source_sha256"]))
        texture_path = require_digest(str(asset["texture_path"]), str(asset["texture_sha256"]))
        metadata_path = require_digest(str(asset["metadata_path"]), str(asset["metadata_sha256"]))

        generator = asset.get("generated_by")
        if not isinstance(generator, dict):
            raise VerificationError(f"{asset_id} is missing generator provenance")
        require_digest(str(generator["script_path"]), str(generator["script_sha256"]))
        reference = asset.get("reference")
        if not isinstance(reference, dict) or reference.get("shipping") is not False:
            raise VerificationError(f"{asset_id} must mark its generated reference non-shipping")
        require_digest(str(reference["image_path"]), str(reference["image_sha256"]))
        require_digest(str(reference["provenance_path"]), str(reference["provenance_sha256"]))

        width, height, pixels = decode_rgba_png(texture_path)
        if (width, height) != (int(asset["width"]), int(asset["height"])):
            raise VerificationError(f"{asset_id} PNG dimensions do not match the manifest")
        allowed_ids = asset.get("palette_color_ids")
        if not isinstance(allowed_ids, list) or not allowed_ids:
            raise VerificationError(f"{asset_id} has no declared palette colors")
        missing_ids = [color_id for color_id in allowed_ids if color_id not in palette_by_id]
        if missing_ids:
            raise VerificationError(f"{asset_id} names unknown palette colors: {missing_ids}")
        allowed_rgb = {palette_by_id[color_id] for color_id in allowed_ids}
        used_rgb = {(red, green, blue) for red, green, blue, alpha in pixels if alpha > 0}
        if not used_rgb:
            raise VerificationError(f"{asset_id} contains no visible pixels")
        if not used_rgb.issubset(allowed_rgb):
            raise VerificationError(f"{asset_id} contains colors outside its declared palette IDs")

        metadata = load_json(metadata_path)
        frames = metadata.get("frames")
        meta = metadata.get("meta")
        if not isinstance(frames, list) or len(frames) != int(asset["frame_count"]):
            raise VerificationError(f"{asset_id} frame count does not match its Aseprite metadata")
        if not isinstance(meta, dict) or meta.get("format") != "RGBA8888":
            raise VerificationError(f"{asset_id} metadata must declare RGBA8888")
        size = meta.get("size")
        if not isinstance(size, dict) or (size.get("w"), size.get("h")) != (width, height):
            raise VerificationError(f"{asset_id} metadata dimensions do not match its PNG")
        tags = meta.get("frameTags")
        actual_tags = {
            str(tag.get("name")) for tag in tags if isinstance(tag, dict) and tag.get("name")
        } if isinstance(tags, list) else set()
        required_tags = set(asset.get("required_tags", []))
        if not required_tags.issubset(actual_tags):
            raise VerificationError(f"{asset_id} is missing required tags: {required_tags - actual_tags}")

    print(f"art asset verification passed: {len(assets)} asset(s), {len(palette_by_id)} colors")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(verify())
    except (KeyError, TypeError, ValueError, VerificationError) as error:
        print(f"art asset verification failed: {error}", file=sys.stderr)
        raise SystemExit(1) from error
