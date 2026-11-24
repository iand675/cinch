# Data.Pe

Bounds-checked PE32, PE32+ and COFF binary parser for headers, data directories, sections, COFF symbols, imports and exports.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Pe`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Pe.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Pe](Data.Pe.md), [Text.Binary](Text.Binary.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure bounds-checked parser for PE32, PE32+, and COFF object/executable files.
Supports DOS header and PE signature validation, COFF file headers, PE32 and PE32+
optional headers with data directories, section headers and section data, COFF symbol
and string tables (including auxiliary record skipping and long /offset section names),
import directories and imported libraries/symbols, and export directories.

</details>

# Types and constructors

<a id="type-5065446174614469726563746f7279"></a>

## `PeDataDirectory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeDataDirectory</span></code></pre>

Construct a PeDataDirectory from relative virtual address and byte size.

[Source](/reference-sources/Data.Pe.cinch) · line 303

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeDataDirectory</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span>))</code></pre>

<a id="type-50654572726f72"></a>

## `PeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeError</span></code></pre>

Named errors reported when parsing or inspecting a PE or COFF binary.

[Source](/reference-sources/Data.Pe.cinch) · line 45

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeTruncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeBadSignature</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeUnsupportedMachine</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeUnsupportedOptionalMagic</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeInvalidHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeInvalidSection</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeInvalidTable</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span>)</code></pre>

<a id="type-50654578706f7274"></a>

## `PeExport`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeExport</span></code></pre>

Construct a PeExport from exported name, ordinal, RVA, and optional forwarder string.

[Source](/reference-sources/Data.Pe.cinch) · line 565

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeExport</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span>))))</code></pre>

<a id="type-50654578706f72744469726563746f7279"></a>

## `PeExportDirectory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeExportDirectory</span></code></pre>

Construct a PeExportDirectory from DLL name, ordinal base, function count, name count, and export entries.

[Source](/reference-sources/Data.Pe.cinch) · line 586

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeExportDirectory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span>)))))</code></pre>

<a id="type-506546696c65"></a>

## `PeFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeFile</span></code></pre>

Construct a PeFile from file header, optional header, sections, symbols, string table, and backing bytes.

[Source](/reference-sources/Data.Pe.cinch) · line 611

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeFile</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span>))))))</code></pre>

<a id="type-506546696c65486561646572"></a>

## `PeFileHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeFileHeader</span></code></pre>

Construct a PeFileHeader from machine, num sections, timestamp, symtab ptr, num symbols, optional header size, characteristics.

[Source](/reference-sources/Data.Pe.cinch) · line 270

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeFileHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span>)))))))</code></pre>

<a id="type-5065496d706f72744469726563746f7279"></a>

## `PeImportDirectory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeImportDirectory</span></code></pre>

Construct a PeImportDirectory from original first thunk, timestamp, forwarder chain, name RVA, first thunk, and resolved DLL name.

[Source](/reference-sources/Data.Pe.cinch) · line 511

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeImportDirectory</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span>))))))</code></pre>

<a id="type-5065496d706f727453796d626f6c"></a>

## `PeImportSymbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeImportSymbol</span></code></pre>

Construct a PeImportSymbol from DLL name, function name (empty if ordinal), hint, ordinal, and by-ordinal flag.

[Source](/reference-sources/Data.Pe.cinch) · line 540

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeImportSymbol</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span>)))))</code></pre>

<a id="type-50654d616368696e65"></a>

## `PeMachine`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeMachine</span></code></pre>

COFF target machine architecture.

[Source](/reference-sources/Data.Pe.cinch) · line 173

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineUnknown</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineI386</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineAmd64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineArm</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineArmnt</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineArm64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineRiscv32</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineRiscv64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeMachineOther</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span>)</code></pre>

<a id="type-50654f7074696f6e616c486561646572"></a>

## `PeOptionalHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeOptionalHeader</span></code></pre>

