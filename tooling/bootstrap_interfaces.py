"""Structural and dependency checks for genuine self-produced schema 10 files.

These checks supplement the checked writer and independent compiler-consumer
regressions. They do not infer type semantics from display strings.
"""
from __future__ import annotations

import json
from pathlib import Path


def canonical(doc: dict) -> str:
    return json.dumps(doc, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":"), allow_nan=False)


def content_hash(data: bytes) -> str:
    result = 14695981039346656037
    for byte in data:
        result = ((result ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return str(result)


def interface_hash(doc: dict) -> str:
    return content_hash(canonical({**doc, "compiler": "", "source_hash": "", "units": []}).encode())


def read_contract(path: Path) -> dict:
    def unique(pairs):
        result = {}
        for name, value in pairs:
            if name in result:
                raise ValueError(f"duplicate JSON key: {name}")
            result[name] = value
        return result

    text = path.read_text()
    doc = json.loads(text, object_pairs_hook=unique)
    if not isinstance(doc, dict) or canonical(doc) != text:
        raise ValueError("interface is not a canonical JSON object")
    return doc


def _text(value):
    if not isinstance(value, str) or not value:
        raise ValueError("missing nonempty contract text")


def _list(value):
    if not isinstance(value, list):
        raise ValueError("missing contract list")
    return value


def _integer(value):
    if type(value) is not int or value < 0:
        raise ValueError("invalid nonnegative contract integer")


def _parameters(row):
    params = _list(row["params"])
    parameters = _list(row["parameters"])
    if len(params) != len(set(params)) or params != [p["name"] for p in parameters]:
        raise ValueError("declaration parameter identities disagree")
    for p in parameters:
        _text(p["name"])
        _text(p["kind"])
        if type(p["implicit"]) is not bool:
            raise ValueError("invalid declaration parameter visibility")


def _resolved(row):
    # Null is an explicit non-portable contract, never permission to
    # reconstruct an effect/dictionary declaration from its display scheme.
    resolved = row["resolved"]
    if resolved is None:
        return
    _text(resolved["scheme"])
    names = set()
    for nominal in _list(resolved["nominals"]):
        identity = nominal["identity"]
        _text(identity)
        if "." not in identity or identity in names:
            raise ValueError("duplicate or unowned defining nominal")
        names.add(identity)
        _parameters(nominal)
        kind = nominal["kind"]
        if kind not in ("data", "opaque", "signature", "refinement"):
            raise ValueError("unknown defining nominal category")
        if kind == "data":
            for constructor in _list(nominal["constructors"]):
                _text(constructor)
                if not constructor.startswith(identity.rsplit(".", 1)[0] + "."):
                    raise ValueError("constructor has another defining module")
        elif nominal["constructors"] is not None:
            raise ValueError("abstract nominal has constructor identities")
        if kind != "refinement" and nominal["refinement"] is not None:
            raise ValueError("unexpected defining refinement")
    _list(resolved["measures"])


def _law_syntax(syntax):
    parts = _list(syntax)
    if not parts:
        raise ValueError("empty law syntax")
    tag, *items = parts
    if tag == "LawForall" and len(items) == 2:
        binders, body = items
        names = set()
        if not _list(binders):
            raise ValueError("empty law quantifier")
        for binder in binders:
            name, implicit, kind = _list(binder)
            _text(name)
            if name in names or type(implicit) is not bool:
                raise ValueError("invalid law binder")
            names.add(name)
            if kind is not None:
                _list(kind)
        _law_syntax(body)
    elif tag == "LawArrow" and len(items) == 3:
        name, carrier, body = items
        if name is not None:
            _text(name)
        _list(carrier)
        _law_syntax(body)
    elif tag == "LawEquation" and len(items) == 2:
        for operand in items:
            _list(operand)
    else:
        raise ValueError("invalid class law syntax")


def validate_contract(doc: dict, runtime_abi: int) -> None:
    fields = {"schema", "runtime_abi", "language", "compiler", "module", "source_hash",
              "imports", "params", "exports", "abi", "assumptions", "assertions",
              "contracts", "laws", "must_use", "upgrades", "units"}
    if set(doc) != fields:
        raise ValueError("incomplete or unknown interface fields")
    if type(doc["schema"]) is not int or doc["schema"] != 10:
        raise ValueError("wrong interface schema")
    if type(doc["runtime_abi"]) is not int or doc["runtime_abi"] != runtime_abi:
        raise ValueError("wrong runtime ABI")
    if doc["language"] != "draft-0.2+amendments":
        raise ValueError("wrong language contract")
    for key in ("compiler", "module", "source_hash"):
        _text(doc[key])
    if not doc["source_hash"].isascii() or not doc["source_hash"].isdecimal() or int(doc["source_hash"]) >= 1 << 64:
        raise ValueError("invalid source identity")
    for key in ("imports", "params", "assertions", "contracts", "laws", "must_use", "units", "upgrades"):
        _list(doc[key])
    for key in ("primitives", "foreign", "declared"):
        _list(doc["assumptions"][key])
    abi = doc["abi"]
    if abi["convention"] != "uniform-word, curried apply stub":
        raise ValueError("wrong calling convention")
    _list(abi["symbols"])
    _list(abi["layouts"])
    exports = doc["exports"]
    for key in ("values", "types", "signatures", "instances", "fixities", "foreign"):
        _list(exports[key])
    public = set()
    for value in exports["values"]:
        for key in ("name", "symbol", "scheme"):
            _text(value[key])
        if "." not in value["symbol"] or value["name"] in public:
            raise ValueError("duplicate or unowned exported value")
        public.add(value["name"])
        _integer(value["arity"])
        _resolved(value)
        if "total_body" not in value:
            raise ValueError("missing executable contract boundary")
        if "keyed_ordering" in value:
            ordering = value["keyed_ordering"]
            if (not isinstance(ordering, dict) or set(ordering) != {"less_slot", "greater_slot"}
                    or value["total_body"] is not None or value["resolved"] is None or value["arity"] != 4):
                raise ValueError("invalid keyed ordering value body")
            _integer(ordering["less_slot"])
            _integer(ordering["greater_slot"])
            if ordering["less_slot"] == ordering["greater_slot"]:
                raise ValueError("keyed ordering body must read two distinct slots")
    if abi["symbols"] != sorted(f'{v["name"]}={v["symbol"]}' for v in exports["values"]):
        raise ValueError("ABI symbol table disagrees with checked exports")
    for declaration in exports["types"]:
        _text(declaration["name"])
        public.add(declaration["name"])
        _text(declaration["identity"])
        if "." not in declaration["identity"]:
            raise ValueError("unowned exported type")
        _parameters(declaration)
        labels = _list(declaration["fields"])
        if declaration["kind"] not in ("data", "opaque", "signature", "record", "codata", "refinement"):
            raise ValueError("unknown exported type category")
        if "refinement" not in declaration:
            raise ValueError("missing refinement boundary")
        constructors = _list(declaration["ctors"])
        codata = declaration["kind"] == "codata"
        if codata and (len(constructors) != 1 or constructors[0]["resolved"] is None):
            raise ValueError("codata must publish its checked observation contract")
        if declaration["kind"] in ("record", "codata"):
            for label in labels:
                _text(label)
            if len(set(labels)) != len(labels):
                raise ValueError("record field labels must be nonempty and unique")
        for ctor in constructors:
            for key in ("name", "symbol", "scheme"):
                _text(ctor[key])
            _integer(ctor["tag"])
            if not codata:
                public.add(ctor["name"])
            fields, erased = _list(ctor["fields"]), _list(ctor["erased"])
            if len(fields) != len(erased) or any(type(flag) is not bool for flag in erased):
                raise ValueError("constructor field/erasure contract disagrees")
            if declaration["kind"] in ("record", "codata") and (
                    ctor["symbol"] != declaration["identity"] or ctor["tag"] != 0 or len(fields) != len(labels)):
                raise ValueError("record labels disagree with its constructor contract")
            _resolved(ctor)
    for signature in exports["signatures"]:
        _text(signature["identity"])
        _parameters(signature)
        _list(signature["exports"])
        public.update(signature["exports"])
        law_names = set()
        for law in _list(signature["laws"]):
            if type(law) is not dict or set(law) != {"name", "syntax"}:
                raise ValueError("invalid law declaration")
            _text(law["name"])
            if law["name"] in law_names:
                raise ValueError("duplicate law declaration")
            law_names.add(law["name"])
            if law["syntax"] is not None:
                _law_syntax(law["syntax"])
        slots = set()
        for method in _list(signature["methods"]):
            required = {"name", "slot", "exports", "scheme", "resolved"}
            if not required <= method.keys() <= required | {"default"}:
                raise ValueError("unknown or missing method contract field")
            _text(method["name"])
            _text(method["scheme"])
            _list(method["exports"])
            public.update(method["exports"])
            _integer(method["slot"])
            if method["slot"] in slots:
                raise ValueError("duplicate signature slot")
            slots.add(method["slot"])
            _resolved(method)
            resolved = method["resolved"]
            if resolved is not None and method["scheme"] != resolved["scheme"]:
                raise ValueError("class method spelling disagrees with its checked scheme")
            if "default" in method:
                clauses = _list(method["default"])
                if not clauses or any(len(_list(clause)) != 2 or not all(isinstance(part, list) for part in clause)
                                      for clause in clauses):
                    raise ValueError("invalid class default clauses")
    for instance in exports["instances"]:
        for key in ("id", "class", "target"):
            _text(instance[key])
        for key in ("ctx", "methods", "method_contracts"):
            _list(instance[key])
        if type(instance["implicit"]) is not bool or "resolved" not in instance:
            raise ValueError("missing checked instance contract")
        for method in instance["method_contracts"]:
            if type(method) is not dict:
                raise ValueError("invalid conditional method")
            checked = method.get("checked")
            if type(checked) is not dict or type(checked.get("schema")) is not int or checked["schema"] != 1:
                raise ValueError("missing checked conditional method")
            head, implementation = checked["head"], checked["implementation"]
            for declaration in (head, implementation):
                _resolved(declaration)
                if declaration["resolved"] is None:
                    raise ValueError("unresolved conditional method")
            _text(implementation["identity"])
            if "body" in implementation and type(implementation["body"]) is not dict:
                raise ValueError("invalid guarded body")
        if "method_bodies" in instance and not _list(instance["method_bodies"]):
            raise ValueError("empty method_bodies must be omitted")
    operators = set()
    for fixity in exports["fixities"]:
        _text(fixity["op"])
        if fixity["op"] in operators or fixity["op"] not in public:
            raise ValueError("duplicate or nonpublic fixity")
        operators.add(fixity["op"])
        if fixity["assoc"] not in ("left", "right", "none") or type(fixity["prec"]) is not int or not 0 <= fixity["prec"] <= 9:
            raise ValueError("invalid fixity")


def verify_dependencies(documents: dict[str, dict], base: dict[str, dict]) -> None:
    available = {**base, **documents}
    for name, doc in documents.items():
        seen = set()
        for dependency in doc["imports"]:
            owner = dependency["module"]
            if owner == name or owner in seen or owner not in available:
                raise ValueError(f"{name}: duplicate, missing or self dependency: {owner}")
            seen.add(owner)
            if dependency["iface_hash"] != interface_hash(available[owner]):
                raise ValueError(f"{name}: stale dependency contract: {owner}")
