#!/usr/bin/env python3
"""Prepare an unapplied constructor-reuse prototype from explicitly hashed inputs.

This tool reads a reviewed compiler tree and writes only a patch and its input
manifest in this lane. It never edits or builds the input tree.
"""

import argparse
import difflib
import hashlib
import json
from pathlib import Path
import re
import subprocess


def replace(text, old, new, count=1):
    """Replace an exact reviewed fragment, refusing unexpected source drift."""
    actual = text.count(old)
    if actual != count:
        raise ValueError(f"expected {count} occurrences, found {actual}: {old[:100]!r}")
    return text.replace(old, new)


def pattern_identity(path, text):
    """Preserve checked nominal identity while keeping legacy imports untrusted."""
    if path == "compiler/src/core.rs":
        text = replace(text,
            "Con { tag: usize, fields: Vec<LocalId>, classes: Vec<Class> },",
            """Con {
        tag: usize,
        fields: Vec<LocalId>,
        classes: Vec<Class>,
        /// The checked defining constructor, before representation specialization.
        /// Synthetic and decoded legacy patterns carry no identity and cannot
        /// authorize reuse of a payload containing wide fields.
        ctor: Option<GlobalId>,
    },""")
        text = replace(text, "CPat::Con { tag, fields, classes: Vec::new() }",
                       "CPat::Con { tag, fields, classes: Vec::new(), ctor: None }")
        text = replace(text, "    /// A constructor pattern whose field classes lowering fills in.",
                       "    /// A synthetic constructor pattern whose field classes lowering fills in.\n    /// It carries no checked nominal identity and cannot authorize wide reuse.")
        text = replace(text, "CPat::Con { tag, fields, classes } => CPat::Con {",
                       "CPat::Con { tag, fields, classes, ctor } => CPat::Con {")
        text = replace(text, "                            classes: classes.clone(),\n                        },",
                       "                            classes: classes.clone(),\n                            ctor: ctor.clone(),\n                        },")
        text = replace(text, "CPat::Con { tag, fields, classes } => {",
                       "CPat::Con { tag, fields, classes, ctor } => {")
        text = replace(text, "CPat::Con { tag: *tag, fields: out, classes: classes.clone() }",
                       "CPat::Con { tag: *tag, fields: out, classes: classes.clone(), ctor: ctor.clone() }")
        old = "CPat::Con { tag: t1, fields: f1, classes: k1 }, CPat::Con { tag: t2, fields: f2, classes: k2 }"
        new = "CPat::Con { tag: t1, fields: f1, classes: k1, ctor: o1 }, CPat::Con { tag: t2, fields: f2, classes: k2, ctor: o2 }"
        text = replace(text, old, new, 4)
        text = replace(text, "t1 == t2 && f1 == f2 && k1 == k2", "o1 == o2 && t1 == t2 && f1 == f2 && k1 == k2")
        text = replace(text, "t1 == t2 && f1.len() == f2.len() && match_classes(k1, k2, cb)",
                       "o1 == o2 && t1 == t2 && f1.len() == f2.len() && match_classes(k1, k2, cb)")
        text = replace(text, "if t1 == t2 && f1.len() == f2.len() && k1 == k2 =>",
                       "if o1 == o2 && t1 == t2 && f1.len() == f2.len() && k1 == k2 =>")
        text = replace(text, "t1 == t2 && k1 == k2 && f1.len() == f2.len() && f1.iter().zip(f2).all",
                       "o1 == o2 && t1 == t2 && k1 == k2 && f1.len() == f2.len() && f1.iter().zip(f2).all")
        # One equality comparison occurs in a differently indented visitor.
        text = replace(text, "CPat::Con { tag, fields, classes } => (2u8, tag, fields.len(), classes).hash(h),",
                       "CPat::Con { tag, fields, classes, ctor } => (2u8, tag, fields.len(), classes, ctor).hash(h),")
        text = replace(text,
            "`already_set` lists the payload words the kept cell already holds:",
            "`already_set` lists the logical fields the kept cell already holds:")
        text = replace(text,
            "the index `i` is present when `args[i]` is payload word `i` of the",
            "the index `i` is present when `args[i]` is logical field `i` of the")
        text = replace(text, "    /// sorted and deduplicated.\n    Reuse {",
                       "    /// sorted and deduplicated. A wide field occupies two raw words; its\n    /// logical index skips the complete i128 store, not only its low word.\n    Reuse {")
    elif path == "compiler/src/check.rs":
        text = replace(text, "CPat::Con { tag, fields, classes } => CPat::Con {",
                       "CPat::Con { tag, fields, classes, ctor } => CPat::Con {")
        text = replace(text, "                            classes: classes.clone(),\n                        },",
                       "                            classes: classes.clone(),\n                            ctor: ctor.as_ref().map(|id| r.unit(id)),\n                        },")
        text = replace(text, "CPat::Con { tag, fields, .. } => CPat::Con {",
                       "CPat::Con { tag, fields, ctor, .. } => CPat::Con {")
        text = replace(text, "                            classes: fields.iter().map(|f| cx.local(*f)).collect(),",
                       "                            classes: fields.iter().map(|f| cx.local(*f)).collect(),\n                            ctor: ctor.clone(),")
    elif path == "compiler/src/check_terms.rs":
        text = replace(text, "alt_pats.push(CPat::con(tag, slots));",
                       """alt_pats.push(CPat::Con {
                        tag,
                        fields: slots,
                        classes: Vec::new(),
                        ctor: Some(fn_id(&constructor.module, &constructor.name)),
                    });""")
    elif path == "compiler/src/simplify.rs":
        text = replace(text, "CPat::Con { tag: t1, fields: f1, classes: c1 },",
                       "CPat::Con { tag: t1, fields: f1, classes: c1, ctor: o1 },")
        text = replace(text, "CPat::Con { tag: t2, fields: f2, classes: c2 },",
                       "CPat::Con { tag: t2, fields: f2, classes: c2, ctor: o2 },")
        text = replace(text, ") => t1 == t2 && f1 == f2 && c1 == c2,",
                       ") => o1 == o2 && t1 == t2 && f1 == f2 && c1 == c2,")
    elif path == "compiler/src/ifaceopt.rs":
        # No schema extension: imported optimization bodies lack this checked
        # local identity certificate, so their wide reconstruction stays fresh.
        text = replace(text, "CPat::Con { tag, fields, classes } => JsonValue::Arr",
                       "CPat::Con { tag, fields, classes, .. } => JsonValue::Arr")
        text = replace(text, "fields, classes }\n        }", "fields, classes, ctor: None }\n        }")
    elif path == "compiler/src/codegen.rs" or path == "compiler/src/coretext.rs":
        text = replace(text, "CPat::Con { tag, fields, classes } => {",
                       "CPat::Con { tag, fields, classes, .. } => {")
    # Synthetic struct literals with explicit classes have no nominal evidence.
    def synthetic(m):
        """Add an absent certificate without reformatting neighboring code."""
        if "\n" not in m[0]:
            return m[1].rstrip(',') + ", ctor: None" + m[2]
        indent = re.search(r"\n( +)classes:", m[1])[1]
        return m[1].rstrip(',') + ",\n" + indent + "ctor: None," + m[2]

    text = re.sub(r"(CPat::Con \{\s*tag:[^{}]+?\bclasses:\s*vec!\[[^\n]*\],?)(\s*\})",
                  synthetic, text)
    return text


