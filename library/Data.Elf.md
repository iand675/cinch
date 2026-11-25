# Data.Elf

Bounds-checked ELF32 and ELF64 (little/big-endian) parser for headers, segments, sections, symbols, relocations, dynamic tables and zlib-compressed sections.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Elf</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Elf/source/">View source</a><a href="/packages/search/?q=%2BData.Elf">Search module</a><a href="/reference-sources/Data.Elf.cinch" download="Data.Elf.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md), [Text.Zlib](Text.Zlib.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure bounds-checked parser for ELF32 and ELF64 object and executable files.
Supports little-endian and big-endian byte orders, file headers, program headers,
section headers, string tables, static and dynamic symbol tables, REL and RELA
relocations, dynamic section tags and imported libraries/symbols, and zlib
decompression of SHF_COMPRESSED sections.

</details>

<details class="doc-synopsis"><summary>Synopsis (131 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ElfClass">ElfClass</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfClass</strong></code></li><li><a class="syn-name" href="#t-ElfData">ElfData</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfData</strong></code></li><li><a class="syn-name" href="#t-ElfDynEntry">ElfDynEntry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfDynEntry</strong></code></li><li><a class="syn-name" href="#t-ElfDynTag">ElfDynTag</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfDynTag</strong></code></li><li><a class="syn-name" href="#t-ElfError">ElfError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfError</strong></code></li><li><a class="syn-name" href="#t-ElfFile">ElfFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfFile</strong></code></li><li><a class="syn-name" href="#t-ElfHeader">ElfHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfHeader</strong></code></li><li><a class="syn-name" href="#t-ElfImportedSymbol">ElfImportedSymbol</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfImportedSymbol</strong></code></li><li><a class="syn-name" href="#t-ElfMachine">ElfMachine</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfMachine</strong></code></li><li><a class="syn-name" href="#t-ElfOsAbi">ElfOsAbi</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfOsAbi</strong></code></li><li><a class="syn-name" href="#t-ElfProgHeader">ElfProgHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfProgHeader</strong></code></li><li><a class="syn-name" href="#t-ElfProgType">ElfProgType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfProgType</strong></code></li><li><a class="syn-name" href="#t-ElfRel">ElfRel</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfRel</strong></code></li><li><a class="syn-name" href="#t-ElfRela">ElfRela</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfRela</strong></code></li><li><a class="syn-name" href="#t-ElfSectionHeader">ElfSectionHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSectionHeader</strong></code></li><li><a class="syn-name" href="#t-ElfSectionType">ElfSectionType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSectionType</strong></code></li><li><a class="syn-name" href="#t-ElfSymBind">ElfSymBind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymBind</strong></code></li><li><a class="syn-name" href="#t-ElfSymType">ElfSymType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymType</strong></code></li><li><a class="syn-name" href="#t-ElfSymVis">ElfSymVis</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymVis</strong></code></li><li><a class="syn-name" href="#t-ElfSymbol">ElfSymbol</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymbol</strong></code></li><li><a class="syn-name" href="#t-ElfType">ElfType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfType</strong></code></li><li><a class="syn-name" href="#v-elf-abi-version">elf-abi-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-abi-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-byte-order">elf-byte-order</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-byte-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code></li><li><a class="syn-name" href="#v-elf-class">elf-class</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-class</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a></code></li><li><a class="syn-name" href="#v-elf-class-code">elf-class-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-class-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-compress-zlib">elf-compress-zlib</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-compress-zlib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-data">elf-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a></code></li><li><a class="syn-name" href="#v-elf-data-code">elf-data-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-data-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-dyn-raw-tag">elf-dyn-raw-tag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-raw-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-dyn-string">elf-dyn-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-elf-dyn-tag">elf-dyn-tag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li><a class="syn-name" href="#v-elf-dyn-tag-code">elf-dyn-tag-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-tag-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-dyn-val">elf-dyn-val</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-val</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-dyn-value">elf-dyn-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dyn-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-elf-dynamic-entries">elf-dynamic-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dynamic-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a>)</code></li><li><a class="syn-name" href="#v-elf-dynamic-symbols">elf-dynamic-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-dynamic-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a>)</code></li><li><a class="syn-name" href="#v-elf-entry">elf-entry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-error-message">elf-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-file-header">elf-file-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-file-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a></code></li><li><a class="syn-name" href="#v-elf-flags">elf-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-imported-libraries">elf-imported-libraries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-elf-imported-library">elf-imported-library</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-imported-library</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-imported-name">elf-imported-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-imported-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-imported-symbols">elf-imported-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a>)</code></li><li><a class="syn-name" href="#v-elf-imported-version">elf-imported-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-imported-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-machine">elf-machine</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-machine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li><a class="syn-name" href="#v-elf-machine-code">elf-machine-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-machine-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-elf-osabi">elf-osabi</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-osabi</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li><a class="syn-name" href="#v-elf-osabi-code">elf-osabi-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-osabi-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-parse">elf-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a></code></li><li><a class="syn-name" href="#v-elf-prog-align">elf-prog-align</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-prog-data">elf-prog-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-elf-prog-filesz">elf-prog-filesz</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-filesz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-prog-flags">elf-prog-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-prog-memsz">elf-prog-memsz</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-memsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-prog-offset">elf-prog-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-prog-paddr">elf-prog-paddr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-paddr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-prog-type">elf-prog-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li><a class="syn-name" href="#v-elf-prog-type-code">elf-prog-type-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-prog-vaddr">elf-prog-vaddr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-prog-vaddr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-progs">elf-progs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-progs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a></code></li><li><a class="syn-name" href="#v-elf-r-sym32">elf-r-sym32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-r-sym32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-r-sym64">elf-r-sym64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-r-sym64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-r-type32">elf-r-type32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-r-type32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-r-type64">elf-r-type64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-r-type64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-rel-info">elf-rel-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rel-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-rel-offset">elf-rel-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rel-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-rel-sym">elf-rel-sym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rel-sym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-rel-type">elf-rel-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rel-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-rela-addend">elf-rela-addend</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rela-addend</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-elf-rela-info">elf-rela-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rela-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-rela-offset">elf-rela-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rela-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-rela-sym">elf-rela-sym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rela-sym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-rela-type">elf-rela-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-rela-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-relocations-rel">elf-relocations-rel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-relocations-rel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a>)</code></li><li><a class="syn-name" href="#v-elf-relocations-rela">elf-relocations-rela</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-relocations-rela</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a>)</code></li><li><a class="syn-name" href="#v-elf-section">elf-section</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code></li><li><a class="syn-name" href="#v-elf-section-addr">elf-section-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-addralign">elf-section-addralign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-addralign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-by-type">elf-section-by-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-by-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code></li><li><a class="syn-name" href="#v-elf-section-compressed">elf-section-compressed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-elf-section-compression-type">elf-section-compression-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-compression-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-section-data">elf-section-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-elf-section-entsize">elf-section-entsize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-entsize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-file-size">elf-section-file-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-file-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-flags">elf-section-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-info">elf-section-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-section-link">elf-section-link</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-section-name">elf-section-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-section-offset">elf-section-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-size">elf-section-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-section-type">elf-section-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li><a class="syn-name" href="#v-elf-section-type-code">elf-section-type-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-section-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-sections">elf-sections</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code></li><li><a class="syn-name" href="#v-elf-st-bind">elf-st-bind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-st-bind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li><li><a class="syn-name" href="#v-elf-st-info">elf-st-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-st-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-st-type">elf-st-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-st-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li><a class="syn-name" href="#v-elf-st-visibility">elf-st-visibility</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-st-visibility</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li><li><a class="syn-name" href="#v-elf-sym-bind">elf-sym-bind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-bind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li><li><a class="syn-name" href="#v-elf-sym-bind-code">elf-sym-bind-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-bind-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-sym-info">elf-sym-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-sym-name">elf-sym-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-elf-sym-other">elf-sym-other</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-other</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-sym-section">elf-sym-section</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-elf-sym-size">elf-sym-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-sym-type">elf-sym-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li><a class="syn-name" href="#v-elf-sym-type-code">elf-sym-type-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-sym-value">elf-sym-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-elf-sym-vis-code">elf-sym-vis-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-vis-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-elf-sym-visibility">elf-sym-visibility</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-sym-visibility</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li><li><a class="syn-name" href="#v-elf-symbols">elf-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a>)</code></li><li><a class="syn-name" href="#v-elf-type">elf-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li><a class="syn-name" href="#v-elf-type-code">elf-type-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-elf-version">elf-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elf-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-pf-r">pf-r</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pf-r</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pf-w">pf-w</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pf-w</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pf-x">pf-x</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pf-x</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-shf-alloc">shf-alloc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-alloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-compressed">shf-compressed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-execinstr">shf-execinstr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-execinstr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-group">shf-group</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-group</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-info-link">shf-info-link</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-info-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-link-order">shf-link-order</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-link-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-merge">shf-merge</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-strings">shf-strings</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-strings</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-tls">shf-tls</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-tls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shf-write">shf-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shf-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-shn-abs">shn-abs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shn-abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-shn-common">shn-common</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shn-common</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-shn-undef">shn-undef</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shn-undef</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-shn-xindex">shn-xindex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shn-xindex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ElfClass">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfClass</strong></code><span class="doc-sig-links"><a href="#t-ElfClass" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

ELF address-width class (EI_CLASS).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfClassNone"><code><strong class="tok-constructor">ElfClassNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a></code></li><li id="k-ElfClass32"><code><strong class="tok-constructor">ElfClass32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a></code></li><li id="k-ElfClass64"><code><strong class="tok-constructor">ElfClass64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfData">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfData</strong></code><span class="doc-sig-links"><a href="#t-ElfData" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

ELF data encoding (EI_DATA).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfDataNone"><code><strong class="tok-constructor">ElfDataNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a></code></li><li id="k-ElfData2Lsb"><code><strong class="tok-constructor">ElfData2Lsb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a></code></li><li id="k-ElfData2Msb"><code><strong class="tok-constructor">ElfData2Msb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfDynEntry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfDynEntry</strong></code><span class="doc-sig-links"><a href="#t-ElfDynEntry" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1044" title="View source at line 1044">Source</a></span></div>
<div class="doc-body">

Construct an ElfDynEntry from decoded tag, raw tag value, and entry value/pointer.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfDynEntry"><code><strong class="tok-constructor">ElfDynEntry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfDynTag">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfDynTag</strong></code><span class="doc-sig-links"><a href="#t-ElfDynTag" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L619" title="View source at line 619">Source</a></span></div>
<div class="doc-body">

Dynamic section tag (d_tag).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DtNull"><code><strong class="tok-constructor">DtNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtNeeded"><code><strong class="tok-constructor">DtNeeded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtPltrelsz"><code><strong class="tok-constructor">DtPltrelsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtPltgot"><code><strong class="tok-constructor">DtPltgot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtHash"><code><strong class="tok-constructor">DtHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtStrtab"><code><strong class="tok-constructor">DtStrtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtSymtab"><code><strong class="tok-constructor">DtSymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRela"><code><strong class="tok-constructor">DtRela</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRelasz"><code><strong class="tok-constructor">DtRelasz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRelaent"><code><strong class="tok-constructor">DtRelaent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtStrsz"><code><strong class="tok-constructor">DtStrsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtSyment"><code><strong class="tok-constructor">DtSyment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtInit"><code><strong class="tok-constructor">DtInit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtFini"><code><strong class="tok-constructor">DtFini</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtSoname"><code><strong class="tok-constructor">DtSoname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRpath"><code><strong class="tok-constructor">DtRpath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtSymbolic"><code><strong class="tok-constructor">DtSymbolic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRel"><code><strong class="tok-constructor">DtRel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRelsz"><code><strong class="tok-constructor">DtRelsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRelent"><code><strong class="tok-constructor">DtRelent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtPltrel"><code><strong class="tok-constructor">DtPltrel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtDebug"><code><strong class="tok-constructor">DtDebug</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtTextrel"><code><strong class="tok-constructor">DtTextrel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtJmprel"><code><strong class="tok-constructor">DtJmprel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtBindNow"><code><strong class="tok-constructor">DtBindNow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtInitArray"><code><strong class="tok-constructor">DtInitArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtFiniArray"><code><strong class="tok-constructor">DtFiniArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtInitArraysz"><code><strong class="tok-constructor">DtInitArraysz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtFiniArraysz"><code><strong class="tok-constructor">DtFiniArraysz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtRunpath"><code><strong class="tok-constructor">DtRunpath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtFlags"><code><strong class="tok-constructor">DtFlags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtFlags1"><code><strong class="tok-constructor">DtFlags1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtGnuHash"><code><strong class="tok-constructor">DtGnuHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li><li id="k-DtOther"><code><strong class="tok-constructor">DtOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfError</strong></code><span class="doc-sig-links"><a href="#t-ElfError" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Named errors reported when parsing or inspecting an ELF binary.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfTruncated"><code><strong class="tok-constructor">ElfTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfBadMagic"><code><strong class="tok-constructor">ElfBadMagic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfUnsupportedClass"><code><strong class="tok-constructor">ElfUnsupportedClass</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfUnsupportedData"><code><strong class="tok-constructor">ElfUnsupportedData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfUnsupportedVersion"><code><strong class="tok-constructor">ElfUnsupportedVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfInvalidHeader"><code><strong class="tok-constructor">ElfInvalidHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfInvalidSection"><code><strong class="tok-constructor">ElfInvalidSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfInvalidProgram"><code><strong class="tok-constructor">ElfInvalidProgram</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfNoSymbols"><code><strong class="tok-constructor">ElfNoSymbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfNobitsSection"><code><strong class="tok-constructor">ElfNobitsSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfInvalidDynTag"><code><strong class="tok-constructor">ElfInvalidDynTag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfUnsupportedCompression"><code><strong class="tok-constructor">ElfUnsupportedCompression</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li><li id="k-ElfDecompressFailed"><code><strong class="tok-constructor">ElfDecompressFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfFile</strong></code><span class="doc-sig-links"><a href="#t-ElfFile" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1061" title="View source at line 1061">Source</a></span></div>
<div class="doc-body">

Construct an ElfFile from its file header, program headers, section headers, and backing bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfFile"><code><strong class="tok-constructor">ElfFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfHeader</strong></code><span class="doc-sig-links"><a href="#t-ElfHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L788" title="View source at line 788">Source</a></span></div>
<div class="doc-body">

Construct an ElfHeader from class, data, version, OSABI, ABI version, byte order, type, machine, entry, flags.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfHeader"><code><strong class="tok-constructor">ElfHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfImportedSymbol">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfImportedSymbol</strong></code><span class="doc-sig-links"><a href="#t-ElfImportedSymbol" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L981" title="View source at line 981">Source</a></span></div>
<div class="doc-body">

Construct an ElfImportedSymbol from symbol name, version, and library name.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfImportedSymbol"><code><strong class="tok-constructor">ElfImportedSymbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfMachine">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfMachine</strong></code><span class="doc-sig-links"><a href="#t-ElfMachine" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

ELF machine architecture (e_machine).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfMachineNone"><code><strong class="tok-constructor">ElfMachineNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineSparc"><code><strong class="tok-constructor">ElfMachineSparc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachine386"><code><strong class="tok-constructor">ElfMachine386</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineMips"><code><strong class="tok-constructor">ElfMachineMips</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachinePpc"><code><strong class="tok-constructor">ElfMachinePpc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachinePpc64"><code><strong class="tok-constructor">ElfMachinePpc64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineS390"><code><strong class="tok-constructor">ElfMachineS390</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineArm"><code><strong class="tok-constructor">ElfMachineArm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineX8664"><code><strong class="tok-constructor">ElfMachineX8664</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineAarch64"><code><strong class="tok-constructor">ElfMachineAarch64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineRiscv"><code><strong class="tok-constructor">ElfMachineRiscv</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineLoongArch"><code><strong class="tok-constructor">ElfMachineLoongArch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li><li id="k-ElfMachineOther"><code><strong class="tok-constructor">ElfMachineOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfOsAbi">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfOsAbi</strong></code><span class="doc-sig-links"><a href="#t-ElfOsAbi" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

ELF OS/ABI identification (EI_OSABI).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfOsAbiNone"><code><strong class="tok-constructor">ElfOsAbiNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiNetBsd"><code><strong class="tok-constructor">ElfOsAbiNetBsd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiLinux"><code><strong class="tok-constructor">ElfOsAbiLinux</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiSolaris"><code><strong class="tok-constructor">ElfOsAbiSolaris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiFreeBsd"><code><strong class="tok-constructor">ElfOsAbiFreeBsd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiOpenBsd"><code><strong class="tok-constructor">ElfOsAbiOpenBsd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiArm"><code><strong class="tok-constructor">ElfOsAbiArm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiStandalone"><code><strong class="tok-constructor">ElfOsAbiStandalone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li><li id="k-ElfOsAbiOther"><code><strong class="tok-constructor">ElfOsAbiOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfProgHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfProgHeader</strong></code><span class="doc-sig-links"><a href="#t-ElfProgHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L833" title="View source at line 833">Source</a></span></div>
<div class="doc-body">

Construct an ElfProgHeader from type, flags, offset, vaddr, paddr, filesz, memsz, align.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfProgHeader"><code><strong class="tok-constructor">ElfProgHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfProgType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfProgType</strong></code><span class="doc-sig-links"><a href="#t-ElfProgType" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L352" title="View source at line 352">Source</a></span></div>
<div class="doc-body">

ELF program header segment type (p_type).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PtNull"><code><strong class="tok-constructor">PtNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtLoad"><code><strong class="tok-constructor">PtLoad</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtDynamic"><code><strong class="tok-constructor">PtDynamic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtInterp"><code><strong class="tok-constructor">PtInterp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtNote"><code><strong class="tok-constructor">PtNote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtShlib"><code><strong class="tok-constructor">PtShlib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtPhdr"><code><strong class="tok-constructor">PtPhdr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtTls"><code><strong class="tok-constructor">PtTls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtGnuEhFrame"><code><strong class="tok-constructor">PtGnuEhFrame</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtGnuStack"><code><strong class="tok-constructor">PtGnuStack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtGnuRelro"><code><strong class="tok-constructor">PtGnuRelro</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtGnuProperty"><code><strong class="tok-constructor">PtGnuProperty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li><li id="k-PtOther"><code><strong class="tok-constructor">PtOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfRel">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfRel</strong></code><span class="doc-sig-links"><a href="#t-ElfRel" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L998" title="View source at line 998">Source</a></span></div>
<div class="doc-body">

Construct an ElfRel from offset, raw info, symbol index, and relocation type.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfRel"><code><strong class="tok-constructor">ElfRel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfRela">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfRela</strong></code><span class="doc-sig-links"><a href="#t-ElfRela" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1019" title="View source at line 1019">Source</a></span></div>
<div class="doc-body">

Construct an ElfRela from offset, raw info, signed addend, symbol index, and relocation type.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfRela"><code><strong class="tok-constructor">ElfRela</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSectionHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSectionHeader</strong></code><span class="doc-sig-links"><a href="#t-ElfSectionHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L877" title="View source at line 877">Source</a></span></div>
<div class="doc-body">

Construct an ElfSectionHeader from name, type, flags, addr, offset, size, link, info, addralign, entsize, file-size, compression.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfSectionHeader"><code><strong class="tok-constructor">ElfSectionHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">ElfCompressionInfo</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSectionType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSectionType</strong></code><span class="doc-sig-links"><a href="#t-ElfSectionType" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

ELF section header type (sh_type).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ShtNull"><code><strong class="tok-constructor">ShtNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtProgbits"><code><strong class="tok-constructor">ShtProgbits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtSymtab"><code><strong class="tok-constructor">ShtSymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtStrtab"><code><strong class="tok-constructor">ShtStrtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtRela"><code><strong class="tok-constructor">ShtRela</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtHash"><code><strong class="tok-constructor">ShtHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtDynamic"><code><strong class="tok-constructor">ShtDynamic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtNote"><code><strong class="tok-constructor">ShtNote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtNobits"><code><strong class="tok-constructor">ShtNobits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtRel"><code><strong class="tok-constructor">ShtRel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtShlib"><code><strong class="tok-constructor">ShtShlib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtDynsym"><code><strong class="tok-constructor">ShtDynsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtInitArray"><code><strong class="tok-constructor">ShtInitArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtFiniArray"><code><strong class="tok-constructor">ShtFiniArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtPreinitArray"><code><strong class="tok-constructor">ShtPreinitArray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtGroup"><code><strong class="tok-constructor">ShtGroup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtSymtabShndx"><code><strong class="tok-constructor">ShtSymtabShndx</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtGnuHash"><code><strong class="tok-constructor">ShtGnuHash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtGnuVerdef"><code><strong class="tok-constructor">ShtGnuVerdef</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtGnuVerneed"><code><strong class="tok-constructor">ShtGnuVerneed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtGnuVersym"><code><strong class="tok-constructor">ShtGnuVersym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li><li id="k-ShtOther"><code><strong class="tok-constructor">ShtOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSymBind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymBind</strong></code><span class="doc-sig-links"><a href="#t-ElfSymBind" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L514" title="View source at line 514">Source</a></span></div>
<div class="doc-body">

Symbol binding extracted from the upper nibble of st_info.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-StbLocal"><code><strong class="tok-constructor">StbLocal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li><li id="k-StbGlobal"><code><strong class="tok-constructor">StbGlobal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li><li id="k-StbWeak"><code><strong class="tok-constructor">StbWeak</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li><li id="k-StbOther"><code><strong class="tok-constructor">StbOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSymType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymType</strong></code><span class="doc-sig-links"><a href="#t-ElfSymType" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L533" title="View source at line 533">Source</a></span></div>
<div class="doc-body">

Symbol type extracted from the lower nibble of st_info.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SttNotype"><code><strong class="tok-constructor">SttNotype</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttObject"><code><strong class="tok-constructor">SttObject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttFunc"><code><strong class="tok-constructor">SttFunc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttSection"><code><strong class="tok-constructor">SttSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttFile"><code><strong class="tok-constructor">SttFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttCommon"><code><strong class="tok-constructor">SttCommon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttTls"><code><strong class="tok-constructor">SttTls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li><li id="k-SttOther"><code><strong class="tok-constructor">SttOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSymVis">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymVis</strong></code><span class="doc-sig-links"><a href="#t-ElfSymVis" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L564" title="View source at line 564">Source</a></span></div>
<div class="doc-body">

Symbol visibility extracted from the low two bits of st_other.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-StvDefault"><code><strong class="tok-constructor">StvDefault</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li><li id="k-StvInternal"><code><strong class="tok-constructor">StvInternal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li><li id="k-StvHidden"><code><strong class="tok-constructor">StvHidden</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li><li id="k-StvProtected"><code><strong class="tok-constructor">StvProtected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfSymbol">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfSymbol</strong></code><span class="doc-sig-links"><a href="#t-ElfSymbol" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L940" title="View source at line 940">Source</a></span></div>
<div class="doc-body">

Construct an ElfSymbol from name, info, other, section index, value, size.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfSymbol"><code><strong class="tok-constructor">ElfSymbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ElfType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ElfType</strong></code><span class="doc-sig-links"><a href="#t-ElfType" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

ELF object file type (e_type).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ElfTypeNone"><code><strong class="tok-constructor">ElfTypeNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li id="k-ElfTypeRel"><code><strong class="tok-constructor">ElfTypeRel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li id="k-ElfTypeExec"><code><strong class="tok-constructor">ElfTypeExec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li id="k-ElfTypeDyn"><code><strong class="tok-constructor">ElfTypeDyn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li id="k-ElfTypeCore"><code><strong class="tok-constructor">ElfTypeCore</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li><li id="k-ElfTypeOther"><code><strong class="tok-constructor">ElfTypeOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-elf-abi-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-abi-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-abi-version" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L807" title="View source at line 807">Source</a></span></div>
<div class="doc-body">

ELF ABI version byte.

</div>
</article>

<article class="doc-entry" id="v-elf-byte-order">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-byte-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code><span class="doc-sig-links"><a href="#v-elf-byte-order" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L811" title="View source at line 811">Source</a></span></div>
<div class="doc-body">

Byte order used to decode multi-byte integers in this ELF file.

</div>
</article>

<article class="doc-entry" id="v-elf-class">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-class</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a></code><span class="doc-sig-links"><a href="#v-elf-class" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L791" title="View source at line 791">Source</a></span></div>
<div class="doc-body">

ELF class (32-bit or 64-bit).

</div>
</article>

<article class="doc-entry" id="v-elf-class-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-class-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfClass" title="Data.Elf.ElfClass">ElfClass</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-class-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Numeric EI_CLASS value for an ElfClass.

</div>
</article>

<article class="doc-entry" id="v-elf-compress-zlib">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-compress-zlib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-compress-zlib" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Zlib compression identifier (ELFCOMPRESS_ZLIB).

</div>
</article>

<article class="doc-entry" id="v-elf-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a></code><span class="doc-sig-links"><a href="#v-elf-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L795" title="View source at line 795">Source</a></span></div>
<div class="doc-body">

ELF data encoding (LSB or MSB).

</div>
</article>

<article class="doc-entry" id="v-elf-data-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-data-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfData" title="Data.Elf.ElfData">ElfData</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-data-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

Numeric EI_DATA value for an ElfData.

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-raw-tag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-raw-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-dyn-raw-tag" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1051" title="View source at line 1051">Source</a></span></div>
<div class="doc-body">

Raw numeric d_tag value.

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-elf-dyn-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1912" title="View source at line 1912">Source</a></span></div>
<div class="doc-body">

Return the strings listed for a string-valued dynamic tag (DtNeeded, DtSoname, DtRpath, DtRunpath).

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-tag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a></code><span class="doc-sig-links"><a href="#v-elf-dyn-tag" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1047" title="View source at line 1047">Source</a></span></div>
<div class="doc-body">

Decoded dynamic tag.

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-tag-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-tag-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-dyn-tag-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L728" title="View source at line 728">Source</a></span></div>
<div class="doc-body">

Numeric d_tag code for an ElfDynTag.

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-val">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-val</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-dyn-val" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1055" title="View source at line 1055">Source</a></span></div>
<div class="doc-body">

Entry value or virtual address (d_val / d_ptr).

</div>
</article>

<article class="doc-entry" id="v-elf-dyn-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dyn-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynTag" title="Data.Elf.ElfDynTag">ElfDynTag</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-elf-dyn-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1887" title="View source at line 1887">Source</a></span></div>
<div class="doc-body">

Return all values listed for the given tag in the SHT_DYNAMIC section.

</div>
</article>

<article class="doc-entry" id="v-elf-dynamic-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dynamic-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfDynEntry" title="Data.Elf.ElfDynEntry">ElfDynEntry</a>)</code><span class="doc-sig-links"><a href="#v-elf-dynamic-entries" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1865" title="View source at line 1865">Source</a></span></div>
<div class="doc-body">

Return all dynamic entries from the SHT_DYNAMIC section, or empty list if none exists.

</div>
</article>

<article class="doc-entry" id="v-elf-dynamic-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-dynamic-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a>)</code><span class="doc-sig-links"><a href="#v-elf-dynamic-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1759" title="View source at line 1759">Source</a></span></div>
<div class="doc-body">

Return all symbols from the SHT_DYNSYM section, omitting the dummy symbol at index 0.

</div>
</article>

<article class="doc-entry" id="v-elf-entry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-entry" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L823" title="View source at line 823">Source</a></span></div>
<div class="doc-body">

Virtual address of the program entry point.

</div>
</article>

<article class="doc-entry" id="v-elf-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Render a human-readable diagnostic for an ELF error.

</div>
</article>

<article class="doc-entry" id="v-elf-file-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-file-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a></code><span class="doc-sig-links"><a href="#v-elf-file-header" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1064" title="View source at line 1064">Source</a></span></div>
<div class="doc-body">

File header of a parsed ELF binary.

</div>
</article>

<article class="doc-entry" id="v-elf-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-flags" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L827" title="View source at line 827">Source</a></span></div>
<div class="doc-body">

Processor-specific header flags.

</div>
</article>

<article class="doc-entry" id="v-elf-imported-libraries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-elf-imported-libraries" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1926" title="View source at line 1926">Source</a></span></div>
<div class="doc-body">

Return the shared library names listed by DT_NEEDED entries.

</div>
</article>

<article class="doc-entry" id="v-elf-imported-library">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-imported-library</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-imported-library" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L992" title="View source at line 992">Source</a></span></div>
<div class="doc-body">

Associated defining library name, or empty when unversioned.

</div>
</article>

<article class="doc-entry" id="v-elf-imported-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-imported-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-imported-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L984" title="View source at line 984">Source</a></span></div>
<div class="doc-body">

Imported symbol name.

</div>
</article>

<article class="doc-entry" id="v-elf-imported-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a>)</code><span class="doc-sig-links"><a href="#v-elf-imported-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1939" title="View source at line 1939">Source</a></span></div>
<div class="doc-body">

Return all global undefined symbols from SHT_DYNSYM expected to be resolved at dynamic load time.

</div>
</article>

<article class="doc-entry" id="v-elf-imported-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-imported-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfImportedSymbol" title="Data.Elf.ElfImportedSymbol">ElfImportedSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-imported-version" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L988" title="View source at line 988">Source</a></span></div>
<div class="doc-body">

Associated symbol version string, or empty when unversioned.

</div>
</article>

<article class="doc-entry" id="v-elf-machine">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-machine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a></code><span class="doc-sig-links"><a href="#v-elf-machine" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L819" title="View source at line 819">Source</a></span></div>
<div class="doc-body">

Machine architecture.

</div>
</article>

<article class="doc-entry" id="v-elf-machine-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-machine-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfMachine" title="Data.Elf.ElfMachine">ElfMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-elf-machine-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L288" title="View source at line 288">Source</a></span></div>
<div class="doc-body">

Numeric e_machine code for an ElfMachine.

</div>
</article>

<article class="doc-entry" id="v-elf-osabi">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-osabi</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a></code><span class="doc-sig-links"><a href="#v-elf-osabi" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L803" title="View source at line 803">Source</a></span></div>
<div class="doc-body">

ELF OS/ABI identification.

</div>
</article>

<article class="doc-entry" id="v-elf-osabi-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-osabi-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfOsAbi" title="Data.Elf.ElfOsAbi">ElfOsAbi</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-osabi-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

Numeric EI_OSABI code for an ElfOsAbi.

</div>
</article>

<article class="doc-entry" id="v-elf-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a></code><span class="doc-sig-links"><a href="#v-elf-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1560" title="View source at line 1560">Source</a></span></div>
<div class="doc-body">

Parse an ELF32 or ELF64 binary from raw bytes.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-align">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-align" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L864" title="View source at line 864">Source</a></span></div>
<div class="doc-body">

Segment alignment in bytes.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-elf-prog-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1662" title="View source at line 1662">Source</a></span></div>
<div class="doc-body">

Read the file-backed bytes of a program header segment.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-filesz">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-filesz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-filesz" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L856" title="View source at line 856">Source</a></span></div>
<div class="doc-body">

Segment size in the file in bytes.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-prog-flags" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L840" title="View source at line 840">Source</a></span></div>
<div class="doc-body">

Segment permission flags bitmask (pf-r, pf-w, pf-x).

</div>
</article>

<article class="doc-entry" id="v-elf-prog-memsz">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-memsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-memsz" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L860" title="View source at line 860">Source</a></span></div>
<div class="doc-body">

Segment size in memory in bytes.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L844" title="View source at line 844">Source</a></span></div>
<div class="doc-body">

File byte offset of the segment data.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-paddr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-paddr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-paddr" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L852" title="View source at line 852">Source</a></span></div>
<div class="doc-body">

Physical memory address of the segment.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a></code><span class="doc-sig-links"><a href="#v-elf-prog-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L836" title="View source at line 836">Source</a></span></div>
<div class="doc-body">

Segment type.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-type-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgType" title="Data.Elf.ElfProgType">ElfProgType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-prog-type-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L398" title="View source at line 398">Source</a></span></div>
<div class="doc-body">

Numeric p_type code for an ElfProgType.

</div>
</article>

<article class="doc-entry" id="v-elf-prog-vaddr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-prog-vaddr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-prog-vaddr" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L848" title="View source at line 848">Source</a></span></div>
<div class="doc-body">

Virtual memory address of the segment.

</div>
</article>

<article class="doc-entry" id="v-elf-progs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-progs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfProgHeader" title="Data.Elf.ElfProgHeader">ElfProgHeader</a></code><span class="doc-sig-links"><a href="#v-elf-progs" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1068" title="View source at line 1068">Source</a></span></div>
<div class="doc-body">

Program headers (segments) of a parsed ELF binary.

</div>
</article>

<article class="doc-entry" id="v-elf-r-sym32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-r-sym32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-r-sym32" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L766" title="View source at line 766">Source</a></span></div>
<div class="doc-body">

Extract the symbol index from a 32-bit relocation r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-r-sym64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-r-sym64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-r-sym64" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L774" title="View source at line 774">Source</a></span></div>
<div class="doc-body">

Extract the symbol index from a 64-bit relocation r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-r-type32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-r-type32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-r-type32" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L770" title="View source at line 770">Source</a></span></div>
<div class="doc-body">

Extract the relocation type from a 32-bit relocation r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-r-type64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-r-type64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-r-type64" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L778" title="View source at line 778">Source</a></span></div>
<div class="doc-body">

Extract the relocation type from a 64-bit relocation r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-rel-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rel-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-rel-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1005" title="View source at line 1005">Source</a></span></div>
<div class="doc-body">

Raw r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-rel-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rel-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-rel-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1001" title="View source at line 1001">Source</a></span></div>
<div class="doc-body">

Offset to apply the relocation.

</div>
</article>

<article class="doc-entry" id="v-elf-rel-sym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rel-sym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-rel-sym" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1009" title="View source at line 1009">Source</a></span></div>
<div class="doc-body">

Symbol table index referenced by the relocation.

</div>
</article>

<article class="doc-entry" id="v-elf-rel-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rel-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-rel-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1013" title="View source at line 1013">Source</a></span></div>
<div class="doc-body">

Architecture-specific relocation type code.

</div>
</article>

<article class="doc-entry" id="v-elf-rela-addend">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rela-addend</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-elf-rela-addend" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1030" title="View source at line 1030">Source</a></span></div>
<div class="doc-body">

Signed constant addend.

</div>
</article>

<article class="doc-entry" id="v-elf-rela-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rela-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-rela-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1026" title="View source at line 1026">Source</a></span></div>
<div class="doc-body">

Raw r_info word.

</div>
</article>

<article class="doc-entry" id="v-elf-rela-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rela-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-rela-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1022" title="View source at line 1022">Source</a></span></div>
<div class="doc-body">

Offset to apply the relocation.

</div>
</article>

<article class="doc-entry" id="v-elf-rela-sym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rela-sym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-rela-sym" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1034" title="View source at line 1034">Source</a></span></div>
<div class="doc-body">

Symbol table index referenced by the relocation.

</div>
</article>

<article class="doc-entry" id="v-elf-rela-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-rela-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-rela-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1038" title="View source at line 1038">Source</a></span></div>
<div class="doc-body">

Architecture-specific relocation type code.

</div>
</article>

<article class="doc-entry" id="v-elf-relocations-rel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-relocations-rel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRel" title="Data.Elf.ElfRel">ElfRel</a>)</code><span class="doc-sig-links"><a href="#v-elf-relocations-rel" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1788" title="View source at line 1788">Source</a></span></div>
<div class="doc-body">

