"""Pretty-printer and type-hyperlinking HTML renderer for Cinch signatures.

Transforms canonical `.cinchif` type schemes such as:
  forall (e :: Type) (a :: Type) (b :: Type). ((e -> b) -> ((a -> b) -> ((Data.Core.Result e a) -> b)))
into Hackage-readable, hyperlinked signatures:
  forall e a b. (e -> b) -> (a -> b) -> Result e a -> b
while preserving full canonical module identities in link targets and tooltips.
"""
from __future__ import annotations

import html
import re
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple


@dataclass
class Tok:
    kind: str
    val: str


def short_name(name: str) -> str:
    return name.rsplit('.', 1)[-1]


def tokenize(text: str) -> List[Tok]:
    out: List[Tok] = []
    n = len(text)
    i = 0
    while i < n:
        ch = text[i]
        if ch.isspace():
            i += 1
            continue
        if ch in '(),.':
            out.append(Tok(ch, ch))
            i += 1
            continue
        if ch == ':' and i + 1 < n and text[i + 1] == ':':
            out.append(Tok('::', '::'))
            i += 2
            continue
        if ch == '=' and i + 1 < n and text[i + 1] == '>':
            out.append(Tok('=>', '=>'))
            i += 2
            continue
        if ch == '-' and i + 1 < n and text[i + 1] == '>':
            i += 2
            if i < n and text[i] == '{':
                start = i + 1
                depth = 1
                i += 1
                while i < n and depth > 0:
                    if text[i] == '{':
                        depth += 1
                    elif text[i] == '}':
                        depth -= 1
                    i += 1
                end = i - 1 if depth == 0 else i
                out.append(Tok('->eff', text[start:end].strip()))
            else:
                out.append(Tok('->', '->'))
            continue
        if ch == '%':
            start = i
            i += 1
            while i < n and (text[i].isalnum() or text[i] == '_'):
                i += 1
            out.append(Tok('mult', text[start:i]))
            continue
        if ch == '{':
            start = i
            depth = 1
            i += 1
            while i < n and depth > 0:
                if text[i] == '{':
                    depth += 1
                elif text[i] == '}':
                    depth -= 1
                i += 1
            out.append(Tok('other', text[start:i]))
            continue
        if ch == '[':
            start = i
            depth = 1
            i += 1
            while i < n and depth > 0:
                if text[i] == '[':
                    depth += 1
                elif text[i] == ']':
                    depth -= 1
                i += 1
            out.append(Tok('other', text[start:i]))
            continue
        if ch.isalpha() or ch == '_':
            start = i
            starts_upper = ch.isupper()
            i += 1
            while i < n:
                c = text[i]
                if c.isalnum() or c in "_'":
                    i += 1
                elif c == '-' and i + 1 < n and (text[i + 1].isalnum() or text[i + 1] == '_'):
                    i += 1
                elif starts_upper and c == '.' and i + 1 < n and (text[i + 1].isalpha() or text[i + 1] == '_'):
                    i += 1
                else:
                    break
            word = text[start:i]
            if word in ('forall', 'using', 'Sigma', 'refine', 'with'):
                out.append(Tok(word, word))
            else:
                out.append(Tok('ident', word))
            continue
        if ch.isdigit():
            start = i
            while i < n and (text[i].isalnum() or text[i] == '_'):
                i += 1
            out.append(Tok('num', text[start:i]))
            continue
        start = i
        i += 1
        while i < n and not text[i].isspace() and text[i] not in '(),.{}[]' and not text[i].isalnum() and text[i] != '_':
            if (text[i] == '-' and i + 1 < n and text[i + 1] == '>') or \
               (text[i] == '=' and i + 1 < n and text[i + 1] == '>') or \
               (text[i] == ':' and i + 1 < n and text[i + 1] == ':'):
                break
            i += 1
        out.append(Tok('other', text[start:i]))
    return out