Optional header present in a PE image, or absent in a COFF object file.

[Source](/reference-sources/Data.Pe.cinch) · line 334

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeOptNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeOpt32</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeOpt64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader</span>)</code></pre>

<a id="type-50654f7074696f6e616c4865616465723332"></a>

## `PeOptionalHeader32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeOptionalHeader32</span></code></pre>

Construct a PeOptionalHeader32 from standard fields, base of data, 32-bit image base, Windows fields, and data directories.

[Source](/reference-sources/Data.Pe.cinch) · line 326

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeOptionalHeader32</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptStd</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptWin</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span>)))))</code></pre>

<a id="type-50654f7074696f6e616c4865616465723634"></a>

## `PeOptionalHeader64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeOptionalHeader64</span></code></pre>

Construct a PeOptionalHeader64 from standard fields, 64-bit image base, Windows fields, and data directories.

[Source](/reference-sources/Data.Pe.cinch) · line 331

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeOptionalHeader64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptStd</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptWin</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span>))))</code></pre>

<a id="type-506553656374696f6e"></a>

## `PeSection`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeSection</span></code></pre>

Construct a PeSection from name, virtual size, virtual address, raw size, raw offset, reloc ptr, line ptr, num relocs, num lines, characteristics.

[Source](/reference-sources/Data.Pe.cinch) · line 441

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSection</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span>))))))))))</code></pre>

<a id="type-506553756273797374656d"></a>

## `PeSubsystem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeSubsystem</span></code></pre>

Windows subsystem required to run a PE image.

[Source](/reference-sources/Data.Pe.cinch) · line 225

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemUnknown</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemNative</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemWindowsGui</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemWindowsCui</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemPosixCui</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemEfiApplication</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSubsystemOther</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span>)</code></pre>

<a id="type-506553796d626f6c"></a>

## `PeSymbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PeSymbol</span></code></pre>

Construct a PeSymbol from name, value, signed section number, type, storage class.

[Source](/reference-sources/Data.Pe.cinch) · line 486

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PeSymbol</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span>)))))</code></pre>

# Values

<a id="value-70652d646174612d6469726563746f7279"></a>

## `pe-data-directory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-data-directory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span>)))</code></pre>

Return the DataDirectory entry at index (e.g. pe-dir-export or pe-dir-import), if present.

[Source](/reference-sources/Data.Pe.cinch) · line 1025

<a id="value-70652d64642d73697a65"></a>

## `pe-dd-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dd-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Byte size of the table.

[Source](/reference-sources/Data.Pe.cinch) · line 310

<a id="value-70652d64642d7669727475616c2d61646472657373"></a>

## `pe-dd-virtual-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dd-virtual-address</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Relative virtual address (RVA) of the table.

[Source](/reference-sources/Data.Pe.cinch) · line 306

<a id="value-70652d6469722d6261736572656c6f63"></a>

## `pe-dir-basereloc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-basereloc</span> :: <span class="tok-constructor">Nat</span></code></pre>

Base relocation table data directory index (5).

[Source](/reference-sources/Data.Pe.cinch) · line 105

<a id="value-70652d6469722d636c72"></a>

## `pe-dir-clr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-clr</span> :: <span class="tok-constructor">Nat</span></code></pre>

CLR runtime header data directory index (14).

[Source](/reference-sources/Data.Pe.cinch) · line 121

<a id="value-70652d6469722d6465627567"></a>

## `pe-dir-debug`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-debug</span> :: <span class="tok-constructor">Nat</span></code></pre>

Debug data directory index (6).

[Source](/reference-sources/Data.Pe.cinch) · line 109

<a id="value-70652d6469722d657863657074696f6e"></a>

## `pe-dir-exception`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-exception</span> :: <span class="tok-constructor">Nat</span></code></pre>

Exception table data directory index (3).

[Source](/reference-sources/Data.Pe.cinch) · line 97

<a id="value-70652d6469722d6578706f7274"></a>

