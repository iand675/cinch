#!/usr/bin/env python3
"""Observe scalar authority, ABI bridges, source-absent reexports, and effect order."""
from __future__ import annotations

import argparse
import copy
import json
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects
from bootstrap_interfaces import canonical, interface_hash, read_contract

HEADER = "-- | Exercise checked scalar authority independently of display names.\nmodule Main where\n  export main\n"
REEXPORT = '''-- | Reexport a checked defining intrinsic through two renamed module boundaries.
module ScalarRelay where
  import Data.Machine (wrapping-add-u64 as increment)
  export increment
-- | Retain the defining intrinsic identity through a qualified facade.
module ScalarFacade where
  import ScalarRelay as Relay
  export increment
  import ScalarRelay (increment)
-- | Supply a genuine defining intrinsic contract for independent interface contradiction controls.
module ScalarDeclaration where
  export native, invert, before
  -- | Declare the canonical operation under a separate source-owned binding.
  foreign import ccall "cinch.prim.wrappingAddU64" native :: U64 -> U64 -> U64
  -- | Retain a unary narrow argument and result in the source-owned descriptor.
  foreign import ccall "cinch.prim.bitNotU8" invert :: U8 -> U8
  -- | Retain signed comparison arguments and the ordinary Boolean result bridge.
  foreign import ccall "cinch.prim.ltI16" before :: I16 -> I16 -> Bool
-- | Supply a genuine ordinary implementation whose display name resembles a scalar helper.
module ScalarOrdinary where
  export wrapping-add-u64
  -- | Retain an ordinary source definition instead of granting intrinsic authority.
  wrapping-add-u64 :: U64 -> U64 -> U64
  wrapping-add-u64 left right = 42
-- | Supply the interface writer with a closed process entry.
module Main where
  -- | Finish after writing the two facade contracts.
  main :: Int
  main = 0
'''
EFFECTS = HEADER + '''  import Data.Machine as W
  -- | Record an observed foreign action without claiming a pure intrinsic contract.
  foreign import ccall "scalar_tick" tick :: U64 ->{Foreign Test} U64
  -- | Read the sequence of foreign observations.
  foreign import ccall "scalar_trace" trace :: U64 ->{Foreign Test} U64
  -- | Apply a checked scalar through its ordinary higher-order ABI.
  apply :: (U64 -> U64 -> U64) -> U64 -> U64 -> U64
  apply function left right = function left right
  -- | Check nested, partial, and higher-order operand evaluation exactly once in source order.
  main :: Unit ->{Foreign Test} Int
  main unit =
    let nested = W.wrapping-add-u64 (tick 1) (tick 2) in
    let partial = W.wrapping-add-u64 (tick 3) in
    let applied = partial (tick 4) in
    let higher = apply W.wrapping-add-u64 (tick 5) (tick 6) in
    let recorded = trace 0 in
    case nested of 3 -> (case applied of 7 -> (case higher of 11 -> (case recorded of 123456 -> 0; _ -> 4); _ -> 3); _ -> 2); _ -> 1
'''
FOREIGN_C = '''#include <stdint.h>
/* Record exact call order and count using a separately compiled foreign function. */
static uint64_t observed;
uint64_t scalar_tick(uint64_t value) { observed = observed * 10 + value; return value; }
/* Return the externally observable trace after all scalar operands have run. */
uint64_t scalar_trace(uint64_t ignored) { (void)ignored; return observed; }
/* Preserve an ordinary foreign function even when its source display name resembles a scalar helper. */
uint64_t scalar_fake_add(uint64_t left, uint64_t right) { (void)left; (void)right; return UINT64_C(42); }
'''