def _join_raw(parts: List[str]) -> str:
    out: List[str] = []
    for idx, p in enumerate(parts):
        if idx > 0 and parts[idx - 1] != '(' and p not in (')', ',', '.'):
            out.append(' ')
        out.append(p)
    return ''.join(out)


class _Parser:
    def __init__(self, toks: List[Tok]):
        self.toks = toks
        self.pos = 0

    def peek(self, offset: int = 0) -> Optional[Tok]:
        idx = self.pos + offset
        return self.toks[idx] if idx < len(self.toks) else None

    def next(self) -> Optional[Tok]:
        if self.pos < len(self.toks):
            t = self.toks[self.pos]
            self.pos += 1
            return t
        return None

    def parse(self):
        res = self.parse_type()
        if res is None or self.pos < len(self.toks):
            return None
        return res

    def parse_type(self):
        p = self.peek()
        if p is None:
            return None
        if p.kind == 'forall':
            return self.parse_forall()
        if p.kind == 'using':
            return self.parse_using()
        if p.kind == 'Sigma':
            return self.parse_sigma()
        if p.kind == 'refine':
            return self.parse_refine()
        return self.parse_qual_or_arrow()

    def parse_forall(self):
        self.next()
        binders = []
        while self.peek():
            p = self.peek()
            if p.kind == '.':
                self.next()
                break
            if p.kind == '(':
                save = self.pos
                self.next()
                t1 = self.next()
                if t1 and t1.kind == 'ident' and self.peek() and self.peek().kind == '::':
                    self.next()
                    kind_ty = self.parse_type()
                    if kind_ty is not None and self.peek() and self.peek().kind == ')':
                        self.next()
                        binders.append((t1.val, kind_ty))
                        continue
                self.pos = save
                break
            if p.kind == 'ident':
                self.next()
                binders.append((p.val, None))
                continue
            break
        body = self.parse_type()
        if body is None:
            return None
        return ('forall', binders, body) if binders else body

    def parse_using(self):
        self.next()
        constraints = []
        if self.peek() and self.peek().kind == '(':
            self.next()
            while self.peek() and self.peek().kind != ')':
                p0 = self.peek(0)
                p1 = self.peek(1)
                binder = None
                if p0 and p0.kind == 'ident' and p1 and p1.kind == '::':
                    binder = p0.val
                    self.next()
                    self.next()
                ty = self.parse_type()
                if ty is None:
                    break
                constraints.append((binder, ty))
                if self.peek() and self.peek().kind == ',':
                    self.next()
                else:
                    break
            if self.peek() and self.peek().kind == ')':
                self.next()
        if self.peek() and self.peek().kind == '=>':
            self.next()
        body = self.parse_type()
        if body is None:
            return None
        return ('qual', constraints, body) if constraints else body

    def parse_sigma(self):
        self.next()
        name = '_'
        ty = ('con', 'Type')
        if self.peek() and self.peek().kind == '(':
            self.next()
            t = self.next()
            if t and t.kind == 'ident':
                name = t.val
            if self.peek() and self.peek().kind == '::':
                self.next()
                ty = self.parse_type() or ty
            if self.peek() and self.peek().kind == ')':
                self.next()
        if self.peek() and self.peek().kind == '.':
            self.next()
        body = self.parse_type()
        if body is None:
            return None
        return ('sigma', name, ty, body)

    def parse_refine(self):
        self.next()
        binder = 'x'
        carrier = ('var', 'a')
        if self.peek() and self.peek().kind == '(':
            self.next()
            t = self.next()
            if t and t.kind == 'ident':
                binder = t.val
            if self.peek() and self.peek().kind == '::':
                self.next()
                carrier = self.parse_type() or carrier
            if self.peek() and self.peek().kind == ')':
                self.next()
        if self.peek() and self.peek().kind == 'with':
            self.next()
        parts: List[str] = []
        depth = 0
        while self.peek():
            t = self.peek()
            if depth == 0 and t.kind in (')', ','):
                break
            tok = self.next()
            if tok.kind == '(':
                depth += 1
                parts.append('(')
            elif tok.kind == ')':
                depth = max(0, depth - 1)
                parts.append(')')
            elif tok.kind == '->eff':
                parts.append(f'->{{{tok.val}}}')
            else:
                parts.append(tok.val)
        return ('refine', binder, carrier, _join_raw(parts))

    def parse_qual_or_arrow(self):
        if self.peek() and self.peek().kind == '(' and self.peek(1) and self.peek(1).kind == 'ident' and self.peek(2) and self.peek(2).kind == '::':
            save = self.pos
            self.next()
            name_tok = self.next()
            self.next()
            from_ty = self.parse_type()
            if from_ty is not None and self.peek() and self.peek().kind == ')':
                self.next()
                mult = None
                if self.peek() and self.peek().kind == 'mult':
                    mult = self.next().val
                arr = self.peek()
                if arr and arr.kind in ('->', '->eff'):
                    self.next()
                    eff = arr.val if arr.kind == '->eff' else None
                    to_ty = self.parse_type()
                    if to_ty is not None:
                        return ('arrow', name_tok.val, mult, eff, from_ty, to_ty)
            self.pos = save

        lhs = self.parse_app()
        if lhs is None:
            return None

        if self.peek() and self.peek().kind == '=>':
            self.next()
            body = self.parse_type()
            if body is None:
                return None
            if lhs[0] == 'tuple':
                cs = [(None, e) for e in lhs[1]]
            else:
                cs = [(None, lhs)]
            return ('qual', cs, body)

        mult = None
        if self.peek() and self.peek().kind == 'mult':
            mult = self.next().val

        arr = self.peek()
        if arr and arr.kind in ('->', '->eff'):
            self.next()
            eff = arr.val if arr.kind == '->eff' else None
            rhs = self.parse_type()
            if rhs is None:
                return None
            return ('arrow', None, mult, eff, lhs, rhs)

        return lhs

    def parse_app(self):
        items = []
        while True:
            atom = self.parse_atom()
            if atom is None:
                break
            items.append(atom)
        if not items:
            return None
        if len(items) == 1:
            return items[0]
        return ('app', items)

    def parse_atom(self):
        p = self.peek()
        if p is None:
            return None
        if p.kind == '(':
            self.next()
            if self.peek() and self.peek().kind == ')':
                self.next()
                return ('con', 'Unit')
            first = self.parse_type()
            if first is None:
                return None
            if self.peek() and self.peek().kind == ',':
                elems = [first]
                while self.peek() and self.peek().kind == ',':
                    self.next()
                    nxt = self.parse_type()
                    if nxt is None:
                        return None
                    elems.append(nxt)
                if self.peek() and self.peek().kind == ')':
                    self.next()
                return ('tuple', elems)
            if self.peek() and self.peek().kind == ')':
                self.next()
            return first
        if p.kind == 'ident':
            self.next()
            return ('con', p.val) if p.val[0].isupper() else ('var', p.val)
        if p.kind in ('num', 'other'):
            self.next()
            return ('raw', p.val)
        return None


