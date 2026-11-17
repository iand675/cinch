#!/usr/bin/env python3
"""Generate exact streaming tabwriter operation traces without deriving any expectation from the candidate."""
from pathlib import Path
import argparse
import hashlib
import json
from generate_format import byte_expression, selected_cases

HERE = Path(__file__).resolve().parent


def digest(path):
    """Bind every generated trace to literal SDK source observations and the exact generator bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def integer(value):
    """Use mathematical integer expressions for signed malformed reports without ambiguous application syntax."""
    return str(value) if value >= 0 else f'(0 - {-value})'


def optional(value, convert=str):
    """Represent absent and present observations distinctly, including zero counts and empty original error codes."""
    return 'None' if value is None else '(Some ' + convert(value) + ')'


def list_expression(values):
    """Construct a bounded literal list in original observation order."""
    result = 'Nil'
    for value in reversed(values):
        result = '(Cons (' + value + ') ' + result + ')'
    return result


def policy(sink):
    """Describe only physical callback behavior; formatter bytes and public error outcomes come from the SDK fixture."""
    failure = 'None'
    if sink.get('fail_at') or sink.get('fail_empty'):
        failure = ('(Some (Failure ' + str(sink.get('fail_at', 0)) + ' ' + optional(sink.get('fail_prefix')) + ' ' +
                   json.dumps(sink.get('error_code', '')) + ' ' + str(sink.get('fail_empty', False)) + '))')
    override = 'None'
    if sink.get('reported') is not None:
        override = '(Some (' + str(sink.get('report_at', 0)) + ', ' + integer(sink['reported']) + '))'
    return ('Policy ' + optional(sink.get('max_per_call')) + ' ' + failure + ' ' + override + ' ' +
            str(sink.get('offer_limit', 1024)) + ' ' + str(sink.get('byte_limit', 4096)))


def config_arguments(config):
    """Keep every initializer coordinate and all requested flag bits literal."""
    return ' '.join(str(config[key]) for key in ('min_width', 'tab_width', 'padding', 'pad', 'flags'))


def failure(error):
    """Map only the actual public error family, preserving opaque original codes exactly."""
    if error is None:
        return 'NoFailure'
    if error['kind'] == 'short-write':
        return 'ShortFailure'
    assert error['kind'] == 'sink'
    return '(SinkFailure ' + json.dumps(error.get('code', '')) + ')'


def expected_offer(row):
    """Preserve reported signed count independently of physically accepted bytes and original failure."""
    assert 'reported' in row and not row.get('panic')
    code = (row.get('error') or {}).get('code')
    return ('ExpectedOffer (' + byte_expression(bytes.fromhex(row['data'])) + ') ' + integer(row['reported']) +
            ' ' + str(row['accepted']) + ' ' + optional(code, json.dumps))


def protocol_cases():
    """Keep all612 cases partitioned explicitly into549 pure,51 ordinary streaming and12 native adaptation cases."""
    _, remaining = selected_cases()
    requests = json.loads((HERE / 'fixtures/requests.json').read_text())
    observations = json.loads((HERE / 'fixtures/observations.json').read_text())['cases']
    selected, adaptations = [], []
    for request, observed in zip(requests, observations):
        if request['id'] not in remaining:
            continue
        adapted = (request.get('zero') or observed.get('constructor_panic') or
                   any(row.get('panic') for row in observed['actions'] or []))
        if adapted:
            adaptations.append(request['id'])
        else:
            selected.append((request, observed))
    assert len(selected) == 51 and len(adaptations) == 12
    return selected, adaptations


def consumer(rows):
    """Execute each public operation and compare cumulative exact calls, physical bytes and public outcomes immediately."""
    text = '''-- | Verify literal independent streaming tabwriter traces and advanced alias behavior.
module Main where
  import Text.Tabwriter.Config
  import Effect.Tabwriter
  import Fixture.Tabwriter.Protocol
  import Effect.Io
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Option
  import Data.Result
  import Data.String

  -- | Current and original writer handles share only the alias topology actually requested by the fixture.
  data Context (r :: Region) where
    -- | Independent underlying observers stay outside the opaque library writer state.
    Context :: TabWriter r String -> TabWriter r String -> Spy r -> Spy r -> Context r
'''
    for case_index, (request, observed) in enumerate(rows, 1):
        cumulative = {sink['id']: [] for sink in request['sinks']}
        assert list(cumulative) == ['a', 'b']
        for action_index, (action, result) in enumerate(zip(request['actions'], observed['actions']), 1):
            name = f'r{case_index}s{action_index}'
            target = 'alias' if action.get('alias') else 'current'
            body = []
            if action['operation'] == 'write':
                body.append('let input = ' + byte_expression(bytes.fromhex(action['data'])) + ' in')
                body.append(f'let (outcome, advanced) = tab-writer-write {target} input in')
                validation = f'outcome-equals outcome {result["count"]} {failure(result.get("error"))}'
            elif action['operation'] == 'flush':
                body.append(f'let (error, advanced) = tab-writer-flush {target} in')
                validation = 'error-equals error ' + failure(result.get('error'))
            else:
                assert action['operation'] in ('init', 'new') and not result.get('error')
                body.append('let config-result = tab-config ' + config_arguments(action['config']) + ' in')
                body.append('case config-result of')
                body.append(f'  Err _ -> {action_index * 4 + 1}')
                body.append('  Ok config ->')
                sink_name = 'first' if action['sink'] == 'a' else 'second'
                body.append('    let output = spy-capability ' + sink_name + ' in')
                if action['operation'] == 'new':
                    body.append('    let advanced = tab-writer config output in')
                    validation = 'True'
                else:
                    body.append(f'    let (error, advanced) = tab-writer-init {target} config output in')
                    validation = 'error-equals error NoFailure'
            for offer in result['offers']:
                cumulative[offer['sink']].append(offer)
            physical = {row['id']: row['data'] for row in result['outputs']}
            continuation = '0'
            if action_index < len(request['actions']):
                current = 'current' if action.get('alias') else 'advanced'
                continuation = f'r{case_index}s{action_index + 1} (Context {current} alias first second)'
            checks = [f'if not ({validation}) then {action_index * 4 + 1} else']
            for delta, key, label in ((2, 'a', 'first'), (3, 'b', 'second')):
                expected = list_expression([expected_offer(row) for row in cumulative[key]])
                raw = byte_expression(bytes.fromhex(physical[key]))
                checks.append(f'if not (spy-matches {label} {expected} ({raw})) then {action_index * 4 + delta} else')
            checks.append(continuation)
            inner_indent = '    ' if action['operation'] in ('init', 'new') else ''
            body.extend(inner_indent + line for line in checks)
            text += f'''\n  -- | Exact independent {request['id']} operation{action_index}; each refusal index identifies this action and observation.
  {name} :: forall (r :: Region). Context r ->{{IO r, Mut r}} Int
  {name} context = case context of
    Context current alias first second ->
''' + ''.join('      ' + line + '\n' for line in body)
        initial = config_arguments(request['initial'])
        text += f'''\n  -- | Construct the actual requested alias graph for independent case {request['id']}.
  row{case_index} :: forall (r :: Region). Unit ->{{IO r, Mut r}} Int
  row{case_index} unit = case tab-config {initial} of
    Err _ -> 1
    Ok config ->
      let first = spy-new ({policy(request['sinks'][0])}) in
      let second = spy-new ({policy(request['sinks'][1])}) in
      let output = spy-capability first in
      let writer = tab-writer config output in r{case_index}s1 (Context writer writer first second)
'''
    text += '\n  -- | Nonzero exit codes are unique across the adjacent manifest\'s case/action/observation identities.\n'
    text += '  main :: Unit ->{IO r, Mut r} Int\n  main unit = with (io = newIO ()) in\n'
    for index in range(1, len(rows) + 1):
        text += f'    let result{index} = row{index} () in\n    if result{index} /= 0 then result{index} + {(index - 1) * 32} else\n'
    text += '    0\n'
    return text


def main():
    """Write only new bounded consumer sources; panic/initialization adaptations remain separately visible and unqualified."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    assert not args.output.exists(), 'never overwrite a streaming qualification epoch'
    selected, adaptations = protocol_cases()
    args.output.mkdir(parents=True)
    manifest = {'scope': __doc__, 'requests_sha256': digest(HERE / 'fixtures/requests.json'),
                'observations_sha256': digest(HERE / 'fixtures/observations.json'), 'generator_sha256': digest(Path(__file__)),
                'ordinary_streaming_count': len(selected), 'native_adaptation_count': len(adaptations),
                'native_adaptation_cases': adaptations, 'qualified': False, 'production_adopted': False, 'batches': []}
    for offset in range(0, len(selected), 3):
        batch = selected[offset:offset + 3]
        name = f'protocol-{offset // 3:03d}.cinch'
        path = args.output / name
        path.write_text(consumer(batch))
        failures = []
        for index, (request, observed) in enumerate(batch, 1):
            for action_index in range(1, len(request['actions']) + 1):
                for delta, kind in ((1, 'public-outcome'), (2, 'sink-a'), (3, 'sink-b')):
                    code = (index - 1) * 32 + action_index * 4 + delta
                    assert 0 < code < 256
                    failures.append({'exit_code': code, 'case': request['id'], 'action': action_index, 'observation': kind})
        assert len({row['exit_code'] for row in failures}) == len(failures)
        manifest['batches'].append({'source': name, 'sha256': digest(path), 'cases': [q['id'] for q, r in batch],
                                    'failures': failures})
    (args.output / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print('Generated51 literal streaming cases;12 panic/uninitialized adaptations remain explicitly in scope')


if __name__ == '__main__':
    main()