Parse all SHT_REL entries from a relocation section.

</div>
</article>

<article class="doc-entry" id="v-elf-relocations-rela">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-relocations-rela</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfRela" title="Data.Elf.ElfRela">ElfRela</a>)</code><span class="doc-sig-links"><a href="#v-elf-relocations-rela" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1827" title="View source at line 1827">Source</a></span></div>
<div class="doc-body">

Parse all SHT_RELA entries from a relocation section.

</div>
</article>

<article class="doc-entry" id="v-elf-section">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code><span class="doc-sig-links"><a href="#v-elf-section" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1607" title="View source at line 1607">Source</a></span></div>
<div class="doc-body">

Find the first section with the given name.

</div>
</article>

<article class="doc-entry" id="v-elf-section-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L892" title="View source at line 892">Source</a></span></div>
<div class="doc-body">

Virtual address of the section in memory.

</div>
</article>

<article class="doc-entry" id="v-elf-section-addralign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-addralign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-addralign" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L912" title="View source at line 912">Source</a></span></div>
<div class="doc-body">

Required alignment in bytes (uncompressed alignment when compressed).

</div>
</article>

<article class="doc-entry" id="v-elf-section-by-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-by-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code><span class="doc-sig-links"><a href="#v-elf-section-by-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1619" title="View source at line 1619">Source</a></span></div>
<div class="doc-body">