def _needs_parens(node, ctx: str) -> bool:
    tag = node[0]
    if ctx == 'top':
        return False
    if ctx == 'arrow_lhs':
        return tag in ('forall', 'qual', 'arrow', 'sigma', 'refine')
    if ctx == 'app_arg':
        return tag in ('forall', 'qual', 'arrow', 'sigma', 'refine', 'app')
    return False


def _is_plain_type(node) -> bool:
    return node[0] == 'con' and node[1] == 'Type'


def format_plain(node, ctx: str = 'top', shorten: bool = True) -> str:
    tag = node[0]
    if tag == 'forall':
        _, binders, body = node
        parts = []
        for name, kind in binders:
            if kind is not None and not _is_plain_type(kind):
                parts.append(f'({name} :: {format_plain(kind, "top", shorten)})')
            else:
                parts.append(name)
        inner = f'forall {" ".join(parts)}. {format_plain(body, "top", shorten)}'
    elif tag == 'qual':
        _, cs, body = node
        rendered = []
        has_named = any(b is not None for b, _ in cs)
        for b, ty in cs:
            t = format_plain(ty, 'top', shorten)
            rendered.append(f'{b} :: {t}' if b else t)
        if has_named:
            inner = f'using ({", ".join(rendered)}) => {format_plain(body, "top", shorten)}'
        elif len(rendered) == 1:
            inner = f'{rendered[0]} => {format_plain(body, "top", shorten)}'
        else:
            inner = f'({", ".join(rendered)}) => {format_plain(body, "top", shorten)}'
    elif tag == 'arrow':
        _, pname, mult, eff, lhs, rhs = node
        lhs_s = f'({pname} :: {format_plain(lhs, "top", shorten)})' if pname else format_plain(lhs, 'arrow_lhs', shorten)
        arrow = f'->{{{eff}}}' if eff is not None else '->'
        if mult:
            inner = f'{lhs_s} {mult} {arrow} {format_plain(rhs, "top", shorten)}'
        else:
            inner = f'{lhs_s} {arrow} {format_plain(rhs, "top", shorten)}'
    elif tag == 'sigma':
        _, name, ty, body = node
        inner = f'Sigma ({name} :: {format_plain(ty, "top", shorten)}). {format_plain(body, "top", shorten)}'
    elif tag == 'refine':
        _, binder, carrier, pred = node
        inner = f'refine ({binder} :: {format_plain(carrier, "top", shorten)}) with {pred}'
    elif tag == 'tuple':
        _, elems = node
        inner = '(' + ', '.join(format_plain(e, 'top', shorten) for e in elems) + ')'
    elif tag == 'app':
        _, elems = node
        out = []
        for idx, e in enumerate(elems):
            e_ctx = 'top' if idx == 0 and e[0] == 'app' else 'app_arg'
            out.append(format_plain(e, e_ctx, shorten))
        inner = ' '.join(out)
    elif tag == 'con':
        inner = short_name(node[1]) if shorten else node[1]
    elif tag == 'var':
        inner = node[1]
    else:
        inner = node[1]
    return f'({inner})' if _needs_parens(node, ctx) else inner


