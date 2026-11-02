"""Experimental LSP origin adapter around the actual Cinch language server.

The server checks expanded documents. Editors see original sources and positions.
Only requests supported by the native server are advertised; rename is not
inferred from a potentially one-to-many origin mapping.
"""
from pathlib import Path
import copy
import hashlib
import json
import re
import subprocess
import sys
import tempfile
from urllib.parse import unquote, urlparse

from engine import Engine, SyntaxFailure, utf16_position, utf16_offset
from poc import HERE, compiler


def read_frame(stream):
    headers = {}
    while True:
        line = stream.readline()
        if not line: return None
        if line in (b"\r\n", b"\n"): break
        key, value = line.decode().split(":", 1)
        headers[key.lower()] = value.strip()
    length = int(headers["content-length"])
    chunks = bytearray()
    while len(chunks) < length:
        chunk = stream.read(length - len(chunks))
        if not chunk: raise EOFError("incomplete LSP frame")
        chunks.extend(chunk)
    return json.loads(chunks)


def write_frame(stream, message):
    data = json.dumps(message, ensure_ascii=False).encode()
    stream.write(f"Content-Length: {len(data)}\r\n\r\n".encode() + data)
    stream.flush()


def native_offset(text, position):
    lines = text.splitlines(keepends=True)
    return sum(len(s) for s in lines[:position["line"]]) + position["character"]


def native_position(text, offset):
    before = text[:offset]
    return {"line": before.count("\n"), "character": len(before.rsplit("\n", 1)[-1])}


