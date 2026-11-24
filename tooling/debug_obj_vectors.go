// Generate deterministic reference vectors for Data.Elf, Data.MachO, and Data.Pe
// using Go's official debug/elf, debug/macho, and debug/pe packages.
//
// Usage:
//   GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/debug_obj_vectors.go > seed/tests/fixtures/debug-obj-vectors.json
package main

import (
	"bytes"
	"compress/zlib"
	"debug/elf"
	"debug/macho"
	"debug/pe"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
)

type ElfProgOut struct {
	Type   uint32 `json:"type"`
	Flags  uint32 `json:"flags"`
	Off    uint64 `json:"off"`
	Vaddr  uint64 `json:"vaddr"`
	Paddr  uint64 `json:"paddr"`
	Filesz uint64 `json:"filesz"`
	Memsz  uint64 `json:"memsz"`
	Align  uint64 `json:"align"`
}

type ElfSecOut struct {
	Name       string `json:"name"`
	Type       uint32 `json:"type"`
	Flags      uint64 `json:"flags"`
	Addr       uint64 `json:"addr"`
	Offset     uint64 `json:"offset"`
	Size       uint64 `json:"size"`
	FileSize   uint64 `json:"file_size"`
	Link       uint32 `json:"link"`
	Info       uint32 `json:"info"`
	Addralign  uint64 `json:"addralign"`
	Entsize    uint64 `json:"entsize"`
	DataHex    string `json:"data_hex"`
	DataErr    bool   `json:"data_err"`
}

type ElfSymOut struct {
	Name    string `json:"name"`
	Info    uint8  `json:"info"`
	Other   uint8  `json:"other"`
	Section uint32 `json:"section"`
	Value   uint64 `json:"value"`
	Size    uint64 `json:"size"`
}

type ElfRelOut struct {
	Offset uint64 `json:"offset"`
	Sym    uint32 `json:"sym"`
	Type   uint32 `json:"type"`
}

type ElfRelaOut struct {
	Offset uint64 `json:"offset"`
	Sym    uint32 `json:"sym"`
	Type   uint32 `json:"type"`
	Addend int64  `json:"addend"`
}

type ElfCase struct {
	Name              string       `json:"name"`
	Hex               string       `json:"hex"`
	Class             uint8        `json:"class"`
	Data              uint8        `json:"data"`
	Version           uint8        `json:"version"`
	OSABI             uint8        `json:"osabi"`
	ABIVersion        uint8        `json:"abi_version"`
	Type              uint16       `json:"type"`
	Machine           uint16       `json:"machine"`
	Entry             uint64       `json:"entry"`
	Progs             []ElfProgOut `json:"progs"`
	Sections          []ElfSecOut  `json:"sections"`
	Symbols           []ElfSymOut  `json:"symbols"`
	HasSymbols        bool         `json:"has_symbols"`
	DynSymbols        []ElfSymOut  `json:"dyn_symbols"`
	HasDynSymbols     bool         `json:"has_dyn_symbols"`
	ImportedLibraries []string     `json:"imported_libraries"`
	ImportedSymbols   []string     `json:"imported_symbols"`
	Soname            []string     `json:"soname"`
	Runpath           []string     `json:"runpath"`
	Rpath             []string     `json:"rpath"`
	RelSection        string       `json:"rel_section"`
	Rels              []ElfRelOut  `json:"rels"`
	RelaSection       string       `json:"rela_section"`
	Relas             []ElfRelaOut `json:"relas"`
}

type MachOSegOut struct {
	Cmd     uint32 `json:"cmd"`
	Name    string `json:"name"`
	Addr    uint64 `json:"addr"`
	Memsz   uint64 `json:"memsz"`
	Offset  uint64 `json:"offset"`
	Filesz  uint64 `json:"filesz"`
	Maxprot uint32 `json:"maxprot"`
	Prot    uint32 `json:"prot"`
	Nsect   uint32 `json:"nsect"`
	Flag    uint32 `json:"flag"`
}

type MachOSecOut struct {
	Name    string `json:"name"`
	Seg     string `json:"seg"`
	Addr    uint64 `json:"addr"`
	Size    uint64 `json:"size"`
	Offset  uint32 `json:"offset"`
	Align   uint32 `json:"align"`
	Flags   uint32 `json:"flags"`
	DataHex string `json:"data_hex"`
}

type MachOSymOut struct {
	Name  string `json:"name"`
	Type  uint8  `json:"type"`
	Sect  uint8  `json:"sect"`
	Desc  uint16 `json:"desc"`
	Value uint64 `json:"value"`
}

type MachOCase struct {
	Name              string        `json:"name"`
	Hex               string        `json:"hex"`
	Magic             uint32        `json:"magic"`
	BigEndian         bool          `json:"big_endian"`
	Cpu               uint32        `json:"cpu"`
	SubCpu            uint32        `json:"subcpu"`
	Type              uint32        `json:"type"`
	Ncmd              uint32        `json:"ncmd"`
	Cmdsz             uint32        `json:"cmdsz"`
	Flags             uint32        `json:"flags"`
	Segments          []MachOSegOut `json:"segments"`
	Sections          []MachOSecOut `json:"sections"`
	Symbols           []MachOSymOut `json:"symbols"`
	ImportedLibraries []string      `json:"imported_libraries"`
	ImportedSymbols   []string      `json:"imported_symbols"`
	HasImportedSyms   bool          `json:"has_imported_syms"`
	Rpaths            []string      `json:"rpaths"`
	UuidHex           string        `json:"uuid_hex"`
	Dylinker          string        `json:"dylinker"`
}

type MachOFatArchOut struct {
	Cpu    uint32 `json:"cpu"`
	SubCpu uint32 `json:"subcpu"`
	Offset uint32 `json:"offset"`
	Size   uint32 `json:"size"`
	Align  uint32 `json:"align"`
	Magic  uint32 `json:"magic"`
	Nsect  int    `json:"nsect"`
}

type MachOFatCase struct {
	Name   string            `json:"name"`
	Hex    string            `json:"hex"`
	Magic  uint32            `json:"magic"`
	Arches []MachOFatArchOut `json:"arches"`
}

type PeSecOut struct {
	Name            string `json:"name"`
	VirtualSize     uint32 `json:"virtual_size"`
	VirtualAddress  uint32 `json:"virtual_address"`
	Size            uint32 `json:"size"`
	Offset          uint32 `json:"offset"`
	Characteristics uint32 `json:"characteristics"`
	DataHex         string `json:"data_hex"`
}

type PeSymOut struct {
	Name          string `json:"name"`
	Value         uint32 `json:"value"`
	SectionNumber int16  `json:"section_number"`
	Type          uint16 `json:"type"`
	StorageClass  uint8  `json:"storage_class"`
}

type PeExportOut struct {
	Name      string `json:"name"`
	Ordinal   uint32 `json:"ordinal"`
	Rva       uint32 `json:"rva"`
	Forwarder string `json:"forwarder"`
}

type PeCase struct {
	Name              string        `json:"name"`
	Hex               string        `json:"hex"`
	Machine           uint16        `json:"machine"`
	NumberOfSections  uint16        `json:"number_of_sections"`
	TimeDateStamp     uint32        `json:"timestamp"`
	Characteristics   uint16        `json:"characteristics"`
	OptKind           string        `json:"opt_kind"` // "none", "pe32", "pe64"
	EntryPoint        uint32        `json:"entry_point"`
	ImageBase         uint64        `json:"image_base"`
	Subsystem         uint16        `json:"subsystem"`
	Sections          []PeSecOut    `json:"sections"`
	Symbols           []PeSymOut    `json:"symbols"`
	ImportedLibraries []string      `json:"imported_libraries"`
	ImportedSymbols   []string      `json:"imported_symbols"`
	ImportCountAll    int           `json:"import_count_all"`
	ExportDll         string        `json:"export_dll"`
	ExportBase        uint32        `json:"export_base"`
	Exports           []PeExportOut `json:"exports"`
}

type BadCase struct {
	Name string `json:"name"`
	Hex  string `json:"hex"`
}

type Output struct {
	Elf        []ElfCase      `json:"elf"`
	ElfBad     []BadCase      `json:"elf_bad"`
	MachO      []MachOCase    `json:"macho"`
	MachOFat   []MachOFatCase `json:"macho_fat"`
	MachOBad   []BadCase      `json:"macho_bad"`
	MachOFatBad []BadCase     `json:"macho_fat_bad"`
	Pe         []PeCase       `json:"pe"`
	PeBad      []BadCase      `json:"pe_bad"`
}

func zlibCompress(raw []byte) []byte {
	var b bytes.Buffer
	w := zlib.NewWriter(&b)
	_, _ = w.Write(raw)
	_ = w.Close()
	return b.Bytes()
}

type strtabBuilder struct {
	buf []byte
}

func newStrtab() *strtabBuilder {
	return &strtabBuilder{buf: []byte{0}}
}

func (s *strtabBuilder) add(str string) uint32 {
	if str == "" {
		return 0
	}
	off := uint32(len(s.buf))
	s.buf = append(s.buf, []byte(str)...)
	s.buf = append(s.buf, 0)
	return off
}

func alignUp(n, align int) int {
	if align <= 1 {
		return n
	}
	return (n + align - 1) &^ (align - 1)
}

func padTo(b *bytes.Buffer, off int) {
	for b.Len() < off {
		b.WriteByte(0)
	}
}

// =========================================================================
// ELF builders
// =========================================================================