def _link_con_html(full_name: str, type_links: Dict[str, str], shorten: bool = True) -> str:
    display = short_name(full_name) if shorten else full_name
    href = type_links.get(full_name) or type_links.get(display)
    if href:
        title = html.escape(full_name, quote=True)
        return f'<a class="doc-type-link tok-constructor" href="{html.escape(href, quote=True)}" title="{title}">{html.escape(display)}</a>'
    return f'<span class="tok-constructor">{html.escape(display)}</span>'


def _link_effect_row_html(eff: str, type_links: Dict[str, str]) -> str:
    def repl(match: re.Match) -> str:
        tok = match.group(0)
        return _link_con_html(tok, type_links, shorten=True)
    # Link capitalized effect labels like IO, Async, Cancel, Mut, STM, Diverge
    linked = re.sub(r'\b[A-Z][A-Za-z0-9_.]*\b', repl, html.escape(eff))
    return f'<span class="tok-operator">-&gt;</span>{{{linked}}}'


def format_html(node, type_links: Dict[str, str], ctx: str = 'top', shorten: bool = True) -> str:
    tag = node[0]
    if tag == 'forall':
        _, binders, body = node
        parts = []
        for name, kind in binders:
            v = f'<span class="tok-variable">{html.escape(name)}</span>'
            if kind is not None and not _is_plain_type(kind):
                parts.append(f'({v} :: {format_html(kind, type_links, "top", shorten)})')
            else:
                parts.append(v)
        inner = f'<span class="tok-keyword">forall</span> {" ".join(parts)}. {format_html(body, type_links, "top", shorten)}'
    elif tag == 'qual':
        _, cs, body = node
        rendered = []
        has_named = any(b is not None for b, _ in cs)
        for b, ty in cs:
            t = format_html(ty, type_links, 'top', shorten)
            if b:
                rendered.append(f'<span class="tok-variable">{html.escape(b)}</span> :: {t}')
            else:
                rendered.append(t)
        if has_named:
            inner = f'<span class="tok-keyword">using</span> ({", ".join(rendered)}) <span class="tok-operator">=&gt;</span> {format_html(body, type_links, "top", shorten)}'
        elif len(rendered) == 1:
            inner = f'{rendered[0]} <span class="tok-operator">=&gt;</span> {format_html(body, type_links, "top", shorten)}'
        else:
            inner = f'({", ".join(rendered)}) <span class="tok-operator">=&gt;</span> {format_html(body, type_links, "top", shorten)}'
    elif tag == 'arrow':
        _, pname, mult, eff, lhs, rhs = node
        if pname:
            lhs_s = f'(<span class="tok-variable">{html.escape(pname)}</span> :: {format_html(lhs, type_links, "top", shorten)})'
        else:
            lhs_s = format_html(lhs, type_links, 'arrow_lhs', shorten)
        arrow_s = _link_effect_row_html(eff, type_links) if eff is not None else '<span class="tok-operator">-&gt;</span>'
        if mult:
            inner = f'{lhs_s} <span class="tok-keyword">{html.escape(mult)}</span> {arrow_s} {format_html(rhs, type_links, "top", shorten)}'
        else:
            inner = f'{lhs_s} {arrow_s} {format_html(rhs, type_links, "top", shorten)}'
    elif tag == 'sigma':
        _, name, ty, body = node
        inner = (
            f'<span class="tok-keyword">Sigma</span> '
            f'(<span class="tok-variable">{html.escape(name)}</span> :: {format_html(ty, type_links, "top", shorten)}). '
            f'{format_html(body, type_links, "top", shorten)}'
        )
    elif tag == 'refine':
        _, binder, carrier, pred = node
        pred_html = re.sub(
            r'\b[A-Z][A-Za-z0-9_.]*\b',
            lambda m: _link_con_html(m.group(0), type_links, shorten=True),
            html.escape(pred),
        )
        inner = (
            f'<span class="tok-keyword">refine</span> '
            f'(<span class="tok-variable">{html.escape(binder)}</span> :: {format_html(carrier, type_links, "top", shorten)}) '
            f'<span class="tok-keyword">with</span> {pred_html}'
        )
    elif tag == 'tuple':
        _, elems = node
        inner = '(' + ', '.join(format_html(e, type_links, 'top', shorten) for e in elems) + ')'
    elif tag == 'app':
        _, elems = node
        out = []
        for idx, e in enumerate(elems):
            e_ctx = 'top' if idx == 0 and e[0] == 'app' else 'app_arg'
            out.append(format_html(e, type_links, e_ctx, shorten))
        inner = ' '.join(out)
    elif tag == 'con':
        inner = _link_con_html(node[1], type_links, shorten)
    elif tag == 'var':
        inner = f'<span class="tok-variable">{html.escape(node[1])}</span>'
    else:
        inner = html.escape(node[1])
    return f'({inner})' if _needs_parens(node, ctx) else inner


