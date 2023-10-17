from dataclasses import asdict, dataclass, field
import json
from typing import Any
import uuid


@dataclass
class JsonRpcBase:

    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    jsonrpc: float = 2.0

    def to_json(self):
        return json.dumps(asdict(self))


@dataclass
class JsonRpcRequest(JsonRpcBase):

    method: str = ""
    params: dict[str, Any] = None


@dataclass
class JsonRpcResponse(JsonRpcBase):

    result: str = ""

    @classmethod
    def from_string(cls, raw_body: bytes):
        doc = json.loads(raw_body.decode())
        return cls(id=doc["id"], result=doc["result"])


class LSPEvent:
    pass

class NeedData(LSPEvent):
    pass


@dataclass
class BaseLSPNetworkEvent(LSPEvent):

    body: dict[str, Any] = None
    raw_body: str = None
    headers: dict[str, str] = None

    def jsonrpc(self):
        if self.body:
            return self.body

        doc = json.loads(self.raw_body.decode())
        if "result" in doc or "error" in doc:
            return JsonRpcResponse(**doc)
        elif "method" in doc:
            return JsonRpcRequest(**doc)

        raise Exception(f"Could not process {self.raw_body}")

class LSPRequest(BaseLSPNetworkEvent):
    pass


class LSPResponse(BaseLSPNetworkEvent):
    pass

class LSPConnection:

    def __init__(self):
        self._buffer = []
        self._cur_event = NeedData
        self._next_event = None


    @staticmethod
    def _process_headers(buff):
        ret = {}
        txt = b"".join(buff)

        for line in txt.split(b"\r\n"):
            if not line:
                continue

            key, value = line.split(b":")
            ret[key] = value.strip()

        return ret

    def next_event(self):
        ret = self._cur_event
        if self._cur_event != NeedData:
            self._cur_event = NeedData

        return ret

    def send(self, req: LSPRequest) -> bytes:
        raw_body = req.body.to_json().encode()

        return f"Content-Length: {len(raw_body)}\r\n\r\n".encode() + raw_body

    def receive_bytes(self, data: bytes):
        if not self._next_event:
            self._next_event = [LSPResponse(), "headers"]

        for i in data:
            b = i.to_bytes(length=1, byteorder='big')
            self._buffer.append(b)
            if b == b"\n":
                tail = b"".join(self._buffer[-4:])
                if tail == b"\r\n\r\n":
                    self._next_event[1] = "body"
                    self._next_event[0].headers = self._process_headers(self._buffer)

                    self._buffer = []

            else:
                if self._next_event[1] == "body":
                    if b"Content-Length" not in self._next_event[0].headers:
                        raise Exception("Cannot Proceed")

                    if len(self._buffer) == int(self._next_event[0].headers[b"Content-Length"]):
                        self._next_event[0].raw_body = b"".join(self._buffer)
                        self._cur_event = self._next_event
                        self._next_event = None
