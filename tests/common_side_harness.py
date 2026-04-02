from __future__ import annotations

import contextlib
import json
import os
import subprocess
import tempfile
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Iterator


ROOT = Path(__file__).resolve().parents[1]
SITE_CONTRACT_FILES = (
    "assets/route-manifest.json",
    "assets/theme-tokens.json",
    "assets/runtime-contract.json",
    "docs/docs-index.json",
)


def run_checked(
    command: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)

    result = subprocess.run(
        command,
        cwd=str(cwd or ROOT),
        env=merged_env,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            "command failed: "
            + " ".join(command)
            + f"\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def read_json(path: Path) -> object:
    return json.loads(path.read_text(encoding="utf-8"))


def assert_site_contract_parity(left_root: Path, right_root: Path) -> None:
    for relative_path in SITE_CONTRACT_FILES:
        left = left_root / relative_path
        right = right_root / relative_path
        if not left.is_file():
            raise AssertionError(f"missing site contract file: {left}")
        if not right.is_file():
            raise AssertionError(f"missing site contract file: {right}")
        if read_json(left) != read_json(right):
            raise AssertionError(f"site contract drift detected for {relative_path}")


@contextlib.contextmanager
def temporary_directory(prefix: str) -> Iterator[Path]:
    path = Path(tempfile.mkdtemp(prefix=prefix))
    try:
        yield path
    finally:
        for child in sorted(path.rglob("*"), reverse=True):
            if child.is_file() or child.is_symlink():
                child.unlink()
            elif child.is_dir():
                child.rmdir()
        path.rmdir()


@contextlib.contextmanager
def mock_release_api_server(payload: dict) -> Iterator[str]:
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:  # noqa: N802
            if self.path.startswith("/repos/") and "/releases/" in self.path:
                body = json.dumps(payload).encode("utf-8")
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return

            self.send_response(404)
            self.end_headers()

        def log_message(self, format: str, *args: object) -> None:  # noqa: A003
            return

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        host, port = server.server_address
        yield f"http://{host}:{port}"
    finally:
        server.shutdown()
        thread.join(timeout=5)
        server.server_close()