def parse_type(scheme: str):
    toks = tokenize(scheme.strip())
    if not toks:
        return None
    return _Parser(toks).parse()


def pretty_scheme(scheme: str, shorten: bool = True) -> str:
    ast = parse_type(scheme)
    return format_plain(ast, 'top', shorten) if ast is not None else scheme.strip()


def scheme_to_html(scheme: str, type_links: Dict[str, str], shorten: bool = True) -> str:
    ast = parse_type(scheme)
    if ast is not None:
        return format_html(ast, type_links, 'top', shorten)
    # Fallback: highlight uppercase identifiers
    return re.sub(
        r'\b[A-Z][A-Za-z0-9_.]*\b',
        lambda m: _link_con_html(m.group(0), type_links, shorten),
        html.escape(scheme.strip()),
    )


def format_type_params(params: list) -> Tuple[str, str]:
    """Format type header parameter list into (plain, html), dropping redundant `:: Type`."""
    plain_parts: List[str] = []
    html_parts: List[str] = []
    for p in params or []:
        if isinstance(p, dict):
            name = p.get('name', '')
            kind = p.get('kind', 'Type')
            if kind and kind != 'Type':
                plain_parts.append(f'({name} :: {kind})')
                html_parts.append(
                    f'(<span class="tok-variable">{html.escape(name)}</span> :: '
                    f'<span class="tok-constructor">{html.escape(kind)}</span>)'
                )
            else:
                plain_parts.append(name)
                html_parts.append(f'<span class="tok-variable">{html.escape(name)}</span>')
        else:
            s = str(p)
            plain_parts.append(s)
            html_parts.append(f'<span class="tok-variable">{html.escape(s)}</span>')
    return ' '.join(plain_parts), ' '.join(html_parts)


