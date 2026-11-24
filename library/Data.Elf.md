# Data.Elf

Bounds-checked ELF32 and ELF64 (little/big-endian) parser for headers, segments, sections, symbols, relocations, dynamic tables and zlib-compressed sections.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Elf`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Elf.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Show](Text.Show.md), [Text.Zlib](Text.Zlib.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Elf](Data.Elf.md), [Text.Binary](Text.Binary.md), [Text.Flate](Text.Flate.md), [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure bounds-checked parser for ELF32 and ELF64 object and executable files.
Supports little-endian and big-endian byte orders, file headers, program headers,
section headers, string tables, static and dynamic symbol tables, REL and RELA
relocations, dynamic section tags and imported libraries/symbols, and zlib
decompression of SHF_COMPRESSED sections.

</details>

# Types and constructors

<a id="type-456c66436c617373"></a>

## `ElfClass`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfClass</span></code></pre>

ELF address-width class (EI_CLASS).

[Source](/reference-sources/Data.Elf.cinch) · line 175

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfClassNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfClass32</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfClass64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span></code></pre>

<a id="type-456c6644617461"></a>

## `ElfData`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfData</span></code></pre>

ELF data encoding (EI_DATA).

[Source](/reference-sources/Data.Elf.cinch) · line 191

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfDataNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfData2Lsb</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfData2Msb</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span></code></pre>

<a id="type-456c6644796e456e747279"></a>

## `ElfDynEntry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfDynEntry</span></code></pre>

Construct an ElfDynEntry from decoded tag, raw tag value, and entry value/pointer.

[Source](/reference-sources/Data.Elf.cinch) · line 1044

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfDynEntry</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynEntry</span>)))</code></pre>

<a id="type-456c6644796e546167"></a>

## `ElfDynTag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfDynTag</span></code></pre>

Dynamic section tag (d_tag).

[Source](/reference-sources/Data.Elf.cinch) · line 619

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtNull</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtNeeded</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtPltrelsz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtPltgot</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtHash</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtStrtab</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtSymtab</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRela</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRelasz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRelaent</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtStrsz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtSyment</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtInit</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtFini</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtSoname</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRpath</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtSymbolic</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRelsz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRelent</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtPltrel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtDebug</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtTextrel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtJmprel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtBindNow</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtInitArray</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtFiniArray</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtInitArraysz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtFiniArraysz</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtRunpath</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtFlags</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtFlags1</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtGnuHash</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DtOther</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span>)</code></pre>

<a id="type-456c664572726f72"></a>

## `ElfError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfError</span></code></pre>

Named errors reported when parsing or inspecting an ELF binary.

[Source](/reference-sources/Data.Elf.cinch) · line 53

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTruncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfBadMagic</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfUnsupportedClass</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfUnsupportedData</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfUnsupportedVersion</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfInvalidHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfInvalidSection</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfInvalidProgram</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfNoSymbols</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfNobitsSection</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfInvalidDynTag</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfUnsupportedCompression</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfDecompressFailed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span>)</code></pre>

<a id="type-456c6646696c65"></a>

## `ElfFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfFile</span></code></pre>

Construct an ElfFile from its file header, program headers, section headers, and backing bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 1061

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfFile</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span>))))</code></pre>

<a id="type-456c66486561646572"></a>

## `ElfHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfHeader</span></code></pre>

Construct an ElfHeader from class, data, version, OSABI, ABI version, byte order, type, machine, entry, flags.

[Source](/reference-sources/Data.Elf.cinch) · line 788

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span>))))))))))</code></pre>

<a id="type-456c66496d706f7274656453796d626f6c"></a>

## `ElfImportedSymbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfImportedSymbol</span></code></pre>

Construct an ElfImportedSymbol from symbol name, version, and library name.

[Source](/reference-sources/Data.Elf.cinch) · line 981

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfImportedSymbol</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfImportedSymbol</span>)))</code></pre>

<a id="type-456c664d616368696e65"></a>

## `ElfMachine`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfMachine</span></code></pre>

ELF machine architecture (e_machine).

[Source](/reference-sources/Data.Elf.cinch) · line 242

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineSparc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachine386</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineMips</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachinePpc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachinePpc64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineS390</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineArm</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineX8664</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineAarch64</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineRiscv</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineLoongArch</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfMachineOther</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span>)</code></pre>

