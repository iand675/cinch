# Data.Pe

Bounds-checked PE32, PE32+ and COFF binary parser for headers, data directories, sections, COFF symbols, imports and exports.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Pe</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Pe/source/">View source</a><a href="/packages/search/?q=%2BData.Pe">Search module</a><a href="/reference-sources/Data.Pe.cinch" download="Data.Pe.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure bounds-checked parser for PE32, PE32+, and COFF object/executable files.
Supports DOS header and PE signature validation, COFF file headers, PE32 and PE32+
optional headers with data directories, section headers and section data, COFF symbol
and string tables (including auxiliary record skipping and long /offset section names),
import directories and imported libraries/symbols, and export directories.

</details>

<details class="doc-synopsis"><summary>Synopsis (123 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PeDataDirectory">PeDataDirectory</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeDataDirectory</strong></code></li><li><a class="syn-name" href="#t-PeError">PeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeError</strong></code></li><li><a class="syn-name" href="#t-PeExport">PeExport</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeExport</strong></code></li><li><a class="syn-name" href="#t-PeExportDirectory">PeExportDirectory</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeExportDirectory</strong></code></li><li><a class="syn-name" href="#t-PeFile">PeFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeFile</strong></code></li><li><a class="syn-name" href="#t-PeFileHeader">PeFileHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeFileHeader</strong></code></li><li><a class="syn-name" href="#t-PeImportDirectory">PeImportDirectory</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeImportDirectory</strong></code></li><li><a class="syn-name" href="#t-PeImportSymbol">PeImportSymbol</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeImportSymbol</strong></code></li><li><a class="syn-name" href="#t-PeMachine">PeMachine</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeMachine</strong></code></li><li><a class="syn-name" href="#t-PeOptionalHeader">PeOptionalHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader</strong></code></li><li><a class="syn-name" href="#t-PeOptionalHeader32">PeOptionalHeader32</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader32</strong></code></li><li><a class="syn-name" href="#t-PeOptionalHeader64">PeOptionalHeader64</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader64</strong></code></li><li><a class="syn-name" href="#t-PeSection">PeSection</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSection</strong></code></li><li><a class="syn-name" href="#t-PeSubsystem">PeSubsystem</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSubsystem</strong></code></li><li><a class="syn-name" href="#t-PeSymbol">PeSymbol</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSymbol</strong></code></li><li><a class="syn-name" href="#v-pe-data-directory">pe-data-directory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-data-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code></li><li><a class="syn-name" href="#v-pe-dd-size">pe-dd-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dd-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-dd-virtual-address">pe-dd-virtual-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dd-virtual-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-dir-basereloc">pe-dir-basereloc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-basereloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-clr">pe-dir-clr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-clr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-debug">pe-dir-debug</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-debug</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-exception">pe-dir-exception</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-exception</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-export">pe-dir-export</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-export</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-iat">pe-dir-iat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-iat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-import">pe-dir-import</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-import</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-resource">pe-dir-resource</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-resource</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-security">pe-dir-security</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-security</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-dir-tls">pe-dir-tls</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-dir-tls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pe-error-message">pe-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-expdir-base">pe-expdir-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-expdir-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-expdir-entries">pe-expdir-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-expdir-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a></code></li><li><a class="syn-name" href="#v-pe-expdir-name">pe-expdir-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-expdir-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-expdir-num-functions">pe-expdir-num-functions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-expdir-num-functions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-expdir-num-names">pe-expdir-num-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-expdir-num-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-export-forwarder">pe-export-forwarder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-export-forwarder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-export-name">pe-export-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-export-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-export-ordinal">pe-export-ordinal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-export-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-export-rva">pe-export-rva</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-export-rva</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-exports">pe-exports</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-exports</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a>)</code></li><li><a class="syn-name" href="#v-pe-fh-characteristics">pe-fh-characteristics</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-fh-machine">pe-fh-machine</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-machine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li><a class="syn-name" href="#v-pe-fh-num-sections">pe-fh-num-sections</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-num-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-fh-num-symbols">pe-fh-num-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-num-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-fh-optional-header-size">pe-fh-optional-header-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-optional-header-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-fh-symbol-table-ptr">pe-fh-symbol-table-ptr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-symbol-table-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-fh-timestamp">pe-fh-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-fh-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-file-dll">pe-file-dll</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-file-executable-image">pe-file-executable-image</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-executable-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-file-header">pe-file-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a></code></li><li><a class="syn-name" href="#v-pe-file-large-address-aware">pe-file-large-address-aware</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-large-address-aware</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-file-machine32">pe-file-machine32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-machine32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-file-relocs-stripped">pe-file-relocs-stripped</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-file-relocs-stripped</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-imp-dll">pe-imp-dll</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-imp-first-thunk">pe-imp-first-thunk</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-first-thunk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-imp-forwarder-chain">pe-imp-forwarder-chain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-forwarder-chain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-imp-name-rva">pe-imp-name-rva</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-name-rva</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-imp-orig-first-thunk">pe-imp-orig-first-thunk</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-orig-first-thunk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-imp-timestamp">pe-imp-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imp-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-import-directories">pe-import-directories</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-import-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a>)</code></li><li><a class="syn-name" href="#v-pe-imported-libraries">pe-imported-libraries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-pe-imported-symbol-entries">pe-imported-symbol-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imported-symbol-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a>)</code></li><li><a class="syn-name" href="#v-pe-imported-symbols">pe-imported-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-pe-impsym-by-ordinal">pe-impsym-by-ordinal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-impsym-by-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pe-impsym-dll">pe-impsym-dll</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-impsym-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-impsym-hint">pe-impsym-hint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-impsym-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-impsym-name">pe-impsym-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-impsym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-impsym-ordinal">pe-impsym-ordinal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-impsym-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-machine-code">pe-machine-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-machine-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-magic-pe32">pe-magic-pe32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-magic-pe32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-magic-pe64">pe-magic-pe64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-magic-pe64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-opt32-checksum">pe-opt32-checksum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-checksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-data-directories">pe-opt32-data-directories</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-data-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code></li><li><a class="syn-name" href="#v-pe-opt32-dll-characteristics">pe-opt32-dll-characteristics</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-dll-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-opt32-entry-point">pe-opt32-entry-point</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-entry-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-file-alignment">pe-opt32-file-alignment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-file-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-image-base">pe-opt32-image-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-image-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-magic">pe-opt32-magic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-opt32-num-rva-and-sizes">pe-opt32-num-rva-and-sizes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-num-rva-and-sizes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-section-alignment">pe-opt32-section-alignment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-section-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-size-of-headers">pe-opt32-size-of-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-size-of-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-size-of-image">pe-opt32-size-of-image</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-size-of-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt32-subsystem">pe-opt32-subsystem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt32-subsystem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li><a class="syn-name" href="#v-pe-opt64-checksum">pe-opt64-checksum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-checksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-data-directories">pe-opt64-data-directories</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-data-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code></li><li><a class="syn-name" href="#v-pe-opt64-dll-characteristics">pe-opt64-dll-characteristics</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-dll-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-opt64-entry-point">pe-opt64-entry-point</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-entry-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-file-alignment">pe-opt64-file-alignment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-file-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-image-base">pe-opt64-image-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-image-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-pe-opt64-magic">pe-opt64-magic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-opt64-num-rva-and-sizes">pe-opt64-num-rva-and-sizes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-num-rva-and-sizes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-section-alignment">pe-opt64-section-alignment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-section-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-size-of-headers">pe-opt64-size-of-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-size-of-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-size-of-image">pe-opt64-size-of-image</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-size-of-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-opt64-subsystem">pe-opt64-subsystem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-opt64-subsystem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li><a class="syn-name" href="#v-pe-optional-header">pe-optional-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-optional-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a></code></li><li><a class="syn-name" href="#v-pe-parse">pe-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a></code></li><li><a class="syn-name" href="#v-pe-scn-cnt-code">pe-scn-cnt-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-cnt-initialized-data">pe-scn-cnt-initialized-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-initialized-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-cnt-uninitialized-data">pe-scn-cnt-uninitialized-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-uninitialized-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-mem-discardable">pe-scn-mem-discardable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-discardable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-mem-execute">pe-scn-mem-execute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-mem-read">pe-scn-mem-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-scn-mem-write">pe-scn-mem-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-characteristics">pe-sec-characteristics</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-line-ptr">pe-sec-line-ptr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-line-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-name">pe-sec-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-sec-num-lines">pe-sec-num-lines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-num-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-sec-num-relocs">pe-sec-num-relocs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-num-relocs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-sec-offset">pe-sec-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-reloc-ptr">pe-sec-reloc-ptr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-reloc-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-size">pe-sec-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-virtual-address">pe-sec-virtual-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-virtual-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-sec-virtual-size">pe-sec-virtual-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sec-virtual-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-section">pe-section</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a></code></li><li><a class="syn-name" href="#v-pe-section-data">pe-section-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pe-sections">pe-sections</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a></code></li><li><a class="syn-name" href="#v-pe-subsystem-code">pe-subsystem-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-subsystem-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-sym-name">pe-sym-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pe-sym-section-number">pe-sym-section-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sym-section-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code></li><li><a class="syn-name" href="#v-pe-sym-storage-class">pe-sym-storage-class</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sym-storage-class</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-pe-sym-type">pe-sym-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-pe-sym-value">pe-sym-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-sym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-pe-symbols">pe-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pe-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PeDataDirectory">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeDataDirectory</strong></code><span class="doc-sig-links"><a href="#t-PeDataDirectory" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L303" title="View source at line 303">Source</a></span></div>
<div class="doc-body">

Construct a PeDataDirectory from relative virtual address and byte size.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeDataDirectory"><code><strong class="tok-constructor">PeDataDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeError</strong></code><span class="doc-sig-links"><a href="#t-PeError" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Named errors reported when parsing or inspecting a PE or COFF binary.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeTruncated"><code><strong class="tok-constructor">PeTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeBadSignature"><code><strong class="tok-constructor">PeBadSignature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeUnsupportedMachine"><code><strong class="tok-constructor">PeUnsupportedMachine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeUnsupportedOptionalMagic"><code><strong class="tok-constructor">PeUnsupportedOptionalMagic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeInvalidHeader"><code><strong class="tok-constructor">PeInvalidHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeInvalidSection"><code><strong class="tok-constructor">PeInvalidSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li><li id="k-PeInvalidTable"><code><strong class="tok-constructor">PeInvalidTable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeExport">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeExport</strong></code><span class="doc-sig-links"><a href="#t-PeExport" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L565" title="View source at line 565">Source</a></span></div>
<div class="doc-body">

Construct a PeExport from exported name, ordinal, RVA, and optional forwarder string.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeExport"><code><strong class="tok-constructor">PeExport</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeExportDirectory">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeExportDirectory</strong></code><span class="doc-sig-links"><a href="#t-PeExportDirectory" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L586" title="View source at line 586">Source</a></span></div>
<div class="doc-body">

Construct a PeExportDirectory from DLL name, ordinal base, function count, name count, and export entries.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeExportDirectory"><code><strong class="tok-constructor">PeExportDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeFile</strong></code><span class="doc-sig-links"><a href="#t-PeFile" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L611" title="View source at line 611">Source</a></span></div>
<div class="doc-body">

Construct a PeFile from file header, optional header, sections, symbols, string table, and backing bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeFile"><code><strong class="tok-constructor">PeFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeFileHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeFileHeader</strong></code><span class="doc-sig-links"><a href="#t-PeFileHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Construct a PeFileHeader from machine, num sections, timestamp, symtab ptr, num symbols, optional header size, characteristics.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeFileHeader"><code><strong class="tok-constructor">PeFileHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeImportDirectory">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeImportDirectory</strong></code><span class="doc-sig-links"><a href="#t-PeImportDirectory" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L511" title="View source at line 511">Source</a></span></div>
<div class="doc-body">

Construct a PeImportDirectory from original first thunk, timestamp, forwarder chain, name RVA, first thunk, and resolved DLL name.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeImportDirectory"><code><strong class="tok-constructor">PeImportDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeImportSymbol">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeImportSymbol</strong></code><span class="doc-sig-links"><a href="#t-PeImportSymbol" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L540" title="View source at line 540">Source</a></span></div>
<div class="doc-body">

Construct a PeImportSymbol from DLL name, function name (empty if ordinal), hint, ordinal, and by-ordinal flag.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeImportSymbol"><code><strong class="tok-constructor">PeImportSymbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeMachine">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeMachine</strong></code><span class="doc-sig-links"><a href="#t-PeMachine" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

COFF target machine architecture.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeMachineUnknown"><code><strong class="tok-constructor">PeMachineUnknown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineI386"><code><strong class="tok-constructor">PeMachineI386</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineAmd64"><code><strong class="tok-constructor">PeMachineAmd64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineArm"><code><strong class="tok-constructor">PeMachineArm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineArmnt"><code><strong class="tok-constructor">PeMachineArmnt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineArm64"><code><strong class="tok-constructor">PeMachineArm64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineRiscv32"><code><strong class="tok-constructor">PeMachineRiscv32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineRiscv64"><code><strong class="tok-constructor">PeMachineRiscv64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li><li id="k-PeMachineOther"><code><strong class="tok-constructor">PeMachineOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeOptionalHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader</strong></code><span class="doc-sig-links"><a href="#t-PeOptionalHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

Optional header present in a PE image, or absent in a COFF object file.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeOptNone"><code><strong class="tok-constructor">PeOptNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a></code></li><li id="k-PeOpt32"><code><strong class="tok-constructor">PeOpt32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a></code></li><li id="k-PeOpt64"><code><strong class="tok-constructor">PeOpt64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeOptionalHeader32">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader32</strong></code><span class="doc-sig-links"><a href="#t-PeOptionalHeader32" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L326" title="View source at line 326">Source</a></span></div>
<div class="doc-body">

Construct a PeOptionalHeader32 from standard fields, base of data, 32-bit image base, Windows fields, and data directories.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeOptionalHeader32"><code><strong class="tok-constructor">PeOptionalHeader32</strong> :: <span class="tok-constructor">PeOptStd</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">PeOptWin</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeOptionalHeader64">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeOptionalHeader64</strong></code><span class="doc-sig-links"><a href="#t-PeOptionalHeader64" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L331" title="View source at line 331">Source</a></span></div>
<div class="doc-body">

Construct a PeOptionalHeader64 from standard fields, 64-bit image base, Windows fields, and data directories.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeOptionalHeader64"><code><strong class="tok-constructor">PeOptionalHeader64</strong> :: <span class="tok-constructor">PeOptStd</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">PeOptWin</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeSection">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSection</strong></code><span class="doc-sig-links"><a href="#t-PeSection" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L441" title="View source at line 441">Source</a></span></div>
<div class="doc-body">

Construct a PeSection from name, virtual size, virtual address, raw size, raw offset, reloc ptr, line ptr, num relocs, num lines, characteristics.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeSection"><code><strong class="tok-constructor">PeSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeSubsystem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSubsystem</strong></code><span class="doc-sig-links"><a href="#t-PeSubsystem" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Windows subsystem required to run a PE image.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeSubsystemUnknown"><code><strong class="tok-constructor">PeSubsystemUnknown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemNative"><code><strong class="tok-constructor">PeSubsystemNative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemWindowsGui"><code><strong class="tok-constructor">PeSubsystemWindowsGui</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemWindowsCui"><code><strong class="tok-constructor">PeSubsystemWindowsCui</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemPosixCui"><code><strong class="tok-constructor">PeSubsystemPosixCui</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemEfiApplication"><code><strong class="tok-constructor">PeSubsystemEfiApplication</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li><li id="k-PeSubsystemOther"><code><strong class="tok-constructor">PeSubsystemOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PeSymbol">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PeSymbol</strong></code><span class="doc-sig-links"><a href="#t-PeSymbol" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L486" title="View source at line 486">Source</a></span></div>
<div class="doc-body">

Construct a PeSymbol from name, value, signed section number, type, storage class.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PeSymbol"><code><strong class="tok-constructor">PeSymbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-pe-data-directory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-data-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code><span class="doc-sig-links"><a href="#v-pe-data-directory" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1025" title="View source at line 1025">Source</a></span></div>
<div class="doc-body">

Return the DataDirectory entry at index (e.g. pe-dir-export or pe-dir-import), if present.

</div>
</article>

<article class="doc-entry" id="v-pe-dd-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dd-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-dd-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Byte size of the table.

</div>
</article>

<article class="doc-entry" id="v-pe-dd-virtual-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dd-virtual-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-dd-virtual-address" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

Relative virtual address (RVA) of the table.

</div>
</article>

<article class="doc-entry" id="v-pe-dir-basereloc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-basereloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-basereloc" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Base relocation table data directory index (5).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-clr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-clr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-clr" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

CLR runtime header data directory index (14).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-debug">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-debug</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-debug" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Debug data directory index (6).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-exception">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-exception</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-exception" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Exception table data directory index (3).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-export">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-export</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-export" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Export table data directory index (0).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-iat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-iat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-iat" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Import address table data directory index (12).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-import">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-import</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-import" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Import table data directory index (1).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-resource">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-resource</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-resource" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Resource table data directory index (2).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-security">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-security</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-security" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Certificate/security table data directory index (4).

</div>
</article>

<article class="doc-entry" id="v-pe-dir-tls">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-dir-tls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pe-dir-tls" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Thread-local storage data directory index (9).

</div>
</article>

<article class="doc-entry" id="v-pe-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Render a human-readable diagnostic for a PE error.

</div>
</article>

<article class="doc-entry" id="v-pe-expdir-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-expdir-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-expdir-base" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L593" title="View source at line 593">Source</a></span></div>
<div class="doc-body">

Starting ordinal number (Base).

</div>
</article>

<article class="doc-entry" id="v-pe-expdir-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-expdir-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a></code><span class="doc-sig-links"><a href="#v-pe-expdir-entries" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

Named export entries in Export Name Pointer Table order.

</div>
</article>

<article class="doc-entry" id="v-pe-expdir-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-expdir-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-expdir-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Internal DLL name stored in the export directory.

</div>
</article>

<article class="doc-entry" id="v-pe-expdir-num-functions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-expdir-num-functions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-expdir-num-functions" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L597" title="View source at line 597">Source</a></span></div>
<div class="doc-body">

Total number of entries in the Export Address Table.

</div>
</article>

<article class="doc-entry" id="v-pe-expdir-num-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-expdir-num-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-expdir-num-names" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L601" title="View source at line 601">Source</a></span></div>
<div class="doc-body">

Number of named exports in the Export Name Pointer Table.

</div>
</article>

<article class="doc-entry" id="v-pe-export-forwarder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-export-forwarder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-export-forwarder" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L580" title="View source at line 580">Source</a></span></div>
<div class="doc-body">

Forwarder string when the export forwards to another DLL, or None.

</div>
</article>

<article class="doc-entry" id="v-pe-export-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-export-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-export-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L568" title="View source at line 568">Source</a></span></div>
<div class="doc-body">

Exported symbol name.

</div>
</article>

<article class="doc-entry" id="v-pe-export-ordinal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-export-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-export-ordinal" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L572" title="View source at line 572">Source</a></span></div>
<div class="doc-body">

Exported ordinal (ordinal base + unbiased ordinal).

</div>
</article>

<article class="doc-entry" id="v-pe-export-rva">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-export-rva</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExport" title="Data.Pe.PeExport">PeExport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-export-rva" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

Relative virtual address of the exported function or data.

</div>
</article>

<article class="doc-entry" id="v-pe-exports">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-exports</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeExportDirectory" title="Data.Pe.PeExportDirectory">PeExportDirectory</a>)</code><span class="doc-sig-links"><a href="#v-pe-exports" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1245" title="View source at line 1245">Source</a></span></div>
<div class="doc-body">

Parse the PE Export Directory Table (IMAGE_DIRECTORY_ENTRY_EXPORT), or None when absent.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-characteristics">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-fh-characteristics" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Image characteristics bitmask.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-machine">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-machine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a></code><span class="doc-sig-links"><a href="#v-pe-fh-machine" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

Target machine architecture.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-num-sections">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-num-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-fh-num-sections" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

Number of section headers.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-num-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-num-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-fh-num-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

Number of entries in the COFF symbol table.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-optional-header-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-optional-header-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-fh-optional-header-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

Byte size of the optional header (0 for COFF object files).

</div>
</article>

<article class="doc-entry" id="v-pe-fh-symbol-table-ptr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-symbol-table-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-fh-symbol-table-ptr" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

File offset of the COFF symbol table, or 0 if absent.

</div>
</article>

<article class="doc-entry" id="v-pe-fh-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-fh-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-fh-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Low 32 bits of the file creation Unix timestamp.

</div>
</article>

<article class="doc-entry" id="v-pe-file-dll">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-file-dll" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

File characteristic: image is a dynamic-link library (0x2000).

</div>
</article>

<article class="doc-entry" id="v-pe-file-executable-image">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-executable-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-file-executable-image" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

File characteristic: image is executable (0x0002).

</div>
</article>

<article class="doc-entry" id="v-pe-file-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFileHeader" title="Data.Pe.PeFileHeader">PeFileHeader</a></code><span class="doc-sig-links"><a href="#v-pe-file-header" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L614" title="View source at line 614">Source</a></span></div>
<div class="doc-body">

COFF file header of a parsed PE binary.

</div>
</article>

<article class="doc-entry" id="v-pe-file-large-address-aware">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-large-address-aware</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-file-large-address-aware" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

File characteristic: application handles addresses above 2 GiB (0x0020).

</div>
</article>

<article class="doc-entry" id="v-pe-file-machine32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-machine32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-file-machine32" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

File characteristic: 32-bit word architecture (0x0100).

</div>
</article>

<article class="doc-entry" id="v-pe-file-relocs-stripped">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-file-relocs-stripped</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-file-relocs-stripped" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

File characteristic: base relocations stripped (0x0001).

</div>
</article>

<article class="doc-entry" id="v-pe-imp-dll">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-imp-dll" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L534" title="View source at line 534">Source</a></span></div>
<div class="doc-body">

Resolved imported DLL name.

</div>
</article>

<article class="doc-entry" id="v-pe-imp-first-thunk">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-first-thunk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-imp-first-thunk" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L530" title="View source at line 530">Source</a></span></div>
<div class="doc-body">

RVA of the Import Address Table (FirstThunk).

</div>
</article>

<article class="doc-entry" id="v-pe-imp-forwarder-chain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-forwarder-chain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-imp-forwarder-chain" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L522" title="View source at line 522">Source</a></span></div>
<div class="doc-body">

Index of first forwarder reference.

</div>
</article>

<article class="doc-entry" id="v-pe-imp-name-rva">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-name-rva</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-imp-name-rva" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L526" title="View source at line 526">Source</a></span></div>
<div class="doc-body">

RVA of the imported DLL name string.

</div>
</article>

<article class="doc-entry" id="v-pe-imp-orig-first-thunk">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-orig-first-thunk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-imp-orig-first-thunk" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L514" title="View source at line 514">Source</a></span></div>
<div class="doc-body">

RVA of the Import Lookup Table (OriginalFirstThunk).

</div>
</article>

<article class="doc-entry" id="v-pe-imp-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imp-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-imp-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L518" title="View source at line 518">Source</a></span></div>
<div class="doc-body">

Binding timestamp.

</div>
</article>

<article class="doc-entry" id="v-pe-import-directories">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-import-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportDirectory" title="Data.Pe.PeImportDirectory">PeImportDirectory</a>)</code><span class="doc-sig-links"><a href="#v-pe-import-directories" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1104" title="View source at line 1104">Source</a></span></div>
<div class="doc-body">

Parse all entries in the PE Import Directory Table (IMAGE_DIRECTORY_ENTRY_IMPORT).

</div>
</article>

<article class="doc-entry" id="v-pe-imported-libraries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-pe-imported-libraries" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1127" title="View source at line 1127">Source</a></span></div>
<div class="doc-body">

Return the names of all DLLs referenced in the PE Import Directory.

</div>
</article>

<article class="doc-entry" id="v-pe-imported-symbol-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imported-symbol-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a>)</code><span class="doc-sig-links"><a href="#v-pe-imported-symbol-entries" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1195" title="View source at line 1195">Source</a></span></div>
<div class="doc-body">

Return structured imported symbol entries across all imported DLLs (including ordinal imports).

</div>
</article>

<article class="doc-entry" id="v-pe-imported-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-pe-imported-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1212" title="View source at line 1212">Source</a></span></div>
<div class="doc-body">

Return all named imported symbols formatted as "symbol:dll".

</div>
</article>

<article class="doc-entry" id="v-pe-impsym-by-ordinal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-impsym-by-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pe-impsym-by-ordinal" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L559" title="View source at line 559">Source</a></span></div>
<div class="doc-body">

True when imported by ordinal rather than by name.

</div>
</article>

<article class="doc-entry" id="v-pe-impsym-dll">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-impsym-dll</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-impsym-dll" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L543" title="View source at line 543">Source</a></span></div>
<div class="doc-body">

Name of the imported DLL.

</div>
</article>

<article class="doc-entry" id="v-pe-impsym-hint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-impsym-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-impsym-hint" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L551" title="View source at line 551">Source</a></span></div>
<div class="doc-body">

Hint index into the DLL's export name pointer table (0 when imported by ordinal).

</div>
</article>

<article class="doc-entry" id="v-pe-impsym-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-impsym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-impsym-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L547" title="View source at line 547">Source</a></span></div>
<div class="doc-body">

Imported function name, or empty string when imported by ordinal.

</div>
</article>

<article class="doc-entry" id="v-pe-impsym-ordinal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-impsym-ordinal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeImportSymbol" title="Data.Pe.PeImportSymbol">PeImportSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-impsym-ordinal" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L555" title="View source at line 555">Source</a></span></div>
<div class="doc-body">

Imported ordinal number (0 when imported by name).

</div>
</article>

<article class="doc-entry" id="v-pe-machine-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-machine-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeMachine" title="Data.Pe.PeMachine">PeMachine</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-machine-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

Numeric COFF machine code for a PeMachine.

</div>
</article>

<article class="doc-entry" id="v-pe-magic-pe32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-magic-pe32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-magic-pe32" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Optional header magic for 32-bit PE32 images (0x10b).

</div>
</article>

<article class="doc-entry" id="v-pe-magic-pe64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-magic-pe64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-magic-pe64" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Optional header magic for 64-bit PE32+ images (0x20b).

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-checksum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-checksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-checksum" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L371" title="View source at line 371">Source</a></span></div>
<div class="doc-body">

Image checksum for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-data-directories">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-data-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-data-directories" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L387" title="View source at line 387">Source</a></span></div>
<div class="doc-body">

Parsed data directory entries for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-dll-characteristics">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-dll-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-dll-characteristics" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L379" title="View source at line 379">Source</a></span></div>
<div class="doc-body">

DLL characteristics bitmask for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-entry-point">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-entry-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-entry-point" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L347" title="View source at line 347">Source</a></span></div>
<div class="doc-body">

Entry point RVA for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-file-alignment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-file-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-file-alignment" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L359" title="View source at line 359">Source</a></span></div>
<div class="doc-body">

Section alignment in the file for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-image-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-image-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-image-base" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

Preferred 32-bit image load address for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-magic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-magic" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L343" title="View source at line 343">Source</a></span></div>
<div class="doc-body">

Optional header magic for PE32 (0x10b).

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-num-rva-and-sizes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-num-rva-and-sizes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-num-rva-and-sizes" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Declared count of data directory entries for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-section-alignment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-section-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-section-alignment" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L355" title="View source at line 355">Source</a></span></div>
<div class="doc-body">

Section alignment in memory for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-size-of-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-size-of-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-size-of-headers" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L367" title="View source at line 367">Source</a></span></div>
<div class="doc-body">

Combined header size for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-size-of-image">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-size-of-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-size-of-image" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L363" title="View source at line 363">Source</a></span></div>
<div class="doc-body">

Total image size in memory for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt32-subsystem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt32-subsystem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader32" title="Data.Pe.PeOptionalHeader32">PeOptionalHeader32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code><span class="doc-sig-links"><a href="#v-pe-opt32-subsystem" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L375" title="View source at line 375">Source</a></span></div>
<div class="doc-body">

Required subsystem for PE32.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-checksum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-checksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-checksum" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L419" title="View source at line 419">Source</a></span></div>
<div class="doc-body">

Image checksum for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-data-directories">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-data-directories</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeDataDirectory" title="Data.Pe.PeDataDirectory">PeDataDirectory</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-data-directories" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L435" title="View source at line 435">Source</a></span></div>
<div class="doc-body">

Parsed data directory entries for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-dll-characteristics">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-dll-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-dll-characteristics" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L427" title="View source at line 427">Source</a></span></div>
<div class="doc-body">

DLL characteristics bitmask for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-entry-point">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-entry-point</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-entry-point" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L395" title="View source at line 395">Source</a></span></div>
<div class="doc-body">

Entry point RVA for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-file-alignment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-file-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-file-alignment" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L407" title="View source at line 407">Source</a></span></div>
<div class="doc-body">

Section alignment in the file for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-image-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-image-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-image-base" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L399" title="View source at line 399">Source</a></span></div>
<div class="doc-body">

Preferred 64-bit image load address for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-magic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-magic" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L391" title="View source at line 391">Source</a></span></div>
<div class="doc-body">

Optional header magic for PE32+ (0x20b).

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-num-rva-and-sizes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-num-rva-and-sizes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-num-rva-and-sizes" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L431" title="View source at line 431">Source</a></span></div>
<div class="doc-body">

Declared count of data directory entries for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-section-alignment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-section-alignment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-section-alignment" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L403" title="View source at line 403">Source</a></span></div>
<div class="doc-body">

Section alignment in memory for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-size-of-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-size-of-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-size-of-headers" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Combined header size for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-size-of-image">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-size-of-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-size-of-image" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L411" title="View source at line 411">Source</a></span></div>
<div class="doc-body">

Total image size in memory for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-opt64-subsystem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-opt64-subsystem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader64" title="Data.Pe.PeOptionalHeader64">PeOptionalHeader64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a></code><span class="doc-sig-links"><a href="#v-pe-opt64-subsystem" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L423" title="View source at line 423">Source</a></span></div>
<div class="doc-body">

Required subsystem for PE32+.

</div>
</article>

<article class="doc-entry" id="v-pe-optional-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-optional-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeOptionalHeader" title="Data.Pe.PeOptionalHeader">PeOptionalHeader</a></code><span class="doc-sig-links"><a href="#v-pe-optional-header" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L618" title="View source at line 618">Source</a></span></div>
<div class="doc-body">

Optional header of a parsed PE binary (PeOptNone for COFF object files).

</div>
</article>

<article class="doc-entry" id="v-pe-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a></code><span class="doc-sig-links"><a href="#v-pe-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L960" title="View source at line 960">Source</a></span></div>
<div class="doc-body">

Parse a PE32, PE32+, or COFF binary from raw bytes.

</div>
</article>

<article class="doc-entry" id="v-pe-scn-cnt-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-cnt-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Section characteristic: contains executable code (0x00000020).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-cnt-initialized-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-initialized-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-cnt-initialized-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Section characteristic: contains initialized data (0x00000040).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-cnt-uninitialized-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-cnt-uninitialized-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-cnt-uninitialized-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Section characteristic: contains uninitialized data (0x00000080).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-mem-discardable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-discardable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-mem-discardable" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Section characteristic: section can be discarded (0x02000000).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-mem-execute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-mem-execute" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Section characteristic: section can be executed as code (0x20000000).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-mem-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-mem-read" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Section characteristic: section can be read (0x40000000).

</div>
</article>

<article class="doc-entry" id="v-pe-scn-mem-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-scn-mem-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-scn-mem-write" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Section characteristic: section can be written (0x80000000).

</div>
</article>

<article class="doc-entry" id="v-pe-sec-characteristics">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-characteristics</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-characteristics" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L480" title="View source at line 480">Source</a></span></div>
<div class="doc-body">

Section characteristics bitmask.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-line-ptr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-line-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-line-ptr" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L468" title="View source at line 468">Source</a></span></div>
<div class="doc-body">

File offset of COFF line numbers.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-sec-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L444" title="View source at line 444">Source</a></span></div>
<div class="doc-body">

Resolved section name (including /offset COFF string table lookup).

</div>
</article>

<article class="doc-entry" id="v-pe-sec-num-lines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-num-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-sec-num-lines" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L476" title="View source at line 476">Source</a></span></div>
<div class="doc-body">

Number of line number entries.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-num-relocs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-num-relocs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-sec-num-relocs" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Number of relocation entries.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L460" title="View source at line 460">Source</a></span></div>
<div class="doc-body">

File byte offset of the section data (PointerToRawData).

</div>
</article>

<article class="doc-entry" id="v-pe-sec-reloc-ptr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-reloc-ptr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-reloc-ptr" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L464" title="View source at line 464">Source</a></span></div>
<div class="doc-body">

File offset of relocation entries.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L456" title="View source at line 456">Source</a></span></div>
<div class="doc-body">

Size of the initialized section data on disk (SizeOfRawData).

</div>
</article>

<article class="doc-entry" id="v-pe-sec-virtual-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-virtual-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-virtual-address" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L452" title="View source at line 452">Source</a></span></div>
<div class="doc-body">

Relative virtual address of the section.

</div>
</article>

<article class="doc-entry" id="v-pe-sec-virtual-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sec-virtual-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sec-virtual-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L448" title="View source at line 448">Source</a></span></div>
<div class="doc-body">

Total size of the section when loaded into memory.

</div>
</article>

<article class="doc-entry" id="v-pe-section">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a></code><span class="doc-sig-links"><a href="#v-pe-section" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1005" title="View source at line 1005">Source</a></span></div>
<div class="doc-body">

Find the first section with the given name.

</div>
</article>

<article class="doc-entry" id="v-pe-section-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeError" title="Data.Pe.PeError">PeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pe-section-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L1017" title="View source at line 1017">Source</a></span></div>
<div class="doc-body">

Read the raw bytes of a PE section.

</div>
</article>

<article class="doc-entry" id="v-pe-sections">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSection" title="Data.Pe.PeSection">PeSection</a></code><span class="doc-sig-links"><a href="#v-pe-sections" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L622" title="View source at line 622">Source</a></span></div>
<div class="doc-body">

Section headers of a parsed PE binary.

</div>
</article>

<article class="doc-entry" id="v-pe-subsystem-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-subsystem-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSubsystem" title="Data.Pe.PeSubsystem">PeSubsystem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-subsystem-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Numeric subsystem code for a PeSubsystem.

</div>
</article>

<article class="doc-entry" id="v-pe-sym-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pe-sym-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L489" title="View source at line 489">Source</a></span></div>
<div class="doc-body">

Resolved COFF symbol name.

</div>
</article>

<article class="doc-entry" id="v-pe-sym-section-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sym-section-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code><span class="doc-sig-links"><a href="#v-pe-sym-section-number" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L497" title="View source at line 497">Source</a></span></div>
<div class="doc-body">

Signed 1-based section number (0 = undefined, -1 = absolute, -2 = debug).

</div>
</article>

<article class="doc-entry" id="v-pe-sym-storage-class">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sym-storage-class</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-pe-sym-storage-class" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L505" title="View source at line 505">Source</a></span></div>
<div class="doc-body">

COFF symbol storage class byte.

</div>
</article>

<article class="doc-entry" id="v-pe-sym-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-pe-sym-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L501" title="View source at line 501">Source</a></span></div>
<div class="doc-body">

COFF symbol type word.

</div>
</article>

<article class="doc-entry" id="v-pe-sym-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-sym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-pe-sym-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L493" title="View source at line 493">Source</a></span></div>
<div class="doc-body">

Symbol value.

</div>
</article>

<article class="doc-entry" id="v-pe-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pe-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeFile" title="Data.Pe.PeFile">PeFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Pe/#t-PeSymbol" title="Data.Pe.PeSymbol">PeSymbol</a></code><span class="doc-sig-links"><a href="#v-pe-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.Pe/source/#L626" title="View source at line 626">Source</a></span></div>
<div class="doc-body">

COFF symbols with auxiliary records omitted.

</div>
</article>