func buildElf64LEExec() []byte {
	bo := binary.LittleEndian
	shstr := newStrtab()
	strtab := newStrtab()
	dynstr := newStrtab()

	// Section payloads
	textData := []byte{0x55, 0x48, 0x89, 0xe5, 0xb8, 0x2a, 0x00, 0x00, 0x00, 0x5d, 0xc3}

	uncompRodata := []byte("hello compressed elf64 section payload! hello compressed elf64 section payload!")
	zRodata := zlibCompress(uncompRodata)
	var compSec bytes.Buffer
	_ = binary.Write(&compSec, bo, uint32(elf.COMPRESS_ZLIB))
	_ = binary.Write(&compSec, bo, uint32(0)) // reserved
	_ = binary.Write(&compSec, bo, uint64(len(uncompRodata)))
	_ = binary.Write(&compSec, bo, uint64(8))
	compSec.Write(zRodata)

	// .symtab (index 0 null + 3 symbols)
	var symtab bytes.Buffer
	writeSym64 := func(name uint32, info, other uint8, shndx uint16, val, sz uint64) {
		_ = binary.Write(&symtab, bo, name)
		_ = binary.Write(&symtab, bo, info)
		_ = binary.Write(&symtab, bo, other)
		_ = binary.Write(&symtab, bo, shndx)
		_ = binary.Write(&symtab, bo, val)
		_ = binary.Write(&symtab, bo, sz)
	}
	writeSym64(0, 0, 0, 0, 0, 0)
	writeSym64(strtab.add("helper"), elf.ST_INFO(elf.STB_LOCAL, elf.STT_FUNC), uint8(elf.STV_HIDDEN), 1, 0x401000, 4)
	writeSym64(strtab.add("main"), elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), uint8(elf.STV_DEFAULT), 1, 0x401004, 7)
	writeSym64(strtab.add("weak_sym"), elf.ST_INFO(elf.STB_WEAK, elf.STT_OBJECT), uint8(elf.STV_PROTECTED), 2, 0x402000, 8)

	// .dynsym (index 0 null + 4 symbols: 2 global undef, 1 weak undef, 1 global def)
	var dynsym bytes.Buffer
	writeDynSym64 := func(name uint32, info, other uint8, shndx uint16, val, sz uint64) {
		_ = binary.Write(&dynsym, bo, name)
		_ = binary.Write(&dynsym, bo, info)
		_ = binary.Write(&dynsym, bo, other)
		_ = binary.Write(&dynsym, bo, shndx)
		_ = binary.Write(&dynsym, bo, val)
		_ = binary.Write(&dynsym, bo, sz)
	}
	writeDynSym64(0, 0, 0, 0, 0, 0)
	writeDynSym64(dynstr.add("puts"), elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), 0, uint16(elf.SHN_UNDEF), 0, 0)
	writeDynSym64(dynstr.add("exit"), elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), 0, uint16(elf.SHN_UNDEF), 0, 0)
	writeDynSym64(dynstr.add("__gmon_start__"), elf.ST_INFO(elf.STB_WEAK, elf.STT_NOTYPE), 0, uint16(elf.SHN_UNDEF), 0, 0)
	writeDynSym64(dynstr.add("exported_api"), elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), 0, 1, 0x401004, 7)

	// .dynamic
	var dyn bytes.Buffer
	writeDyn64 := func(tag elf.DynTag, val uint64) {
		_ = binary.Write(&dyn, bo, uint64(tag))
		_ = binary.Write(&dyn, bo, val)
	}
	writeDyn64(elf.DT_NEEDED, uint64(dynstr.add("libc.so.6")))
	writeDyn64(elf.DT_NEEDED, uint64(dynstr.add("libm.so.6")))
	writeDyn64(elf.DT_SONAME, uint64(dynstr.add("libdemo.so.1")))
	writeDyn64(elf.DT_RUNPATH, uint64(dynstr.add("/opt/cinch/lib")))
	writeDyn64(elf.DT_FLAGS, uint64(elf.DF_BIND_NOW))
	writeDyn64(elf.DT_NULL, 0)

	// .rela.text
	var rela bytes.Buffer
	_ = binary.Write(&rela, bo, uint64(0x401005))
	_ = binary.Write(&rela, bo, elf.R_INFO(1, uint32(elf.R_X86_64_PC32)))
	_ = binary.Write(&rela, bo, int64(-4))
	_ = binary.Write(&rela, bo, uint64(0x401008))
	_ = binary.Write(&rela, bo, elf.R_INFO(2, uint32(elf.R_X86_64_64)))
	_ = binary.Write(&rela, bo, int64(16))

	// Section names
	nameText := shstr.add(".text")
	nameBss := shstr.add(".bss")
	nameRodataZ := shstr.add(".rodata.z")
	nameSymtab := shstr.add(".symtab")
	nameStrtab := shstr.add(".strtab")
	nameDynsym := shstr.add(".dynsym")
	nameDynstr := shstr.add(".dynstr")
	nameDynamic := shstr.add(".dynamic")
	nameRela := shstr.add(".rela.text")
	nameShstr := shstr.add(".shstrtab")

	// Layout:
	// 0: Elf64_Ehdr (64)
	// 64: 2 x Elf64_Phdr (112) -> end 176
	// 176..: section payloads (aligned to 8)
	// then section header table (11 x 64)
	var body bytes.Buffer
	padTo(&body, 176)

	offText := uint64(body.Len())
	body.Write(textData)
	padTo(&body, alignUp(body.Len(), 8))

	offRodataZ := uint64(body.Len())
	body.Write(compSec.Bytes())
	padTo(&body, alignUp(body.Len(), 8))

	offSymtab := uint64(body.Len())
	body.Write(symtab.Bytes())
	padTo(&body, alignUp(body.Len(), 8))

	offStrtab := uint64(body.Len())
	body.Write(strtab.buf)
	padTo(&body, alignUp(body.Len(), 8))

	offDynsym := uint64(body.Len())
	body.Write(dynsym.Bytes())
	padTo(&body, alignUp(body.Len(), 8))

	offDynstr := uint64(body.Len())
	body.Write(dynstr.buf)
	padTo(&body, alignUp(body.Len(), 8))

	offDynamic := uint64(body.Len())
	body.Write(dyn.Bytes())
	padTo(&body, alignUp(body.Len(), 8))

	offRela := uint64(body.Len())
	body.Write(rela.Bytes())
	padTo(&body, alignUp(body.Len(), 8))

	offShstr := uint64(body.Len())
	body.Write(shstr.buf)
	padTo(&body, alignUp(body.Len(), 8))

	shoff := uint64(body.Len())

	type sh64Spec struct {
		name      uint32
		typ       elf.SectionType
		flags     elf.SectionFlag
		addr      uint64
		off       uint64
		size      uint64
		link      uint32
		info      uint32
		addralign uint64
		entsize   uint64
	}
	secs := []sh64Spec{
		{}, // 0: NULL
		{nameText, elf.SHT_PROGBITS, elf.SHF_ALLOC | elf.SHF_EXECINSTR, 0x401000, offText, uint64(len(textData)), 0, 0, 16, 0},                     // 1: .text
		{nameBss, elf.SHT_NOBITS, elf.SHF_ALLOC | elf.SHF_WRITE, 0x402000, offRodataZ, 4096, 0, 0, 16, 0},                                            // 2: .bss
		{nameRodataZ, elf.SHT_PROGBITS, elf.SHF_COMPRESSED, 0, offRodataZ, uint64(compSec.Len()), 0, 0, 8, 0},                                         // 3: .rodata.z
		{nameSymtab, elf.SHT_SYMTAB, 0, 0, offSymtab, uint64(symtab.Len()), 5, 2, 8, 24},                                                             // 4: .symtab -> link 5 (.strtab)
		{nameStrtab, elf.SHT_STRTAB, 0, 0, offStrtab, uint64(len(strtab.buf)), 0, 0, 1, 0},                                                           // 5: .strtab
		{nameDynsym, elf.SHT_DYNSYM, elf.SHF_ALLOC, 0x404000, offDynsym, uint64(dynsym.Len()), 7, 1, 8, 24},                                          // 6: .dynsym -> link 7 (.dynstr)
		{nameDynstr, elf.SHT_STRTAB, elf.SHF_ALLOC, 0x404100, offDynstr, uint64(len(dynstr.buf)), 0, 0, 1, 0},                                        // 7: .dynstr
		{nameDynamic, elf.SHT_DYNAMIC, elf.SHF_ALLOC | elf.SHF_WRITE, 0x405000, offDynamic, uint64(dyn.Len()), 7, 0, 8, 16},                          // 8: .dynamic -> link 7 (.dynstr)
		{nameRela, elf.SHT_RELA, 0, 0, offRela, uint64(rela.Len()), 4, 1, 8, 24},                                                                     // 9: .rela.text
		{nameShstr, elf.SHT_STRTAB, 0, 0, offShstr, uint64(len(shstr.buf)), 0, 0, 1, 0},                                                              // 10: .shstrtab
	}
	for _, s := range secs {
		_ = binary.Write(&body, bo, elf.Section64{
			Name:      s.name,
			Type:      uint32(s.typ),
			Flags:     uint64(s.flags),
			Addr:      s.addr,
			Off:       s.off,
			Size:      s.size,
			Link:      s.link,
			Info:      s.info,
			Addralign: s.addralign,
			Entsize:   s.entsize,
		})
	}

	raw := body.Bytes()
	// Write ELF64 header at offset 0
	var hdrBuf bytes.Buffer
	var ident [16]byte
	copy(ident[0:4], "\x7fELF")
	ident[elf.EI_CLASS] = uint8(elf.ELFCLASS64)
	ident[elf.EI_DATA] = uint8(elf.ELFDATA2LSB)
	ident[elf.EI_VERSION] = uint8(elf.EV_CURRENT)
	ident[elf.EI_OSABI] = uint8(elf.ELFOSABI_LINUX)
	ident[elf.EI_ABIVERSION] = 1
	_ = binary.Write(&hdrBuf, bo, elf.Header64{
		Ident:     ident,
		Type:      uint16(elf.ET_EXEC),
		Machine:   uint16(elf.EM_X86_64),
		Version:   uint32(elf.EV_CURRENT),
		Entry:     0x401000,
		Phoff:     64,
		Shoff:     shoff,
		Flags:     0,
		Ehsize:    64,
		Phentsize: 56,
		Phnum:     2,
		Shentsize: 64,
		Shnum:     uint16(len(secs)),
		Shstrndx:  10,
	})
	// Write 2 program headers at offset 64
	_ = binary.Write(&hdrBuf, bo, elf.Prog64{
		Type:   uint32(elf.PT_LOAD),
		Flags:  uint32(elf.PF_R | elf.PF_X),
		Off:    offText,
		Vaddr:  0x401000,
		Paddr:  0x401000,
		Filesz: uint64(len(textData)),
		Memsz:  uint64(len(textData)),
		Align:  0x1000,
	})
	_ = binary.Write(&hdrBuf, bo, elf.Prog64{
		Type:   uint32(elf.PT_DYNAMIC),
		Flags:  uint32(elf.PF_R | elf.PF_W),
		Off:    offDynamic,
		Vaddr:  0x405000,
		Paddr:  0x405000,
		Filesz: uint64(dyn.Len()),
		Memsz:  uint64(dyn.Len()),
		Align:  8,
	})
	copy(raw[0:176], hdrBuf.Bytes())
	return raw
}