Find the first section matching the given section type.

</div>
</article>

<article class="doc-entry" id="v-elf-section-compressed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-elf-section-compressed" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L924" title="View source at line 924">Source</a></span></div>
<div class="doc-body">

Whether the section carries SHF_COMPRESSED.

</div>
</article>

<article class="doc-entry" id="v-elf-section-compression-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-compression-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-section-compression-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L931" title="View source at line 931">Source</a></span></div>
<div class="doc-body">

Compression type code (0 when uncompressed, 1 for zlib).

</div>
</article>

<article class="doc-entry" id="v-elf-section-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-elf-section-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1631" title="View source at line 1631">Source</a></span></div>
<div class="doc-body">

Read the contents of an ELF section, automatically decompressing zlib SHF_COMPRESSED sections.

</div>
</article>

<article class="doc-entry" id="v-elf-section-entsize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-entsize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-entsize" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L916" title="View source at line 916">Source</a></span></div>
<div class="doc-body">

Entry size in bytes for fixed-size tables.

</div>
</article>

<article class="doc-entry" id="v-elf-section-file-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-file-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-file-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L920" title="View source at line 920">Source</a></span></div>
<div class="doc-body">

Number of bytes occupied by the section in the file.

</div>
</article>

<article class="doc-entry" id="v-elf-section-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-flags" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L888" title="View source at line 888">Source</a></span></div>
<div class="doc-body">