class Bridge:
    def __init__(self):
        self.workspace = tempfile.TemporaryDirectory(prefix="cinch-syntax-lsp-")
        self.root = Path(self.workspace.name)
        self.server = subprocess.Popen([compiler(), "lsp"], stdin=subprocess.PIPE,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        self.originals, self.generated, self.expansions, self.documents = {}, {}, {}, {}
        self.serial = 0

    def close(self):
        if self.server.poll() is None:
            self.server.terminate()
        self.server.wait(timeout=10)
        for stream in (self.server.stdin, self.server.stdout, self.server.stderr): stream.close()
        self.workspace.cleanup()

    def native(self, message, barrier=False):
        write_frame(self.server.stdin, message)
        if barrier:
            self.serial += 1
            target = f"syntax-barrier-{self.serial}"
            write_frame(self.server.stdin, {"jsonrpc": "2.0", "id": target,
                "method": "textDocument/hover", "params": {
                    "textDocument": {"uri": (self.root / "__barrier.cinch").as_uri()},
                    "position": {"line": 0, "character": 0}}})
        elif "id" in message: target = message["id"]
        else: return []
        messages = []
        while True:
            response = read_frame(self.server.stdout)
            if response is None: raise RuntimeError("native Cinch LSP stopped")
            if response.get("id") == target:
                if not barrier: messages.append(response)
                return messages
            messages.append(response)

    def map_position(self, generated_uri, position):
        original = self.originals.get(generated_uri)
        expansion = self.expansions.get(original)
        if expansion is None: return position
        offset = native_offset(expansion.fragment.text, position)
        origins = expansion.fragment.origins
        origin = origins[min(offset, len(origins) - 1)] if origins else 0
        line, char = utf16_position(expansion.source, origin)
        return {"line": line, "character": char}

    def map_value(self, value, uri=None):
        if isinstance(value, list): return [self.map_value(v, uri) for v in value]
        if not isinstance(value, dict): return value
        uri = value.get("uri", uri)
        result = {}
        for key, child in value.items():
            if key == "uri": result[key] = self.originals.get(child, child)
            elif key in ("start", "end") and isinstance(child, dict) and "line" in child:
                result[key] = self.map_position(uri, child)
            else: result[key] = self.map_value(child, uri)
        if "start" in result and "end" in result:
            start, end = result["start"], result["end"]
            if (end["line"], end["character"]) <= (start["line"], start["character"]):
                result["end"] = {"line": start["line"], "character": start["character"] + 1}
        return result

    def semantic_tokens(self, data, original):
        expansion = self.expansions.get(original)
        if expansion is None: return []
        line, char, tokens = 0, 0, {}
        for i in range(0, len(data), 5):
            dl, dc, length, kind, mods = data[i:i + 5]
            line += dl; char = char + dc if dl == 0 else dc
            start = native_offset(expansion.fragment.text, {"line": line, "character": char})
            end = start + length
            origin = expansion.fragment.origins[start:end]
            copied = expansion.fragment.copied[start:end]
            if not origin or not all(copied) or origin != list(range(origin[0], origin[0] + len(origin))): continue
            sl, sc = utf16_position(expansion.source, origin[0])
            el, ec = utf16_position(expansion.source, origin[-1] + 1)
            if sl == el: tokens[(sl, sc, ec - sc)] = (kind, mods)
        encoded, last_line, last_char = [], 0, 0
        for (line, char, length), (kind, mods) in sorted(tokens.items()):
            encoded.extend([line - last_line, char - last_char if line == last_line else char,
                            length, kind, mods])
            last_line, last_char = line, char
        return encoded

    def mapped(self, messages, request_uri=None, semantic=False):
        output = []
        for message in messages:
            if semantic and "result" in message and isinstance(message["result"], dict) and "data" in message["result"]:
                message["result"]["data"] = self.semantic_tokens(message["result"]["data"], request_uri)
            output.append(self.map_value(message, self.generated.get(request_uri)))
        return output

    def handle(self, message):
        method = message.get("method")
        request = copy.deepcopy(message)
        params = request.setdefault("params", {})
        if method == "initialize":
            params["rootUri"] = self.root.as_uri()
            responses = self.native(request)
            # The adapter deliberately promises only full synchronization.
            return responses
        if method in ("initialized", "shutdown", "exit", "workspace/didChangeConfiguration"):
            return self.native(request)
        document = params.get("textDocument", {})
        uri = document.get("uri")
        if method in ("textDocument/didOpen", "textDocument/didChange"):
            source = document["text"] if method.endswith("didOpen") else params["contentChanges"][-1]["text"]
            self.documents[uri] = source
            path = Path(unquote(urlparse(uri).path))
            generated_path = self.root / (hashlib.sha256(uri.encode()).hexdigest()[:12] + "-" + path.name)
            generated_uri = generated_path.as_uri()
            self.generated[uri] = generated_uri; self.originals[generated_uri] = uri
            # Open actual library source so the native server resolves its
            # symbols through its normal shared analysis engine.
            notifications = []
            library_index = {p.stem: p for p in (HERE / "libraries").glob("*.cinch")}
            pending = re.findall(r"(?m)^\s*import ([A-Z][\w.]*)", source)
            needed = {}
            while pending:
                module = pending.pop()
                if module in needed or module not in library_index: continue
                library = library_index[module]
                needed[module] = library
                pending.extend(re.findall(r"(?m)^\s*import ([A-Z][\w.]*)", library.read_text()))
            for library in sorted(needed.values()):
                target = self.root / library.name
                if target.as_uri() in self.originals: continue
                text = library.read_text(); target.write_text(text)
                self.originals[target.as_uri()] = library.as_uri()
                notifications += self.native({"jsonrpc": "2.0", "method": "textDocument/didOpen",
                    "params": {"textDocument": {"uri": target.as_uri(), "languageId": "cinch", "version": 1, "text": text}}}, barrier=True)
            try:
                expansion = Engine(source, path).expand()
                self.expansions[uri] = expansion
            except (SyntaxFailure, ValueError, OSError) as error:
                self.expansions.pop(uri, None)
                notifications += self.native({"jsonrpc": "2.0", "method": "textDocument/didClose",
                    "params": {"textDocument": {"uri": generated_uri}}}, barrier=True)
                pos = getattr(error, "position", 0)
                line, char = utf16_position(source, pos)
                notifications = self.mapped(notifications)
                notifications.append({"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                    "params": {"uri": uri, "diagnostics": [{"range": {"start": {"line": line, "character": char},
                        "end": {"line": line, "character": char + 1}}, "severity": 1,
                        "source": "cinch-syntax", "message": str(error)}]}})
                return notifications
            generated_path.write_text(expansion.fragment.text)
            # didOpen replaces a document in the native analysis, including
            # reopening after an unfinished island closed it.
            native_open = {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {
                "uri": generated_uri, "languageId": "cinch", "version": document.get("version", 1), "text": expansion.fragment.text}}}
            notifications += self.native(native_open, barrier=True)
            return self.mapped(notifications)
        if uri:
            document["uri"] = self.generated.get(uri, uri)
            if "position" in params:
                expansion = self.expansions.get(uri)
                generated = expansion.generated_position(params["position"]["line"], params["position"]["character"]) if expansion else None
                if generated is None:
                    return [{"jsonrpc": "2.0", "id": message.get("id"), "result": None}] if "id" in message else []
                offset = utf16_offset(expansion.fragment.text, *generated)
                params["position"] = native_position(expansion.fragment.text, offset)
            if method == "textDocument/didClose":
                result = self.native(request, barrier=True)
                self.expansions.pop(uri, None)
            else: result = self.native(request)
            return self.mapped(result, uri, method == "textDocument/semanticTokens/full")
        return self.native(request)


def serve():
    bridge = Bridge()
    try:
        while (message := read_frame(sys.stdin.buffer)) is not None:
            for response in bridge.handle(message): write_frame(sys.stdout.buffer, response)
            if message.get("method") == "exit": break
    finally: bridge.close()