<a id="type-456c664f73416269"></a>

## `ElfOsAbi`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfOsAbi</span></code></pre>

ELF OS/ABI identification (EI_OSABI).

[Source](/reference-sources/Data.Elf.cinch) · line 305

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiNetBsd</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiLinux</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiSolaris</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiFreeBsd</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiOpenBsd</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiArm</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiStandalone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfOsAbiOther</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span>)</code></pre>

<a id="type-456c6650726f67486561646572"></a>

## `ElfProgHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfProgHeader</span></code></pre>

Construct an ElfProgHeader from type, flags, offset, vaddr, paddr, filesz, memsz, align.

[Source](/reference-sources/Data.Elf.cinch) · line 833

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfProgHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span>))))))))</code></pre>

<a id="type-456c6650726f6754797065"></a>

## `ElfProgType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfProgType</span></code></pre>

ELF program header segment type (p_type).

[Source](/reference-sources/Data.Elf.cinch) · line 352

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtNull</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtLoad</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtDynamic</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtInterp</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtNote</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtShlib</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtPhdr</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtTls</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtGnuEhFrame</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtGnuStack</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtGnuRelro</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtGnuProperty</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PtOther</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span>)</code></pre>

<a id="type-456c6652656c"></a>

## `ElfRel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfRel</span></code></pre>

Construct an ElfRel from offset, raw info, symbol index, and relocation type.

[Source](/reference-sources/Data.Elf.cinch) · line 998

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfRel</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span>))))</code></pre>

<a id="type-456c6652656c61"></a>

## `ElfRela`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfRela</span></code></pre>

Construct an ElfRela from offset, raw info, signed addend, symbol index, and relocation type.

[Source](/reference-sources/Data.Elf.cinch) · line 1019

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfRela</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span>)))))</code></pre>

<a id="type-456c6653656374696f6e486561646572"></a>

## `ElfSectionHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSectionHeader</span></code></pre>

Construct an ElfSectionHeader from name, type, flags, addr, offset, size, link, info, addralign, entsize, file-size, compression.

[Source](/reference-sources/Data.Elf.cinch) · line 877

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfSectionHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfCompressionInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span>))))))))))))</code></pre>

<a id="type-456c6653656374696f6e54797065"></a>

## `ElfSectionType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSectionType</span></code></pre>

ELF section header type (sh_type).

[Source](/reference-sources/Data.Elf.cinch) · line 415

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtNull</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtProgbits</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtSymtab</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtStrtab</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtRela</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtHash</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtDynamic</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtNote</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtNobits</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtRel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtShlib</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtDynsym</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtInitArray</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtFiniArray</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtPreinitArray</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtGroup</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtSymtabShndx</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtGnuHash</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtGnuVerdef</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtGnuVerneed</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtGnuVersym</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ShtOther</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span>)</code></pre>

<a id="type-456c6653796d42696e64"></a>

## `ElfSymBind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSymBind</span></code></pre>

Symbol binding extracted from the upper nibble of st_info.

[Source](/reference-sources/Data.Elf.cinch) · line 514

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StbLocal</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StbGlobal</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StbWeak</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StbOther</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span>)</code></pre>

<a id="type-456c6653796d54797065"></a>

## `ElfSymType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSymType</span></code></pre>

Symbol type extracted from the lower nibble of st_info.

[Source](/reference-sources/Data.Elf.cinch) · line 533

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttNotype</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttObject</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttFunc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttSection</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttFile</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttCommon</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttTls</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SttOther</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span>)</code></pre>

<a id="type-456c6653796d566973"></a>

## `ElfSymVis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSymVis</span></code></pre>

Symbol visibility extracted from the low two bits of st_other.

[Source](/reference-sources/Data.Elf.cinch) · line 564

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StvDefault</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StvInternal</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StvHidden</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StvProtected</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span></code></pre>

<a id="type-456c6653796d626f6c"></a>

## `ElfSymbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfSymbol</span></code></pre>

Construct an ElfSymbol from name, info, other, section index, value, size.

[Source](/reference-sources/Data.Elf.cinch) · line 940

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfSymbol</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span>))))))</code></pre>