Section flags bitmask.

</div>
</article>

<article class="doc-entry" id="v-elf-section-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-section-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L908" title="View source at line 908">Source</a></span></div>
<div class="doc-body">

Extra section-specific information (sh_info).

</div>
</article>

<article class="doc-entry" id="v-elf-section-link">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-section-link" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L904" title="View source at line 904">Source</a></span></div>
<div class="doc-body">

Linked section header index (sh_link).

</div>
</article>

<article class="doc-entry" id="v-elf-section-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-section-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L880" title="View source at line 880">Source</a></span></div>
<div class="doc-body">

Resolved section name from the section header string table.

</div>
</article>

<article class="doc-entry" id="v-elf-section-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L896" title="View source at line 896">Source</a></span></div>
<div class="doc-body">

File byte offset of the section.

</div>
</article>

<article class="doc-entry" id="v-elf-section-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-section-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L900" title="View source at line 900">Source</a></span></div>
<div class="doc-body">

Logical section size in bytes (uncompressed size when SHF_COMPRESSED is set).

</div>
</article>

<article class="doc-entry" id="v-elf-section-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a></code><span class="doc-sig-links"><a href="#v-elf-section-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L884" title="View source at line 884">Source</a></span></div>
<div class="doc-body">

Section type.

</div>
</article>