def reuse(text):
    """Carry a pattern certificate through the existing release/reuse search."""
    text = replace(text, "    planned: bool,\n", "    planned: bool,\n    /// A boundary layout is never eligible for wide-field reuse.\n    boundary: bool,\n    /// The checked constructor tag, independent of the successor expression.\n    tag: usize,\n")
    text = replace(text, "/// interchangeable with a dying block of `words` payload words matched at\n/// `tag`, if any.",
                   "/// interchangeable with a dying constructor matched at `tag`, if any.\n/// `words` is the legacy uniform path's logical field count. Wide fields\n/// additionally require checked nominal identity and an exact class vector;\n/// their physical byte count and stores come from that complete layout.")
    text = replace(text,
        """            let planned = program.plans.contains_key(&f.id.0)
                || program.plans.contains_key(crate::repr::base_unit(&f.id.0).as_ref())
                || f.repr.fields.contains(&Class::Wide);
            let fact = CtorFact { fields, planned, data: f.repr.data.clone(), classes: f.repr.fields.clone() };""",
        """            let boundary = program.plans.contains_key(&f.id.0)
                || program.plans.contains_key(crate::repr::base_unit(&f.id.0).as_ref());
            let planned = boundary || f.repr.fields.contains(&Class::Wide);
            let fact = CtorFact { fields, planned, boundary, tag, data: f.repr.data.clone(), classes: f.repr.fields.clone() };""")
    # All search helpers get the same immutable pattern, so borrow inference
    # forces ownership only when the actual emission search can take the pair.
    start = text.index("fn insert_reuse(")
    end = text.index("fn transform(", start)
    region = text[start:end]
    region = region.replace("    ctors: &Ctors,", "    pat: &CPat,\n    ctors: &Ctors,")
    region = region.replace("tag: usize, ctors: &Ctors,", "tag: usize, pat: &CPat, ctors: &Ctors,")
    region = region.replace(", ctors)", ", pat, ctors)").replace(", ctors, work)", ", pat, ctors, work)")
    needle = """    if fact.planned || fact.fields != words || args.len() != words {
        return None;
    }"""
    region = replace(region, needle, """    if fact.fields != words || args.len() != words {
        return None;
    }
    if fact.planned {
        // A wide payload is admitted only by a checked same-constructor
        // certificate. A tag or a logical field count alone says nothing
        // about physical width, pointer maps, or boundary layouts.
        let CPat::Con { ctor: Some(dying), classes, tag: old_tag, .. } = pat else {
            return None;
        };
        if relaxed || fact.boundary || !fact.classes.contains(&Class::Wide)
            || fact.classes.iter().any(|c| c.is_param())
            || classes != &fact.classes || *old_tag != fact.tag
            || crate::repr::base_unit(&dying.0) != crate::repr::base_unit(&id.0)
        {
            return None;
        }
        return Some((id, args));
    }""")
    text = text[:start] + region + text[end:]
    text = replace(text, "fields.len(), *tag, ctors, work)", "fields.len(), *tag, &a.pat, ctors, work)")
    text = text.replace("fields, false, self.ctors)", "fields, false, &a.pat, self.ctors)")
    text = text.replace("fields, true, self.ctors)", "fields, true, &a.pat, self.ctors)")
    text = replace(text, "fields, self.ctors) {", "fields, &a.pat, self.ctors) {")
    return text


