import io
import sys
import subprocess
import os

from .client import LSPClient

def readline(pipe):
    line = b""
    while (c := pipe.read(1)) != b"\n":
        line += c

    return line + b"\n"

def main():
    cmd = [f"{os.getcwd()}/target/debug/bash-lsp"]
    # cmd = [f"{os.getcwd()}/echo/target/debug/echo"]

    print(f"Starting process {cmd}")

    p = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
        bufsize=0,
        stderr=sys.stderr,
    )

    client = LSPClient(p.stdin, p.stdout)
    resp = client.call(
        method="test",
        args={
            "hello": "world"
        }
    )

    p.kill()

    print(resp)
    print(resp.jsonrpc())


if __name__ == "__main__":
    main()