func buildElf32LERel() []byte {
	bo := binary.LittleEndian
	shstr := newStrtab()
	strtab := newStrtab()

	textData := []byte{0x90, 0x31, 0xc0, 0xc3}
	uncompDbg := []byte("dwarf-debug-info-payload-elf32-compressed")
	zDbg := zlibCompress(uncompDbg)
	var compSec bytes.Buffer
	_ = binary.Write(&compSec, bo, uint32(elf.COMPRESS_ZLIB))
	_ = binary.Write(&compSec, bo, uint32(len(uncompDbg)))
	_ = binary.Write(&compSec, bo, uint32(4))
	compSec.Write(zDbg)

	var symtab bytes.Buffer
	writeSym32 := func(name, val, sz uint32, info, other uint8, shndx uint16) {
		_ = binary.Write(&symtab, bo, name)
		_ = binary.Write(&symtab, bo, val)
		_ = binary.Write(&symtab, bo, sz)
		_ = binary.Write(&symtab, bo, info)
		_ = binary.Write(&symtab, bo, other)
		_ = binary.Write(&symtab, bo, shndx)
	}
	writeSym32(0, 0, 0, 0, 0, 0)
	writeSym32(strtab.add("foo32"), 0, 4, elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), 0, 1)
	writeSym32(strtab.add("ext32"), 0, 0, elf.ST_INFO(elf.STB_GLOBAL, elf.STT_NOTYPE), 0, uint16(elf.SHN_UNDEF))

	var rel bytes.Buffer
	_ = binary.Write(&rel, bo, uint32(0x1))
	_ = binary.Write(&rel, bo, elf.R_INFO32(2, uint32(elf.R_386_PC32)))

	var rela bytes.Buffer
	_ = binary.Write(&rela, bo, uint32(0x2))
	_ = binary.Write(&rela, bo, elf.R_INFO32(1, uint32(elf.R_386_32)))
	_ = binary.Write(&rela, bo, int32(-8))

	nameText := shstr.add(".text")
	nameDbg := shstr.add(".debug_info")
	nameSymtab := shstr.add(".symtab")
	nameStrtab := shstr.add(".strtab")
	nameRel := shstr.add(".rel.text")
	nameRela := shstr.add(".rela.data")
	nameShstr := shstr.add(".shstrtab")

	var body bytes.Buffer
	padTo(&body, 52)
	offText := uint32(body.Len())
	body.Write(textData)
	padTo(&body, alignUp(body.Len(), 4))
	offDbg := uint32(body.Len())
	body.Write(compSec.Bytes())
	padTo(&body, alignUp(body.Len(), 4))
	offSymtab := uint32(body.Len())
	body.Write(symtab.Bytes())
	padTo(&body, alignUp(body.Len(), 4))
	offStrtab := uint32(body.Len())
	body.Write(strtab.buf)
	padTo(&body, alignUp(body.Len(), 4))
	offRel := uint32(body.Len())
	body.Write(rel.Bytes())
	padTo(&body, alignUp(body.Len(), 4))
	offRela := uint32(body.Len())
	body.Write(rela.Bytes())
	padTo(&body, alignUp(body.Len(), 4))
	offShstr := uint32(body.Len())
	body.Write(shstr.buf)
	padTo(&body, alignUp(body.Len(), 4))

	shoff := uint32(body.Len())
	secs := []elf.Section32{
		{},
		{Name: nameText, Type: uint32(elf.SHT_PROGBITS), Flags: uint32(elf.SHF_ALLOC | elf.SHF_EXECINSTR), Off: offText, Size: uint32(len(textData)), Addralign: 4},
		{Name: nameDbg, Type: uint32(elf.SHT_PROGBITS), Flags: uint32(elf.SHF_COMPRESSED), Off: offDbg, Size: uint32(compSec.Len()), Addralign: 4},
		{Name: nameSymtab, Type: uint32(elf.SHT_SYMTAB), Off: offSymtab, Size: uint32(symtab.Len()), Link: 4, Info: 1, Addralign: 4, Entsize: 16},
		{Name: nameStrtab, Type: uint32(elf.SHT_STRTAB), Off: offStrtab, Size: uint32(len(strtab.buf)), Addralign: 1},
		{Name: nameRel, Type: uint32(elf.SHT_REL), Off: offRel, Size: uint32(rel.Len()), Link: 3, Info: 1, Addralign: 4, Entsize: 8},
		{Name: nameRela, Type: uint32(elf.SHT_RELA), Off: offRela, Size: uint32(rela.Len()), Link: 3, Info: 1, Addralign: 4, Entsize: 12},
		{Name: nameShstr, Type: uint32(elf.SHT_STRTAB), Off: offShstr, Size: uint32(len(shstr.buf)), Addralign: 1},
	}
	for _, s := range secs {
		_ = binary.Write(&body, bo, s)
	}
	raw := body.Bytes()
	var hdrBuf bytes.Buffer
	var ident [16]byte
	copy(ident[0:4], "\x7fELF")
	ident[elf.EI_CLASS] = uint8(elf.ELFCLASS32)
	ident[elf.EI_DATA] = uint8(elf.ELFDATA2LSB)
	ident[elf.EI_VERSION] = uint8(elf.EV_CURRENT)
	_ = binary.Write(&hdrBuf, bo, elf.Header32{
		Ident:     ident,
		Type:      uint16(elf.ET_REL),
		Machine:   uint16(elf.EM_386),
		Version:   uint32(elf.EV_CURRENT),
		Shoff:     shoff,
		Ehsize:    52,
		Shentsize: 40,
		Shnum:     uint16(len(secs)),
		Shstrndx:  7,
	})
	copy(raw[0:52], hdrBuf.Bytes())
	return raw
}

func buildElf64BEDyn() []byte {
	bo := binary.BigEndian
	shstr := newStrtab()
	dynstr := newStrtab()

	textData := []byte{0x60, 0x00, 0x00, 0x00, 0x4e, 0x80, 0x00, 0x20}

	var dynsym bytes.Buffer
	writeDynSym64 := func(name uint32, info, other uint8, shndx uint16, val, sz uint64) {
		_ = binary.Write(&dynsym, bo, name)
		_ = binary.Write(&dynsym, bo, info)
		_ = binary.Write(&dynsym, bo, other)
		_ = binary.Write(&dynsym, bo, shndx)
		_ = binary.Write(&dynsym, bo, val)
		_ = binary.Write(&dynsym, bo, sz)
	}
	writeDynSym64(0, 0, 0, 0, 0, 0)
	writeDynSym64(dynstr.add("be_import"), elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC), 0, uint16(elf.SHN_UNDEF), 0, 0)

	var dyn bytes.Buffer
	_ = binary.Write(&dyn, bo, uint64(elf.DT_NEEDED))
	_ = binary.Write(&dyn, bo, uint64(dynstr.add("libppc64.so.2")))
	_ = binary.Write(&dyn, bo, uint64(elf.DT_RPATH))
	_ = binary.Write(&dyn, bo, uint64(dynstr.add("/usr/lib64")))
	_ = binary.Write(&dyn, bo, uint64(elf.DT_NULL))
	_ = binary.Write(&dyn, bo, uint64(0))

	nameText := shstr.add(".text")
	nameDynsym := shstr.add(".dynsym")
	nameDynstr := shstr.add(".dynstr")
	nameDynamic := shstr.add(".dynamic")
	nameShstr := shstr.add(".shstrtab")

	var body bytes.Buffer
	padTo(&body, 64+56)
	offText := uint64(body.Len())
	body.Write(textData)
	padTo(&body, alignUp(body.Len(), 8))
	offDynsym := uint64(body.Len())
	body.Write(dynsym.Bytes())
	padTo(&body, alignUp(body.Len(), 8))
	offDynstr := uint64(body.Len())
	body.Write(dynstr.buf)
	padTo(&body, alignUp(body.Len(), 8))
	offDynamic := uint64(body.Len())
	body.Write(dyn.Bytes())
	padTo(&body, alignUp(body.Len(), 8))
	offShstr := uint64(body.Len())
	body.Write(shstr.buf)
	padTo(&body, alignUp(body.Len(), 8))

	shoff := uint64(body.Len())
	secs := []elf.Section64{
		{},
		{Name: nameText, Type: uint32(elf.SHT_PROGBITS), Flags: uint64(elf.SHF_ALLOC | elf.SHF_EXECINSTR), Addr: 0x1000, Off: offText, Size: uint64(len(textData)), Addralign: 4},
		{Name: nameDynsym, Type: uint32(elf.SHT_DYNSYM), Flags: uint64(elf.SHF_ALLOC), Addr: 0x2000, Off: offDynsym, Size: uint64(dynsym.Len()), Link: 3, Info: 1, Addralign: 8, Entsize: 24},
		{Name: nameDynstr, Type: uint32(elf.SHT_STRTAB), Flags: uint64(elf.SHF_ALLOC), Addr: 0x2100, Off: offDynstr, Size: uint64(len(dynstr.buf)), Addralign: 1},
		{Name: nameDynamic, Type: uint32(elf.SHT_DYNAMIC), Flags: uint64(elf.SHF_ALLOC | elf.SHF_WRITE), Addr: 0x3000, Off: offDynamic, Size: uint64(dyn.Len()), Link: 3, Addralign: 8, Entsize: 16},
		{Name: nameShstr, Type: uint32(elf.SHT_STRTAB), Off: offShstr, Size: uint64(len(shstr.buf)), Addralign: 1},
	}
	for _, s := range secs {
		_ = binary.Write(&body, bo, s)
	}
	raw := body.Bytes()
	var hdrBuf bytes.Buffer
	var ident [16]byte
	copy(ident[0:4], "\x7fELF")
	ident[elf.EI_CLASS] = uint8(elf.ELFCLASS64)
	ident[elf.EI_DATA] = uint8(elf.ELFDATA2MSB)
	ident[elf.EI_VERSION] = uint8(elf.EV_CURRENT)
	ident[elf.EI_OSABI] = uint8(elf.ELFOSABI_LINUX)
	_ = binary.Write(&hdrBuf, bo, elf.Header64{
		Ident:     ident,
		Type:      uint16(elf.ET_DYN),
		Machine:   uint16(elf.EM_PPC64),
		Version:   uint32(elf.EV_CURRENT),
		Entry:     0x1000,
		Phoff:     64,
		Shoff:     shoff,
		Flags:     2,
		Ehsize:    64,
		Phentsize: 56,
		Phnum:     1,
		Shentsize: 64,
		Shnum:     uint16(len(secs)),
		Shstrndx:  5,
	})
	_ = binary.Write(&hdrBuf, bo, elf.Prog64{
		Type:   uint32(elf.PT_LOAD),
		Flags:  uint32(elf.PF_R | elf.PF_X),
		Off:    offText,
		Vaddr:  0x1000,
		Paddr:  0x1000,
		Filesz: uint64(len(textData)),
		Memsz:  uint64(len(textData)),
		Align:  0x10000,
	})
	copy(raw[0:64+56], hdrBuf.Bytes())
	return raw
}