<article class="doc-entry" id="v-elf-section-type-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-section-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionType" title="Data.Elf.ElfSectionType">ElfSectionType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-section-type-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L488" title="View source at line 488">Source</a></span></div>
<div class="doc-body">

Numeric sh_type code for an ElfSectionType.

</div>
</article>

<article class="doc-entry" id="v-elf-sections">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSectionHeader" title="Data.Elf.ElfSectionHeader">ElfSectionHeader</a></code><span class="doc-sig-links"><a href="#v-elf-sections" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1072" title="View source at line 1072">Source</a></span></div>
<div class="doc-body">

Section headers of a parsed ELF binary.

</div>
</article>

<article class="doc-entry" id="v-elf-st-bind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-st-bind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code><span class="doc-sig-links"><a href="#v-elf-st-bind" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L583" title="View source at line 583">Source</a></span></div>
<div class="doc-body">

Extract the symbol binding from st_info (ST_BIND).

</div>
</article>

<article class="doc-entry" id="v-elf-st-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-st-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-st-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L614" title="View source at line 614">Source</a></span></div>
<div class="doc-body">

Combine a symbol binding and symbol type into st_info (ST_INFO).

</div>
</article>

<article class="doc-entry" id="v-elf-st-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-st-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code><span class="doc-sig-links"><a href="#v-elf-st-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L592" title="View source at line 592">Source</a></span></div>
<div class="doc-body">