## `pe-dir-export`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-export</span> :: <span class="tok-constructor">Nat</span></code></pre>

Export table data directory index (0).

[Source](/reference-sources/Data.Pe.cinch) · line 85

<a id="value-70652d6469722d696174"></a>

## `pe-dir-iat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-iat</span> :: <span class="tok-constructor">Nat</span></code></pre>

Import address table data directory index (12).

[Source](/reference-sources/Data.Pe.cinch) · line 117

<a id="value-70652d6469722d696d706f7274"></a>

## `pe-dir-import`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-import</span> :: <span class="tok-constructor">Nat</span></code></pre>

Import table data directory index (1).

[Source](/reference-sources/Data.Pe.cinch) · line 89

<a id="value-70652d6469722d7265736f75726365"></a>

## `pe-dir-resource`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-resource</span> :: <span class="tok-constructor">Nat</span></code></pre>

Resource table data directory index (2).

[Source](/reference-sources/Data.Pe.cinch) · line 93

<a id="value-70652d6469722d7365637572697479"></a>

## `pe-dir-security`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-security</span> :: <span class="tok-constructor">Nat</span></code></pre>

Certificate/security table data directory index (4).

[Source](/reference-sources/Data.Pe.cinch) · line 101

<a id="value-70652d6469722d746c73"></a>

## `pe-dir-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-dir-tls</span> :: <span class="tok-constructor">Nat</span></code></pre>

Thread-local storage data directory index (9).

[Source](/reference-sources/Data.Pe.cinch) · line 113

<a id="value-70652d6572726f722d6d657373616765"></a>

## `pe-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic for a PE error.

[Source](/reference-sources/Data.Pe.cinch) · line 62

<a id="value-70652d6578706469722d62617365"></a>

## `pe-expdir-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-expdir-base</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Starting ordinal number (Base).

[Source](/reference-sources/Data.Pe.cinch) · line 593

<a id="value-70652d6578706469722d656e7472696573"></a>

## `pe-expdir-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-expdir-entries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span>))</code></pre>

Named export entries in Export Name Pointer Table order.

[Source](/reference-sources/Data.Pe.cinch) · line 605

<a id="value-70652d6578706469722d6e616d65"></a>

## `pe-expdir-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-expdir-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Internal DLL name stored in the export directory.

[Source](/reference-sources/Data.Pe.cinch) · line 589

<a id="value-70652d6578706469722d6e756d2d66756e6374696f6e73"></a>

## `pe-expdir-num-functions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-expdir-num-functions</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Total number of entries in the Export Address Table.

[Source](/reference-sources/Data.Pe.cinch) · line 597

<a id="value-70652d6578706469722d6e756d2d6e616d6573"></a>

## `pe-expdir-num-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-expdir-num-names</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of named exports in the Export Name Pointer Table.

[Source](/reference-sources/Data.Pe.cinch) · line 601

<a id="value-70652d6578706f72742d666f72776172646572"></a>

## `pe-export-forwarder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-export-forwarder</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Forwarder string when the export forwards to another DLL, or None.

[Source](/reference-sources/Data.Pe.cinch) · line 580

<a id="value-70652d6578706f72742d6e616d65"></a>

## `pe-export-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-export-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Exported symbol name.

[Source](/reference-sources/Data.Pe.cinch) · line 568

<a id="value-70652d6578706f72742d6f7264696e616c"></a>

## `pe-export-ordinal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-export-ordinal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Exported ordinal (ordinal base + unbiased ordinal).

[Source](/reference-sources/Data.Pe.cinch) · line 572

<a id="value-70652d6578706f72742d727661"></a>

## `pe-export-rva`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-export-rva</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExport</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Relative virtual address of the exported function or data.

[Source](/reference-sources/Data.Pe.cinch) · line 576

<a id="value-70652d6578706f727473"></a>

## `pe-exports`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-exports</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeExportDirectory</span>)))</code></pre>

