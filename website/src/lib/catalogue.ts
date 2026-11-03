export type Declaration = {
  name: string; kind: 'type' | 'function' | 'class'; signature: string; description: string;
  example?: string; constructors?: {name:string; signature:string; description:string}[]; since?: string;
};
export type LibraryModule = {name:string; description:string; imports:string[]; declarations:Declaration[]};
export type LibraryPackage = {
  name:string; version:string; versions:string[]; category:string; synopsis:string; description:string;
  license:string; maintainer:string; updated:string; dependencies:string[]; modules:LibraryModule[];
  readme:string[];
};
const fn=(name:string,signature:string,description:string,example?:string,since?:string):Declaration=>({name,kind:'function',signature,description,example,since});
const typ=(name:string,signature:string,description:string,constructors?:Declaration['constructors']):Declaration=>({name,kind:'type',signature,description,constructors});
const mod=(name:string,description:string,declarations:Declaration[],imports:string[]=[]):LibraryModule=>({name,description,declarations,imports});
const pkg=(name:string,version:string,category:string,synopsis:string,modules:LibraryModule[],dependencies:string[]=[],previous?:string):LibraryPackage=>({
  name,version,versions:previous?[version,previous]:[version],category,synopsis,description:synopsis,
  license:'BSD-3-Clause',maintainer:'Cinch contributors',updated:'2026-10-28',dependencies,modules,
  readme:[synopsis,`Import a module explicitly to use its types and functions. The module reference lists declarations, examples, and links to related types.`, `This package is part of the sample catalogue used to develop Cinch’s package documentation site.`],
});
export const packages:LibraryPackage[]=[
  pkg('cinch-base','0.1.0','Core','The core types, list operations, and abstractions used throughout Cinch programs.',[
    mod('Prelude','Primitive types and the common vocabulary shared by the standard libraries.',[
      typ('Nat','data Nat','An unbounded natural number. Arithmetic on Nat does not overflow.',[{name:'Zero',signature:'Zero :: Nat',description:'The natural number zero.'},{name:'Succ',signature:'Succ :: Nat -> Nat',description:'The successor of a natural number.'}]),
      typ('Bool','data Bool','A truth value.',[{name:'False',signature:'False :: Bool',description:'False.'},{name:'True',signature:'True :: Bool',description:'True.'}]),
      typ('Int','opaque Int','An unbounded signed integer.'),
      typ('String','opaque String','An immutable string of Unicode text.'),
      typ('Ref','opaque Ref region a','A reference that borrows a value for the lifetime of region.'),
      typ('Unit','data Unit','The type with a single value, ().'),
      typ('Bytes','opaque Bytes','An immutable sequence of bytes.'),
      typ('Io','effect Io region','An effect for interaction with an external environment.'),
      {name:'Ord',kind:'class',signature:'signature Ord a',description:'A total ordering on values of a type.'},
      fn('id','id :: forall a. a -> a','Return the supplied value unchanged.','id 42\n-- 42'),
      fn('const','const :: forall a b. a -> b -> a','Return the first argument and ignore the second.'),
    ]),
    mod('Data.Option','Values that may be absent, without sentinel values or null.',[
      typ('Option','data Option a','Either a present value or its absence.',[{name:'None',signature:'None :: Option a',description:'No value is present.'},{name:'Some',signature:'Some :: a -> Option a',description:'A present value.'}]),
      fn('map','map :: forall a b. (a -> b) -> Option a -> Option b','Apply a function to a present value. None remains None.','map (\\x -> x + 1) (Some 4)\n-- Some 5'),
      fn('with-default','with-default :: forall a. a -> Option a -> a','Extract a present value, or return the supplied default.','with-default 0 None\n-- 0'),
      fn('is-some','is-some :: forall a. Option a -> Bool','Report whether a value is present.'),
    ]),
    mod('Data.List','Finite, ordered collections with structural recursion and total projections.',[
      typ('List','data List a','A finite sequence of values of the same type.',[{name:'Nil',signature:'Nil :: List a',description:'An empty list.'},{name:'Cons',signature:'Cons :: a -> List a -> List a',description:'A head element and the remaining list.'}]),
      fn('map','map :: forall a b. (a -> b) -> List a -> List b','Transform each element, retaining the order of the input.','map (\\x -> x * 2) [1, 2, 3]\n-- [2, 4, 6]'),
      fn('foldr','foldr :: forall a b. (a -> b -> b) -> b -> List a -> b','Fold a list from the right using a combining function and an initial value.'),
      fn('length','length :: forall a. List a -> Nat','Count the elements in a finite list.','length [1, 2, 3]\n-- 3'),
      fn('head','head :: forall a. List a -> Option a','Return the first element when present. An empty list produces None.','head []\n-- None'),
      fn('filter','filter :: forall a. (a -> Bool) -> List a -> List a','Retain elements that satisfy the predicate, in their original order.'),
    ],['Prelude','Data.Option']),
    mod('Data.Result','Recoverable errors carried in the result type.',[
      typ('Result','data Result e a','A successful value or a recoverable error.',[{name:'Err',signature:'Err :: e -> Result e a',description:'A recoverable failure.'},{name:'Ok',signature:'Ok :: a -> Result e a',description:'A successful result.'}]),
      fn('map','map :: forall e a b. (a -> b) -> Result e a -> Result e b','Transform a successful result, preserving an error.'),
      fn('map-error','map-error :: forall e f a. (e -> f) -> Result e a -> Result f a','Transform the error while leaving successful values alone.'),
    ]),
  ]),
  pkg('cinch-collections','0.2.1','Data','Persistent maps, sets, and nonempty collections.',[
    mod('Data.Map','Ordered, persistent key-value maps.',[
      typ('Map','opaque Map k v','A persistent mapping from ordered keys to values.'),
      fn('empty','empty :: forall k v. Map k v','Construct an empty map.'),
      fn('insert','insert :: forall k v. using (ord :: Ord k) => k -> v -> Map k v -> Map k v','Insert a value. An existing value at the same key is replaced.'),
      fn('lookup','lookup :: forall k v. using (ord :: Ord k) => k -> Map k v -> Option v','Find the value associated with a key.','lookup "port" (insert "port" 8080 empty)\n-- Some 8080'),
      fn('keys','keys :: forall k v. Map k v -> List k','Return keys in ascending order.',undefined,'0.2.1'),
    ],['Prelude','Data.Option','Data.List']),
    mod('Data.Set','Persistent collections of distinct values.',[
      typ('Set','opaque Set a','A collection containing each value at most once.'),
      fn('empty','empty :: forall a. Set a','Construct an empty set.'),
      fn('member','member :: forall a. using (ord :: Ord a) => a -> Set a -> Bool','Test whether the set contains a value.'),
      fn('union','union :: forall a. using (ord :: Ord a) => Set a -> Set a -> Set a','Combine two sets, retaining each value once.'),
    ]),
    mod('Data.NonEmpty','Collections whose type guarantees at least one element.',[
      typ('NonEmpty','data NonEmpty a','One head element followed by a possibly empty tail.',[{name:'NonEmpty',signature:'NonEmpty :: a -> List a -> NonEmpty a',description:'Construct a nonempty sequence.'}]),
      fn('head','head :: forall a. NonEmpty a -> a','Read the first element. No empty case is possible.'),
      fn('to-list','to-list :: forall a. NonEmpty a -> List a','Forget the nonempty guarantee and retain the values.'),
    ],['Data.List']),
  ],['cinch-base'],'0.2.0'),
  pkg('cinch-text','0.3.0','Text','UTF-8 processing and composable text parsers.',[
    mod('Text.Utf8','Decode, encode, and inspect UTF-8 text.',[
      typ('Utf8Error','data Utf8Error','An invalid byte sequence and the offset at which decoding failed.'),
      fn('decode','decode :: Bytes -> Result Utf8Error String','Decode UTF-8 bytes. Invalid sequences produce a structured error.'),
      fn('encode','encode :: String -> Bytes','Encode a string as UTF-8 bytes.'),
      fn('lines','lines :: String -> List String','Split text at line boundaries.'),
    ],['Prelude','Data.Result','Data.List']),
    mod('Text.Parser','Small parsers that compose into larger parsers.',[
      typ('Parser','opaque Parser a','A parser that either produces a value or reports a position and expectation.'),
      typ('ParseError','data ParseError','A source position and the expectations that were not met.'),
      fn('parse','parse :: forall a. Parser a -> String -> Result ParseError a','Run a parser and require it to consume the full input.'),
      fn('integer','integer :: Parser Int','Parse a signed decimal integer.','parse integer "-42"\n-- Ok (-42)'),
    ],['Data.Result']),
  ],['cinch-base'],'0.2.0'),
  pkg('cinch-json','0.2.4','Data','JSON values, typed decoders, and path-aware error messages.',[
    mod('Data.Json','A JSON value tree and its textual representation.',[
      typ('Json','data Json','A null, boolean, number, string, array, or object value.'),
      typ('JsonError','data JsonError','A byte offset and description of a malformed JSON document.'),
      fn('parse','parse :: String -> Result JsonError Json','Parse a complete JSON document.'),
      fn('render','render :: Json -> String','Render a compact JSON representation.'),
    ],['Prelude','Data.Result']),
    mod('Data.Json.Decode','Convert a JSON value to a specific application type.',[
      typ('Decoder','opaque Decoder a','A typed decoder with a path-aware failure report.'),
      typ('DecodeError','data DecodeError','The path, expected type, and actual value at a failed decoding step.'),
      fn('decode','decode :: forall a. Decoder a -> Json -> Result DecodeError a','Apply a decoder to a JSON value.'),
      fn('field','field :: forall a. String -> Decoder a -> Decoder a','Decode a named field in an object.'),
      fn('string','string :: Decoder String','Accept a JSON string.'),
    ],['Data.Json','Data.Result']),
  ],['cinch-base','cinch-text'],'0.2.3'),
  pkg('cinch-http','0.4.0','Networking','HTTP request and response types with an effect-explicit client.',[
    mod('Net.Http','Shared HTTP message types.',[
      typ('Request','record Request','A request method, URL, headers, and optional body.'),
      typ('Response','record Response','A status code, response headers, and body bytes.'),
      typ('HttpError','data HttpError','A connection, protocol, or timeout failure.'),
      fn('status-code','status-code :: Response -> Nat','Read the numerical HTTP status code.'),
    ]),
    mod('Net.Http.Client','An HTTP client whose network operations appear in its effect signature.',[
      fn('get','get :: String ->{Io r} Result HttpError Response','Send a GET request. Connection failures remain explicit in Result.','response <- get "https://example.com/"\ncase response of\n  Ok value -> status-code value\n  Err problem -> 0'),
      fn('send','send :: Request ->{Io r} Result HttpError Response','Send a request with an explicit method, headers, and body.'),
      fn('with-timeout','with-timeout :: Nat -> Request ->{Io r} Result HttpError Response','Send a request with a timeout expressed in milliseconds.',undefined,'0.4.0'),
    ],['Net.Http','Data.Result']),
  ],['cinch-base','cinch-text'],'0.3.0'),
  pkg('cinch-async','0.1.2','Concurrency','Scoped tasks and bounded channels with explicit ownership transfer.',[
    mod('Control.Task','Tasks whose lifetime is bounded by a surrounding scope.',[
      typ('Task','opaque Task scope a','A handle to a scoped computation. Its result can be joined once.'),
      fn('spawn','spawn :: forall a. (Unit ->{Io r} a) ->{Io r} Task scope a','Start a child computation in the current scope.'),
      fn('join','join :: forall a. Task scope a ->{Io r} a','Wait for a task and consume its result handle.'),
    ]),
    mod('Control.Channel','Bounded communication with backpressure.',[
      typ('Channel','opaque Channel a','A bounded channel used to transfer values between tasks.'),
      fn('send','send :: forall a. Channel a -> a ->{Io r} Unit','Send a value, waiting when the channel is full.'),
      fn('receive','receive :: forall a. Channel a ->{Io r} Option a','Receive the next value. A closed channel produces None.'),
    ],['Data.Option']),
  ],['cinch-base'],'0.1.1'),
  pkg('cinch-time','0.1.0','System','Durations, wall-clock instants, and monotonic measurements.',[
    mod('Data.Time','Time values with distinct wall-clock and duration types.',[
      typ('Duration','opaque Duration','A nonnegative span of time.'),
      typ('Instant','opaque Instant','A point on the monotonic clock.'),
      fn('milliseconds','milliseconds :: Nat -> Duration','Construct a duration from milliseconds.'),
      fn('now','now :: Unit ->{Io r} Instant','Read the monotonic clock.'),
      fn('elapsed','elapsed :: Instant -> Instant -> Duration','Measure the duration between two instants.'),
    ]),
  ],['cinch-base']),
  pkg('cinch-test','0.3.2','Testing','Property checks, generators, and a small test runner.',[
    mod('Test.Check','Describe properties and generate input values.',[
      typ('Gen','opaque Gen a','A deterministic generator that produces values from a seed.'),
      typ('Property','opaque Property','A testable proposition with a failure report and counterexample.'),
      fn('for-all','for-all :: forall a. Gen a -> (a -> Bool) -> Property','Test a property over generated values.'),
      fn('check','check :: Nat -> Property ->{Io r} Result String Unit','Run a property for the requested number of examples.'),
    ],['Data.Result']),
    mod('Test.Runner','Group tests and report their results.',[
      typ('Test','opaque Test','A named test or group of tests.'),
      fn('group','group :: String -> List Test -> Test','Name a collection of tests.'),
      fn('run','run :: List Test ->{Io r} Nat','Run a test collection and return the number of failures.'),
    ],['Data.List']),
  ],['cinch-base'],'0.3.1'),
];
export const packageUrl=(p:LibraryPackage,version=p.version)=>`/packages/${p.name}/${version}/`;
export const moduleUrl=(p:LibraryPackage,m:LibraryModule,version=p.version)=>`${packageUrl(p,version)}${m.name}/`;
export const symbolId=(name:string)=>`decl-${encodeURIComponent(name).replace(/%/g,'_')}`;
export const compareVersions=(a:string,b:string)=>{
  const x=a.split('.').map(Number),y=b.split('.').map(Number);
  for(let i=0;i<3;i++){const d=(x[i]??0)-(y[i]??0);if(d)return d;}
  return 0;
};
export const declarationsFor=(m:LibraryModule,version:string)=>m.declarations.filter(d=>!d.since||compareVersions(version,d.since)>=0);
export const symbolUrl=(p:LibraryPackage,m:LibraryModule,d:Declaration,version=p.version)=>`${moduleUrl(p,m,version)}#${symbolId(d.name)}`;
export const modules=packages.flatMap(p=>p.modules.map(m=>({package:p,module:m})));
export const symbols=modules.flatMap(({package:p,module:m})=>m.declarations.map(d=>({package:p,module:m,declaration:d})));
export function resolveType(name:string,p:LibraryPackage,m:LibraryModule,version:string){
  const candidates=symbols.filter(s=>s.declaration.name===name&&s.declaration.kind!=='function');
  const found=candidates.find(s=>s.package.name===p.name&&s.module.name===m.name)??candidates.find(s=>s.package.name===p.name&&m.imports.includes(s.module.name))??candidates.find(s=>s.package.name===p.name)??candidates.find(s=>s.package.name==='cinch-base')??candidates[0];
  return found?symbolUrl(found.package,found.module,found.declaration,found.package.name===p.name?version:found.package.version):undefined;
}