<a id="type-456c6654797065"></a>

## `ElfType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ElfType</span></code></pre>

ELF object file type (e_type).

[Source](/reference-sources/Data.Elf.cinch) · line 207

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeNone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeRel</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeExec</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeDyn</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeCore</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ElfTypeOther</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span>)</code></pre>

# Values

<a id="value-656c662d6162692d76657273696f6e"></a>

## `elf-abi-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-abi-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

ELF ABI version byte.

[Source](/reference-sources/Data.Elf.cinch) · line 807

<a id="value-656c662d627974652d6f72646572"></a>

## `elf-byte-order`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-byte-order</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span>)</code></pre>

Byte order used to decode multi-byte integers in this ELF file.

[Source](/reference-sources/Data.Elf.cinch) · line 811

<a id="value-656c662d636c617373"></a>

## `elf-class`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-class</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span>)</code></pre>

ELF class (32-bit or 64-bit).

[Source](/reference-sources/Data.Elf.cinch) · line 791

<a id="value-656c662d636c6173732d636f6465"></a>

## `elf-class-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-class-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfClass</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric EI_CLASS value for an ElfClass.

[Source](/reference-sources/Data.Elf.cinch) · line 184

<a id="value-656c662d636f6d70726573732d7a6c6962"></a>

## `elf-compress-zlib`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-compress-zlib</span> :: <span class="tok-constructor">U32</span></code></pre>

Zlib compression identifier (ELFCOMPRESS_ZLIB).

[Source](/reference-sources/Data.Elf.cinch) · line 171

<a id="value-656c662d64617461"></a>

## `elf-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span>)</code></pre>

ELF data encoding (LSB or MSB).

[Source](/reference-sources/Data.Elf.cinch) · line 795

<a id="value-656c662d646174612d636f6465"></a>

## `elf-data-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-data-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfData</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric EI_DATA value for an ElfData.

[Source](/reference-sources/Data.Elf.cinch) · line 200

<a id="value-656c662d64796e2d7261772d746167"></a>

## `elf-dyn-raw-tag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-raw-tag</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynEntry</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Raw numeric d_tag value.

[Source](/reference-sources/Data.Elf.cinch) · line 1051

<a id="value-656c662d64796e2d737472696e67"></a>

## `elf-dyn-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Return the strings listed for a string-valued dynamic tag (DtNeeded, DtSoname, DtRpath, DtRunpath).

[Source](/reference-sources/Data.Elf.cinch) · line 1912

<a id="value-656c662d64796e2d746167"></a>

## `elf-dyn-tag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-tag</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynEntry</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span>)</code></pre>

Decoded dynamic tag.

[Source](/reference-sources/Data.Elf.cinch) · line 1047

<a id="value-656c662d64796e2d7461672d636f6465"></a>

## `elf-dyn-tag-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-tag-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Numeric d_tag code for an ElfDynTag.

[Source](/reference-sources/Data.Elf.cinch) · line 728

<a id="value-656c662d64796e2d76616c"></a>

## `elf-dyn-val`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-val</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynEntry</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Entry value or virtual address (d_val / d_ptr).

[Source](/reference-sources/Data.Elf.cinch) · line 1055

<a id="value-656c662d64796e2d76616c7565"></a>

## `elf-dyn-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dyn-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynTag</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U64</span>))))</code></pre>

Return all values listed for the given tag in the SHT_DYNAMIC section.

[Source](/reference-sources/Data.Elf.cinch) · line 1887

<a id="value-656c662d64796e616d69632d656e7472696573"></a>

## `elf-dynamic-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dynamic-entries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfDynEntry</span>)))</code></pre>

Return all dynamic entries from the SHT_DYNAMIC section, or empty list if none exists.

[Source](/reference-sources/Data.Elf.cinch) · line 1865

<a id="value-656c662d64796e616d69632d73796d626f6c73"></a>

## `elf-dynamic-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-dynamic-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span>)))</code></pre>

Return all symbols from the SHT_DYNSYM section, omitting the dummy symbol at index 0.

[Source](/reference-sources/Data.Elf.cinch) · line 1759

<a id="value-656c662d656e747279"></a>

## `elf-entry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-entry</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual address of the program entry point.

[Source](/reference-sources/Data.Elf.cinch) · line 823