Parse the PE Export Directory Table (IMAGE_DIRECTORY_ENTRY_EXPORT), or None when absent.

[Source](/reference-sources/Data.Pe.cinch) · line 1245

<a id="value-70652d66682d636861726163746572697374696373"></a>

## `pe-fh-characteristics`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-characteristics</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Image characteristics bitmask.

[Source](/reference-sources/Data.Pe.cinch) · line 297

<a id="value-70652d66682d6d616368696e65"></a>

## `pe-fh-machine`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-machine</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span>)</code></pre>

Target machine architecture.

[Source](/reference-sources/Data.Pe.cinch) · line 273

<a id="value-70652d66682d6e756d2d73656374696f6e73"></a>

## `pe-fh-num-sections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-num-sections</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Number of section headers.

[Source](/reference-sources/Data.Pe.cinch) · line 277

<a id="value-70652d66682d6e756d2d73796d626f6c73"></a>

## `pe-fh-num-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-num-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Number of entries in the COFF symbol table.

[Source](/reference-sources/Data.Pe.cinch) · line 289

<a id="value-70652d66682d6f7074696f6e616c2d6865616465722d73697a65"></a>

## `pe-fh-optional-header-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-optional-header-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Byte size of the optional header (0 for COFF object files).

[Source](/reference-sources/Data.Pe.cinch) · line 293

<a id="value-70652d66682d73796d626f6c2d7461626c652d707472"></a>

## `pe-fh-symbol-table-ptr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-symbol-table-ptr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File offset of the COFF symbol table, or 0 if absent.

[Source](/reference-sources/Data.Pe.cinch) · line 285

<a id="value-70652d66682d74696d657374616d70"></a>

## `pe-fh-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-fh-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Low 32 bits of the file creation Unix timestamp.

[Source](/reference-sources/Data.Pe.cinch) · line 281

<a id="value-70652d66696c652d646c6c"></a>

## `pe-file-dll`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-dll</span> :: <span class="tok-constructor">U16</span></code></pre>

File characteristic: image is a dynamic-link library (0x2000).

[Source](/reference-sources/Data.Pe.cinch) · line 141

<a id="value-70652d66696c652d65786563757461626c652d696d616765"></a>

## `pe-file-executable-image`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-executable-image</span> :: <span class="tok-constructor">U16</span></code></pre>

File characteristic: image is executable (0x0002).

[Source](/reference-sources/Data.Pe.cinch) · line 129

<a id="value-70652d66696c652d686561646572"></a>

## `pe-file-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-header</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFileHeader</span>)</code></pre>

COFF file header of a parsed PE binary.

[Source](/reference-sources/Data.Pe.cinch) · line 614

<a id="value-70652d66696c652d6c617267652d616464726573732d6177617265"></a>

## `pe-file-large-address-aware`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-large-address-aware</span> :: <span class="tok-constructor">U16</span></code></pre>

File characteristic: application handles addresses above 2 GiB (0x0020).

[Source](/reference-sources/Data.Pe.cinch) · line 133

<a id="value-70652d66696c652d6d616368696e653332"></a>

## `pe-file-machine32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-machine32</span> :: <span class="tok-constructor">U16</span></code></pre>

File characteristic: 32-bit word architecture (0x0100).

[Source](/reference-sources/Data.Pe.cinch) · line 137

<a id="value-70652d66696c652d72656c6f63732d7374726970706564"></a>

## `pe-file-relocs-stripped`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-file-relocs-stripped</span> :: <span class="tok-constructor">U16</span></code></pre>

File characteristic: base relocations stripped (0x0001).

[Source](/reference-sources/Data.Pe.cinch) · line 125

<a id="value-70652d696d702d646c6c"></a>

## `pe-imp-dll`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-dll</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Resolved imported DLL name.

[Source](/reference-sources/Data.Pe.cinch) · line 534

<a id="value-70652d696d702d66697273742d7468756e6b"></a>

## `pe-imp-first-thunk`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-first-thunk</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