def link_instance_target(target: str, type_links: Dict[str, str]) -> Tuple[str, str]:
    """Return (pretty_plain, linked_html) for an instance target like `Category.Functor.Functor (Data.Core.Result e)`."""
    ast = parse_type(target)
    if ast is not None:
        return format_plain(ast, 'top', True), format_html(ast, type_links, 'top', True)
    plain = re.sub(r'\b(?:[A-Z][A-Za-z0-9_]*\.)+([A-Z][A-Za-z0-9_]*)\b', r'\1', target)
    linked = re.sub(
        r'\b[A-Z][A-Za-z0-9_.]*\b',
        lambda m: _link_con_html(m.group(0), type_links, True),
        html.escape(target),
    )
    return plain, linked


def extract_instance_heads(target: str) -> Tuple[Optional[str], Optional[str]]:
    """Extract (class_short_name, head_type_short_name) from an instance target."""
    ast = parse_type(target)
    if ast is None:
        caps = re.findall(r'\b[A-Z][A-Za-z0-9_.]*\b', target)
        cls = short_name(caps[0]) if len(caps) >= 1 else None
        head = short_name(caps[1]) if len(caps) >= 2 else None
        return cls, head

    def head_con(node) -> Optional[str]:
        if node[0] == 'con':
            return short_name(node[1])
        if node[0] == 'app' and node[1]:
            return head_con(node[1][0])
        return None

    if ast[0] == 'app' and len(ast[1]) >= 2:
        cls = head_con(ast[1][0])
        ty = head_con(ast[1][1])
        return cls, ty
    if ast[0] == 'con':
        return short_name(ast[1]), None
    return None, None


if __name__ == '__main__':
    s1 = 'forall (e :: Type) (a :: Type) (b :: Type). ((e -> b) -> ((a -> b) -> ((Data.Core.Result e a) -> b)))'
    assert pretty_scheme(s1) == 'forall e a b. (e -> b) -> (a -> b) -> Result e a -> b', pretty_scheme(s1)
    s2 = 'forall (r :: Region) (a :: Type). using (io :: Io r) => (Effect.Clock.Duration -> ((Unit ->{Cancel r, IO r} a) %1 ->{Async r, Cancel r, Diverge, IO r} (Effect.Clock.Timeout a)))'
    assert pretty_scheme(s2) == 'forall (r :: Region) a. using (io :: Io r) => Duration -> (Unit ->{Cancel r, IO r} a) %1 ->{Async r, Cancel r, Diverge, IO r} Timeout a', pretty_scheme(s2)
    s3 = 'forall (t :: Type -> Type) (f :: Type -> Type) (a :: Type) (b :: Type). using (dict :: Category.Traversable.Traversable t, app :: Category.Applicative.Applicative f) => ((a -> (f b)) -> ((t a) -> (f (t b))))'
    assert pretty_scheme(s3) == 'forall (t :: Type -> Type) (f :: Type -> Type) a b. using (dict :: Traversable t, app :: Applicative f) => (a -> f b) -> t a -> f (t b)', pretty_scheme(s3)
    assert extract_instance_heads('Category.Functor.Functor (Data.Core.Result e)') == ('Functor', 'Result')
    print('cinch_type_format.py self-tests passed')