func buildElf32BEExec() []byte {
	bo := binary.BigEndian
	shstr := newStrtab()
	strtab := newStrtab()

	textData := []byte{0x03, 0xe0, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00}
	var symtab bytes.Buffer
	_ = binary.Write(&symtab, bo, uint32(0))
	_ = binary.Write(&symtab, bo, uint32(0))
	_ = binary.Write(&symtab, bo, uint32(0))
	_ = binary.Write(&symtab, bo, uint8(0))
	_ = binary.Write(&symtab, bo, uint8(0))
	_ = binary.Write(&symtab, bo, uint16(0))
	_ = binary.Write(&symtab, bo, strtab.add("_start"))
	_ = binary.Write(&symtab, bo, uint32(0x400000))
	_ = binary.Write(&symtab, bo, uint32(8))
	_ = binary.Write(&symtab, bo, elf.ST_INFO(elf.STB_GLOBAL, elf.STT_FUNC))
	_ = binary.Write(&symtab, bo, uint8(0))
	_ = binary.Write(&symtab, bo, uint16(1))

	nameText := shstr.add(".text")
	nameSymtab := shstr.add(".symtab")
	nameStrtab := shstr.add(".strtab")
	nameShstr := shstr.add(".shstrtab")

	var body bytes.Buffer
	padTo(&body, 52+32)
	offText := uint32(body.Len())
	body.Write(textData)
	padTo(&body, alignUp(body.Len(), 4))
	offSymtab := uint32(body.Len())
	body.Write(symtab.Bytes())
	padTo(&body, alignUp(body.Len(), 4))
	offStrtab := uint32(body.Len())
	body.Write(strtab.buf)
	padTo(&body, alignUp(body.Len(), 4))
	offShstr := uint32(body.Len())
	body.Write(shstr.buf)
	padTo(&body, alignUp(body.Len(), 4))

	shoff := uint32(body.Len())
	secs := []elf.Section32{
		{},
		{Name: nameText, Type: uint32(elf.SHT_PROGBITS), Flags: uint32(elf.SHF_ALLOC | elf.SHF_EXECINSTR), Addr: 0x400000, Off: offText, Size: uint32(len(textData)), Addralign: 4},
		{Name: nameSymtab, Type: uint32(elf.SHT_SYMTAB), Off: offSymtab, Size: uint32(symtab.Len()), Link: 3, Info: 1, Addralign: 4, Entsize: 16},
		{Name: nameStrtab, Type: uint32(elf.SHT_STRTAB), Off: offStrtab, Size: uint32(len(strtab.buf)), Addralign: 1},
		{Name: nameShstr, Type: uint32(elf.SHT_STRTAB), Off: offShstr, Size: uint32(len(shstr.buf)), Addralign: 1},
	}
	for _, s := range secs {
		_ = binary.Write(&body, bo, s)
	}
	raw := body.Bytes()
	var hdrBuf bytes.Buffer
	var ident [16]byte
	copy(ident[0:4], "\x7fELF")
	ident[elf.EI_CLASS] = uint8(elf.ELFCLASS32)
	ident[elf.EI_DATA] = uint8(elf.ELFDATA2MSB)
	ident[elf.EI_VERSION] = uint8(elf.EV_CURRENT)
	ident[elf.EI_OSABI] = uint8(elf.ELFOSABI_FREEBSD)
	_ = binary.Write(&hdrBuf, bo, elf.Header32{
		Ident:     ident,
		Type:      uint16(elf.ET_EXEC),
		Machine:   uint16(elf.EM_MIPS),
		Version:   uint32(elf.EV_CURRENT),
		Entry:     0x400000,
		Phoff:     52,
		Shoff:     shoff,
		Ehsize:    52,
		Phentsize: 32,
		Phnum:     1,
		Shentsize: 40,
		Shnum:     uint16(len(secs)),
		Shstrndx:  4,
	})
	_ = binary.Write(&hdrBuf, bo, elf.Prog32{
		Type:   uint32(elf.PT_LOAD),
		Off:    offText,
		Vaddr:  0x400000,
		Paddr:  0x400000,
		Filesz: uint32(len(textData)),
		Memsz:  uint32(len(textData)),
		Flags:  uint32(elf.PF_R | elf.PF_X),
		Align:  0x1000,
	})
	copy(raw[0:52+32], hdrBuf.Bytes())
	return raw
}

func observeElf(name string, raw []byte, relSec, relaSec string) ElfCase {
	f, err := elf.NewFile(bytes.NewReader(raw))
	if err != nil {
		panic(fmt.Sprintf("elf %s: %v", name, err))
	}
	defer f.Close()

	out := ElfCase{
		Name:        name,
		Hex:         hex.EncodeToString(raw),
		Class:       uint8(f.Class),
		Data:        uint8(f.Data),
		Version:     uint8(f.Version),
		OSABI:       uint8(f.OSABI),
		ABIVersion:  f.ABIVersion,
		Type:        uint16(f.Type),
		Machine:     uint16(f.Machine),
		Entry:       f.Entry,
		RelSection:  relSec,
		RelaSection: relaSec,
	}
	for _, p := range f.Progs {
		out.Progs = append(out.Progs, ElfProgOut{
			Type: uint32(p.Type), Flags: uint32(p.Flags), Off: p.Off,
			Vaddr: p.Vaddr, Paddr: p.Paddr, Filesz: p.Filesz, Memsz: p.Memsz, Align: p.Align,
		})
	}
	for _, s := range f.Sections {
		sec := ElfSecOut{
			Name: s.Name, Type: uint32(s.Type), Flags: uint64(s.Flags),
			Addr: s.Addr, Offset: s.Offset, Size: s.Size, FileSize: s.FileSize,
			Link: s.Link, Info: s.Info, Addralign: s.Addralign, Entsize: s.Entsize,
		}
		d, derr := s.Data()
		if derr != nil {
			sec.DataErr = true
		} else {
			sec.DataHex = hex.EncodeToString(d)
		}
		out.Sections = append(out.Sections, sec)
	}
	if syms, err := f.Symbols(); err == nil {
		out.HasSymbols = true
		for _, s := range syms {
			out.Symbols = append(out.Symbols, ElfSymOut{
				Name: s.Name, Info: s.Info, Other: s.Other, Section: uint32(s.Section), Value: s.Value, Size: s.Size,
			})
		}
	}
	if syms, err := f.DynamicSymbols(); err == nil {
		out.HasDynSymbols = true
		for _, s := range syms {
			out.DynSymbols = append(out.DynSymbols, ElfSymOut{
				Name: s.Name, Info: s.Info, Other: s.Other, Section: uint32(s.Section), Value: s.Value, Size: s.Size,
			})
		}
	}
	if libs, err := f.ImportedLibraries(); err == nil {
		out.ImportedLibraries = libs
	}
	if imps, err := f.ImportedSymbols(); err == nil {
		for _, im := range imps {
			out.ImportedSymbols = append(out.ImportedSymbols, im.Name)
		}
	}
	if v, err := f.DynString(elf.DT_SONAME); err == nil {
		out.Soname = v
	}
	if v, err := f.DynString(elf.DT_RUNPATH); err == nil {
		out.Runpath = v
	}
	if v, err := f.DynString(elf.DT_RPATH); err == nil {
		out.Rpath = v
	}
	if relSec != "" {
		s := f.Section(relSec)
		d, err := s.Data()
		if err != nil {
			panic(err)
		}
		if f.Class == elf.ELFCLASS32 {
			for i := 0; i < len(d); i += 8 {
				off := uint64(f.ByteOrder.Uint32(d[i : i+4]))
				info := f.ByteOrder.Uint32(d[i+4 : i+8])
				out.Rels = append(out.Rels, ElfRelOut{Offset: off, Sym: elf.R_SYM32(info), Type: elf.R_TYPE32(info)})
			}
		} else {
			for i := 0; i < len(d); i += 16 {
				off := f.ByteOrder.Uint64(d[i : i+8])
				info := f.ByteOrder.Uint64(d[i+8 : i+16])
				out.Rels = append(out.Rels, ElfRelOut{Offset: off, Sym: elf.R_SYM64(info), Type: elf.R_TYPE64(info)})
			}
		}
	}
	if relaSec != "" {
		s := f.Section(relaSec)
		d, err := s.Data()
		if err != nil {
			panic(err)
		}
		if f.Class == elf.ELFCLASS32 {
			for i := 0; i < len(d); i += 12 {
				off := uint64(f.ByteOrder.Uint32(d[i : i+4]))
				info := f.ByteOrder.Uint32(d[i+4 : i+8])
				add := int64(int32(f.ByteOrder.Uint32(d[i+8 : i+12])))
				out.Relas = append(out.Relas, ElfRelaOut{Offset: off, Sym: elf.R_SYM32(info), Type: elf.R_TYPE32(info), Addend: add})
			}
		} else {
			for i := 0; i < len(d); i += 24 {
				off := f.ByteOrder.Uint64(d[i : i+8])
				info := f.ByteOrder.Uint64(d[i+8 : i+16])
				add := int64(f.ByteOrder.Uint64(d[i+16 : i+24]))
				out.Relas = append(out.Relas, ElfRelaOut{Offset: off, Sym: elf.R_SYM64(info), Type: elf.R_TYPE64(info), Addend: add})
			}
		}
	}
	return out
}

// =========================================================================
// Mach-O builders
// =========================================================================

