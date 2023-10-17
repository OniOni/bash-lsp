from typing import Any

from .protocol import LSPConnection, LSPRequest, JsonRpcRequest, NeedData

class LSPClient:

    def __init__(self, stdin, stdout):
        self._conn = LSPConnection()
        self.stdin = stdin
        self.stdout = stdout

    def call(self, method: str, args: dict[str, Any]):
        msg = self._conn.send(LSPRequest(
            body=JsonRpcRequest(
                method="test", params={
                    "test": True
                }
            )
        ))
        self.stdin.write(msg)
        self.stdin.flush()

        while (next_event := self._conn.next_event()) == NeedData:
            b = self.stdout.read(1)
            self._conn.receive_bytes(b)

        return next_event[0]
