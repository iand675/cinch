// Structural checks for the VS Code extension's cinch-core support.
//
//   node tooling/editors/vscode/test/check.mjs
//
// VS Code is not driven here; the contributions are validated as data:
// both languages are declared with the right extensions and distinct
// configurations, both grammars exist and their scope names match
// package.json, every `include` in the Core grammar resolves, every regex
// in it compiles, the categories from design/core-output.md section 8.3 are
// present, and extension.js still parses and selects both languages.

import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '..');
const failures = [];

function check(name, actual, expected) {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  console.log(`${name.padEnd(30)} ${ok ? 'ok  ' : 'FAIL'} ${JSON.stringify(actual)}`);
  if (!ok) failures.push(name);
}

function readJson(rel) {
  return JSON.parse(readFileSync(join(root, rel), 'utf8'));
}

const pkg = readJson('package.json');
const languages = pkg.contributes.languages;
const grammars = pkg.contributes.grammars;

// Languages.
const ids = languages.map((l) => l.id);
check('language ids', ids, ['cinch', 'cinch-core']);
const core = languages.find((l) => l.id === 'cinch-core');
check('core aliases', core.aliases, ['Cinch Core', 'cinch-core']);
check('core extensions', core.extensions, ['.core']);
check('core configuration', core.configuration, './language-configuration-core.json');
check(
  'distinct configurations',
  new Set(languages.map((l) => l.configuration)).size,
  languages.length,
);
check(
  'activation events',
  pkg.activationEvents,
  ['onLanguage:cinch', 'onLanguage:cinch-core'],
);

// Grammars.
const grammarByLanguage = Object.fromEntries(grammars.map((g) => [g.language, g]));
check('grammar languages', Object.keys(grammarByLanguage).sort(), [
  'cinch',
  'cinch-core',
]);
check(
  'core grammar scope',
  grammarByLanguage['cinch-core'].scopeName,
  'source.cinch-core',
);
check(
  'core grammar path',
  grammarByLanguage['cinch-core'].path,
  './syntaxes/cinch-core.tmLanguage.json',
);

const coreGrammar = readJson('syntaxes/cinch-core.tmLanguage.json');
const surfaceGrammar = readJson('syntaxes/cinch.tmLanguage.json');

for (const [label, grammar, contribution] of [
  ['core', coreGrammar, grammarByLanguage['cinch-core']],
  ['surface', surfaceGrammar, grammarByLanguage['cinch']],
]) {
  check(`${label} scopeName matches`, grammar.scopeName, contribution.scopeName);
  check(`${label} patterns non-empty`, grammar.patterns.length > 0, true);
}

check('core fileTypes', coreGrammar.fileTypes, ['core']);
check('core firstLineMatch', coreGrammar.firstLineMatch, '^core \\d+$');

// Every include resolves, and every regex compiles.
function walkRules(rules, visit) {
  for (const rule of rules ?? []) {
    visit(rule);
    if (rule.patterns) walkRules(rule.patterns, visit);
  }
}

const repository = coreGrammar.repository ?? {};
const missingIncludes = [];
const badRegexes = [];
walkRules(coreGrammar.patterns, (rule) => {
  if (typeof rule.include === 'string' && rule.include.startsWith('#')) {
    const name = rule.include.slice(1);
    if (!(name in repository)) missingIncludes.push(rule.include);
  }
  for (const key of ['match', 'begin', 'end']) {
    if (typeof rule[key] === 'string') {
      try {
        new RegExp(rule[key]);
      } catch (error) {
        badRegexes.push(`${key}: ${rule[key]} (${error.message})`);
      }
    }
  }
});
for (const context of Object.values(repository)) {
  walkRules(context.patterns, (rule) => {
    if (typeof rule.include === 'string' && rule.include.startsWith('#')) {
      const name = rule.include.slice(1);
      if (!(name in repository)) missingIncludes.push(rule.include);
    }
    for (const key of ['match', 'begin', 'end']) {
      if (typeof rule[key] === 'string') {
        try {
          new RegExp(rule[key]);
        } catch (error) {
          badRegexes.push(`${key}: ${rule[key]} (${error.message})`);
        }
      }
    }
  });
}
check('includes resolve', missingIncludes, []);
check('regexes compile', badRegexes, []);