<a id="value-656c662d6572726f722d6d657373616765"></a>

## `elf-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic for an ELF error.

[Source](/reference-sources/Data.Elf.cinch) · line 82

<a id="value-656c662d66696c652d686561646572"></a>

## `elf-file-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-file-header</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span>)</code></pre>

File header of a parsed ELF binary.

[Source](/reference-sources/Data.Elf.cinch) · line 1064

<a id="value-656c662d666c616773"></a>

## `elf-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-flags</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Processor-specific header flags.

[Source](/reference-sources/Data.Elf.cinch) · line 827

<a id="value-656c662d696d706f727465642d6c6962726172696573"></a>

## `elf-imported-libraries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-imported-libraries</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return the shared library names listed by DT_NEEDED entries.

[Source](/reference-sources/Data.Elf.cinch) · line 1926

<a id="value-656c662d696d706f727465642d6c696272617279"></a>

## `elf-imported-library`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-imported-library</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfImportedSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Associated defining library name, or empty when unversioned.

[Source](/reference-sources/Data.Elf.cinch) · line 992

<a id="value-656c662d696d706f727465642d6e616d65"></a>

## `elf-imported-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-imported-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfImportedSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Imported symbol name.

[Source](/reference-sources/Data.Elf.cinch) · line 984

<a id="value-656c662d696d706f727465642d73796d626f6c73"></a>

## `elf-imported-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-imported-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfImportedSymbol</span>)))</code></pre>

Return all global undefined symbols from SHT_DYNSYM expected to be resolved at dynamic load time.

[Source](/reference-sources/Data.Elf.cinch) · line 1939

<a id="value-656c662d696d706f727465642d76657273696f6e"></a>

## `elf-imported-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-imported-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfImportedSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Associated symbol version string, or empty when unversioned.

[Source](/reference-sources/Data.Elf.cinch) · line 988

<a id="value-656c662d6d616368696e65"></a>

## `elf-machine`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-machine</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span>)</code></pre>

Machine architecture.

[Source](/reference-sources/Data.Elf.cinch) · line 819

<a id="value-656c662d6d616368696e652d636f6465"></a>

## `elf-machine-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-machine-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfMachine</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Numeric e_machine code for an ElfMachine.

[Source](/reference-sources/Data.Elf.cinch) · line 288

<a id="value-656c662d6f73616269"></a>

## `elf-osabi`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-osabi</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span>)</code></pre>

ELF OS/ABI identification.

[Source](/reference-sources/Data.Elf.cinch) · line 803

<a id="value-656c662d6f736162692d636f6465"></a>

## `elf-osabi-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-osabi-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfOsAbi</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric EI_OSABI code for an ElfOsAbi.

[Source](/reference-sources/Data.Elf.cinch) · line 339

<a id="value-656c662d7061727365"></a>

## `elf-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-parse</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span>))</code></pre>

Parse an ELF32 or ELF64 binary from raw bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 1560

<a id="value-656c662d70726f672d616c69676e"></a>

## `elf-prog-align`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-align</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Segment alignment in bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 864

<a id="value-656c662d70726f672d64617461"></a>

## `elf-prog-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the file-backed bytes of a program header segment.

[Source](/reference-sources/Data.Elf.cinch) · line 1662

<a id="value-656c662d70726f672d66696c65737a"></a>

## `elf-prog-filesz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-filesz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Segment size in the file in bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 856

<a id="value-656c662d70726f672d666c616773"></a>

## `elf-prog-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-flags</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Segment permission flags bitmask (pf-r, pf-w, pf-x).

[Source](/reference-sources/Data.Elf.cinch) · line 840

<a id="value-656c662d70726f672d6d656d737a"></a>

## `elf-prog-memsz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-memsz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Segment size in memory in bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 860

<a id="value-656c662d70726f672d6f6666736574"></a>

## `elf-prog-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

File byte offset of the segment data.

[Source](/reference-sources/Data.Elf.cinch) · line 844

<a id="value-656c662d70726f672d7061646472"></a>

## `elf-prog-paddr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-paddr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Physical memory address of the segment.

[Source](/reference-sources/Data.Elf.cinch) · line 852

<a id="value-656c662d70726f672d74797065"></a>

## `elf-prog-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span>)</code></pre>