func fixed16(s string) [16]byte {
	var b [16]byte
	copy(b[:], s)
	return b
}

func makeDylibCmd(bo binary.ByteOrder, cmd uint32, path string, ts, cur, compat uint32) []byte {
	var b bytes.Buffer
	strBytes := append([]byte(path), 0)
	cmdsz := alignUp(24+len(strBytes), 8)
	_ = binary.Write(&b, bo, cmd)
	_ = binary.Write(&b, bo, uint32(cmdsz))
	_ = binary.Write(&b, bo, uint32(24))
	_ = binary.Write(&b, bo, ts)
	_ = binary.Write(&b, bo, cur)
	_ = binary.Write(&b, bo, compat)
	b.Write(strBytes)
	padTo(&b, cmdsz)
	return b.Bytes()
}

func makeStrCmd(bo binary.ByteOrder, cmd uint32, path string) []byte {
	var b bytes.Buffer
	strBytes := append([]byte(path), 0)
	cmdsz := alignUp(12+len(strBytes), 8)
	_ = binary.Write(&b, bo, cmd)
	_ = binary.Write(&b, bo, uint32(cmdsz))
	_ = binary.Write(&b, bo, uint32(12))
	b.Write(strBytes)
	padTo(&b, cmdsz)
	return b.Bytes()
}

func makeUuidCmd(bo binary.ByteOrder, uuid [16]byte) []byte {
	var b bytes.Buffer
	_ = binary.Write(&b, bo, uint32(0x1b))
	_ = binary.Write(&b, bo, uint32(24))
	b.Write(uuid[:])
	return b.Bytes()
}

func buildMachO64(bo binary.ByteOrder, cpu macho.Cpu, subcpu uint32, ftype macho.Type, withExtras bool) []byte {
	textData := []byte{0xc0, 0x03, 0x5f, 0xd6, 0x1f, 0x20, 0x03, 0xd5}
	cstrData := []byte("hello macho64\x00")
	dataData := []byte{0xde, 0xad, 0xbe, 0xef}

	strtab := newStrtab()
	var syms bytes.Buffer
	writeNlist64 := func(name string, typ, sect uint8, desc uint16, val uint64) {
		_ = binary.Write(&syms, bo, strtab.add(name))
		_ = binary.Write(&syms, bo, typ)
		_ = binary.Write(&syms, bo, sect)
		_ = binary.Write(&syms, bo, desc)
		_ = binary.Write(&syms, bo, val)
	}
	writeNlist64("_local_fn", 0x0e, 1, 0, 0x100000400)
	writeNlist64("_main", 0x0f, 1, 0, 0x100000404)
	writeNlist64("_printf", 0x01, 0, 0, 0)
	writeNlist64("_malloc", 0x01, 0, 0, 0)

	// Build load commands (first pass with placeholder offsets, then patch)
	var extraCmds [][]byte
	extraCmds = append(extraCmds, makeDylibCmd(bo, uint32(macho.LoadCmdDylib), "/usr/lib/libSystem.B.dylib", 2, 0x05200000, 0x00010000))
	if withExtras {
		extraCmds = append(extraCmds, makeDylibCmd(bo, uint32(macho.LoadCmdDylib), "/usr/lib/libz.1.dylib", 2, 0x0001020b, 0x00010000))
		extraCmds = append(extraCmds, makeDylibCmd(bo, 0xd, "/usr/local/lib/libself.dylib", 2, 0x00010000, 0x00010000)) // LC_ID_DYLIB
		extraCmds = append(extraCmds, makeStrCmd(bo, 0xe, "/usr/lib/dyld"))                                               // LC_LOAD_DYLINKER
		extraCmds = append(extraCmds, makeUuidCmd(bo, [16]byte{0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10}))
		extraCmds = append(extraCmds, makeStrCmd(bo, uint32(macho.LoadCmdRpath), "@executable_path/../Frameworks"))
		extraCmds = append(extraCmds, makeStrCmd(bo, uint32(macho.LoadCmdRpath), "@loader_path/lib"))
	} else {
		extraCmds = append(extraCmds, makeUuidCmd(bo, [16]byte{0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0xab, 0xcd}))
	}

	segTextCmdSz := 72 + 2*80
	segDataCmdSz := 72 + 1*80
	symtabCmdSz := 24
	dysymCmdSz := 80
	totalCmdSz := segTextCmdSz + segDataCmdSz + symtabCmdSz + dysymCmdSz
	for _, c := range extraCmds {
		totalCmdSz += len(c)
	}
	ncmd := uint32(4 + len(extraCmds))

	dataStart := alignUp(32+totalCmdSz, 16)
	offText := dataStart
	offCstr := alignUp(offText+len(textData), 8)
	textSegEnd := alignUp(offCstr+len(cstrData), 8)
	offData := textSegEnd
	dataSegEnd := alignUp(offData+len(dataData), 8)
	offSyms := dataSegEnd
	offStrs := alignUp(offSyms+syms.Len(), 8)

	var out bytes.Buffer
	_ = binary.Write(&out, bo, macho.Magic64)
	_ = binary.Write(&out, bo, uint32(cpu))
	_ = binary.Write(&out, bo, subcpu)
	_ = binary.Write(&out, bo, uint32(ftype))
	_ = binary.Write(&out, bo, ncmd)
	_ = binary.Write(&out, bo, uint32(totalCmdSz))
	_ = binary.Write(&out, bo, uint32(macho.FlagNoUndefs|macho.FlagDyldLink))
	_ = binary.Write(&out, bo, uint32(0)) // reserved

	// __TEXT segment + 2 sections
	_ = binary.Write(&out, bo, macho.Segment64{
		Cmd:     macho.LoadCmdSegment64,
		Len:     uint32(segTextCmdSz),
		Name:    fixed16("__TEXT"),
		Addr:    0x100000000,
		Memsz:   0x4000,
		Offset:  uint64(offText),
		Filesz:  uint64(textSegEnd - offText),
		Maxprot: 7,
		Prot:    5,
		Nsect:   2,
		Flag:    0,
	})
	_ = binary.Write(&out, bo, macho.Section64{
		Name:   fixed16("__text"),
		Seg:    fixed16("__TEXT"),
		Addr:   0x100000400,
		Size:   uint64(len(textData)),
		Offset: uint32(offText),
		Align:  2,
		Flags:  0x80000400,
	})
	_ = binary.Write(&out, bo, macho.Section64{
		Name:   fixed16("__cstring"),
		Seg:    fixed16("__TEXT"),
		Addr:   0x100000410,
		Size:   uint64(len(cstrData)),
		Offset: uint32(offCstr),
		Align:  0,
		Flags:  0x2,
	})

	// __DATA segment + 1 section
	_ = binary.Write(&out, bo, macho.Segment64{
		Cmd:     macho.LoadCmdSegment64,
		Len:     uint32(segDataCmdSz),
		Name:    fixed16("__DATA"),
		Addr:    0x100004000,
		Memsz:   0x4000,
		Offset:  uint64(offData),
		Filesz:  uint64(dataSegEnd - offData),
		Maxprot: 7,
		Prot:    3,
		Nsect:   1,
		Flag:    0,
	})
	_ = binary.Write(&out, bo, macho.Section64{
		Name:   fixed16("__data"),
		Seg:    fixed16("__DATA"),
		Addr:   0x100004000,
		Size:   uint64(len(dataData)),
		Offset: uint32(offData),
		Align:  2,
		Flags:  0,
	})

	// LC_SYMTAB
	_ = binary.Write(&out, bo, macho.SymtabCmd{
		Cmd:     macho.LoadCmdSymtab,
		Len:     uint32(symtabCmdSz),
		Symoff:  uint32(offSyms),
		Nsyms:   4,
		Stroff:  uint32(offStrs),
		Strsize: uint32(len(strtab.buf)),
	})

	// LC_DYSYMTAB
	_ = binary.Write(&out, bo, macho.DysymtabCmd{
		Cmd:        macho.LoadCmdDysymtab,
		Len:        uint32(dysymCmdSz),
		Ilocalsym:  0,
		Nlocalsym:  1,
		Iextdefsym: 1,
		Nextdefsym: 1,
		Iundefsym:  2,
		Nundefsym:  2,
	})

	for _, c := range extraCmds {
		out.Write(c)
	}
	padTo(&out, offText)
	out.Write(textData)
	padTo(&out, offCstr)
	out.Write(cstrData)
	padTo(&out, offData)
	out.Write(dataData)
	padTo(&out, offSyms)
	out.Write(syms.Bytes())
	padTo(&out, offStrs)
	out.Write(strtab.buf)
	return out.Bytes()
}

func buildMachO32(bo binary.ByteOrder, cpu macho.Cpu, ftype macho.Type) []byte {
	textData := []byte{0x90, 0x90, 0xc3, 0x90}
	strtab := newStrtab()
	var syms bytes.Buffer
	_ = binary.Write(&syms, bo, strtab.add("_entry32"))
	_ = binary.Write(&syms, bo, uint8(0x0f))
	_ = binary.Write(&syms, bo, uint8(1))
	_ = binary.Write(&syms, bo, uint16(0))
	_ = binary.Write(&syms, bo, uint32(0x1000))

	dylibCmd := makeDylibCmd(bo, uint32(macho.LoadCmdDylib), "/usr/lib/libSystem.B.dylib", 2, 0x00010000, 0x00010000)
	segCmdSz := 56 + 68
	symtabCmdSz := 24
	totalCmdSz := segCmdSz + symtabCmdSz + len(dylibCmd)
	offText := alignUp(28+totalCmdSz, 8)
	offSyms := alignUp(offText+len(textData), 8)
	offStrs := alignUp(offSyms+syms.Len(), 8)

	var out bytes.Buffer
	_ = binary.Write(&out, bo, macho.Magic32)
	_ = binary.Write(&out, bo, uint32(cpu))
	_ = binary.Write(&out, bo, uint32(3))
	_ = binary.Write(&out, bo, uint32(ftype))
	_ = binary.Write(&out, bo, uint32(3))
	_ = binary.Write(&out, bo, uint32(totalCmdSz))
	_ = binary.Write(&out, bo, uint32(macho.FlagNoUndefs))

	_ = binary.Write(&out, bo, macho.Segment32{
		Cmd:     macho.LoadCmdSegment,
		Len:     uint32(segCmdSz),
		Name:    fixed16("__TEXT"),
		Addr:    0x1000,
		Memsz:   0x1000,
		Offset:  uint32(offText),
		Filesz:  uint32(len(textData)),
		Maxprot: 7,
		Prot:    5,
		Nsect:   1,
	})
	_ = binary.Write(&out, bo, macho.Section32{
		Name:   fixed16("__text"),
		Seg:    fixed16("__TEXT"),
		Addr:   0x1000,
		Size:   uint32(len(textData)),
		Offset: uint32(offText),
		Align:  2,
		Flags:  0x80000400,
	})
	_ = binary.Write(&out, bo, macho.SymtabCmd{
		Cmd:     macho.LoadCmdSymtab,
		Len:     uint32(symtabCmdSz),
		Symoff:  uint32(offSyms),
		Nsyms:   1,
		Stroff:  uint32(offStrs),
		Strsize: uint32(len(strtab.buf)),
	})
	out.Write(dylibCmd)
	padTo(&out, offText)
	out.Write(textData)
	padTo(&out, offSyms)
	out.Write(syms.Bytes())
	padTo(&out, offStrs)
	out.Write(strtab.buf)
	return out.Bytes()
}

