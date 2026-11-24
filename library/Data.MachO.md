# Data.MachO

Bounds-checked Mach-O 32/64 (little/big-endian) and Universal/Fat binary parser for load commands, segments, sections, symbols and dylibs.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.MachO`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.MachO.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.MachO](Data.MachO.md), [Text.Binary](Text.Binary.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure bounds-checked parser for Mach-O 32-bit, 64-bit, and universal (fat) binaries.
Supports little-endian and big-endian byte orders, file headers, load commands
(Segment, Segment64, Symtab, Dysymtab, Dylib, Dylinker, Uuid, Rpath), sections,
symbol tables, imported libraries and symbols, and multi-architecture fat archives.

</details>

# Types and constructors

<a id="type-4d6163684f437075"></a>

## `MachOCpu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOCpu</span></code></pre>

Mach-O target CPU architecture.

[Source](/reference-sources/Data.MachO.cinch) · line 142

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Cpu386</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuAmd64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuArm</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuArm64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuPpc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuPpc64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CpuOther</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span>)</code></pre>

<a id="type-4d6163684f44796c6962"></a>

## `MachODylib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachODylib</span></code></pre>

Construct a MachODylib from cmd code, library path, timestamp, current version, compat version.

[Source](/reference-sources/Data.MachO.cinch) · line 436

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachODylib</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span>)))))</code></pre>

<a id="type-4d6163684f44796c696e6b6572"></a>

## `MachODylinker`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachODylinker</span></code></pre>

Construct a MachODylinker from cmd code and dynamic linker path.

[Source](/reference-sources/Data.MachO.cinch) · line 461

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachODylinker</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylinker</span>))</code></pre>

<a id="type-4d6163684f447973796d746162"></a>

## `MachODysymtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachODysymtab</span></code></pre>

Construct a MachODysymtab from local/extdef/undef/indirect index and count pairs.

[Source](/reference-sources/Data.MachO.cinch) · line 399

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachODysymtab</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span>))))))))</code></pre>

<a id="type-4d6163684f4572726f72"></a>

## `MachOError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOError</span></code></pre>

Named errors reported when parsing or inspecting a Mach-O or fat binary.

[Source](/reference-sources/Data.MachO.cinch) · line 42

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTruncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOBadMagic</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachONotFat</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOInvalidCommand</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOInvalidSection</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOInvalidFatArch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachONoSymtab</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span></code></pre>

<a id="type-4d6163684f46617441726368"></a>

## `MachOFatArch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOFatArch</span></code></pre>

Construct a MachOFatArch from cpu, subcpu, offset, size, align, and parsed MachOFile.

[Source](/reference-sources/Data.MachO.cinch) · line 522

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOFatArch</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span>))))))</code></pre>

<a id="type-4d6163684f46617446696c65"></a>

## `MachOFatFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOFatFile</span></code></pre>

Construct a MachOFatFile from its magic word and list of architecture slices.

[Source](/reference-sources/Data.MachO.cinch) · line 551

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOFatFile</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatFile</span>))</code></pre>

<a id="type-4d6163684f46696c65"></a>

## `MachOFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOFile</span></code></pre>

Construct a MachOFile from header, loads, segments, sections, symtab, dysymtab, and backing bytes.

[Source](/reference-sources/Data.MachO.cinch) · line 493

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOFile</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span>)))))))</code></pre>

<a id="type-4d6163684f486561646572"></a>

## `MachOHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOHeader</span></code></pre>

Construct a MachOHeader from magic, byte order, is64, cpu, subcpu, file type, ncmd, cmdsz, flags.

[Source](/reference-sources/Data.MachO.cinch) · line 218

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOHeader</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span>)))))))))</code></pre>

<a id="type-4d6163684f4c6f6164436d64"></a>

## `MachOLoadCmd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOLoadCmd</span></code></pre>

Decoded Mach-O load command.