Segment type.

[Source](/reference-sources/Data.Elf.cinch) · line 836

<a id="value-656c662d70726f672d747970652d636f6465"></a>

## `elf-prog-type-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-type-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgType</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Numeric p_type code for an ElfProgType.

[Source](/reference-sources/Data.Elf.cinch) · line 398

<a id="value-656c662d70726f672d7661646472"></a>

## `elf-prog-vaddr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-prog-vaddr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual memory address of the segment.

[Source](/reference-sources/Data.Elf.cinch) · line 848

<a id="value-656c662d70726f6773"></a>

## `elf-progs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-progs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfProgHeader</span>))</code></pre>

Program headers (segments) of a parsed ELF binary.

[Source](/reference-sources/Data.Elf.cinch) · line 1068

<a id="value-656c662d722d73796d3332"></a>

## `elf-r-sym32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-r-sym32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extract the symbol index from a 32-bit relocation r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 766

<a id="value-656c662d722d73796d3634"></a>

## `elf-r-sym64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-r-sym64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extract the symbol index from a 64-bit relocation r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 774

<a id="value-656c662d722d747970653332"></a>

## `elf-r-type32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-r-type32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extract the relocation type from a 32-bit relocation r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 770

<a id="value-656c662d722d747970653634"></a>

## `elf-r-type64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-r-type64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extract the relocation type from a 64-bit relocation r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 778

<a id="value-656c662d72656c2d696e666f"></a>

## `elf-rel-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rel-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Raw r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 1005

<a id="value-656c662d72656c2d6f6666736574"></a>

## `elf-rel-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rel-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Offset to apply the relocation.

[Source](/reference-sources/Data.Elf.cinch) · line 1001

<a id="value-656c662d72656c2d73796d"></a>

## `elf-rel-sym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rel-sym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Symbol table index referenced by the relocation.

[Source](/reference-sources/Data.Elf.cinch) · line 1009

<a id="value-656c662d72656c2d74797065"></a>

## `elf-rel-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rel-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Architecture-specific relocation type code.

[Source](/reference-sources/Data.Elf.cinch) · line 1013

<a id="value-656c662d72656c612d616464656e64"></a>

## `elf-rela-addend`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rela-addend</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Signed constant addend.

[Source](/reference-sources/Data.Elf.cinch) · line 1030

<a id="value-656c662d72656c612d696e666f"></a>

## `elf-rela-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rela-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Raw r_info word.

[Source](/reference-sources/Data.Elf.cinch) · line 1026

<a id="value-656c662d72656c612d6f6666736574"></a>

## `elf-rela-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rela-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Offset to apply the relocation.

[Source](/reference-sources/Data.Elf.cinch) · line 1022

<a id="value-656c662d72656c612d73796d"></a>

## `elf-rela-sym`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rela-sym</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Symbol table index referenced by the relocation.

[Source](/reference-sources/Data.Elf.cinch) · line 1034

<a id="value-656c662d72656c612d74797065"></a>

## `elf-rela-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-rela-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Architecture-specific relocation type code.

[Source](/reference-sources/Data.Elf.cinch) · line 1038

<a id="value-656c662d72656c6f636174696f6e732d72656c"></a>

## `elf-relocations-rel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-relocations-rel</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRel</span>))))</code></pre>

Parse all SHT_REL entries from a relocation section.

[Source](/reference-sources/Data.Elf.cinch) · line 1788

<a id="value-656c662d72656c6f636174696f6e732d72656c61"></a>

## `elf-relocations-rela`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-relocations-rela</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfRela</span>))))</code></pre>

Parse all SHT_RELA entries from a relocation section.

[Source](/reference-sources/Data.Elf.cinch) · line 1827

<a id="value-656c662d73656374696f6e"></a>

## `elf-section`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span>)))</code></pre>

Find the first section with the given name.

[Source](/reference-sources/Data.Elf.cinch) · line 1607

<a id="value-656c662d73656374696f6e2d61646472"></a>

## `elf-section-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-addr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Virtual address of the section in memory.

[Source](/reference-sources/Data.Elf.cinch) · line 892

<a id="value-656c662d73656374696f6e2d61646472616c69676e"></a>

## `elf-section-addralign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-addralign</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Required alignment in bytes (uncompressed alignment when compressed).

