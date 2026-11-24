import assert from 'node:assert/strict';
import { parseTypeExpr, runHoogleSearch, type CompactSearchEntry } from '../src/lib/hoogle-search.ts';

const sample: CompactSearchEntry[] = [
  {
    m: 'Data.List',
    n: 'map',
    k: 'v',
    s: 'map :: forall a b. (a -> b) -> List a -> List b',
    d: 'Apply a function to every element of a list.',
    h: 'v-map',
    l: 54,
    e: 1,
  },
  {
    m: 'Data.Result',
    n: 'result-to-option',
    k: 'v',
    s: 'result-to-option :: forall e a. Result e a -> Option a',
    d: 'The value as an option, the error dropped.',
    h: 'v-result-to-option',
    l: 116,
    e: 1,
  },
  {
    m: 'Data.Result',
    n: 'option-to-result',
    k: 'v',
    s: 'option-to-result :: forall e a. e -> Option a -> Result e a',
    d: 'The option as a result, with the error supplied for None.',
    h: 'v-option-to-result',
    l: 123,
    e: 1,
  },
  {
    m: 'Data.List',
    n: 'sort',
    k: 'v',
    s: 'sort :: forall a. using (ord :: Ord a) => List a -> List a',
    d: 'Sort a list in ascending order.',
    h: 'v-sort',
    l: 201,
    e: 1,
  },
  {
    m: 'Function.Combinators',
    n: 'id',
    k: 'v',
    s: 'id :: forall a. a -> a',
    d: 'The identity function.',
    h: 'v-id',
    l: 12,
    e: 1,
  },
  {
    m: 'Crypto.Aes',
    n: 'aes-encrypt',
    k: 'v',
    s: 'aes-encrypt :: Bytes -> Bytes -> Result AesError Bytes',
    d: 'Encrypt a block with AES.',
    h: 'v-aes-encrypt',
    l: 88,
    e: 0,
  },
  {
    m: 'Category.Functor',
    n: 'Functor',
    k: 'c',
    s: 'signature Functor f',
    d: 'Types that can be mapped over.',
    h: 'c-Functor',
    l: 10,
    e: 1,
  },
];

assert.ok(parseTypeExpr('(a -> b) -> List a -> List b'));
assert.ok(parseTypeExpr('forall (r :: Region) a. using (io :: Io r) => STM a ->{Cancel r, STM} Option a'));

// 1. Alpha-renamed type signature query
const mapHits = runHoogleSearch(sample, '(x -> y) -> List x -> List y');
assert.equal(mapHits[0]?.entry.n, 'map');

// 2. Argument-reordered type query
const reordered = runHoogleSearch(sample, 'List a -> (a -> b) -> List b');
assert.equal(reordered[0]?.entry.n, 'map');

// 3. Result -> Option query
const resOpt = runHoogleSearch(sample, 'Result e a -> Option a');
assert.equal(resOpt[0]?.entry.n, 'result-to-option');

// 4. Polymorphic subsumption: Int -> Int finds id :: a -> a
const poly = runHoogleSearch(sample, 'Int -> Int');
assert.ok(poly.some((h) => h.entry.n === 'id'));

// 5. Constrained query
const ordHits = runHoogleSearch(sample, 'Ord a => List a -> List a');
assert.equal(ordHits[0]?.entry.n, 'sort');

// 6. Module include/exclude & kind filters
const scoped = runHoogleSearch(sample, '+Data.Result option');
assert.equal(scoped.length, 2);
assert.ok(scoped.every((h) => h.entry.m === 'Data.Result'));

const excluded = runHoogleSearch(sample, '-Crypto Bytes -> Result');
assert.ok(excluded.every((h) => !h.entry.m.startsWith('Crypto')));

const classes = runHoogleSearch(sample, 'is:class');
assert.equal(classes.length, 1);
assert.equal(classes[0]?.entry.n, 'Functor');

const sourceOnly = runHoogleSearch(sample, 'in:source Bytes');
assert.equal(sourceOnly.length, 1);
assert.equal(sourceOnly[0]?.entry.m, 'Crypto.Aes');

// 7. Combined name :: type query
const combined = runHoogleSearch(sample, 'map :: (a -> b) -> List a -> List b');
assert.equal(combined.length, 1);
assert.equal(combined[0]?.entry.n, 'map');

console.log('Hoogle search tests passed (10 assertions)');