[Source](/reference-sources/Data.MachO.cinch) · line 472

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadSegment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadSymtab</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadDysymtab</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadDylib</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadDylinker</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylinker</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadUuid</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadRpath</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadRaw</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>))</code></pre>

<a id="type-4d6163684f53656374696f6e"></a>

## `MachOSection`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOSection</span></code></pre>

Construct a MachOSection from name, seg, addr, size, offset, align, reloff, nreloc, flags.

[Source](/reference-sources/Data.MachO.cinch) · line 308

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOSection</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span>)))))))))</code></pre>

<a id="type-4d6163684f5365676d656e74"></a>

## `MachOSegment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOSegment</span></code></pre>

Construct a MachOSegment from cmd, len, name, addr, memsz, offset, filesz, maxprot, prot, nsect, flag.

[Source](/reference-sources/Data.MachO.cinch) · line 259

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOSegment</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span>)))))))))))</code></pre>

<a id="type-4d6163684f53796d626f6c"></a>

## `MachOSymbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOSymbol</span></code></pre>

Construct a MachOSymbol from name, n_type, n_sect, n_desc, n_value.

[Source](/reference-sources/Data.MachO.cinch) · line 349

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOSymbol</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span>)))))</code></pre>

<a id="type-4d6163684f53796d746162"></a>

## `MachOSymtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOSymtab</span></code></pre>

Construct a MachOSymtab from symoff, nsyms, stroff, strsize, and symbol list.

[Source](/reference-sources/Data.MachO.cinch) · line 374

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOSymtab</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span>)))))</code></pre>

<a id="type-4d6163684f54797065"></a>

## `MachOType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MachOType</span></code></pre>

Mach-O object file type.

[Source](/reference-sources/Data.MachO.cinch) · line 181

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTypeObj</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTypeExec</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTypeDylib</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTypeBundle</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MachOTypeOther</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span>)</code></pre>

# Values

<a id="value-64796c69622d636d64"></a>

## `dylib-cmd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylib-cmd</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Load command type code.

[Source](/reference-sources/Data.MachO.cinch) · line 439

<a id="value-64796c69622d636f6d7061742d76657273696f6e"></a>

## `dylib-compat-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylib-compat-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compatibility version packed as major.minor.patch.

[Source](/reference-sources/Data.MachO.cinch) · line 455

<a id="value-64796c69622d63757272656e742d76657273696f6e"></a>

## `dylib-current-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylib-current-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Current library version packed as major.minor.patch.

[Source](/reference-sources/Data.MachO.cinch) · line 451

<a id="value-64796c69622d6e616d65"></a>

## `dylib-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylib-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Library path name.

[Source](/reference-sources/Data.MachO.cinch) · line 443

<a id="value-64796c69622d74696d65"></a>

## `dylib-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylib-time</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Build timestamp.

[Source](/reference-sources/Data.MachO.cinch) · line 447

<a id="value-64796c696e6b65722d636d64"></a>

## `dylinker-cmd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylinker-cmd</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylinker</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Load command type code.

[Source](/reference-sources/Data.MachO.cinch) · line 464

<a id="value-64796c696e6b65722d6e616d65"></a>

## `dylinker-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dylinker-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylinker</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Dynamic linker path name.

[Source](/reference-sources/Data.MachO.cinch) · line 468

<a id="value-647973796d2d6965787464656673796d"></a>

## `dysym-iextdefsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-iextdefsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Start index of externally defined symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 410

<a id="value-647973796d2d696c6f63616c73796d"></a>

## `dysym-ilocalsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-ilocalsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Start index of local symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 402

<a id="value-647973796d2d696e6469726563746f6666"></a>

## `dysym-indirectoff`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-indirectoff</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File offset of indirect symbol table.

[Source](/reference-sources/Data.MachO.cinch) · line 426

<a id="value-647973796d2d69756e64656673796d"></a>

## `dysym-iundefsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-iundefsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Start index of undefined symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 418