[Source](/reference-sources/Data.Elf.cinch) · line 912

<a id="value-656c662d73656374696f6e2d62792d74797065"></a>

## `elf-section-by-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-by-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span>)))</code></pre>

Find the first section matching the given section type.

[Source](/reference-sources/Data.Elf.cinch) · line 1619

<a id="value-656c662d73656374696f6e2d636f6d70726573736564"></a>

## `elf-section-compressed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-compressed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the section carries SHF_COMPRESSED.

[Source](/reference-sources/Data.Elf.cinch) · line 924

<a id="value-656c662d73656374696f6e2d636f6d7072657373696f6e2d74797065"></a>

## `elf-section-compression-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-compression-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compression type code (0 when uncompressed, 1 for zlib).

[Source](/reference-sources/Data.Elf.cinch) · line 931

<a id="value-656c662d73656374696f6e2d64617461"></a>

## `elf-section-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-data</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the contents of an ELF section, automatically decompressing zlib SHF_COMPRESSED sections.

[Source](/reference-sources/Data.Elf.cinch) · line 1631

<a id="value-656c662d73656374696f6e2d656e7473697a65"></a>

## `elf-section-entsize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-entsize</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Entry size in bytes for fixed-size tables.

[Source](/reference-sources/Data.Elf.cinch) · line 916

<a id="value-656c662d73656374696f6e2d66696c652d73697a65"></a>

## `elf-section-file-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-file-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Number of bytes occupied by the section in the file.

[Source](/reference-sources/Data.Elf.cinch) · line 920

<a id="value-656c662d73656374696f6e2d666c616773"></a>

## `elf-section-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-flags</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Section flags bitmask.

[Source](/reference-sources/Data.Elf.cinch) · line 888

<a id="value-656c662d73656374696f6e2d696e666f"></a>

## `elf-section-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extra section-specific information (sh_info).

[Source](/reference-sources/Data.Elf.cinch) · line 908

<a id="value-656c662d73656374696f6e2d6c696e6b"></a>

## `elf-section-link`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-link</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Linked section header index (sh_link).

[Source](/reference-sources/Data.Elf.cinch) · line 904

<a id="value-656c662d73656374696f6e2d6e616d65"></a>

## `elf-section-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Resolved section name from the section header string table.

[Source](/reference-sources/Data.Elf.cinch) · line 880

<a id="value-656c662d73656374696f6e2d6f6666736574"></a>

## `elf-section-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

File byte offset of the section.

[Source](/reference-sources/Data.Elf.cinch) · line 896

<a id="value-656c662d73656374696f6e2d73697a65"></a>

## `elf-section-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Logical section size in bytes (uncompressed size when SHF_COMPRESSED is set).

[Source](/reference-sources/Data.Elf.cinch) · line 900

<a id="value-656c662d73656374696f6e2d74797065"></a>

## `elf-section-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span>)</code></pre>

Section type.

[Source](/reference-sources/Data.Elf.cinch) · line 884

<a id="value-656c662d73656374696f6e2d747970652d636f6465"></a>

## `elf-section-type-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-section-type-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionType</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Numeric sh_type code for an ElfSectionType.

[Source](/reference-sources/Data.Elf.cinch) · line 488

<a id="value-656c662d73656374696f6e73"></a>

## `elf-sections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sections</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSectionHeader</span>))</code></pre>

Section headers of a parsed ELF binary.

[Source](/reference-sources/Data.Elf.cinch) · line 1072

<a id="value-656c662d73742d62696e64"></a>

## `elf-st-bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-st-bind</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span>)</code></pre>

Extract the symbol binding from st_info (ST_BIND).

[Source](/reference-sources/Data.Elf.cinch) · line 583

<a id="value-656c662d73742d696e666f"></a>

## `elf-st-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-st-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

Combine a symbol binding and symbol type into st_info (ST_INFO).

[Source](/reference-sources/Data.Elf.cinch) · line 614

<a id="value-656c662d73742d74797065"></a>

## `elf-st-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-st-type</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span>)</code></pre>

Extract the symbol type from st_info (ST_TYPE).

[Source](/reference-sources/Data.Elf.cinch) · line 592

<a id="value-656c662d73742d7669736962696c697479"></a>