RVA of the Import Address Table (FirstThunk).

[Source](/reference-sources/Data.Pe.cinch) · line 530

<a id="value-70652d696d702d666f727761726465722d636861696e"></a>

## `pe-imp-forwarder-chain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-forwarder-chain</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Index of first forwarder reference.

[Source](/reference-sources/Data.Pe.cinch) · line 522

<a id="value-70652d696d702d6e616d652d727661"></a>

## `pe-imp-name-rva`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-name-rva</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

RVA of the imported DLL name string.

[Source](/reference-sources/Data.Pe.cinch) · line 526

<a id="value-70652d696d702d6f7269672d66697273742d7468756e6b"></a>

## `pe-imp-orig-first-thunk`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-orig-first-thunk</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

RVA of the Import Lookup Table (OriginalFirstThunk).

[Source](/reference-sources/Data.Pe.cinch) · line 514

<a id="value-70652d696d702d74696d657374616d70"></a>

## `pe-imp-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imp-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Binding timestamp.

[Source](/reference-sources/Data.Pe.cinch) · line 518

<a id="value-70652d696d706f72742d6469726563746f72696573"></a>

## `pe-import-directories`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-import-directories</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportDirectory</span>)))</code></pre>

Parse all entries in the PE Import Directory Table (IMAGE_DIRECTORY_ENTRY_IMPORT).

[Source](/reference-sources/Data.Pe.cinch) · line 1104

<a id="value-70652d696d706f727465642d6c6962726172696573"></a>

## `pe-imported-libraries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imported-libraries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return the names of all DLLs referenced in the PE Import Directory.

[Source](/reference-sources/Data.Pe.cinch) · line 1127

<a id="value-70652d696d706f727465642d73796d626f6c2d656e7472696573"></a>

## `pe-imported-symbol-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imported-symbol-entries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span>)))</code></pre>

Return structured imported symbol entries across all imported DLLs (including ordinal imports).

[Source](/reference-sources/Data.Pe.cinch) · line 1195

<a id="value-70652d696d706f727465642d73796d626f6c73"></a>

## `pe-imported-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-imported-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return all named imported symbols formatted as "symbol:dll".

[Source](/reference-sources/Data.Pe.cinch) · line 1212

<a id="value-70652d696d7073796d2d62792d6f7264696e616c"></a>

## `pe-impsym-by-ordinal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-impsym-by-ordinal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

True when imported by ordinal rather than by name.

[Source](/reference-sources/Data.Pe.cinch) · line 559

<a id="value-70652d696d7073796d2d646c6c"></a>

## `pe-impsym-dll`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-impsym-dll</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Name of the imported DLL.

[Source](/reference-sources/Data.Pe.cinch) · line 543

<a id="value-70652d696d7073796d2d68696e74"></a>

## `pe-impsym-hint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-impsym-hint</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Hint index into the DLL's export name pointer table (0 when imported by ordinal).

[Source](/reference-sources/Data.Pe.cinch) · line 551

<a id="value-70652d696d7073796d2d6e616d65"></a>

## `pe-impsym-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-impsym-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Imported function name, or empty string when imported by ordinal.

[Source](/reference-sources/Data.Pe.cinch) · line 547

<a id="value-70652d696d7073796d2d6f7264696e616c"></a>

## `pe-impsym-ordinal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-impsym-ordinal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeImportSymbol</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Imported ordinal number (0 when imported by name).

[Source](/reference-sources/Data.Pe.cinch) · line 555

<a id="value-70652d6d616368696e652d636f6465"></a>

## `pe-machine-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-machine-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeMachine</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Numeric COFF machine code for a PeMachine.

[Source](/reference-sources/Data.Pe.cinch) · line 212

<a id="value-70652d6d616769632d70653332"></a>

## `pe-magic-pe32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-magic-pe32</span> :: <span class="tok-constructor">U16</span></code></pre>

Optional header magic for 32-bit PE32 images (0x10b).