<a id="value-647973796d2d6e65787464656673796d"></a>

## `dysym-nextdefsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-nextdefsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of externally defined symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 414

<a id="value-647973796d2d6e696e64697265637473796d73"></a>

## `dysym-nindirectsyms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-nindirectsyms</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of indirect symbol table entries.

[Source](/reference-sources/Data.MachO.cinch) · line 430

<a id="value-647973796d2d6e6c6f63616c73796d"></a>

## `dysym-nlocalsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-nlocalsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of local symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 406

<a id="value-647973796d2d6e756e64656673796d"></a>

## `dysym-nundefsym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dysym-nundefsym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of undefined symbols.

[Source](/reference-sources/Data.MachO.cinch) · line 422

<a id="value-6661742d617263682d616c69676e"></a>

## `fat-arch-align`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-align</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Alignment power of two of the slice.

[Source](/reference-sources/Data.MachO.cinch) · line 541

<a id="value-6661742d617263682d637075"></a>

## `fat-arch-cpu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-cpu</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span>)</code></pre>

Architecture CPU type.

[Source](/reference-sources/Data.MachO.cinch) · line 525

<a id="value-6661742d617263682d66696c65"></a>

## `fat-arch-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-file</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span>)</code></pre>

Parsed Mach-O binary for this architecture slice.

[Source](/reference-sources/Data.MachO.cinch) · line 545

<a id="value-6661742d617263682d6f6666736574"></a>

## `fat-arch-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Byte offset of the slice in the fat archive.

[Source](/reference-sources/Data.MachO.cinch) · line 533

<a id="value-6661742d617263682d73697a65"></a>

## `fat-arch-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Byte size of the slice in the fat archive.

[Source](/reference-sources/Data.MachO.cinch) · line 537

<a id="value-6661742d617263682d737562637075"></a>

## `fat-arch-subcpu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fat-arch-subcpu</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Architecture CPU subtype.

[Source](/reference-sources/Data.MachO.cinch) · line 529

<a id="value-6c632d647973796d746162"></a>

## `lc-dysymtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-dysymtab</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_DYSYMTAB load command code (0xb).

[Source](/reference-sources/Data.MachO.cinch) · line 102

<a id="value-6c632d69642d64796c6962"></a>

## `lc-id-dylib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-id-dylib</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_ID_DYLIB load command code (0xd).

[Source](/reference-sources/Data.MachO.cinch) · line 110

<a id="value-6c632d69642d64796c696e6b6572"></a>

## `lc-id-dylinker`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-id-dylinker</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_ID_DYLINKER load command code (0xf).

[Source](/reference-sources/Data.MachO.cinch) · line 118

<a id="value-6c632d6c6f61642d64796c6962"></a>

## `lc-load-dylib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-load-dylib</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_LOAD_DYLIB load command code (0xc).

[Source](/reference-sources/Data.MachO.cinch) · line 106

<a id="value-6c632d6c6f61642d64796c696e6b6572"></a>

## `lc-load-dylinker`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-load-dylinker</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_LOAD_DYLINKER load command code (0xe).

[Source](/reference-sources/Data.MachO.cinch) · line 114

<a id="value-6c632d6c6f61642d7765616b2d64796c6962"></a>

## `lc-load-weak-dylib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-load-weak-dylib</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_LOAD_WEAK_DYLIB load command code (0x80000018).

[Source](/reference-sources/Data.MachO.cinch) · line 130

<a id="value-6c632d72656578706f72742d64796c6962"></a>

## `lc-reexport-dylib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-reexport-dylib</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_REEXPORT_DYLIB load command code (0x8000001f).

[Source](/reference-sources/Data.MachO.cinch) · line 138

<a id="value-6c632d7270617468"></a>

## `lc-rpath`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-rpath</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_RPATH load command code (0x8000001c).

[Source](/reference-sources/Data.MachO.cinch) · line 134

<a id="value-6c632d7365676d656e74"></a>