func buildMachOFat(sliceA, sliceB []byte) []byte {
	offA := alignUp(8+2*20, 16)
	offB := alignUp(offA+len(sliceA), 16)
	var out bytes.Buffer
	bo := binary.BigEndian
	_ = binary.Write(&out, bo, macho.MagicFat)
	_ = binary.Write(&out, bo, uint32(2))
	_ = binary.Write(&out, bo, macho.FatArchHeader{
		Cpu: macho.CpuAmd64, SubCpu: 3, Offset: uint32(offA), Size: uint32(len(sliceA)), Align: 4,
	})
	_ = binary.Write(&out, bo, macho.FatArchHeader{
		Cpu: macho.CpuArm64, SubCpu: 0, Offset: uint32(offB), Size: uint32(len(sliceB)), Align: 4,
	})
	padTo(&out, offA)
	out.Write(sliceA)
	padTo(&out, offB)
	out.Write(sliceB)
	return out.Bytes()
}

func observeMachO(name string, raw []byte) MachOCase {
	f, err := macho.NewFile(bytes.NewReader(raw))
	if err != nil {
		panic(fmt.Sprintf("macho %s: %v", name, err))
	}
	defer f.Close()

	out := MachOCase{
		Name:      name,
		Hex:       hex.EncodeToString(raw),
		Magic:     f.Magic,
		BigEndian: f.ByteOrder == binary.BigEndian,
		Cpu:       uint32(f.Cpu),
		SubCpu:    f.SubCpu,
		Type:      uint32(f.Type),
		Ncmd:      f.Ncmd,
		Cmdsz:     f.Cmdsz,
		Flags:     f.Flags,
	}
	for _, l := range f.Loads {
		switch v := l.(type) {
		case *macho.Segment:
			out.Segments = append(out.Segments, MachOSegOut{
				Cmd: uint32(v.Cmd), Name: v.Name, Addr: v.Addr, Memsz: v.Memsz,
				Offset: v.Offset, Filesz: v.Filesz, Maxprot: v.Maxprot, Prot: v.Prot, Nsect: v.Nsect, Flag: v.Flag,
			})
		case *macho.Rpath:
			out.Rpaths = append(out.Rpaths, v.Path)
		case macho.LoadBytes:
			if len(v) >= 8 {
				cmd := f.ByteOrder.Uint32(v[0:4])
				if cmd == 0x1b && len(v) >= 24 {
					out.UuidHex = hex.EncodeToString(v[8:24])
				} else if (cmd == 0xe || cmd == 0xf) && len(v) >= 12 {
					off := f.ByteOrder.Uint32(v[8:12])
					if int(off) < len(v) {
						s := v[off:]
						if idx := bytes.IndexByte(s, 0); idx >= 0 {
							s = s[:idx]
						}
						out.Dylinker = string(s)
					}
				}
			}
		}
	}
	for _, s := range f.Sections {
		d, err := s.Data()
		if err != nil {
			panic(err)
		}
		out.Sections = append(out.Sections, MachOSecOut{
			Name: s.Name, Seg: s.Seg, Addr: s.Addr, Size: s.Size,
			Offset: s.Offset, Align: s.Align, Flags: s.Flags, DataHex: hex.EncodeToString(d),
		})
	}
	if f.Symtab != nil {
		for _, sym := range f.Symtab.Syms {
			out.Symbols = append(out.Symbols, MachOSymOut{
				Name: sym.Name, Type: sym.Type, Sect: sym.Sect, Desc: sym.Desc, Value: sym.Value,
			})
		}
	}
	if libs, err := f.ImportedLibraries(); err == nil {
		out.ImportedLibraries = libs
	}
	if imps, err := f.ImportedSymbols(); err == nil {
		out.HasImportedSyms = true
		out.ImportedSymbols = imps
	}
	return out
}

func observeMachOFat(name string, raw []byte) MachOFatCase {
	ff, err := macho.NewFatFile(bytes.NewReader(raw))
	if err != nil {
		panic(fmt.Sprintf("macho fat %s: %v", name, err))
	}
	defer ff.Close()
	out := MachOFatCase{
		Name:  name,
		Hex:   hex.EncodeToString(raw),
		Magic: ff.Magic,
	}
	for _, a := range ff.Arches {
		out.Arches = append(out.Arches, MachOFatArchOut{
			Cpu: uint32(a.Cpu), SubCpu: a.SubCpu, Offset: a.Offset, Size: a.Size, Align: a.Align,
			Magic: a.File.Magic, Nsect: len(a.File.Sections),
		})
	}
	return out
}

// =========================================================================
// PE / COFF builders
// =========================================================================

type coffStrtab struct {
	buf []byte
}

func newCoffStrtab() *coffStrtab {
	return &coffStrtab{buf: make([]byte, 4)} // first 4 bytes hold total length
}

func (c *coffStrtab) add(s string) uint32 {
	off := uint32(len(c.buf))
	c.buf = append(c.buf, []byte(s)...)
	c.buf = append(c.buf, 0)
	binary.LittleEndian.PutUint32(c.buf[0:4], uint32(len(c.buf)))
	return off
}

func fixed8(s string) [8]byte {
	var b [8]byte
	copy(b[:], s)
	return b
}