## `elf-st-visibility`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-st-visibility</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span>)</code></pre>

Extract the symbol visibility from st_other (ST_VISIBILITY).

[Source](/reference-sources/Data.Elf.cinch) · line 605

<a id="value-656c662d73796d2d62696e64"></a>

## `elf-sym-bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-bind</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span>)</code></pre>

Decoded symbol binding.

[Source](/reference-sources/Data.Elf.cinch) · line 967

<a id="value-656c662d73796d2d62696e642d636f6465"></a>

## `elf-sym-bind-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-bind-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymBind</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric binding value for an ElfSymBind.

[Source](/reference-sources/Data.Elf.cinch) · line 525

<a id="value-656c662d73796d2d696e666f"></a>

## `elf-sym-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Raw st_info byte encoding binding and type.

[Source](/reference-sources/Data.Elf.cinch) · line 947

<a id="value-656c662d73796d2d6e616d65"></a>

## `elf-sym-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Symbol name from the linked string table.

[Source](/reference-sources/Data.Elf.cinch) · line 943

<a id="value-656c662d73796d2d6f74686572"></a>

## `elf-sym-other`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-other</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Raw st_other byte encoding symbol visibility.

[Source](/reference-sources/Data.Elf.cinch) · line 951

<a id="value-656c662d73796d2d73656374696f6e"></a>

## `elf-sym-section`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-section</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Section header index (or special index such as shn-undef or shn-abs).

[Source](/reference-sources/Data.Elf.cinch) · line 955

<a id="value-656c662d73796d2d73697a65"></a>

## `elf-sym-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Symbol size in bytes.

[Source](/reference-sources/Data.Elf.cinch) · line 963

<a id="value-656c662d73796d2d74797065"></a>

## `elf-sym-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span>)</code></pre>

Decoded symbol type.

[Source](/reference-sources/Data.Elf.cinch) · line 971

<a id="value-656c662d73796d2d747970652d636f6465"></a>

## `elf-sym-type-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-type-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymType</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric type value for an ElfSymType.

[Source](/reference-sources/Data.Elf.cinch) · line 552

<a id="value-656c662d73796d2d76616c7565"></a>

## `elf-sym-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Symbol value (address or offset).

[Source](/reference-sources/Data.Elf.cinch) · line 959

<a id="value-656c662d73796d2d7669732d636f6465"></a>

## `elf-sym-vis-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-vis-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Numeric visibility code for an ElfSymVis.

[Source](/reference-sources/Data.Elf.cinch) · line 575

<a id="value-656c662d73796d2d7669736962696c697479"></a>

## `elf-sym-visibility`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-sym-visibility</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymVis</span>)</code></pre>

Decoded symbol visibility.

[Source](/reference-sources/Data.Elf.cinch) · line 975

<a id="value-656c662d73796d626f6c73"></a>

## `elf-symbols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-symbols</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfFile</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfSymbol</span>)))</code></pre>

Return all symbols from the SHT_SYMTAB section, omitting the dummy symbol at index 0.

[Source](/reference-sources/Data.Elf.cinch) · line 1755

<a id="value-656c662d74797065"></a>

## `elf-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span>)</code></pre>

Object file type (relocatable, executable, shared object, core).

[Source](/reference-sources/Data.Elf.cinch) · line 815

<a id="value-656c662d747970652d636f6465"></a>

## `elf-type-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-type-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfType</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Numeric e_type code for an ElfType.

[Source](/reference-sources/Data.Elf.cinch) · line 232

<a id="value-656c662d76657273696f6e"></a>

## `elf-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elf-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Elf</span>.<span class="tok-constructor">ElfHeader</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

ELF identification version byte.

[Source](/reference-sources/Data.Elf.cinch) · line 799

<a id="value-70662d72"></a>

## `pf-r`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pf-r</span> :: <span class="tok-constructor">U32</span></code></pre>

Readable permission flag on a program header (PF_R).

[Source](/reference-sources/Data.Elf.cinch) · line 111

<a id="value-70662d77"></a>

## `pf-w`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pf-w</span> :: <span class="tok-constructor">U32</span></code></pre>

Writable permission flag on a program header (PF_W).

[Source](/reference-sources/Data.Elf.cinch) · line 107

<a id="value-70662d78"></a>