## `lc-segment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-segment</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_SEGMENT load command code (0x1).

[Source](/reference-sources/Data.MachO.cinch) · line 86

<a id="value-6c632d7365676d656e743634"></a>

## `lc-segment64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-segment64</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_SEGMENT_64 load command code (0x19).

[Source](/reference-sources/Data.MachO.cinch) · line 122

<a id="value-6c632d73796d746162"></a>

## `lc-symtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-symtab</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_SYMTAB load command code (0x2).

[Source](/reference-sources/Data.MachO.cinch) · line 90

<a id="value-6c632d746872656164"></a>

## `lc-thread`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-thread</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_THREAD load command code (0x4).

[Source](/reference-sources/Data.MachO.cinch) · line 94

<a id="value-6c632d756e69782d746872656164"></a>

## `lc-unix-thread`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-unix-thread</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_UNIXTHREAD load command code (0x5).

[Source](/reference-sources/Data.MachO.cinch) · line 98

<a id="value-6c632d75756964"></a>

## `lc-uuid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lc-uuid</span> :: <span class="tok-constructor">U32</span></code></pre>

LC_UUID load command code (0x1b).

[Source](/reference-sources/Data.MachO.cinch) · line 126

<a id="value-6d6163686f2d627974652d6f72646572"></a>

## `macho-byte-order`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-byte-order</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span>)</code></pre>

Byte order used by this Mach-O binary.

[Source](/reference-sources/Data.MachO.cinch) · line 225

<a id="value-6d6163686f2d636d64737a"></a>

## `macho-cmdsz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-cmdsz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Total byte size of all load commands.

[Source](/reference-sources/Data.MachO.cinch) · line 249

<a id="value-6d6163686f2d637075"></a>

## `macho-cpu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-cpu</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span>)</code></pre>

Target CPU architecture.

[Source](/reference-sources/Data.MachO.cinch) · line 233

<a id="value-6d6163686f2d6370752d636f6465"></a>

## `macho-cpu-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-cpu-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOCpu</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Numeric CPU type code for a MachOCpu.

[Source](/reference-sources/Data.MachO.cinch) · line 170

<a id="value-6d6163686f2d64796c696273"></a>

## `macho-dylibs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-dylibs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODylib</span>))</code></pre>

Return all dynamic library load commands (LC_LOAD_DYLIB, LC_ID_DYLIB, weak, reexport).

[Source](/reference-sources/Data.MachO.cinch) · line 1098

<a id="value-6d6163686f2d64796c696e6b6572"></a>

## `macho-dylinker`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-dylinker</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the dynamic linker path from LC_LOAD_DYLINKER or LC_ID_DYLINKER, if present.

[Source](/reference-sources/Data.MachO.cinch) · line 1167

<a id="value-6d6163686f2d647973796d746162"></a>

## `macho-dysymtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-dysymtab</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachODysymtab</span>))</code></pre>

Parsed LC_DYSYMTAB, if present.

[Source](/reference-sources/Data.MachO.cinch) · line 516

<a id="value-6d6163686f2d6572726f722d6d657373616765"></a>

## `macho-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic for a Mach-O error.

[Source](/reference-sources/Data.MachO.cinch) · line 59

<a id="value-6d6163686f2d6661742d617263686573"></a>

## `macho-fat-arches`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-fat-arches</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatArch</span>))</code></pre>

Architecture slices contained in the fat archive.

[Source](/reference-sources/Data.MachO.cinch) · line 558

<a id="value-6d6163686f2d6661742d6d61676963"></a>

## `macho-fat-magic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-fat-magic</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatFile</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Magic word of the fat archive (macho-magic-fat).

[Source](/reference-sources/Data.MachO.cinch) · line 554

<a id="value-6d6163686f2d66696c652d74797065"></a>

## `macho-file-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-file-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span>)</code></pre>

Mach-O file type.

[Source](/reference-sources/Data.MachO.cinch) · line 241