[Source](/reference-sources/Data.Pe.cinch) · line 77

<a id="value-70652d6d616769632d70653634"></a>

## `pe-magic-pe64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-magic-pe64</span> :: <span class="tok-constructor">U16</span></code></pre>

Optional header magic for 64-bit PE32+ images (0x20b).

[Source](/reference-sources/Data.Pe.cinch) · line 81

<a id="value-70652d6f707433322d636865636b73756d"></a>

## `pe-opt32-checksum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-checksum</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Image checksum for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 371

<a id="value-70652d6f707433322d646174612d6469726563746f72696573"></a>

## `pe-opt32-data-directories`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-data-directories</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span>))</code></pre>

Parsed data directory entries for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 387

<a id="value-70652d6f707433322d646c6c2d636861726163746572697374696373"></a>

## `pe-opt32-dll-characteristics`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-dll-characteristics</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

DLL characteristics bitmask for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 379

<a id="value-70652d6f707433322d656e7472792d706f696e74"></a>

## `pe-opt32-entry-point`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-entry-point</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Entry point RVA for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 347

<a id="value-70652d6f707433322d66696c652d616c69676e6d656e74"></a>

## `pe-opt32-file-alignment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-file-alignment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section alignment in the file for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 359

<a id="value-70652d6f707433322d696d6167652d62617365"></a>

## `pe-opt32-image-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-image-base</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Preferred 32-bit image load address for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 351

<a id="value-70652d6f707433322d6d61676963"></a>

## `pe-opt32-magic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-magic</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Optional header magic for PE32 (0x10b).

[Source](/reference-sources/Data.Pe.cinch) · line 343

<a id="value-70652d6f707433322d6e756d2d7276612d616e642d73697a6573"></a>

## `pe-opt32-num-rva-and-sizes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-num-rva-and-sizes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Declared count of data directory entries for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 383

<a id="value-70652d6f707433322d73656374696f6e2d616c69676e6d656e74"></a>

## `pe-opt32-section-alignment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-section-alignment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section alignment in memory for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 355

<a id="value-70652d6f707433322d73697a652d6f662d68656164657273"></a>

## `pe-opt32-size-of-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-size-of-headers</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Combined header size for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 367

<a id="value-70652d6f707433322d73697a652d6f662d696d616765"></a>

## `pe-opt32-size-of-image`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-size-of-image</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Total image size in memory for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 363

<a id="value-70652d6f707433322d73756273797374656d"></a>

## `pe-opt32-subsystem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt32-subsystem</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span>)</code></pre>

Required subsystem for PE32.

[Source](/reference-sources/Data.Pe.cinch) · line 375

<a id="value-70652d6f707436342d636865636b73756d"></a>

## `pe-opt64-checksum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-checksum</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Image checksum for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 419

<a id="value-70652d6f707436342d646174612d6469726563746f72696573"></a>

## `pe-opt64-data-directories`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-data-directories</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeDataDirectory</span>))</code></pre>

Parsed data directory entries for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 435

<a id="value-70652d6f707436342d646c6c2d636861726163746572697374696373"></a>

## `pe-opt64-dll-characteristics`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-dll-characteristics</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

DLL characteristics bitmask for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 427

<a id="value-70652d6f707436342d656e7472792d706f696e74"></a>

## `pe-opt64-entry-point`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-entry-point</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Entry point RVA for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 395

<a id="value-70652d6f707436342d66696c652d616c69676e6d656e74"></a>

## `pe-opt64-file-alignment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-file-alignment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section alignment in the file for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 407

<a id="value-70652d6f707436342d696d6167652d62617365"></a>

## `pe-opt64-image-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-image-base</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Preferred 64-bit image load address for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 399

<a id="value-70652d6f707436342d6d61676963"></a>

## `pe-opt64-magic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-magic</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Optional header magic for PE32+ (0x20b).

[Source](/reference-sources/Data.Pe.cinch) · line 391

