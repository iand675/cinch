#!/usr/bin/env python3
"""Generate bounded complete-input consumers from literal independent tabwriter protocol observations."""
from pathlib import Path
import argparse
import hashlib
import json

HERE = Path(__file__).resolve().parent


def digest(path):
    """Bind every generated case to the exact request and actual observation bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def byte_expression(raw):
    """Represent arbitrary bytes directly; never ask a text codec to repair malformed input or output."""
    return 'bytes-from-list [' + ', '.join(str(value) for value in raw) + ']'


def selected_cases():
    """Select the complete-input protocol subset while retaining all remaining streaming/refusal cases explicitly."""
    requests = json.loads((HERE / 'fixtures/requests.json').read_text())
    observations = json.loads((HERE / 'fixtures/observations.json').read_text())['cases']
    selected, remaining = [], []
    for request, observed in zip(requests, observations):
        assert request['id'] == observed['id']
        actions = request['actions']
        operations = [row['operation'] for row in actions]
        suitable = (not observed.get('constructor_panic') and not request.get('zero') and
                    operations.count('flush') == 1 and operations[-1] == 'flush' and
                    all(row['operation'] in ('write', 'flush') and not row.get('alias') for row in actions) and
                    not any(row.get('error') or row.get('panic') for row in observed['actions'] or []))
        if suitable:
            selected.append((request, observed))
        else:
            remaining.append(request['id'])
    assert len(selected) == 549 and len(remaining) == 63
    return selected, remaining


def consumer(rows):
    """Compare both full byte output and every exact underlying offer, with a unique local failure index."""
    text = '''-- | Verify literal independent complete-input tabular outputs and exact fresh offer boundaries.
module Main where
  import Text.Tabwriter
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Result

  -- | Compare raw offers in order, including observable empty calls.
  offers-equal :: List Bytes -> List Bytes -> Bool
  offers-equal actual expected = case actual of
    Nil -> case expected of
      Nil -> True
      Cons _ _ -> False
    Cons first rest -> case expected of
      Nil -> False
      Cons next tail -> eq-i64 (bytes-compare first next) 0 && offers-equal rest tail
'''
    for index, (request, observed) in enumerate(rows, 1):
        config = request['initial']
        raw = b''.join(bytes.fromhex(row['data']) for row in request['actions'] if row['operation'] == 'write')
        offers = [bytes.fromhex(offer['data']) for action in observed['actions'] for offer in action['offers']]
        expected = b''.join(offers)
        arguments = ' '.join(str(config[key]) for key in ('min_width', 'tab_width', 'padding', 'pad', 'flags'))
        expected_offers = 'Nil'
        for offer in reversed(offers):
            expected_offers = '(Cons (' + byte_expression(offer) + ') ' + expected_offers + ')'
        text += f'''\n  -- | Preserve independent case {request['id']} without predicting its formatter output.
  row{index} :: Unit -> Bool
  row{index} unit = case tab-config {arguments} of
    Err _ -> False
    Ok config ->
      let input = {byte_expression(raw)} in
      case tab-format-offers config input of
        Err _ -> False
        Ok offers ->
          if offers-equal offers {expected_offers} then
            case tab-format-bytes config input of
              Err _ -> False
              Ok output -> let expected = {byte_expression(expected)} in eq-i64 (bytes-compare output expected) 0
          else False
'''
    text += '\n  -- | A local nonzero index uniquely identifies the failing row in the adjacent manifest.\n  main :: Int\n'
    expression = '0'
    for index in reversed(range(1, len(rows) + 1)):
        expression = f'(if row{index} () then {expression} else {index})'
    text += '  main = ' + expression + '\n'
    return text


def main():
    """Write only new bounded source batches and their complete original observation identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--batch-size', type=int, default=24)
    args = parser.parse_args()
    assert 1 <= args.batch_size <= 32
    assert not args.output.exists(), 'never overwrite an independent consumer epoch'
    selected, remaining = selected_cases()
    args.output.mkdir(parents=True)
    report = {'scope': __doc__, 'requests_sha256': digest(HERE / 'fixtures/requests.json'),
              'observations_sha256': digest(HERE / 'fixtures/observations.json'),
              'generator_sha256': digest(Path(__file__)), 'selected_count': len(selected),
              'remaining_streaming_or_refusal_count': len(remaining), 'remaining_cases': remaining,
              'cases_omitted_from_package_scope': [], 'batch_size': args.batch_size, 'batches': [],
              'qualified': False, 'production_adopted': False}
    for offset in range(0, len(selected), args.batch_size):
        rows = selected[offset:offset + args.batch_size]
        name = f'format-{offset // args.batch_size:03d}.cinch'
        path = args.output / name
        path.write_text(consumer(rows))
        report['batches'].append({'source': name, 'sha256': digest(path),
                                  'cases': [{'local_failure_index': index, 'selected_index': offset + index,
                                             'reference_id': request['id']} for index, (request, observed) in enumerate(rows, 1)]})
    (args.output / 'manifest.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Generated{len(selected)} literal complete-input cases;{len(remaining)} streaming/refusal cases remain explicitly in scope')


if __name__ == '__main__':
    main()