def emitter(text):
    """Use the existing physical field planner for both new and reused cells."""
    start = text.index("    fn emit_reuse(")
    end = text.index("    /// The frame site", start)
    region = text[start:end]
    region = replace(region, "        let desc = self.reg.ctor_desc(ctor)?;", """        let desc = self.reg.ctor_desc(ctor)?;
        let classes = self.reg.unit_params.get(ctor)
            .filter(|classes| classes.len() == fields);
        let wide = classes.is_some_and(|classes| classes.contains(&Class::Wide));
        let offsets = classes.map(|classes| field_offsets(classes));
        let bytes = classes.map_or(8 * fields, |classes| classes.iter().map(|c| 8 * c.words()).sum());
        if wide && (self.reg.plans.contains_key(ctor)
            || self.reg.plans.contains_key(crate::repr::base_unit(ctor).as_ref()))
        {
            return err("internal error: wide reuse reached a boundary layout");
        }""")
    # The existing expression emitter already returns i128 operands for Wide.
    region = replace(region, "        for a in args {\n            vals.push(self.emit_expr(a)?);\n        }", """        for (i, a) in args.iter().enumerate() {
            if wide && self.rep_wide(a) != (classes.expect("wide classes")[i] == Class::Wide) {
                return err("internal error: reuse field width disagrees with its constructor");
            }
            vals.push(self.emit_expr(a)?);
        }""")
    region = replace(region, "&ctor_alloc_call(&fresh, 8 * fields, desc)", "&ctor_alloc_call(&fresh, bytes, desc)")
    region = replace(region, "            self.b.store_at(&fresh, 8 * i, v);", """            if wide {
                let at = self.b.gep(&fresh, offsets.as_ref().expect("wide offsets")[i]);
                let ty = if classes.expect("wide classes")[i] == Class::Wide { "i128" } else { "i64" };
                self.b.line(&format!("store {ty} {v}, ptr {at}, align 8"));
            } else {
                self.b.store_at(&fresh, 8 * i, v);
            }""")
    region = replace(region, "                self.b.store_at(&cell, 8 * i, v);", """                if wide {
                    let at = self.b.gep(&cell, offsets.as_ref().expect("wide offsets")[i]);
                    let ty = if classes.expect("wide classes")[i] == Class::Wide { "i128" } else { "i64" };
                    self.b.line(&format!("store {ty} {v}, ptr {at}, align 8"));
                } else {
                    self.b.store_at(&cell, 8 * i, v);
                }""")
    text = text[:start] + region + text[end:]
    # Share the actual typed store with ordinary wide constructor emission.
    old = """            let at = if *off == 0 { p.clone() } else { b.gep(&p, *off) };
            let ty = if *c == Class::Wide { "i128" } else { "i64" };
            b.line(&format!("store {ty} %p{i}, ptr {at}, align 8"));"""
    text = replace(text, old, "            store_uniform_field(b, &p, *off, *c, &format!(\"%p{i}\"));")
    text = replace(text, "fn ctor_body(\n", """/// Store one ordinary constructor field using its physical representation.
/// Wide fields occupy two raw words; every other field occupies one word.
/// Boundary-layout fields use their separate recorded layout machinery.
fn store_uniform_field(b: &mut Builder, payload: &str, offset: usize, class: Class, value: &str) {
    let at = if offset == 0 { payload.to_string() } else { b.gep(payload, offset) };
    let ty = if class == Class::Wide { "i128" } else { "i64" };
    b.line(&format!("store {ty} {value}, ptr {at}, align 8"));
}

fn ctor_body(
""")
    for base in ("fresh", "cell"):
        old = f'''let at = self.b.gep(&{base}, offsets.as_ref().expect("wide offsets")[i]);
                let ty = if classes.expect("wide classes")[i] == Class::Wide {{ "i128" }} else {{ "i64" }};
                self.b.line(&format!("store {{ty}} {{v}}, ptr {{at}}, align 8"));'''
        # The hot path has one additional indentation level.
        if base == "cell":
            old = old.replace("\n                ", "\n                    ")
        text = replace(text, old,
            f'''store_uniform_field(self.b, &{base}, offsets.as_ref().expect("wide offsets")[i], classes.expect("wide classes")[i], v);''')
    return text