<a id="value-70652d6f707436342d6e756d2d7276612d616e642d73697a6573"></a>

## `pe-opt64-num-rva-and-sizes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-num-rva-and-sizes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Declared count of data directory entries for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 431

<a id="value-70652d6f707436342d73656374696f6e2d616c69676e6d656e74"></a>

## `pe-opt64-section-alignment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-section-alignment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section alignment in memory for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 403

<a id="value-70652d6f707436342d73697a652d6f662d68656164657273"></a>

## `pe-opt64-size-of-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-size-of-headers</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Combined header size for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 415

<a id="value-70652d6f707436342d73697a652d6f662d696d616765"></a>

## `pe-opt64-size-of-image`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-size-of-image</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Total image size in memory for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 411

<a id="value-70652d6f707436342d73756273797374656d"></a>

## `pe-opt64-subsystem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-opt64-subsystem</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span>)</code></pre>

Required subsystem for PE32+.

[Source](/reference-sources/Data.Pe.cinch) · line 423

<a id="value-70652d6f7074696f6e616c2d686561646572"></a>

## `pe-optional-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-optional-header</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeOptionalHeader</span>)</code></pre>

Optional header of a parsed PE binary (PeOptNone for COFF object files).

[Source](/reference-sources/Data.Pe.cinch) · line 618

<a id="value-70652d7061727365"></a>

## `pe-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-parse</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span>))</code></pre>

Parse a PE32, PE32+, or COFF binary from raw bytes.

[Source](/reference-sources/Data.Pe.cinch) · line 960

<a id="value-70652d73636e2d636e742d636f6465"></a>

## `pe-scn-cnt-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-cnt-code</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: contains executable code (0x00000020).

[Source](/reference-sources/Data.Pe.cinch) · line 145

<a id="value-70652d73636e2d636e742d696e697469616c697a65642d64617461"></a>

## `pe-scn-cnt-initialized-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-cnt-initialized-data</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: contains initialized data (0x00000040).

[Source](/reference-sources/Data.Pe.cinch) · line 149

<a id="value-70652d73636e2d636e742d756e696e697469616c697a65642d64617461"></a>

## `pe-scn-cnt-uninitialized-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-cnt-uninitialized-data</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: contains uninitialized data (0x00000080).

[Source](/reference-sources/Data.Pe.cinch) · line 153

<a id="value-70652d73636e2d6d656d2d6469736361726461626c65"></a>

## `pe-scn-mem-discardable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-mem-discardable</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: section can be discarded (0x02000000).

[Source](/reference-sources/Data.Pe.cinch) · line 157

<a id="value-70652d73636e2d6d656d2d65786563757465"></a>

## `pe-scn-mem-execute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-mem-execute</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: section can be executed as code (0x20000000).

[Source](/reference-sources/Data.Pe.cinch) · line 161

<a id="value-70652d73636e2d6d656d2d72656164"></a>

## `pe-scn-mem-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-mem-read</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: section can be read (0x40000000).

[Source](/reference-sources/Data.Pe.cinch) · line 165

<a id="value-70652d73636e2d6d656d2d7772697465"></a>

## `pe-scn-mem-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-scn-mem-write</span> :: <span class="tok-constructor">U32</span></code></pre>

Section characteristic: section can be written (0x80000000).

[Source](/reference-sources/Data.Pe.cinch) · line 169

<a id="value-70652d7365632d636861726163746572697374696373"></a>

## `pe-sec-characteristics`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-characteristics</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section characteristics bitmask.

[Source](/reference-sources/Data.Pe.cinch) · line 480

<a id="value-70652d7365632d6c696e652d707472"></a>

## `pe-sec-line-ptr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-line-ptr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File offset of COFF line numbers.

[Source](/reference-sources/Data.Pe.cinch) · line 468

<a id="value-70652d7365632d6e616d65"></a>

## `pe-sec-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Resolved section name (including /offset COFF string table lookup).

[Source](/reference-sources/Data.Pe.cinch) · line 444