<a id="value-6d6163686f2d666c616773"></a>

## `macho-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-flags</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Header flags bitmask.

[Source](/reference-sources/Data.MachO.cinch) · line 253

<a id="value-6d6163686f2d686561646572"></a>

## `macho-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-header</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span>)</code></pre>

File header of a parsed Mach-O binary.

[Source](/reference-sources/Data.MachO.cinch) · line 496

<a id="value-6d6163686f2d696d706f727465642d6c6962726172696573"></a>

## `macho-imported-libraries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-imported-libraries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the paths of all libraries referenced by LC_LOAD_DYLIB commands.

[Source](/reference-sources/Data.MachO.cinch) · line 1112

<a id="value-6d6163686f2d696d706f727465642d73796d626f6c73"></a>

## `macho-imported-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-imported-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return the names of all undefined symbols referenced by LC_DYSYMTAB.

[Source](/reference-sources/Data.MachO.cinch) · line 1122

<a id="value-6d6163686f2d69733634"></a>

## `macho-is64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-is64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

True for 64-bit Mach-O files, False for 32-bit.

[Source](/reference-sources/Data.MachO.cinch) · line 229

<a id="value-6d6163686f2d6c6f616473"></a>

## `macho-loads`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-loads</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOLoadCmd</span>))</code></pre>

All load commands in file order.

[Source](/reference-sources/Data.MachO.cinch) · line 500

<a id="value-6d6163686f2d6d61676963"></a>

## `macho-magic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-magic</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Normalized magic number (macho-magic32 or macho-magic64).

[Source](/reference-sources/Data.MachO.cinch) · line 221

<a id="value-6d6163686f2d6d616769632d666174"></a>

## `macho-magic-fat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-magic-fat</span> :: <span class="tok-constructor">U32</span></code></pre>

Universal (fat) Mach-O magic number (0xcafebabe).

[Source](/reference-sources/Data.MachO.cinch) · line 82

<a id="value-6d6163686f2d6d616769633332"></a>

## `macho-magic32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-magic32</span> :: <span class="tok-constructor">U32</span></code></pre>

32-bit Mach-O magic number (0xfeedface).

[Source](/reference-sources/Data.MachO.cinch) · line 74

<a id="value-6d6163686f2d6d616769633634"></a>

## `macho-magic64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-magic64</span> :: <span class="tok-constructor">U32</span></code></pre>

64-bit Mach-O magic number (0xfeedfacf).

[Source](/reference-sources/Data.MachO.cinch) · line 78

<a id="value-6d6163686f2d6e636d64"></a>

## `macho-ncmd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-ncmd</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of load commands.

[Source](/reference-sources/Data.MachO.cinch) · line 245

<a id="value-6d6163686f2d7061727365"></a>

## `macho-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-parse</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span>))</code></pre>

Parse a single-architecture 32-bit or 64-bit Mach-O binary from raw bytes.

[Source](/reference-sources/Data.MachO.cinch) · line 1001

<a id="value-6d6163686f2d70617273652d666174"></a>

## `macho-parse-fat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-parse-fat</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFatFile</span>))</code></pre>

Parse a universal (fat) Mach-O binary from raw bytes.

[Source](/reference-sources/Data.MachO.cinch) · line 1214

<a id="value-6d6163686f2d727061746873"></a>

## `macho-rpaths`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-rpaths</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return all runtime library search paths from LC_RPATH load commands.

[Source](/reference-sources/Data.MachO.cinch) · line 1143

<a id="value-6d6163686f2d73656374696f6e"></a>

## `macho-section`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-section</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span>)))</code></pre>

Find the first section with the given name.

[Source](/reference-sources/Data.MachO.cinch) · line 1052

<a id="value-6d6163686f2d73656374696f6e2d64617461"></a>

## `macho-section-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-section-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the raw bytes of a Mach-O section.

[Source](/reference-sources/Data.MachO.cinch) · line 1064