func buildPE(pe64 bool) ([]byte, []string, int, string, uint32, []PeExportOut) {
	bo := binary.LittleEndian
	textData := []byte{0x48, 0x31, 0xc0, 0xc3}

	// Build .rdata at RVA 0x2000 containing Import Directory + Export Directory
	const rdataVA uint32 = 0x2000
	var rdata bytes.Buffer
	appendStr := func(s string) uint32 {
		rva := rdataVA + uint32(rdata.Len())
		rdata.WriteString(s)
		rdata.WriteByte(0)
		if rdata.Len()%2 != 0 {
			rdata.WriteByte(0)
		}
		return rva
	}
	appendHintName := func(hint uint16, s string) uint32 {
		if rdata.Len()%2 != 0 {
			rdata.WriteByte(0)
		}
		rva := rdataVA + uint32(rdata.Len())
		_ = binary.Write(&rdata, bo, hint)
		rdata.WriteString(s)
		rdata.WriteByte(0)
		if rdata.Len()%2 != 0 {
			rdata.WriteByte(0)
		}
		return rva
	}

	// Reserve space at start of .rdata for 3 import descriptors (2 DLLs + null terminator = 60 bytes)
	impDirRVA := rdataVA
	rdata.Write(make([]byte, 64))

	dll1Name := appendStr("KERNEL32.dll")
	dll2Name := appendStr("USER32.dll")
	hnExit := appendHintName(10, "ExitProcess")
	hnStd := appendHintName(25, "GetStdHandle")
	hnMsg := appendHintName(7, "MessageBoxA")

	padTo(&rdata, alignUp(rdata.Len(), 8))
	oft1 := rdataVA + uint32(rdata.Len())
	if pe64 {
		_ = binary.Write(&rdata, bo, uint64(hnExit))
		_ = binary.Write(&rdata, bo, uint64(hnStd))
		_ = binary.Write(&rdata, bo, uint64(0x800000000000002a)) // ordinal 42
		_ = binary.Write(&rdata, bo, uint64(0))
	} else {
		_ = binary.Write(&rdata, bo, uint32(hnExit))
		_ = binary.Write(&rdata, bo, uint32(hnStd))
		_ = binary.Write(&rdata, bo, uint32(0x8000002a)) // ordinal 42
		_ = binary.Write(&rdata, bo, uint32(0))
	}
	padTo(&rdata, alignUp(rdata.Len(), 8))
	oft2 := rdataVA + uint32(rdata.Len())
	if pe64 {
		_ = binary.Write(&rdata, bo, uint64(hnMsg))
		_ = binary.Write(&rdata, bo, uint64(0))
	} else {
		_ = binary.Write(&rdata, bo, uint32(hnMsg))
		_ = binary.Write(&rdata, bo, uint32(0))
	}

	// Patch import descriptors at offset 0 of rdata
	var idesc bytes.Buffer
	writeImpDesc := func(oft, ts, fc, nameRva, ft uint32) {
		_ = binary.Write(&idesc, bo, oft)
		_ = binary.Write(&idesc, bo, ts)
		_ = binary.Write(&idesc, bo, fc)
		_ = binary.Write(&idesc, bo, nameRva)
		_ = binary.Write(&idesc, bo, ft)
	}
	writeImpDesc(oft1, 0, 0, dll1Name, oft1)
	writeImpDesc(oft2, 0, 0, dll2Name, oft2)
	writeImpDesc(0, 0, 0, 0, 0) // null terminator
	copy(rdata.Bytes()[0:60], idesc.Bytes())
	impDirSize := uint32(60)

	// Build Export Directory inside .rdata
	padTo(&rdata, alignUp(rdata.Len(), 8))
	expDirOff := rdata.Len()
	expDirRVA := rdataVA + uint32(expDirOff)
	rdata.Write(make([]byte, 40)) // placeholder for IMAGE_EXPORT_DIRECTORY

	expDll := "demo32.dll"
	if pe64 {
		expDll = "demo64.dll"
	}
	expDllNameRVA := appendStr(expDll)
	expNameAdd := appendStr("Add")
	expNameSub := appendStr("Sub")
	expNameFwd := appendStr("HeapAllocFwd")
	expFwdTargetRVA := appendStr("NTDLL.RtlAllocateHeap")

	padTo(&rdata, alignUp(rdata.Len(), 4))
	aofRVA := rdataVA + uint32(rdata.Len())
	_ = binary.Write(&rdata, bo, uint32(0x1000))        // Add
	_ = binary.Write(&rdata, bo, uint32(0x1010))        // Sub
	_ = binary.Write(&rdata, bo, expFwdTargetRVA)       // HeapAllocFwd (forwarder inside export dir)
	aonRVA := rdataVA + uint32(rdata.Len())
	_ = binary.Write(&rdata, bo, expNameAdd)
	_ = binary.Write(&rdata, bo, expNameSub)
	_ = binary.Write(&rdata, bo, expNameFwd)
	aooRVA := rdataVA + uint32(rdata.Len())
	_ = binary.Write(&rdata, bo, uint16(0))
	_ = binary.Write(&rdata, bo, uint16(1))
	_ = binary.Write(&rdata, bo, uint16(2))
	expDirSize := uint32(rdata.Len() - expDirOff)

	var ed bytes.Buffer
	_ = binary.Write(&ed, bo, uint32(0))          // Characteristics
	_ = binary.Write(&ed, bo, uint32(0x65000000)) // TimeDateStamp
	_ = binary.Write(&ed, bo, uint16(1))          // MajorVersion
	_ = binary.Write(&ed, bo, uint16(0))          // MinorVersion
	_ = binary.Write(&ed, bo, expDllNameRVA)      // Name
	_ = binary.Write(&ed, bo, uint32(1))          // Base
	_ = binary.Write(&ed, bo, uint32(3))          // NumberOfFunctions
	_ = binary.Write(&ed, bo, uint32(3))          // NumberOfNames
	_ = binary.Write(&ed, bo, aofRVA)
	_ = binary.Write(&ed, bo, aonRVA)
	_ = binary.Write(&ed, bo, aooRVA)
	copy(rdata.Bytes()[expDirOff:expDirOff+40], ed.Bytes())

	// COFF string table and long section name
	cstr := newCoffStrtab()
	longSecOff := cstr.add(".debug_abbrev")
	longSymOff := cstr.add("very_long_coff_symbol_name")

	var symtab bytes.Buffer
	// Symbol 0: "main" with 1 auxiliary symbol record (to test aux skipping!)
	_ = binary.Write(&symtab, bo, pe.COFFSymbol{
		Name: fixed8("main"), Value: 0x1000, SectionNumber: 1, Type: 0x20, StorageClass: 2, NumberOfAuxSymbols: 1,
	})
	_ = binary.Write(&symtab, bo, pe.COFFSymbol{}) // aux record (must be skipped)
	// Symbol 2: long symbol from string table
	var longName [8]byte
	binary.LittleEndian.PutUint32(longName[0:4], 0)
	binary.LittleEndian.PutUint32(longName[4:8], longSymOff)
	_ = binary.Write(&symtab, bo, pe.COFFSymbol{
		Name: longName, Value: 0x2000, SectionNumber: 2, Type: 0, StorageClass: 3, NumberOfAuxSymbols: 0,
	})
	numSyms := uint32(3)

	debugAbbrevData := []byte{0x01, 0x11, 0x01, 0x00, 0x00}

	// Layout:
	// 0..127: DOS header with e_lfanew = 0x80 (128)
	// 128..131: "PE\0\0"
	// 132..151: COFF FileHeader (20)
	// 152..: OptionalHeader (224 for PE32, 240 for PE32+)
	// then 3 section headers (120 bytes)
	// 0x200 (512): .text
	// 0x400 (1024): .rdata
	// 0x600 (1536): .debug_abbrev
	// 0x700 (1792): COFF symbol table + string table
	var out bytes.Buffer
	dos := make([]byte, 0x80)
	dos[0] = 'M'
	dos[1] = 'Z'
	binary.LittleEndian.PutUint32(dos[0x3c:0x40], 0x80)
	out.Write(dos)
	out.Write([]byte{'P', 'E', 0, 0})

	optSz := uint16(224)
	machine := uint16(pe.IMAGE_FILE_MACHINE_I386)
	chars := uint16(pe.IMAGE_FILE_EXECUTABLE_IMAGE | pe.IMAGE_FILE_32BIT_MACHINE)
	if pe64 {
		optSz = 240
		machine = pe.IMAGE_FILE_MACHINE_AMD64
		chars = pe.IMAGE_FILE_EXECUTABLE_IMAGE | pe.IMAGE_FILE_LARGE_ADDRESS_AWARE
	}
	_ = binary.Write(&out, bo, pe.FileHeader{
		Machine:              machine,
		NumberOfSections:     3,
		TimeDateStamp:        0x65123456,
		PointerToSymbolTable: 0x700,
		NumberOfSymbols:      numSyms,
		SizeOfOptionalHeader: optSz,
		Characteristics:      chars,
	})

	var dds [16]pe.DataDirectory
	dds[pe.IMAGE_DIRECTORY_ENTRY_EXPORT] = pe.DataDirectory{VirtualAddress: expDirRVA, Size: expDirSize}
	dds[pe.IMAGE_DIRECTORY_ENTRY_IMPORT] = pe.DataDirectory{VirtualAddress: impDirRVA, Size: impDirSize}

	if pe64 {
		_ = binary.Write(&out, bo, pe.OptionalHeader64{
			Magic:               0x20b,
			MajorLinkerVersion:  14,
			MinorLinkerVersion:  0,
			SizeOfCode:          uint32(len(textData)),
			AddressOfEntryPoint: 0x1000,
			BaseOfCode:          0x1000,
			ImageBase:           0x140000000,
			SectionAlignment:    0x1000,
			FileAlignment:       0x200,
			SizeOfImage:         0x4000,
			SizeOfHeaders:       0x200,
			Subsystem:           pe.IMAGE_SUBSYSTEM_WINDOWS_CUI,
			DllCharacteristics:  pe.IMAGE_DLLCHARACTERISTICS_NX_COMPAT,
			NumberOfRvaAndSizes: 16,
			DataDirectory:       dds,
		})
	} else {
		_ = binary.Write(&out, bo, pe.OptionalHeader32{
			Magic:               0x10b,
			MajorLinkerVersion:  14,
			MinorLinkerVersion:  0,
			SizeOfCode:          uint32(len(textData)),
			AddressOfEntryPoint: 0x1000,
			BaseOfCode:          0x1000,
			BaseOfData:          0x2000,
			ImageBase:           0x00400000,
			SectionAlignment:    0x1000,
			FileAlignment:       0x200,
			SizeOfImage:         0x4000,
			SizeOfHeaders:       0x200,
			Subsystem:           pe.IMAGE_SUBSYSTEM_WINDOWS_GUI,
			DllCharacteristics:  pe.IMAGE_DLLCHARACTERISTICS_NX_COMPAT,
			NumberOfRvaAndSizes: 16,
			DataDirectory:       dds,
		})
	}

	// Section headers
	_ = binary.Write(&out, bo, pe.SectionHeader32{
		Name:             fixed8(".text"),
		VirtualSize:      uint32(len(textData)),
		VirtualAddress:   0x1000,
		SizeOfRawData:    uint32(len(textData)),
		PointerToRawData: 0x200,
		Characteristics:  pe.IMAGE_SCN_CNT_CODE | pe.IMAGE_SCN_MEM_EXECUTE | pe.IMAGE_SCN_MEM_READ,
	})
	_ = binary.Write(&out, bo, pe.SectionHeader32{
		Name:             fixed8(".rdata"),
		VirtualSize:      uint32(rdata.Len()),
		VirtualAddress:   rdataVA,
		SizeOfRawData:    uint32(rdata.Len()),
		PointerToRawData: 0x400,
		Characteristics:  pe.IMAGE_SCN_CNT_INITIALIZED_DATA | pe.IMAGE_SCN_MEM_READ,
	})
	_ = binary.Write(&out, bo, pe.SectionHeader32{
		Name:             fixed8(fmt.Sprintf("/%d", longSecOff)),
		VirtualSize:      uint32(len(debugAbbrevData)),
		VirtualAddress:   0x3000,
		SizeOfRawData:    uint32(len(debugAbbrevData)),
		PointerToRawData: 0x600,
		Characteristics:  pe.IMAGE_SCN_CNT_INITIALIZED_DATA | pe.IMAGE_SCN_MEM_DISCARDABLE | pe.IMAGE_SCN_MEM_READ,
	})

	padTo(&out, 0x200)
	out.Write(textData)
	padTo(&out, 0x400)
	out.Write(rdata.Bytes())
	padTo(&out, 0x600)
	out.Write(debugAbbrevData)
	padTo(&out, 0x700)
	out.Write(symtab.Bytes())
	out.Write(cstr.buf)

	wantExports := []PeExportOut{
		{Name: "Add", Ordinal: 1, Rva: 0x1000, Forwarder: ""},
		{Name: "Sub", Ordinal: 2, Rva: 0x1010, Forwarder: ""},
		{Name: "HeapAllocFwd", Ordinal: 3, Rva: expFwdTargetRVA, Forwarder: "NTDLL.RtlAllocateHeap"},
	}
	return out.Bytes(), []string{"KERNEL32.dll", "USER32.dll"}, 4, expDll, 1, wantExports
}

func buildCoffObj() []byte {
	bo := binary.LittleEndian
	textData := []byte{0xc0, 0x03, 0x5f, 0xd6}
	cstr := newCoffStrtab()
	_ = cstr.add("unused")
	var symtab bytes.Buffer
	_ = binary.Write(&symtab, bo, pe.COFFSymbol{
		Name: fixed8("arm64fn"), Value: 0, SectionNumber: 1, Type: 0x20, StorageClass: 2,
	})
	var out bytes.Buffer
	_ = binary.Write(&out, bo, pe.FileHeader{
		Machine:              pe.IMAGE_FILE_MACHINE_ARM64,
		NumberOfSections:     1,
		TimeDateStamp:        0x65000001,
		PointerToSymbolTable: 0x80,
		NumberOfSymbols:      1,
		SizeOfOptionalHeader: 0,
		Characteristics:      0,
	})
	_ = binary.Write(&out, bo, pe.SectionHeader32{
		Name:             fixed8(".text"),
		VirtualSize:      uint32(len(textData)),
		VirtualAddress:   0,
		SizeOfRawData:    uint32(len(textData)),
		PointerToRawData: 0x60,
		Characteristics:  pe.IMAGE_SCN_CNT_CODE | pe.IMAGE_SCN_MEM_EXECUTE | pe.IMAGE_SCN_MEM_READ,
	})
	padTo(&out, 0x60)
	out.Write(textData)
	padTo(&out, 0x80)
	out.Write(symtab.Bytes())
	out.Write(cstr.buf)
	return out.Bytes()
}