Extract the symbol type from st_info (ST_TYPE).

</div>
</article>

<article class="doc-entry" id="v-elf-st-visibility">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-st-visibility</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code><span class="doc-sig-links"><a href="#v-elf-st-visibility" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

Extract the symbol visibility from st_other (ST_VISIBILITY).

</div>
</article>

<article class="doc-entry" id="v-elf-sym-bind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-bind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a></code><span class="doc-sig-links"><a href="#v-elf-sym-bind" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L967" title="View source at line 967">Source</a></span></div>
<div class="doc-body">

Decoded symbol binding.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-bind-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-bind-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymBind" title="Data.Elf.ElfSymBind">ElfSymBind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-sym-bind-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L525" title="View source at line 525">Source</a></span></div>
<div class="doc-body">

Numeric binding value for an ElfSymBind.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-sym-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L947" title="View source at line 947">Source</a></span></div>
<div class="doc-body">

Raw st_info byte encoding binding and type.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-elf-sym-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L943" title="View source at line 943">Source</a></span></div>
<div class="doc-body">

Symbol name from the linked string table.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-other">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-other</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-sym-other" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L951" title="View source at line 951">Source</a></span></div>
<div class="doc-body">

Raw st_other byte encoding symbol visibility.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-section">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-elf-sym-section" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L955" title="View source at line 955">Source</a></span></div>
<div class="doc-body">