## `pf-x`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pf-x</span> :: <span class="tok-constructor">U32</span></code></pre>

Executable permission flag on a program header (PF_X).

[Source](/reference-sources/Data.Elf.cinch) · line 103

<a id="value-7368662d616c6c6f63"></a>

## `shf-alloc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-alloc</span> :: <span class="tok-constructor">U64</span></code></pre>

Section occupies memory during execution (SHF_ALLOC).

[Source](/reference-sources/Data.Elf.cinch) · line 119

<a id="value-7368662d636f6d70726573736564"></a>

## `shf-compressed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-compressed</span> :: <span class="tok-constructor">U64</span></code></pre>

Section data is compressed with a Chdr prefix (SHF_COMPRESSED).

[Source](/reference-sources/Data.Elf.cinch) · line 151

<a id="value-7368662d65786563696e737472"></a>

## `shf-execinstr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-execinstr</span> :: <span class="tok-constructor">U64</span></code></pre>

Section contains executable instructions (SHF_EXECINSTR).

[Source](/reference-sources/Data.Elf.cinch) · line 123

<a id="value-7368662d67726f7570"></a>

## `shf-group`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-group</span> :: <span class="tok-constructor">U64</span></code></pre>

Section is a member of a section group (SHF_GROUP).

[Source](/reference-sources/Data.Elf.cinch) · line 143

<a id="value-7368662d696e666f2d6c696e6b"></a>

## `shf-info-link`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-info-link</span> :: <span class="tok-constructor">U64</span></code></pre>

Section sh_info holds a section header index (SHF_INFO_LINK).

[Source](/reference-sources/Data.Elf.cinch) · line 135

<a id="value-7368662d6c696e6b2d6f72646572"></a>

## `shf-link-order`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-link-order</span> :: <span class="tok-constructor">U64</span></code></pre>

Section preserves special ordering requirements (SHF_LINK_ORDER).

[Source](/reference-sources/Data.Elf.cinch) · line 139

<a id="value-7368662d6d65726765"></a>

## `shf-merge`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-merge</span> :: <span class="tok-constructor">U64</span></code></pre>

Section elements may be merged (SHF_MERGE).

[Source](/reference-sources/Data.Elf.cinch) · line 127

<a id="value-7368662d737472696e6773"></a>

## `shf-strings`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-strings</span> :: <span class="tok-constructor">U64</span></code></pre>

Section contains NUL-terminated strings (SHF_STRINGS).

[Source](/reference-sources/Data.Elf.cinch) · line 131

<a id="value-7368662d746c73"></a>

## `shf-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-tls</span> :: <span class="tok-constructor">U64</span></code></pre>

Section holds thread-local storage (SHF_TLS).

[Source](/reference-sources/Data.Elf.cinch) · line 147

<a id="value-7368662d7772697465"></a>

## `shf-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shf-write</span> :: <span class="tok-constructor">U64</span></code></pre>

Section contains writable data (SHF_WRITE).

[Source](/reference-sources/Data.Elf.cinch) · line 115

<a id="value-73686e2d616273"></a>

## `shn-abs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shn-abs</span> :: <span class="tok-constructor">U32</span></code></pre>

Absolute symbol section index (SHN_ABS).

[Source](/reference-sources/Data.Elf.cinch) · line 159

<a id="value-73686e2d636f6d6d6f6e"></a>

## `shn-common`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shn-common</span> :: <span class="tok-constructor">U32</span></code></pre>

Common symbol section index (SHN_COMMON).

[Source](/reference-sources/Data.Elf.cinch) · line 163

<a id="value-73686e2d756e646566"></a>

## `shn-undef`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shn-undef</span> :: <span class="tok-constructor">U32</span></code></pre>

Undefined section index (SHN_UNDEF).

[Source](/reference-sources/Data.Elf.cinch) · line 155

<a id="value-73686e2d78696e646578"></a>

## `shn-xindex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shn-xindex</span> :: <span class="tok-constructor">U32</span></code></pre>

Extended section index escape value (SHN_XINDEX).

[Source](/reference-sources/Data.Elf.cinch) · line 167

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Elf.cinch`. Source SHA-256: `9746b35d9a2953a863326bda57bc953cef954eeaf4b01525d83ab2fe3bd2d5d8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
