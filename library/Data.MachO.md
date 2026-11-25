# Data.MachO

Bounds-checked Mach-O 32/64 (little/big-endian) and Universal/Fat binary parser for load commands, segments, sections, symbols and dylibs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.MachO</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.MachO/source/">View source</a><a href="/packages/search/?q=%2BData.MachO">Search module</a><a href="/reference-sources/Data.MachO.cinch" download="Data.MachO.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure bounds-checked parser for Mach-O 32-bit, 64-bit, and universal (fat) binaries.
Supports little-endian and big-endian byte orders, file headers, load commands
(Segment, Segment64, Symtab, Dysymtab, Dylib, Dylinker, Uuid, Rpath), sections,
symbol tables, imported libraries and symbols, and multi-architecture fat archives.

</details>

<details class="doc-synopsis"><summary>Synopsis (116 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MachOCpu">MachOCpu</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOCpu</strong></code></li><li><a class="syn-name" href="#t-MachODylib">MachODylib</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODylib</strong></code></li><li><a class="syn-name" href="#t-MachODylinker">MachODylinker</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODylinker</strong></code></li><li><a class="syn-name" href="#t-MachODysymtab">MachODysymtab</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODysymtab</strong></code></li><li><a class="syn-name" href="#t-MachOError">MachOError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOError</strong></code></li><li><a class="syn-name" href="#t-MachOFatArch">MachOFatArch</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFatArch</strong></code></li><li><a class="syn-name" href="#t-MachOFatFile">MachOFatFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFatFile</strong></code></li><li><a class="syn-name" href="#t-MachOFile">MachOFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFile</strong></code></li><li><a class="syn-name" href="#t-MachOHeader">MachOHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOHeader</strong></code></li><li><a class="syn-name" href="#t-MachOLoadCmd">MachOLoadCmd</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOLoadCmd</strong></code></li><li><a class="syn-name" href="#t-MachOSection">MachOSection</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSection</strong></code></li><li><a class="syn-name" href="#t-MachOSegment">MachOSegment</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSegment</strong></code></li><li><a class="syn-name" href="#t-MachOSymbol">MachOSymbol</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSymbol</strong></code></li><li><a class="syn-name" href="#t-MachOSymtab">MachOSymtab</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSymtab</strong></code></li><li><a class="syn-name" href="#t-MachOType">MachOType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOType</strong></code></li><li><a class="syn-name" href="#v-dylib-cmd">dylib-cmd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylib-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dylib-compat-version">dylib-compat-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylib-compat-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dylib-current-version">dylib-current-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylib-current-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dylib-name">dylib-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylib-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-dylib-time">dylib-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylib-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dylinker-cmd">dylinker-cmd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylinker-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dylinker-name">dylinker-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dylinker-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-dysym-iextdefsym">dysym-iextdefsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-iextdefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-ilocalsym">dysym-ilocalsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-ilocalsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-indirectoff">dysym-indirectoff</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-indirectoff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-iundefsym">dysym-iundefsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-iundefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-nextdefsym">dysym-nextdefsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-nextdefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-nindirectsyms">dysym-nindirectsyms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-nindirectsyms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-nlocalsym">dysym-nlocalsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-nlocalsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-dysym-nundefsym">dysym-nundefsym</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dysym-nundefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fat-arch-align">fat-arch-align</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fat-arch-cpu">fat-arch-cpu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-cpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li><a class="syn-name" href="#v-fat-arch-file">fat-arch-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a></code></li><li><a class="syn-name" href="#v-fat-arch-offset">fat-arch-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fat-arch-size">fat-arch-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fat-arch-subcpu">fat-arch-subcpu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fat-arch-subcpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-dysymtab">lc-dysymtab</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-dysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-id-dylib">lc-id-dylib</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-id-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-id-dylinker">lc-id-dylinker</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-id-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-load-dylib">lc-load-dylib</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-load-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-load-dylinker">lc-load-dylinker</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-load-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-load-weak-dylib">lc-load-weak-dylib</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-load-weak-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-reexport-dylib">lc-reexport-dylib</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-reexport-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-rpath">lc-rpath</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-rpath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-segment">lc-segment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-segment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-segment64">lc-segment64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-segment64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-symtab">lc-symtab</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-symtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-thread">lc-thread</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-thread</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-unix-thread">lc-unix-thread</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-unix-thread</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-lc-uuid">lc-uuid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lc-uuid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-byte-order">macho-byte-order</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-byte-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code></li><li><a class="syn-name" href="#v-macho-cmdsz">macho-cmdsz</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-cmdsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-cpu">macho-cpu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-cpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li><a class="syn-name" href="#v-macho-cpu-code">macho-cpu-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-cpu-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-dylibs">macho-dylibs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-dylibs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a></code></li><li><a class="syn-name" href="#v-macho-dylinker">macho-dylinker</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-macho-dysymtab">macho-dysymtab</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-dysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a></code></li><li><a class="syn-name" href="#v-macho-error-message">macho-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-macho-fat-arches">macho-fat-arches</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-fat-arches</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a></code></li><li><a class="syn-name" href="#v-macho-fat-magic">macho-fat-magic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-fat-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-file-type">macho-file-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-file-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li><li><a class="syn-name" href="#v-macho-flags">macho-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-header">macho-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a></code></li><li><a class="syn-name" href="#v-macho-imported-libraries">macho-imported-libraries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-macho-imported-symbols">macho-imported-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-macho-is64">macho-is64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-is64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-macho-loads">macho-loads</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-loads</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li><a class="syn-name" href="#v-macho-magic">macho-magic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-magic-fat">macho-magic-fat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-magic-fat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-magic32">macho-magic32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-magic32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-magic64">macho-magic64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-magic64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-ncmd">macho-ncmd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-ncmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-parse">macho-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a></code></li><li><a class="syn-name" href="#v-macho-parse-fat">macho-parse-fat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-parse-fat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a></code></li><li><a class="syn-name" href="#v-macho-rpaths">macho-rpaths</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-rpaths</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-macho-section">macho-section</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a></code></li><li><a class="syn-name" href="#v-macho-section-data">macho-section-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-macho-sections">macho-sections</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a></code></li><li><a class="syn-name" href="#v-macho-segment">macho-segment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-segment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a></code></li><li><a class="syn-name" href="#v-macho-segment-data">macho-segment-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-segment-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-macho-segments">macho-segments</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-segments</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a></code></li><li><a class="syn-name" href="#v-macho-subcpu">macho-subcpu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-subcpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-symbols">macho-symbols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a></code></li><li><a class="syn-name" href="#v-macho-symtab">macho-symtab</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-symtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a></code></li><li><a class="syn-name" href="#v-macho-type-code">macho-type-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-macho-uuid">macho-uuid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">macho-uuid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-msym-desc">msym-desc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">msym-desc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-msym-name">msym-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">msym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-msym-sect">msym-sect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">msym-sect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-msym-type">msym-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">msym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-msym-value">msym-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">msym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-sec-addr">sec-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-sec-align">sec-align</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-sec-flags">sec-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-sec-name">sec-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sec-nreloc">sec-nreloc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-nreloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-sec-offset">sec-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-sec-reloff">sec-reloff</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-reloff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-sec-seg">sec-seg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-seg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sec-size">sec-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sec-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-seg-addr">seg-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-seg-cmd">seg-cmd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-seg-filesz">seg-filesz</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-filesz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-seg-flag">seg-flag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-flag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-seg-len">seg-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-seg-maxprot">seg-maxprot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-maxprot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-seg-memsz">seg-memsz</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-memsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-seg-name">seg-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-seg-nsect">seg-nsect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-nsect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-seg-offset">seg-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-seg-prot">seg-prot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">seg-prot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-symtab-nsyms">symtab-nsyms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">symtab-nsyms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-symtab-stroff">symtab-stroff</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">symtab-stroff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-symtab-strsize">symtab-strsize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">symtab-strsize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-symtab-symoff">symtab-symoff</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">symtab-symoff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-symtab-syms">symtab-syms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">symtab-syms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MachOCpu">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOCpu</strong></code><span class="doc-sig-links"><a href="#t-MachOCpu" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

Mach-O target CPU architecture.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Cpu386"><code><strong class="tok-constructor">Cpu386</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuAmd64"><code><strong class="tok-constructor">CpuAmd64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuArm"><code><strong class="tok-constructor">CpuArm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuArm64"><code><strong class="tok-constructor">CpuArm64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuPpc"><code><strong class="tok-constructor">CpuPpc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuPpc64"><code><strong class="tok-constructor">CpuPpc64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li><li id="k-CpuOther"><code><strong class="tok-constructor">CpuOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachODylib">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODylib</strong></code><span class="doc-sig-links"><a href="#t-MachODylib" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L436" title="View source at line 436">Source</a></span></div>
<div class="doc-body">

Construct a MachODylib from cmd code, library path, timestamp, current version, compat version.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachODylib"><code><strong class="tok-constructor">MachODylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachODylinker">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODylinker</strong></code><span class="doc-sig-links"><a href="#t-MachODylinker" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L461" title="View source at line 461">Source</a></span></div>
<div class="doc-body">

Construct a MachODylinker from cmd code and dynamic linker path.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachODylinker"><code><strong class="tok-constructor">MachODylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachODysymtab">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachODysymtab</strong></code><span class="doc-sig-links"><a href="#t-MachODysymtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L399" title="View source at line 399">Source</a></span></div>
<div class="doc-body">

Construct a MachODysymtab from local/extdef/undef/indirect index and count pairs.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachODysymtab"><code><strong class="tok-constructor">MachODysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOError</strong></code><span class="doc-sig-links"><a href="#t-MachOError" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Named errors reported when parsing or inspecting a Mach-O or fat binary.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOTruncated"><code><strong class="tok-constructor">MachOTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachOBadMagic"><code><strong class="tok-constructor">MachOBadMagic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachONotFat"><code><strong class="tok-constructor">MachONotFat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachOInvalidCommand"><code><strong class="tok-constructor">MachOInvalidCommand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachOInvalidSection"><code><strong class="tok-constructor">MachOInvalidSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachOInvalidFatArch"><code><strong class="tok-constructor">MachOInvalidFatArch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li><li id="k-MachONoSymtab"><code><strong class="tok-constructor">MachONoSymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOFatArch">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFatArch</strong></code><span class="doc-sig-links"><a href="#t-MachOFatArch" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L522" title="View source at line 522">Source</a></span></div>
<div class="doc-body">

Construct a MachOFatArch from cpu, subcpu, offset, size, align, and parsed MachOFile.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOFatArch"><code><strong class="tok-constructor">MachOFatArch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOFatFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFatFile</strong></code><span class="doc-sig-links"><a href="#t-MachOFatFile" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L551" title="View source at line 551">Source</a></span></div>
<div class="doc-body">

Construct a MachOFatFile from its magic word and list of architecture slices.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOFatFile"><code><strong class="tok-constructor">MachOFatFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOFile</strong></code><span class="doc-sig-links"><a href="#t-MachOFile" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L493" title="View source at line 493">Source</a></span></div>
<div class="doc-body">

Construct a MachOFile from header, loads, segments, sections, symtab, dysymtab, and backing bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOFile"><code><strong class="tok-constructor">MachOFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOHeader</strong></code><span class="doc-sig-links"><a href="#t-MachOHeader" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Construct a MachOHeader from magic, byte order, is64, cpu, subcpu, file type, ncmd, cmdsz, flags.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOHeader"><code><strong class="tok-constructor">MachOHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOLoadCmd">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOLoadCmd</strong></code><span class="doc-sig-links"><a href="#t-MachOLoadCmd" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Decoded Mach-O load command.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LoadSegment"><code><strong class="tok-constructor">LoadSegment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadSymtab"><code><strong class="tok-constructor">LoadSymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadDysymtab"><code><strong class="tok-constructor">LoadDysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadDylib"><code><strong class="tok-constructor">LoadDylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadDylinker"><code><strong class="tok-constructor">LoadDylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadUuid"><code><strong class="tok-constructor">LoadUuid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadRpath"><code><strong class="tok-constructor">LoadRpath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li><li id="k-LoadRaw"><code><strong class="tok-constructor">LoadRaw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOSection">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSection</strong></code><span class="doc-sig-links"><a href="#t-MachOSection" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L308" title="View source at line 308">Source</a></span></div>
<div class="doc-body">

Construct a MachOSection from name, seg, addr, size, offset, align, reloff, nreloc, flags.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOSection"><code><strong class="tok-constructor">MachOSection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOSegment">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSegment</strong></code><span class="doc-sig-links"><a href="#t-MachOSegment" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Construct a MachOSegment from cmd, len, name, addr, memsz, offset, filesz, maxprot, prot, nsect, flag.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOSegment"><code><strong class="tok-constructor">MachOSegment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOSymbol">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSymbol</strong></code><span class="doc-sig-links"><a href="#t-MachOSymbol" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

Construct a MachOSymbol from name, n_type, n_sect, n_desc, n_value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOSymbol"><code><strong class="tok-constructor">MachOSymbol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOSymtab">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOSymtab</strong></code><span class="doc-sig-links"><a href="#t-MachOSymtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L374" title="View source at line 374">Source</a></span></div>
<div class="doc-body">

Construct a MachOSymtab from symoff, nsyms, stroff, strsize, and symbol list.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOSymtab"><code><strong class="tok-constructor">MachOSymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MachOType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MachOType</strong></code><span class="doc-sig-links"><a href="#t-MachOType" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Mach-O object file type.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MachOTypeObj"><code><strong class="tok-constructor">MachOTypeObj</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li><li id="k-MachOTypeExec"><code><strong class="tok-constructor">MachOTypeExec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li><li id="k-MachOTypeDylib"><code><strong class="tok-constructor">MachOTypeDylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li><li id="k-MachOTypeBundle"><code><strong class="tok-constructor">MachOTypeBundle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li><li id="k-MachOTypeOther"><code><strong class="tok-constructor">MachOTypeOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-dylib-cmd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylib-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dylib-cmd" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L439" title="View source at line 439">Source</a></span></div>
<div class="doc-body">

Load command type code.

</div>
</article>

<article class="doc-entry" id="v-dylib-compat-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylib-compat-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dylib-compat-version" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L455" title="View source at line 455">Source</a></span></div>
<div class="doc-body">

Compatibility version packed as major.minor.patch.

</div>
</article>

<article class="doc-entry" id="v-dylib-current-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylib-current-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dylib-current-version" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Current library version packed as major.minor.patch.

</div>
</article>

<article class="doc-entry" id="v-dylib-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylib-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-dylib-name" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L443" title="View source at line 443">Source</a></span></div>
<div class="doc-body">

Library path name.

</div>
</article>

<article class="doc-entry" id="v-dylib-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylib-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dylib-time" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L447" title="View source at line 447">Source</a></span></div>
<div class="doc-body">

Build timestamp.

</div>
</article>

<article class="doc-entry" id="v-dylinker-cmd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylinker-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dylinker-cmd" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L464" title="View source at line 464">Source</a></span></div>
<div class="doc-body">

Load command type code.

</div>
</article>

<article class="doc-entry" id="v-dylinker-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dylinker-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylinker" title="Data.MachO.MachODylinker">MachODylinker</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-dylinker-name" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L468" title="View source at line 468">Source</a></span></div>
<div class="doc-body">

Dynamic linker path name.

</div>
</article>

<article class="doc-entry" id="v-dysym-iextdefsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-iextdefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-iextdefsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L410" title="View source at line 410">Source</a></span></div>
<div class="doc-body">

Start index of externally defined symbols.

</div>
</article>

<article class="doc-entry" id="v-dysym-ilocalsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-ilocalsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-ilocalsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L402" title="View source at line 402">Source</a></span></div>
<div class="doc-body">

Start index of local symbols.

</div>
</article>

<article class="doc-entry" id="v-dysym-indirectoff">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-indirectoff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-indirectoff" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L426" title="View source at line 426">Source</a></span></div>
<div class="doc-body">

File offset of indirect symbol table.

</div>
</article>

<article class="doc-entry" id="v-dysym-iundefsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-iundefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-iundefsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L418" title="View source at line 418">Source</a></span></div>
<div class="doc-body">

Start index of undefined symbols.

</div>
</article>

<article class="doc-entry" id="v-dysym-nextdefsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-nextdefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-nextdefsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L414" title="View source at line 414">Source</a></span></div>
<div class="doc-body">

Number of externally defined symbols.

</div>
</article>

<article class="doc-entry" id="v-dysym-nindirectsyms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-nindirectsyms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-nindirectsyms" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L430" title="View source at line 430">Source</a></span></div>
<div class="doc-body">

Number of indirect symbol table entries.

</div>
</article>

<article class="doc-entry" id="v-dysym-nlocalsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-nlocalsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-nlocalsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L406" title="View source at line 406">Source</a></span></div>
<div class="doc-body">

Number of local symbols.

</div>
</article>

<article class="doc-entry" id="v-dysym-nundefsym">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dysym-nundefsym</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dysym-nundefsym" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L422" title="View source at line 422">Source</a></span></div>
<div class="doc-body">

Number of undefined symbols.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-align">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fat-arch-align" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L541" title="View source at line 541">Source</a></span></div>
<div class="doc-body">

Alignment power of two of the slice.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-cpu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-cpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code><span class="doc-sig-links"><a href="#v-fat-arch-cpu" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L525" title="View source at line 525">Source</a></span></div>
<div class="doc-body">

Architecture CPU type.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a></code><span class="doc-sig-links"><a href="#v-fat-arch-file" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L545" title="View source at line 545">Source</a></span></div>
<div class="doc-body">

Parsed Mach-O binary for this architecture slice.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fat-arch-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L533" title="View source at line 533">Source</a></span></div>
<div class="doc-body">

Byte offset of the slice in the fat archive.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fat-arch-size" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L537" title="View source at line 537">Source</a></span></div>
<div class="doc-body">

Byte size of the slice in the fat archive.

</div>
</article>

<article class="doc-entry" id="v-fat-arch-subcpu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fat-arch-subcpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fat-arch-subcpu" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L529" title="View source at line 529">Source</a></span></div>
<div class="doc-body">

Architecture CPU subtype.

</div>
</article>

<article class="doc-entry" id="v-lc-dysymtab">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-dysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-dysymtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

LC_DYSYMTAB load command code (0xb).

</div>
</article>

<article class="doc-entry" id="v-lc-id-dylib">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-id-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-id-dylib" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

LC_ID_DYLIB load command code (0xd).

</div>
</article>

<article class="doc-entry" id="v-lc-id-dylinker">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-id-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-id-dylinker" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

LC_ID_DYLINKER load command code (0xf).

</div>
</article>

<article class="doc-entry" id="v-lc-load-dylib">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-load-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-load-dylib" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

LC_LOAD_DYLIB load command code (0xc).

</div>
</article>

<article class="doc-entry" id="v-lc-load-dylinker">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-load-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-load-dylinker" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

LC_LOAD_DYLINKER load command code (0xe).

</div>
</article>

<article class="doc-entry" id="v-lc-load-weak-dylib">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-load-weak-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-load-weak-dylib" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

LC_LOAD_WEAK_DYLIB load command code (0x80000018).

</div>
</article>

<article class="doc-entry" id="v-lc-reexport-dylib">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-reexport-dylib</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-reexport-dylib" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

LC_REEXPORT_DYLIB load command code (0x8000001f).

</div>
</article>

<article class="doc-entry" id="v-lc-rpath">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-rpath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-rpath" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

LC_RPATH load command code (0x8000001c).

</div>
</article>

<article class="doc-entry" id="v-lc-segment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-segment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-segment" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

LC_SEGMENT load command code (0x1).

</div>
</article>

<article class="doc-entry" id="v-lc-segment64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-segment64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-segment64" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

LC_SEGMENT_64 load command code (0x19).

</div>
</article>

<article class="doc-entry" id="v-lc-symtab">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-symtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-symtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

LC_SYMTAB load command code (0x2).

</div>
</article>

<article class="doc-entry" id="v-lc-thread">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-thread</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-thread" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

LC_THREAD load command code (0x4).

</div>
</article>

<article class="doc-entry" id="v-lc-unix-thread">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-unix-thread</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-unix-thread" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

LC_UNIXTHREAD load command code (0x5).

</div>
</article>

<article class="doc-entry" id="v-lc-uuid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lc-uuid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-lc-uuid" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

LC_UUID load command code (0x1b).

</div>
</article>

<article class="doc-entry" id="v-macho-byte-order">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-byte-order</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a></code><span class="doc-sig-links"><a href="#v-macho-byte-order" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Byte order used by this Mach-O binary.

</div>
</article>

<article class="doc-entry" id="v-macho-cmdsz">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-cmdsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-cmdsz" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

Total byte size of all load commands.

</div>
</article>

<article class="doc-entry" id="v-macho-cpu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-cpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a></code><span class="doc-sig-links"><a href="#v-macho-cpu" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

Target CPU architecture.

</div>
</article>

<article class="doc-entry" id="v-macho-cpu-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-cpu-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOCpu" title="Data.MachO.MachOCpu">MachOCpu</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-cpu-code" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

Numeric CPU type code for a MachOCpu.

</div>
</article>

<article class="doc-entry" id="v-macho-dylibs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-dylibs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODylib" title="Data.MachO.MachODylib">MachODylib</a></code><span class="doc-sig-links"><a href="#v-macho-dylibs" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1098" title="View source at line 1098">Source</a></span></div>
<div class="doc-body">

Return all dynamic library load commands (LC_LOAD_DYLIB, LC_ID_DYLIB, weak, reexport).

</div>
</article>

<article class="doc-entry" id="v-macho-dylinker">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-dylinker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-macho-dylinker" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1167" title="View source at line 1167">Source</a></span></div>
<div class="doc-body">

Return the dynamic linker path from LC_LOAD_DYLINKER or LC_ID_DYLINKER, if present.

</div>
</article>

<article class="doc-entry" id="v-macho-dysymtab">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-dysymtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachODysymtab" title="Data.MachO.MachODysymtab">MachODysymtab</a></code><span class="doc-sig-links"><a href="#v-macho-dysymtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L516" title="View source at line 516">Source</a></span></div>
<div class="doc-body">

Parsed LC_DYSYMTAB, if present.

</div>
</article>

<article class="doc-entry" id="v-macho-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-macho-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Render a human-readable diagnostic for a Mach-O error.

</div>
</article>

<article class="doc-entry" id="v-macho-fat-arches">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-fat-arches</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatArch" title="Data.MachO.MachOFatArch">MachOFatArch</a></code><span class="doc-sig-links"><a href="#v-macho-fat-arches" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L558" title="View source at line 558">Source</a></span></div>
<div class="doc-body">

Architecture slices contained in the fat archive.

</div>
</article>

<article class="doc-entry" id="v-macho-fat-magic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-fat-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-fat-magic" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L554" title="View source at line 554">Source</a></span></div>
<div class="doc-body">

Magic word of the fat archive (macho-magic-fat).

</div>
</article>

<article class="doc-entry" id="v-macho-file-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-file-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a></code><span class="doc-sig-links"><a href="#v-macho-file-type" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

Mach-O file type.

</div>
</article>

<article class="doc-entry" id="v-macho-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-flags" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Header flags bitmask.

</div>
</article>

<article class="doc-entry" id="v-macho-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a></code><span class="doc-sig-links"><a href="#v-macho-header" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

File header of a parsed Mach-O binary.

</div>
</article>

<article class="doc-entry" id="v-macho-imported-libraries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-imported-libraries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-macho-imported-libraries" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1112" title="View source at line 1112">Source</a></span></div>
<div class="doc-body">

Return the paths of all libraries referenced by LC_LOAD_DYLIB commands.

</div>
</article>

<article class="doc-entry" id="v-macho-imported-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-imported-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-macho-imported-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1122" title="View source at line 1122">Source</a></span></div>
<div class="doc-body">

Return the names of all undefined symbols referenced by LC_DYSYMTAB.

</div>
</article>

<article class="doc-entry" id="v-macho-is64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-is64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-macho-is64" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

True for 64-bit Mach-O files, False for 32-bit.

</div>
</article>

<article class="doc-entry" id="v-macho-loads">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-loads</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOLoadCmd" title="Data.MachO.MachOLoadCmd">MachOLoadCmd</a></code><span class="doc-sig-links"><a href="#v-macho-loads" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L500" title="View source at line 500">Source</a></span></div>
<div class="doc-body">

All load commands in file order.

</div>
</article>

<article class="doc-entry" id="v-macho-magic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-magic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-magic" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

Normalized magic number (macho-magic32 or macho-magic64).

</div>
</article>

<article class="doc-entry" id="v-macho-magic-fat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-magic-fat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-magic-fat" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Universal (fat) Mach-O magic number (0xcafebabe).

</div>
</article>

<article class="doc-entry" id="v-macho-magic32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-magic32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-magic32" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

32-bit Mach-O magic number (0xfeedface).

</div>
</article>

<article class="doc-entry" id="v-macho-magic64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-magic64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-magic64" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

64-bit Mach-O magic number (0xfeedfacf).

</div>
</article>

<article class="doc-entry" id="v-macho-ncmd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-ncmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-ncmd" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

Number of load commands.

</div>
</article>

<article class="doc-entry" id="v-macho-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a></code><span class="doc-sig-links"><a href="#v-macho-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1001" title="View source at line 1001">Source</a></span></div>
<div class="doc-body">

Parse a single-architecture 32-bit or 64-bit Mach-O binary from raw bytes.

</div>
</article>

<article class="doc-entry" id="v-macho-parse-fat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-parse-fat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFatFile" title="Data.MachO.MachOFatFile">MachOFatFile</a></code><span class="doc-sig-links"><a href="#v-macho-parse-fat" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1214" title="View source at line 1214">Source</a></span></div>
<div class="doc-body">

Parse a universal (fat) Mach-O binary from raw bytes.

</div>
</article>

<article class="doc-entry" id="v-macho-rpaths">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-rpaths</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-macho-rpaths" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1143" title="View source at line 1143">Source</a></span></div>
<div class="doc-body">

Return all runtime library search paths from LC_RPATH load commands.

</div>
</article>

<article class="doc-entry" id="v-macho-section">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-section</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a></code><span class="doc-sig-links"><a href="#v-macho-section" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1052" title="View source at line 1052">Source</a></span></div>
<div class="doc-body">

Find the first section with the given name.

</div>
</article>

<article class="doc-entry" id="v-macho-section-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-section-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-macho-section-data" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1064" title="View source at line 1064">Source</a></span></div>
<div class="doc-body">

Read the raw bytes of a Mach-O section.

</div>
</article>

<article class="doc-entry" id="v-macho-sections">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-sections</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a></code><span class="doc-sig-links"><a href="#v-macho-sections" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L508" title="View source at line 508">Source</a></span></div>
<div class="doc-body">

All parsed sections across all segments in file order.

</div>
</article>

<article class="doc-entry" id="v-macho-segment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-segment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a></code><span class="doc-sig-links"><a href="#v-macho-segment" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1040" title="View source at line 1040">Source</a></span></div>
<div class="doc-body">

Find the first segment with the given name.

</div>
</article>

<article class="doc-entry" id="v-macho-segment-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-segment-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOError" title="Data.MachO.MachOError">MachOError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-macho-segment-data" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1074" title="View source at line 1074">Source</a></span></div>
<div class="doc-body">

Read the file-backed bytes of a Mach-O segment.

</div>
</article>

<article class="doc-entry" id="v-macho-segments">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-segments</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a></code><span class="doc-sig-links"><a href="#v-macho-segments" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L504" title="View source at line 504">Source</a></span></div>
<div class="doc-body">

All parsed segments in file order.

</div>
</article>

<article class="doc-entry" id="v-macho-subcpu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-subcpu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOHeader" title="Data.MachO.MachOHeader">MachOHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-subcpu" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L237" title="View source at line 237">Source</a></span></div>
<div class="doc-body">

Target CPU subtype.

</div>
</article>

<article class="doc-entry" id="v-macho-symbols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-symbols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a></code><span class="doc-sig-links"><a href="#v-macho-symbols" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1084" title="View source at line 1084">Source</a></span></div>
<div class="doc-body">

Return all symbols from LC_SYMTAB, or Nil when no symbol table is present.

</div>
</article>

<article class="doc-entry" id="v-macho-symtab">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-symtab</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a></code><span class="doc-sig-links"><a href="#v-macho-symtab" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L512" title="View source at line 512">Source</a></span></div>
<div class="doc-body">

Parsed LC_SYMTAB, if present.

</div>
</article>

<article class="doc-entry" id="v-macho-type-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-type-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOType" title="Data.MachO.MachOType">MachOType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-macho-type-code" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Numeric file type code for a MachOType.

</div>
</article>

<article class="doc-entry" id="v-macho-uuid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">macho-uuid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOFile" title="Data.MachO.MachOFile">MachOFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-macho-uuid" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L1155" title="View source at line 1155">Source</a></span></div>
<div class="doc-body">

Return the 16-byte UUID from LC_UUID, if present.

</div>
</article>

<article class="doc-entry" id="v-msym-desc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">msym-desc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-msym-desc" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L364" title="View source at line 364">Source</a></span></div>
<div class="doc-body">

Symbol description flags (n_desc).

</div>
</article>

<article class="doc-entry" id="v-msym-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">msym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-msym-name" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L352" title="View source at line 352">Source</a></span></div>
<div class="doc-body">

Symbol name from the Mach-O string table.

</div>
</article>

<article class="doc-entry" id="v-msym-sect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">msym-sect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-msym-sect" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

1-based section index (or 0 for NO_SECT).

</div>
</article>

<article class="doc-entry" id="v-msym-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">msym-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-msym-type" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L356" title="View source at line 356">Source</a></span></div>
<div class="doc-body">

Raw n_type byte.

</div>
</article>

<article class="doc-entry" id="v-msym-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">msym-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-msym-value" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L368" title="View source at line 368">Source</a></span></div>
<div class="doc-body">

Symbol value (address or offset).

</div>
</article>

<article class="doc-entry" id="v-sec-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-sec-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

Virtual memory address of the section.

</div>
</article>

<article class="doc-entry" id="v-sec-align">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-align</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-sec-align" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L331" title="View source at line 331">Source</a></span></div>
<div class="doc-body">

Section alignment power of two.

</div>
</article>

<article class="doc-entry" id="v-sec-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-sec-flags" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L343" title="View source at line 343">Source</a></span></div>
<div class="doc-body">

Section type and attribute flags.

</div>
</article>

<article class="doc-entry" id="v-sec-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sec-name" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L311" title="View source at line 311">Source</a></span></div>
<div class="doc-body">

Section name (up to 16 bytes, NUL-trimmed).

</div>
</article>

<article class="doc-entry" id="v-sec-nreloc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-nreloc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-sec-nreloc" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

Number of relocation entries.

</div>
</article>

<article class="doc-entry" id="v-sec-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-sec-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L327" title="View source at line 327">Source</a></span></div>
<div class="doc-body">

File byte offset of the section data.

</div>
</article>

<article class="doc-entry" id="v-sec-reloff">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-reloff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-sec-reloff" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L335" title="View source at line 335">Source</a></span></div>
<div class="doc-body">

File offset of relocation entries.

</div>
</article>

<article class="doc-entry" id="v-sec-seg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-seg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sec-seg" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L315" title="View source at line 315">Source</a></span></div>
<div class="doc-body">

Containing segment name.

</div>
</article>

<article class="doc-entry" id="v-sec-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sec-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSection" title="Data.MachO.MachOSection">MachOSection</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-sec-size" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L323" title="View source at line 323">Source</a></span></div>
<div class="doc-body">

Section size in bytes.

</div>
</article>

<article class="doc-entry" id="v-seg-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-seg-addr" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Virtual memory address of the segment.

</div>
</article>

<article class="doc-entry" id="v-seg-cmd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-cmd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-cmd" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Load command type (lc-segment or lc-segment64).

</div>
</article>

<article class="doc-entry" id="v-seg-filesz">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-filesz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-seg-filesz" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

File byte size of the segment data.

</div>
</article>

<article class="doc-entry" id="v-seg-flag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-flag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-flag" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L302" title="View source at line 302">Source</a></span></div>
<div class="doc-body">

Segment flags bitmask.

</div>
</article>

<article class="doc-entry" id="v-seg-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-len" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Byte length of the load command including its section headers.

</div>
</article>

<article class="doc-entry" id="v-seg-maxprot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-maxprot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-maxprot" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

Maximum VM protection flags.

</div>
</article>

<article class="doc-entry" id="v-seg-memsz">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-memsz</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-seg-memsz" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

Virtual memory size of the segment.

</div>
</article>

<article class="doc-entry" id="v-seg-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-seg-name" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Segment name (up to 16 bytes, NUL-trimmed).

</div>
</article>

<article class="doc-entry" id="v-seg-nsect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-nsect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-nsect" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Number of sections contained in this segment.

</div>
</article>

<article class="doc-entry" id="v-seg-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-seg-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

File byte offset of the segment data.

</div>
</article>

<article class="doc-entry" id="v-seg-prot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">seg-prot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSegment" title="Data.MachO.MachOSegment">MachOSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-seg-prot" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Initial VM protection flags.

</div>
</article>

<article class="doc-entry" id="v-symtab-nsyms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">symtab-nsyms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-symtab-nsyms" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L381" title="View source at line 381">Source</a></span></div>
<div class="doc-body">

Number of symbol table entries.

</div>
</article>

<article class="doc-entry" id="v-symtab-stroff">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">symtab-stroff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-symtab-stroff" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L385" title="View source at line 385">Source</a></span></div>
<div class="doc-body">

File byte offset of the string table.

</div>
</article>

<article class="doc-entry" id="v-symtab-strsize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">symtab-strsize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-symtab-strsize" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L389" title="View source at line 389">Source</a></span></div>
<div class="doc-body">

Byte length of the string table.

</div>
</article>

<article class="doc-entry" id="v-symtab-symoff">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">symtab-symoff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-symtab-symoff" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L377" title="View source at line 377">Source</a></span></div>
<div class="doc-body">

File byte offset of the symbol table.

</div>
</article>

<article class="doc-entry" id="v-symtab-syms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">symtab-syms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymtab" title="Data.MachO.MachOSymtab">MachOSymtab</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MachO/#t-MachOSymbol" title="Data.MachO.MachOSymbol">MachOSymbol</a></code><span class="doc-sig-links"><a href="#v-symtab-syms" title="Permalink">#</a><a href="/packages/stdlib/Data.MachO/source/#L393" title="View source at line 393">Source</a></span></div>
<div class="doc-body">

Decoded symbol entries in table order.

</div>
</article>