<a id="value-6d6163686f2d73656374696f6e73"></a>

## `macho-sections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-sections</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span>))</code></pre>

All parsed sections across all segments in file order.

[Source](/reference-sources/Data.MachO.cinch) · line 508

<a id="value-6d6163686f2d7365676d656e74"></a>

## `macho-segment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-segment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span>)))</code></pre>

Find the first segment with the given name.

[Source](/reference-sources/Data.MachO.cinch) · line 1040

<a id="value-6d6163686f2d7365676d656e742d64617461"></a>

## `macho-segment-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-segment-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the file-backed bytes of a Mach-O segment.

[Source](/reference-sources/Data.MachO.cinch) · line 1074

<a id="value-6d6163686f2d7365676d656e7473"></a>

## `macho-segments`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-segments</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span>))</code></pre>

All parsed segments in file order.

[Source](/reference-sources/Data.MachO.cinch) · line 504

<a id="value-6d6163686f2d737562637075"></a>

## `macho-subcpu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-subcpu</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Target CPU subtype.

[Source](/reference-sources/Data.MachO.cinch) · line 237

<a id="value-6d6163686f2d73796d626f6c73"></a>

## `macho-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span>))</code></pre>

Return all symbols from LC_SYMTAB, or Nil when no symbol table is present.

[Source](/reference-sources/Data.MachO.cinch) · line 1084

<a id="value-6d6163686f2d73796d746162"></a>

## `macho-symtab`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-symtab</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span>))</code></pre>

Parsed LC_SYMTAB, if present.

[Source](/reference-sources/Data.MachO.cinch) · line 512

<a id="value-6d6163686f2d747970652d636f6465"></a>

## `macho-type-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-type-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOType</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Numeric file type code for a MachOType.

[Source](/reference-sources/Data.MachO.cinch) · line 203

<a id="value-6d6163686f2d75756964"></a>

## `macho-uuid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">macho-uuid</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return the 16-byte UUID from LC_UUID, if present.

[Source](/reference-sources/Data.MachO.cinch) · line 1155

<a id="value-6d73796d2d64657363"></a>

## `msym-desc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">msym-desc</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Symbol description flags (n_desc).

[Source](/reference-sources/Data.MachO.cinch) · line 364

<a id="value-6d73796d2d6e616d65"></a>

## `msym-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">msym-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Symbol name from the Mach-O string table.

[Source](/reference-sources/Data.MachO.cinch) · line 352

<a id="value-6d73796d2d73656374"></a>

## `msym-sect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">msym-sect</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

1-based section index (or 0 for NO_SECT).

[Source](/reference-sources/Data.MachO.cinch) · line 360

<a id="value-6d73796d2d74797065"></a>

## `msym-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">msym-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Raw n_type byte.

[Source](/reference-sources/Data.MachO.cinch) · line 356

<a id="value-6d73796d2d76616c7565"></a>

## `msym-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">msym-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Symbol value (address or offset).

[Source](/reference-sources/Data.MachO.cinch) · line 368

<a id="value-7365632d61646472"></a>

## `sec-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual memory address of the section.

[Source](/reference-sources/Data.MachO.cinch) · line 319

<a id="value-7365632d616c69676e"></a>

## `sec-align`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-align</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section alignment power of two.

[Source](/reference-sources/Data.MachO.cinch) · line 331

<a id="value-7365632d666c616773"></a>

## `sec-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-flags</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section type and attribute flags.

[Source](/reference-sources/Data.MachO.cinch) · line 343

<a id="value-7365632d6e616d65"></a>

## `sec-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Section name (up to 16 bytes, NUL-trimmed).

[Source](/reference-sources/Data.MachO.cinch) · line 311

<a id="value-7365632d6e72656c6f63"></a>

## `sec-nreloc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-nreloc</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of relocation entries.

[Source](/reference-sources/Data.MachO.cinch) · line 339

<a id="value-7365632d6f6666736574"></a>

## `sec-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File byte offset of the section data.