// Section 8.3 categories, read off the grammar's own scopes.
const text = JSON.stringify(coreGrammar);
for (const [category, scope] of [
  ['unit names', 'entity.name.function.cinch-core'],
  ['dictionary bindings', 'variable.other.dictionary.cinch-core'],
  ['ownership classes', 'storage.modifier.ownership.cinch-core'],
  ['annotations', 'storage.type.annotation.cinch-core'],
  ['numerals', 'constant.numeric.cinch-core'],
  ['string literals', 'string.quoted.double.cinch-core'],
  ['byte literals', 'string.quoted.double.bytes.cinch-core'],
  ['header', 'keyword.other.header.cinch-core'],
  ['end', 'keyword.other.unit.cinch-core'],
  ['comments', 'comment.line.double-dash.cinch-core'],
]) {
  check(category, text.includes(`"${scope}"`), true);
}

const forms = coreGrammar.repository.forms.patterns[0].match;
const expectedForms = [
  'int', 'nat', 'bytes', 'string', 'sym', 'bool', 'unit', 'local', 'global',
  'ctor', 'prim', 'foreign', 'app', 'lam', 'dictproj', 'proj', 'instdict',
  'let', 'if', 'case', 'alt', 'pat', 'dup', 'drop', 'release', 'boxinit',
  'boxget', 'boxclear', 'refset', 'quote', 'splice', 'syntaxlit', 'demand',
];
check('all node forms', expectedForms.filter((f) => !forms.includes(f)), []);

// The Core language configuration.
const config = readJson('language-configuration-core.json');
check('line comment', config.comments.lineComment, '--');
check('brackets', config.brackets, [['(', ')']]);
check('folding', config.folding, { offSide: true });

// extension.js still parses and attaches to both languages.
let syntaxOk = true;
try {
  execFileSync(process.execPath, ['--check', join(root, 'extension.js')], {
    stdio: 'pipe',
  });
} catch {
  syntaxOk = false;
}
check('extension.js parses', syntaxOk, true);
const extension = readFileSync(join(root, 'extension.js'), 'utf8');
check(
  'extension.js selects both',
  ['cinch', 'cinch-core'].every((id) => extension.includes(`'${id}'`)),
  true,
);

// The surface grammar's sigil names (surface grammar local amendments
// section 8): `!` and `?` continue a name after its first character, never
// start one, and a keyword is a whole spelling.
// Resolve a scope the way the grammar's own top-level `patterns` order does:
// the first include whose first rule matches at the position wins.
const surfaceScope = (text) => {
  for (const entry of surfaceGrammar.patterns) {
    const context = surfaceGrammar.repository[entry.include.slice(1)];
    for (const rule of context.patterns ?? []) {
      if (typeof rule.match !== 'string') continue;
      const found = new RegExp(rule.match).exec(text);
      if (found && found.index === 0) return rule.name;
    }
  }
  return undefined;
};

for (const [text, scope] of [
  ['empty? x', 'variable.other.cinch'],
  ['print! x', 'variable.other.cinch'],
  ['read!? x', 'variable.other.cinch'],
  ['int-digits-into x', 'variable.other.cinch'],
  ['if? x', 'variable.other.cinch'],
  ['when? x', 'variable.other.cinch'],
  ['if x', 'keyword.control.conditional.cinch'],
  ['when x', 'variable.other.contextual.cinch'],
  ['!= y', 'keyword.operator.cinch'],
  ['Empty? x', 'entity.name.type.cinch'],
  ["'empty?", 'constant.other.symbol.cinch'],
  ["''Empty?", 'entity.name.type.cinch'],
]) {
  check(`surface ${text}`, surfaceScope(text), scope);
}

if (failures.length > 0) {
  console.log(`\n${failures.length} check(s) failed`);
  process.exit(1);
}
console.log('\nall checks passed');