<a id="value-70652d7365632d6e756d2d6c696e6573"></a>

## `pe-sec-num-lines`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-num-lines</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Number of line number entries.

[Source](/reference-sources/Data.Pe.cinch) · line 476

<a id="value-70652d7365632d6e756d2d72656c6f6373"></a>

## `pe-sec-num-relocs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-num-relocs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Number of relocation entries.

[Source](/reference-sources/Data.Pe.cinch) · line 472

<a id="value-70652d7365632d6f6666736574"></a>

## `pe-sec-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File byte offset of the section data (PointerToRawData).

[Source](/reference-sources/Data.Pe.cinch) · line 460

<a id="value-70652d7365632d72656c6f632d707472"></a>

## `pe-sec-reloc-ptr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-reloc-ptr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

File offset of relocation entries.

[Source](/reference-sources/Data.Pe.cinch) · line 464

<a id="value-70652d7365632d73697a65"></a>

## `pe-sec-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Size of the initialized section data on disk (SizeOfRawData).

[Source](/reference-sources/Data.Pe.cinch) · line 456

<a id="value-70652d7365632d7669727475616c2d61646472657373"></a>

## `pe-sec-virtual-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-virtual-address</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Relative virtual address of the section.

[Source](/reference-sources/Data.Pe.cinch) · line 452

<a id="value-70652d7365632d7669727475616c2d73697a65"></a>

## `pe-sec-virtual-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sec-virtual-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Total size of the section when loaded into memory.

[Source](/reference-sources/Data.Pe.cinch) · line 448

<a id="value-70652d73656374696f6e"></a>

## `pe-section`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-section</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span>)))</code></pre>

Find the first section with the given name.

[Source](/reference-sources/Data.Pe.cinch) · line 1005

<a id="value-70652d73656374696f6e2d64617461"></a>

## `pe-section-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-section-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the raw bytes of a PE section.

[Source](/reference-sources/Data.Pe.cinch) · line 1017

<a id="value-70652d73656374696f6e73"></a>

## `pe-sections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sections</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSection</span>))</code></pre>

Section headers of a parsed PE binary.

[Source](/reference-sources/Data.Pe.cinch) · line 622

<a id="value-70652d73756273797374656d2d636f6465"></a>

## `pe-subsystem-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-subsystem-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSubsystem</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Numeric subsystem code for a PeSubsystem.

[Source](/reference-sources/Data.Pe.cinch) · line 253

<a id="value-70652d73796d2d6e616d65"></a>

## `pe-sym-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sym-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Resolved COFF symbol name.

[Source](/reference-sources/Data.Pe.cinch) · line 489

<a id="value-70652d73796d2d73656374696f6e2d6e756d626572"></a>

## `pe-sym-section-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sym-section-number</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

Signed 1-based section number (0 = undefined, -1 = absolute, -2 = debug).

[Source](/reference-sources/Data.Pe.cinch) · line 497

<a id="value-70652d73796d2d73746f726167652d636c617373"></a>

## `pe-sym-storage-class`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sym-storage-class</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

COFF symbol storage class byte.

[Source](/reference-sources/Data.Pe.cinch) · line 505

<a id="value-70652d73796d2d74797065"></a>

## `pe-sym-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sym-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

COFF symbol type word.

[Source](/reference-sources/Data.Pe.cinch) · line 501

<a id="value-70652d73796d2d76616c7565"></a>

## `pe-sym-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-sym-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Symbol value.

[Source](/reference-sources/Data.Pe.cinch) · line 493

<a id="value-70652d73796d626f6c73"></a>

## `pe-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pe-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Pe</span>.<span class="tok-constructor">PeSymbol</span>))</code></pre>

COFF symbols with auxiliary records omitted.

[Source](/reference-sources/Data.Pe.cinch) · line 626

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Pe.cinch`. Source SHA-256: `d9737af168909d028f4e4fc09933b41165646df2f944bb2b150c03f7160df71d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