[Source](/reference-sources/Data.MachO.cinch) · line 327

<a id="value-7365632d72656c6f6666"></a>

## `sec-reloff`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-reloff</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File offset of relocation entries.

[Source](/reference-sources/Data.MachO.cinch) · line 335

<a id="value-7365632d736567"></a>

## `sec-seg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-seg</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Containing segment name.

[Source](/reference-sources/Data.MachO.cinch) · line 315

<a id="value-7365632d73697a65"></a>

## `sec-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sec-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSection</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Section size in bytes.

[Source](/reference-sources/Data.MachO.cinch) · line 323

<a id="value-7365672d61646472"></a>

## `seg-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual memory address of the segment.

[Source](/reference-sources/Data.MachO.cinch) · line 274

<a id="value-7365672d636d64"></a>

## `seg-cmd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-cmd</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Load command type (lc-segment or lc-segment64).

[Source](/reference-sources/Data.MachO.cinch) · line 262

<a id="value-7365672d66696c65737a"></a>

## `seg-filesz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-filesz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

File byte size of the segment data.

[Source](/reference-sources/Data.MachO.cinch) · line 286

<a id="value-7365672d666c6167"></a>

## `seg-flag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-flag</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Segment flags bitmask.

[Source](/reference-sources/Data.MachO.cinch) · line 302

<a id="value-7365672d6c656e"></a>

## `seg-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-len</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Byte length of the load command including its section headers.

[Source](/reference-sources/Data.MachO.cinch) · line 266

<a id="value-7365672d6d617870726f74"></a>

## `seg-maxprot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-maxprot</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Maximum VM protection flags.

[Source](/reference-sources/Data.MachO.cinch) · line 290

<a id="value-7365672d6d656d737a"></a>

## `seg-memsz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-memsz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual memory size of the segment.

[Source](/reference-sources/Data.MachO.cinch) · line 278

<a id="value-7365672d6e616d65"></a>

## `seg-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Segment name (up to 16 bytes, NUL-trimmed).

[Source](/reference-sources/Data.MachO.cinch) · line 270

<a id="value-7365672d6e73656374"></a>

## `seg-nsect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-nsect</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of sections contained in this segment.

[Source](/reference-sources/Data.MachO.cinch) · line 298

<a id="value-7365672d6f6666736574"></a>

## `seg-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

File byte offset of the segment data.

[Source](/reference-sources/Data.MachO.cinch) · line 282

<a id="value-7365672d70726f74"></a>

## `seg-prot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">seg-prot</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSegment</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Initial VM protection flags.

[Source](/reference-sources/Data.MachO.cinch) · line 294

<a id="value-73796d7461622d6e73796d73"></a>

## `symtab-nsyms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">symtab-nsyms</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of symbol table entries.

[Source](/reference-sources/Data.MachO.cinch) · line 381

<a id="value-73796d7461622d7374726f6666"></a>

## `symtab-stroff`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">symtab-stroff</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File byte offset of the string table.

[Source](/reference-sources/Data.MachO.cinch) · line 385

<a id="value-73796d7461622d73747273697a65"></a>

## `symtab-strsize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">symtab-strsize</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Byte length of the string table.

[Source](/reference-sources/Data.MachO.cinch) · line 389

<a id="value-73796d7461622d73796d6f6666"></a>

## `symtab-symoff`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">symtab-symoff</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File byte offset of the symbol table.

[Source](/reference-sources/Data.MachO.cinch) · line 377

<a id="value-73796d7461622d73796d73"></a>

## `symtab-syms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">symtab-syms</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymtab</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MachO</span>.<span class="tok-constructor">MachOSymbol</span>))</code></pre>

Decoded symbol entries in table order.

[Source](/reference-sources/Data.MachO.cinch) · line 393

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/MachO.cinch`. Source SHA-256: `f74ea71949e205401177655e8d4886e2d72916691c62a9c8074d12391e2588a0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