def main() -> None:
    """Retain positive and named negative evidence against one explicit frontend/provider epoch."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--opt-level', type=int, choices=[0, 2], required=True)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    manifest = provider['interface_package']
    objects = args.artifacts / 'objects.json'
    base_fixture = bootstrap_objects.fixture_interfaces(provider, args.artifacts / 'interface-fixtures')
    bootstrap.write_json(objects, provider['configuration'])
    report = {'compiler_sha256': bootstrap.file_hash(args.compiler), 'provider_receipt_sha256': provider['identity']['receipt_sha256'],
              'object_sha256': bootstrap.file_hash(provider['object']), 'runtime_sha256': bootstrap.file_hash(provider['runtime']),
              'opt_level': args.opt_level, 'status': 'running', 'results': []}

    def retain() -> None:
        """Persist completed observations before any later independent check can fail."""
        bootstrap.write_json(args.output, report)

    def invoke(directory: Path, source: str, selected: Path = manifest, mode: str = '--emit-ir',
               accepted: bool = True, diagnostic: str = '', extra: list[Path] | None = None, use_objects: bool = True) -> Path:
        """Require the precise frontend verdict and, for accepted code, exact leak-free runtime behavior."""
        directory.mkdir()
        path, output = directory / 'Main.cinch', directory / 'consumer.ll'
        path.write_text(source)
        command = [str(args.compiler), mode]
        if use_objects or mode != '--emit-ir':
            command.append('--rc-debug')
        command.extend(['--interfaces', str(selected)])
        if mode == '--emit-ir' and use_objects:
            command.extend(['--objects', str(objects)])
        command.extend([str(path), str(output)])
        compiled = subprocess.run(command, capture_output=True, text=True, env=bootstrap.environment())
        (directory / 'compile.log').write_text(compiled.stdout + compiled.stderr)
        diagnostics = Path(str(output) + '.diagnostics')
        details = diagnostics.read_text() if diagnostics.exists() else compiled.stderr
        row = {'name': directory.name, 'source_sha256': bootstrap.file_hash(path), 'command': command,
               'accepted': compiled.returncode == 0, 'exit': compiled.returncode, 'diagnostic': details,
               'uses_library_object': use_objects and mode == '--emit-ir', 'checked_rc_compilation': use_objects and mode == '--emit-ir'}
        report['results'].append(row)
        retain()
        if (compiled.returncode == 0) != accepted or (diagnostic and diagnostic not in details):
            raise bootstrap.BootstrapError(f'{directory.name}: unexpected verdict: {details}')
        if not accepted or mode != '--emit-ir':
            return output
        executable = directory / 'consumer'
        command = [os.environ.get('CINCH_CLANG', 'clang'), f'-O{args.opt_level}', str(output), *map(str, extra or []),
                   *([str(provider['object'])] if use_objects else []), str(provider['runtime']), '-o', str(executable), '-ldl', '-lpthread', '-lm']
        linked = subprocess.run(command, capture_output=True, text=True, env=bootstrap.environment())
        (directory / 'link.log').write_text(linked.stdout + linked.stderr)
        linked.check_returncode()
        row.update(llvm_sha256=bootstrap.file_hash(output), executable_sha256=bootstrap.file_hash(executable), link_command=command, runs=[])
        for debug in [False, True]:
            ran = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
                env={**os.environ, 'CINCH_STACK_SIZE': '8M', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                     'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1'})
            row['runs'].append({'debug': debug, 'exit': ran.returncode, 'stderr': ran.stderr})
            retain()
            if ran.returncode or 'leaked=0' not in ran.stderr:
                raise bootstrap.BootstrapError(directory.name + ': runtime observation failed: ' + ran.stderr)
        return output

    try:
        retain()
        same_name = HEADER + '''  -- | Preserve an ordinary source definition whose display name resembles a machine helper.
  wrapping-add-u64 :: U64 -> U64 -> U64
  wrapping-add-u64 left right = 42
  -- | Observe the user definition rather than substituting an intrinsic by name.
  main :: Int
  main = case wrapping-add-u64 18446744073709551615 1 of 42 -> 0; _ -> 1
'''
        invoke(args.artifacts / 'same-display-name', same_name)
        invoke(args.artifacts / 'same-defining-module-override', '''-- | Replace an imported module name with an ordinary source-owned implementation.
module Data.Machine where
  export wrapping-add-u64
  -- | Preserve the source-owned behavior under the same defining display name.
  wrapping-add-u64 :: U64 -> U64 -> U64
  wrapping-add-u64 left right = 42
-- | Observe the actual override rather than stale provider intrinsic metadata.
module Main where
  export main
  import Data.Machine as W
  -- | Require the source-owned implementation after source module resolution.
  main :: Int
  main = case W.wrapping-add-u64 18446744073709551615 1 of 42 -> 0; _ -> 1
''')
        unsupported = invoke(args.artifacts / 'unsupported-operation', HEADER + "  import Data.Machine as W\n  -- | Keep a valid operation outside the optimized subset on its existing object call path.\n  main :: Int\n  main = case W.wrapping-neg-u64 1 of 18446744073709551615 -> 0; _ -> 1\n")
        external = '@"' + ''.join('\\' + format(byte, '02X') for byte in b'Data.Machine.wrapping-neg-u64') + '"'
        if external not in unsupported.read_text():
            raise bootstrap.BootstrapError('unsupported operation lost its original object call path')
        # These genuine source declarations must fail before lowering; no interface body is supplied.
        for name, declaration, message in [
            ('wrong-carrier', 'ccall "cinch.prim.wrappingAddU64" native :: I64 -> I64 -> I64', 'canonical nominal scheme'),
            ('wrong-result', 'ccall "cinch.prim.wrappingAddU64" native :: U64 -> U64 -> Bool', 'canonical nominal scheme'),
            ('polymorphic', 'ccall "cinch.prim.wrappingAddU64" native :: forall a. a -> a -> a', 'cannot be polymorphic'),
            ('preserved-carrier', 'ccall "cinch.prim.eqI64" native :: Int -> Int -> Bool', 'type mismatch'),
            ('preserved-polymorphic', 'ccall "cinch.prim.eqI64" native :: forall a. a -> a -> Bool', 'cannot be polymorphic'),
            ('wrong-effects', 'ccall "cinch.prim.wrappingAddU64" native :: U64 -> U64 ->{Global} U64', 'canonical nominal scheme'),
            ('effect-region-parameter', 'ccall "cinch.prim.wrappingAddU64" native :: U64 -> U64 ->{Foreign Test} U64', 'cannot be polymorphic'),
            ('wrong-abi', 'component "cinch.prim.wrappingAddU64" native :: U64 -> U64 -> U64', 'requires foreign import ccall'),
            ('unknown-opcode', 'ccall "cinch.prim.wrappingImaginaryU64" native :: U64 -> U64 -> U64', 'not a cinch intrinsic'),
        ]:
            invoke(args.artifacts / name, HEADER + '  -- | Attempt an unsupported intrinsic declaration.\n  foreign import ' + declaration + '\n  -- | Return only after successful checking.\n  main :: Int\n  main = 0\n',
                   mode='--check', accepted=False, diagnostic=message)
        directory = args.artifacts / 'foreign-fixture'
        directory.mkdir()
        fixture = directory / 'fixture.c'
        fixture.write_text(FOREIGN_C)
        fixture_object = directory / 'fixture.o'
        compiled = subprocess.run(['clang', '-O2', '-c', str(fixture), '-o', str(fixture_object)], capture_output=True, text=True)
        (directory / 'compile.log').write_text(compiled.stdout + compiled.stderr)
        compiled.check_returncode()
        invoke(args.artifacts / 'effect-order', EFFECTS, extra=[fixture_object])
        invoke(args.artifacts / 'same-display-foreign', HEADER + '''  -- | Keep a checked ordinary foreign call under a helper-like source name.
  foreign import ccall "scalar_fake_add" wrapping-add-u64 :: U64 -> U64 ->{Foreign Test} U64
  -- | Observe the checked external body rather than a source-name substitution.
  main :: Unit ->{Foreign Test} Int
  main unit = case wrapping-add-u64 18446744073709551615 1 of 42 -> 0; _ -> 1
''', extra=[fixture_object])
        producer = invoke(args.artifacts / 'reexport-producer', REEXPORT, mode='--emit-interfaces')
        admitted = copy.deepcopy(base_fixture)
        for name in ['ScalarRelay', 'ScalarFacade']:
            contract = Path(str(producer) + '.' + name + '.cinchif')
            document = read_contract(contract)
            admitted['interfaces'].append([name, interface_hash(document), str(contract)])
        source = producer.parent / 'Main.cinch'
        report['reexport_source_sha256'] = bootstrap.file_hash(source)
        source.unlink()
        admitted_path = args.artifacts / 'reexport-interfaces.json'
        bootstrap.write_json(admitted_path, admitted)
        invoke(args.artifacts / 'source-absent-reexport', HEADER + '''  import ScalarFacade as F
  -- | Preserve the original intrinsic through a source-absent double reexport and partial application.
  main :: Int
  main = let partial = F.increment 18446744073709551615 in case partial 2 of 1 -> 0; _ -> 1
''', selected=admitted_path)
        report['producer_source_removed_before_consumer'] = not source.exists()
        defining_contract = Path(str(producer) + '.ScalarDeclaration.cinchif')
        defining_document = read_contract(defining_contract)
        defining_rows = defining_document['exports']['foreign']
        expected_opcodes = {'native': 'cinch.prim.wrappingAddU64', 'invert': 'cinch.prim.bitNotU8', 'before': 'cinch.prim.ltI16'}
        if {row['symbol']: row['external'] for row in defining_rows} != expected_opcodes:
            raise bootstrap.BootstrapError('the checked source scalar descriptors were lost during interface export')
        for row in defining_rows:
            value = next(value for value in defining_document['exports']['values'] if value['name'] == row['symbol'])
            if row['type'] != value['resolved']['scheme'] or row['abi'] != 'ccall' or row['effects'] != '':
                raise bootstrap.BootstrapError('the source scalar descriptor lost its exact checked scheme or ABI')
        configuration = copy.deepcopy(base_fixture)
        configuration['interfaces'].append(['ScalarDeclaration', interface_hash(defining_document), str(defining_contract)])
        selected = args.artifacts / 'defining-intrinsic-interfaces.json'
        bootstrap.write_json(selected, configuration)
        # This pure word-only client needs no library object: the defining source is absent and only the runtime links.
        invoke(args.artifacts / 'source-absent-defining-intrinsic', HEADER + '  import ScalarDeclaration (native as increment, invert, before)\n  -- | Observe binary, unary narrow, and Boolean operations from source-absent defining contracts.\n  main :: Int\n  main = let partial = increment 18446744073709551615 in case partial 2 of 1 -> (case invert 128 of 127 -> (case before (-32768) 32767 of True -> 0; _ -> 3); _ -> 2); _ -> 1\n',
               selected=selected, use_objects=False)
        legacy = copy.deepcopy(defining_document)
        next(row for row in legacy['exports']['values'] if row['name'] == 'native')['resolved'] = None
        legacy_path = args.artifacts / 'legacy-display-only.cinchif'
        legacy_path.write_text(canonical(legacy))
        configuration = copy.deepcopy(base_fixture)
        configuration['interfaces'].append(['ScalarDeclaration', interface_hash(legacy), str(legacy_path)])
        selected = args.artifacts / 'legacy-display-only-interfaces.json'
        bootstrap.write_json(selected, configuration)
        legacy_client = HEADER + '  import ScalarDeclaration (native)\n  -- | Preserve the legacy display-only scheme on its ordinary external path.\n  main :: Int\n  main = case native 18446744073709551615 1 of 0 -> 0; _ -> 1\n'
        invoke(args.artifacts / 'legacy-display-check', legacy_client, selected=selected, mode='--check')
        invoke(args.artifacts / 'legacy-display-no-intrinsic-authority', legacy_client, selected=selected,
               use_objects=False, accepted=False, diagnostic='external runtime implementation is unavailable')
        facade_path = Path(str(producer) + '.ScalarFacade.cinchif')
        original_facade = read_contract(facade_path)
        for name in ['facade-scheme', 'facade-arity', 'facade-serialized-unit', 'facade-new-foreign']:
            document = copy.deepcopy(original_facade)
            if name == 'facade-scheme':
                document['exports']['values'][0]['scheme'] = '(I64 -> (I64 -> I64))'
                document['exports']['values'][0]['resolved']['scheme'] = '(I64 -> (I64 -> I64))'
            elif name == 'facade-arity':
                document['exports']['values'][0]['arity'] = 1
            elif name == 'facade-serialized-unit':
                document['units'] = [{'body': 'ret i64 0'}]
            else:
                defining = read_contract(Path(str(producer) + '.ScalarDeclaration.cinchif'))
                if not defining['exports']['foreign']:
                    raise bootstrap.BootstrapError('the forged-control payload must contain an actual checked foreign row')
                document['exports']['foreign'] = copy.deepcopy(defining['exports']['foreign'])
            contract = args.artifacts / (name + '.cinchif')
            contract.write_text(canonical(document))
            configuration = copy.deepcopy(admitted)
            index = next(i for i, row in enumerate(configuration['interfaces']) if row[0] == 'ScalarFacade')
            configuration['interfaces'][index] = ['ScalarFacade', interface_hash(document), str(contract)]
            selected = args.artifacts / (name + '-interfaces.json')
            bootstrap.write_json(selected, configuration)
            invoke(args.artifacts / name, HEADER + '  import ScalarFacade\n  -- | Require exact represented metadata before admitting a code-free facade.\n  main :: Int\n  main = 0\n',
                   selected=selected, accepted=False, diagnostic='runtime object checked interface identity mismatch')
        ordinary_contract = Path(str(producer) + '.ScalarOrdinary.cinchif')
        ordinary_doc = read_contract(ordinary_contract)
        configuration = copy.deepcopy(base_fixture)
        configuration['interfaces'].append(['ScalarOrdinary', interface_hash(ordinary_doc), str(ordinary_contract)])
        selected = args.artifacts / 'ordinary-interfaces.json'
        bootstrap.write_json(selected, configuration)
        invoke(args.artifacts / 'unrepresented-ordinary', HEADER + '  import ScalarOrdinary\n  -- | Refuse to grant an implementation from an ordinary source-absent interface.\n  main :: Int\n  main = 0\n',
               selected=selected, accepted=False, diagnostic='runtime object checked interface identity mismatch')
        # Preserve the defining identity while contradicting exactly one checked foreign fact.
        defining_contract = Path(str(producer) + '.ScalarDeclaration.cinchif')
        original = read_contract(defining_contract)
        for name, field, value, message in [
            ('metadata-effects', 'effects', 'Foreign Test', 'defining ABI'),
            ('metadata-abi', 'abi', 'component', 'defining ABI'),
            ('metadata-type', 'type', '(I64 -> (I64 -> I64))', 'defining ABI'),
        ]:
            document = copy.deepcopy(original)
            row = next(row for row in document['exports']['foreign'] if row['symbol'] == 'native')
            row[field] = value
            contract = args.artifacts / (name + '.cinchif')
            contract.write_text(canonical(document))
            configuration = copy.deepcopy(base_fixture)
            configuration['interfaces'].append(['ScalarDeclaration', interface_hash(document), str(contract)])
            selected = args.artifacts / (name + '-interfaces.json')
            bootstrap.write_json(selected, configuration)
            invoke(args.artifacts / name, HEADER + '  import ScalarDeclaration (native)\n  -- | Require the genuine defining contract before calling its imported operation.\n  main :: Int\n  main = case native 18446744073709551615 1 of 0 -> 0; _ -> 1\n',
                   selected=selected, mode='--check', accepted=False, diagnostic=message)
        report['status'] = 'passed'
        retain()
        print(f'{len(report["results"])} scalar authority observations passed', flush=True)
    except Exception as error:
        report.update(status='failed', failure=str(error))
        retain()
        raise


if __name__ == '__main__':
    main()