def main():
    """Write an exact-base patch without applying it to either compiler tree."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    changed = {}
    claims = {}
    for path in sorted(args.input.glob("compiler/**/*.rs")):
        relative = str(path.relative_to(args.input))
        original = path.read_text()
        if "CPat::Con" not in original and relative not in ("compiler/src/rc.rs", "compiler/src/codegen.rs"):
            continue
        target = pattern_identity(relative, original)
        if relative == "compiler/src/rc.rs":
            target = reuse(target)
        if relative == "compiler/src/codegen.rs":
            target = emitter(target)
        if target != original:
            changed[relative] = "".join(difflib.unified_diff(original.splitlines(True), target.splitlines(True),
                fromfile="a/" + relative, tofile="b/" + relative))
            claims[relative] = {"base_sha256": hashlib.sha256(original.encode()).hexdigest(),
                                "target_sha256": hashlib.sha256(target.encode()).hexdigest()}
    args.output.mkdir(parents=True, exist_ok=True)
    tests = args.output / "wide_reuse.rs"
    if tests.exists():
        relative = "compiler/tests/wide_reuse.rs"
        target = tests.read_text()
        changed[relative] = "".join(difflib.unified_diff([], target.splitlines(True),
            fromfile="/dev/null", tofile="b/" + relative))
        claims[relative] = {"base_sha256": None,
                            "target_sha256": hashlib.sha256(target.encode()).hexdigest()}
    patch = "".join(changed.values())
    (args.output / "prototype.patch").write_text(patch)
    manifest = {"scope": "Uncompiled source-only prototype; no runtime or compiler admission claim.",
                "input_root": str(args.input),
                "input_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=args.input, text=True).strip(),
                "patch_sha256": hashlib.sha256(patch.encode()).hexdigest(), "files": claims}
    (args.output / "inputs.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({"files": len(claims), "patch_bytes": len(patch), "patch_sha256": manifest["patch_sha256"]}))


if __name__ == "__main__":
    main()