Section header index (or special index such as shn-undef or shn-abs).

</div>
</article>

<article class="doc-entry" id="v-elf-sym-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-sym-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L963" title="View source at line 963">Source</a></span></div>
<div class="doc-body">

Symbol size in bytes.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a></code><span class="doc-sig-links"><a href="#v-elf-sym-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L971" title="View source at line 971">Source</a></span></div>
<div class="doc-body">

Decoded symbol type.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-type-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymType" title="Data.Elf.ElfSymType">ElfSymType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-sym-type-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L552" title="View source at line 552">Source</a></span></div>
<div class="doc-body">

Numeric type value for an ElfSymType.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-elf-sym-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L959" title="View source at line 959">Source</a></span></div>
<div class="doc-body">

Symbol value (address or offset).

</div>
</article>

<article class="doc-entry" id="v-elf-sym-vis-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-vis-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-sym-vis-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L575" title="View source at line 575">Source</a></span></div>
<div class="doc-body">

Numeric visibility code for an ElfSymVis.

</div>
</article>

<article class="doc-entry" id="v-elf-sym-visibility">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-sym-visibility</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymVis" title="Data.Elf.ElfSymVis">ElfSymVis</a></code><span class="doc-sig-links"><a href="#v-elf-sym-visibility" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L975" title="View source at line 975">Source</a></span></div>
<div class="doc-body">