func observePE(name string, raw []byte, wantLibs []string, importCountAll int, expDll string, expBase uint32, wantExports []PeExportOut) PeCase {
	f, err := pe.NewFile(bytes.NewReader(raw))
	if err != nil {
		panic(fmt.Sprintf("pe %s: %v", name, err))
	}
	defer f.Close()

	out := PeCase{
		Name:              name,
		Hex:               hex.EncodeToString(raw),
		Machine:           f.Machine,
		NumberOfSections:  f.NumberOfSections,
		TimeDateStamp:     f.TimeDateStamp,
		Characteristics:   f.Characteristics,
		OptKind:           "none",
		ImportedLibraries: wantLibs,
		ImportCountAll:    importCountAll,
		ExportDll:         expDll,
		ExportBase:        expBase,
		Exports:           wantExports,
	}
	switch opt := f.OptionalHeader.(type) {
	case *pe.OptionalHeader32:
		out.OptKind = "pe32"
		out.EntryPoint = opt.AddressOfEntryPoint
		out.ImageBase = uint64(opt.ImageBase)
		out.Subsystem = opt.Subsystem
	case *pe.OptionalHeader64:
		out.OptKind = "pe64"
		out.EntryPoint = opt.AddressOfEntryPoint
		out.ImageBase = opt.ImageBase
		out.Subsystem = opt.Subsystem
	}
	for _, s := range f.Sections {
		d, err := s.Data()
		if err != nil {
			panic(err)
		}
		out.Sections = append(out.Sections, PeSecOut{
			Name: s.Name, VirtualSize: s.VirtualSize, VirtualAddress: s.VirtualAddress,
			Size: s.Size, Offset: s.Offset, Characteristics: s.Characteristics,
			DataHex: hex.EncodeToString(d),
		})
	}
	for _, s := range f.Symbols {
		out.Symbols = append(out.Symbols, PeSymOut{
			Name: s.Name, Value: s.Value, SectionNumber: s.SectionNumber,
			Type: s.Type, StorageClass: s.StorageClass,
		})
	}
	if imps, err := f.ImportedSymbols(); err == nil {
		out.ImportedSymbols = imps
	} else {
		panic(fmt.Sprintf("pe %s ImportedSymbols: %v", name, err))
	}
	return out
}

func main() {
	out := Output{}

	// 1. Valid ELF cases
	elf64LE := buildElf64LEExec()
	elf32LE := buildElf32LERel()
	elf64BE := buildElf64BEDyn()
	elf32BE := buildElf32BEExec()

	out.Elf = []ElfCase{
		observeElf("elf64_le_exec", elf64LE, "", ".rela.text"),
		observeElf("elf32_le_rel", elf32LE, ".rel.text", ".rela.data"),
		observeElf("elf64_be_dyn", elf64BE, "", ""),
		observeElf("elf32_be_exec", elf32BE, "", ""),
	}

	// Bad ELF cases (all verified to fail in Go's debug/elf too)
	badClass := append([]byte(nil), elf64LE[:64]...)
	badClass[elf.EI_CLASS] = 3
	badData := append([]byte(nil), elf64LE[:64]...)
	badData[elf.EI_DATA] = 3
	badVer := append([]byte(nil), elf64LE[:64]...)
	badVer[elf.EI_VERSION] = 2
	badShstr := append([]byte(nil), elf64LE...)
	binary.LittleEndian.PutUint16(badShstr[62:64], 99) // shstrndx out of range

	elfBadRaw := []struct {
		name string
		raw  []byte
	}{
		{"elf_empty", []byte{}},
		{"elf_short", []byte{0x7f, 'E', 'L', 'F', 2, 1, 1}},
		{"elf_bad_magic", []byte("NOT_AN_ELF_FILE_0123456789")},
		{"elf_bad_class", badClass},
		{"elf_bad_data", badData},
		{"elf_bad_version", badVer},
		{"elf_trunc_hdr64", elf64LE[:40]},
		{"elf_trunc_shdrs", elf64LE[:len(elf64LE)-20]},
		{"elf_bad_shstrndx", badShstr},
	}
	for _, c := range elfBadRaw {
		if f, err := elf.NewFile(bytes.NewReader(c.raw)); err == nil {
			_ = f.Close()
			panic(fmt.Sprintf("expected Go debug/elf to reject %s", c.name))
		}
		out.ElfBad = append(out.ElfBad, BadCase{Name: c.name, Hex: hex.EncodeToString(c.raw)})
	}

	// 2. Valid Mach-O cases
	m64Arm := buildMachO64(binary.LittleEndian, macho.CpuArm64, 0, macho.TypeExec, true)
	m64Amd := buildMachO64(binary.LittleEndian, macho.CpuAmd64, 3, macho.TypeExec, false)
	m32I386 := buildMachO32(binary.LittleEndian, macho.Cpu386, macho.TypeObj)
	m64PpcBE := buildMachO64(binary.BigEndian, macho.CpuPpc64, 0, macho.TypeDylib, false)
	m32PpcBE := buildMachO32(binary.BigEndian, macho.CpuPpc, macho.TypeExec)
	mFat := buildMachOFat(m64Amd, m64Arm)

	out.MachO = []MachOCase{
		observeMachO("macho64_le_arm64", m64Arm),
		observeMachO("macho32_le_386", m32I386),
		observeMachO("macho64_be_ppc64", m64PpcBE),
		observeMachO("macho32_be_ppc", m32PpcBE),
	}
	out.MachOFat = []MachOFatCase{
		observeMachOFat("macho_fat_universal", mFat),
	}

	// Bad Mach-O cases
	badCmdSz := append([]byte(nil), m64Arm[:40]...)
	badZeroCmd := append([]byte(nil), m64Arm...)
	binary.LittleEndian.PutUint32(badZeroCmd[36:40], 4) // cmdsize < 8

	machoBadRaw := []struct {
		name string
		raw  []byte
	}{
		{"macho_empty", []byte{}},
		{"macho_short", m64Arm[:12]},
		{"macho_bad_magic", make([]byte, 32)},
		{"macho_trunc_cmds", badCmdSz},
		{"macho_small_cmdsize", badZeroCmd},
	}
	for _, c := range machoBadRaw {
		if f, err := macho.NewFile(bytes.NewReader(c.raw)); err == nil {
			_ = f.Close()
			panic(fmt.Sprintf("expected Go debug/macho to reject %s", c.name))
		}
		out.MachOBad = append(out.MachOBad, BadCase{Name: c.name, Hex: hex.EncodeToString(c.raw)})
	}

	// Bad Fat cases
	zeroFat := []byte{0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x00}
	oobFat := append([]byte(nil), mFat...)
	// Point arch 1 past end of file
	binary.BigEndian.PutUint32(oobFat[8+20+8:8+20+12], uint32(len(oobFat)+64))
	machoFatBadRaw := []struct {
		name string
		raw  []byte
	}{
		{"fat_not_fat", m64Arm},
		{"fat_zero_arches", zeroFat},
		{"fat_trunc", mFat[:20]},
		{"fat_oob_slice", oobFat},
	}
	for _, c := range machoFatBadRaw {
		if ff, err := macho.NewFatFile(bytes.NewReader(c.raw)); err == nil {
			_ = ff.Close()
			panic(fmt.Sprintf("expected Go debug/macho NewFatFile to reject %s", c.name))
		}
		out.MachOFatBad = append(out.MachOFatBad, BadCase{Name: c.name, Hex: hex.EncodeToString(c.raw)})
	}

	// 3. Valid PE / COFF cases
	pe64Raw, pe64Libs, pe64ImpAll, pe64ExpDll, pe64ExpBase, pe64Exps := buildPE(true)
	pe32Raw, pe32Libs, pe32ImpAll, pe32ExpDll, pe32ExpBase, pe32Exps := buildPE(false)
	coffRaw := buildCoffObj()

	out.Pe = []PeCase{
		observePE("pe64_amd64_exe", pe64Raw, pe64Libs, pe64ImpAll, pe64ExpDll, pe64ExpBase, pe64Exps),
		observePE("pe32_i386_exe", pe32Raw, pe32Libs, pe32ImpAll, pe32ExpDll, pe32ExpBase, pe32Exps),
		observePE("pe_coff_arm64_obj", coffRaw, nil, 0, "", 0, nil),
	}

	// Bad PE cases
	badSig := append([]byte(nil), pe64Raw...)
	badSig[0x81] = 'X'
	badLfanew := append([]byte(nil), pe64Raw[:0x80]...)
	binary.LittleEndian.PutUint32(badLfanew[0x3c:0x40], 0x9999)
	badMach := append([]byte(nil), pe64Raw...)
	binary.LittleEndian.PutUint16(badMach[0x84:0x86], 0x9999)
	badOptMagic := append([]byte(nil), pe64Raw...)
	binary.LittleEndian.PutUint16(badOptMagic[0x84+20:0x84+22], 0x9999)

	peBadRaw := []struct {
		name string
		raw  []byte
	}{
		{"pe_empty", []byte{}},
		{"pe_short", []byte{'M', 'Z', 0, 0}},
		{"pe_bad_sig", badSig},
		{"pe_bad_lfanew", badLfanew},
		{"pe_bad_machine", badMach},
		{"pe_bad_opt_magic", badOptMagic},
		{"pe_trunc_sections", pe64Raw[:0x84+20+240+20]},
	}
	for _, c := range peBadRaw {
		if f, err := pe.NewFile(bytes.NewReader(c.raw)); err == nil {
			_ = f.Close()
			panic(fmt.Sprintf("expected Go debug/pe to reject %s", c.name))
		}
		out.PeBad = append(out.PeBad, BadCase{Name: c.name, Hex: hex.EncodeToString(c.raw)})
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