Decoded symbol visibility.

</div>
</article>

<article class="doc-entry" id="v-elf-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfFile" title="Data.Elf.ElfFile">ElfFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfError" title="Data.Elf.ElfError">ElfError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfSymbol" title="Data.Elf.ElfSymbol">ElfSymbol</a>)</code><span class="doc-sig-links"><a href="#v-elf-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L1755" title="View source at line 1755">Source</a></span></div>
<div class="doc-body">

Return all symbols from the SHT_SYMTAB section, omitting the dummy symbol at index 0.

</div>
</article>

<article class="doc-entry" id="v-elf-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a></code><span class="doc-sig-links"><a href="#v-elf-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L815" title="View source at line 815">Source</a></span></div>
<div class="doc-body">

Object file type (relocatable, executable, shared object, core).

</div>
</article>

<article class="doc-entry" id="v-elf-type-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfType" title="Data.Elf.ElfType">ElfType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-elf-type-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Numeric e_type code for an ElfType.

</div>
</article>

<article class="doc-entry" id="v-elf-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elf-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Elf/#t-ElfHeader" title="Data.Elf.ElfHeader">ElfHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-elf-version" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L799" title="View source at line 799">Source</a></span></div>
<div class="doc-body">

ELF identification version byte.

</div>
</article>

<article class="doc-entry" id="v-pf-r">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pf-r</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pf-r" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Readable permission flag on a program header (PF_R).

</div>
</article>

<article class="doc-entry" id="v-pf-w">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pf-w</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pf-w" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Writable permission flag on a program header (PF_W).

</div>
</article>

<article class="doc-entry" id="v-pf-x">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pf-x</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pf-x" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Executable permission flag on a program header (PF_X).

</div>
</article>

<article class="doc-entry" id="v-shf-alloc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-alloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-alloc" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Section occupies memory during execution (SHF_ALLOC).

</div>
</article>

<article class="doc-entry" id="v-shf-compressed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-compressed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-compressed" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

Section data is compressed with a Chdr prefix (SHF_COMPRESSED).

</div>
</article>

<article class="doc-entry" id="v-shf-execinstr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-execinstr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-execinstr" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Section contains executable instructions (SHF_EXECINSTR).

</div>
</article>

<article class="doc-entry" id="v-shf-group">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-group</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-group" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Section is a member of a section group (SHF_GROUP).

</div>
</article>

<article class="doc-entry" id="v-shf-info-link">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-info-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-info-link" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Section sh_info holds a section header index (SHF_INFO_LINK).

</div>
</article>

<article class="doc-entry" id="v-shf-link-order">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-link-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-link-order" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Section preserves special ordering requirements (SHF_LINK_ORDER).

</div>
</article>

<article class="doc-entry" id="v-shf-merge">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-merge" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Section elements may be merged (SHF_MERGE).

</div>
</article>

<article class="doc-entry" id="v-shf-strings">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-strings</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-strings" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Section contains NUL-terminated strings (SHF_STRINGS).

</div>
</article>

<article class="doc-entry" id="v-shf-tls">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-tls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-tls" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Section holds thread-local storage (SHF_TLS).

</div>
</article>

<article class="doc-entry" id="v-shf-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shf-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-shf-write" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Section contains writable data (SHF_WRITE).

</div>
</article>

<article class="doc-entry" id="v-shn-abs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shn-abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-shn-abs" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Absolute symbol section index (SHN_ABS).

</div>
</article>

<article class="doc-entry" id="v-shn-common">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shn-common</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-shn-common" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Common symbol section index (SHN_COMMON).

</div>
</article>

<article class="doc-entry" id="v-shn-undef">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shn-undef</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-shn-undef" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Undefined section index (SHN_UNDEF).

</div>
</article>

<article class="doc-entry" id="v-shn-xindex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shn-xindex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-shn-xindex" title="Permalink">#</a><a href="/packages/stdlib/Data.Elf/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Extended section index escape value (SHN_XINDEX).

</div>
</article>
