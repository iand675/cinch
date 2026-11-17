# Archive operation mapping

All 85 pinned public declaration records are mapped individually in `operation-mapping.json`. Repeated constant declarations and historical annotations remain separate records. This is a static design audit; no record is claimed implemented or qualified by the mapping alone.

| Records | Public capability | Candidate boundary |
| --- | --- | --- |
| 1, 2 | `Wire/format constants` | `TarGnu` |
| 3, 4 | `Wire/format constants` | `TarPax` |
| 5, 6 | `Wire/format constants` | `TarUstar` |
| 7, 8 | `Wire/format constants` | `TarAuto` |
| 9, 10 | `Wire/format constants` | `TarBlockDevice` |
| 11, 12 | `Wire/format constants` | `TarCharacterDevice` |
| 13, 14 | `Wire/format constants` | `TarContinuous` |
| 15, 16 | `Wire/format constants` | `TarDirectory` |
| 17, 18 | `Wire/format constants` | `TarFifo` |
| 19, 20 | `Wire/format constants` | `TarLongLink` |
| 21, 22 | `Wire/format constants` | `TarLongName` |
| 23, 24 | `Wire/format constants` | `TarSparse` |
| 25, 26 | `Wire/format constants` | `TarHardLink` |
| 27, 28 | `Wire/format constants` | `TarRegular` |
| 29, 30, 31 | `Wire/format constants` | `TarLegacyRegular` |
| 32, 33 | `Wire/format constants` | `TarSymbolicLink` |
| 34, 35 | `Wire/format constants` | `TarPaxGlobal` |
| 36, 37 | `Wire/format constants` | `TarPaxLocal` |
| 38 | `func FileInfoHeader(os.FileInfo, string) (*Header, error)` | `tar-header-from-metadata` |
| 39 | `func NewReader(io.Reader) *Reader` | `tar-reader` |
| 40 | `func NewWriter(io.Writer) *Writer` | `tar-writer` |
| 41 | `method (*Header) FileInfo() os.FileInfo` | `tar-header-metadata` |
| 42 | `method (*Reader) Next() (*Header, error)` | `tar-next` |
| 43 | `method (*Reader) Read([]uint8) (int, error)` | `tar-read` |
| 44 | `method (*Writer) AddFS(fs.FS) error` | `tar-add-tree` |
| 45 | `method (*Writer) Close() error` | `tar-finish` |
| 46 | `method (*Writer) Flush() error` | `tar-flush` |
| 47 | `method (*Writer) Write([]uint8) (int, error)` | `tar-write` |
| 48 | `method (*Writer) WriteHeader(*Header) error` | `tar-write-header` |
| 49 | `method (Format) String() string` | `tar-format-name` |
| 50 | `type FileInfoNames interface { Gname, IsDir, ModTime, Mode, Name, Size, Sys, Uname }` | `TarFileMetadata + owner-name provider` |
| 51 | `type FileInfoNames interface, Gname() (string, error)` | `TarFileMetadata/Gname` |
| 52 | `type FileInfoNames interface, IsDir() bool` | `TarFileMetadata/IsDir` |
| 53 | `type FileInfoNames interface, ModTime() time.Time` | `TarFileMetadata/ModTime` |
| 54 | `type FileInfoNames interface, Mode() fs.FileMode` | `TarFileMetadata/Mode` |
| 55 | `type FileInfoNames interface, Name() string` | `TarFileMetadata/Name` |
| 56 | `type FileInfoNames interface, Size() int64` | `TarFileMetadata/Size` |
| 57 | `type FileInfoNames interface, Sys() interface{}` | `TarFileMetadata/Sys` |
| 58 | `type FileInfoNames interface, Uname() (string, error)` | `TarFileMetadata/Uname` |
| 59 | `type Format int` | `TarFormat` |
| 60 | `type Header struct` | `TarHeader` |
| 61 | `AccessTime` | `TarTimes/access; PAX fractional or GNU whole seconds, ignored in auto writer policy` |
| 62 | `ChangeTime` | `TarTimes/change; PAX fractional or GNU whole seconds, ignored in auto writer policy` |
| 63 | `Devmajor` | `TarDevice/major signed I64` |
| 64 | `Devminor` | `TarDevice/minor signed I64` |
| 65 | `Format` | `TarFormat request/reader best guess; internal V7/STAR distinction retained` |
| 66 | `Gid` | `TarPermissions/group-id signed I64, avoid host-width truncation` |
| 67 | `Gname` | `TarPermissions/group-name immutable raw bytes` |
| 68 | `Linkname` | `TarPaths/link; preserve raw bytes and do not subject link target to entry-name locality policy` |
| 69 | `ModTime` | `TarTimes/modification Option TarTimestamp; signed seconds + canonical nanoseconds, distinguish absent from Unix epoch` |
| 70 | `Mode` | `TarPermissions/mode signed I64, preserve full mode bits` |
| 71 | `Name` | `TarPaths/name, raw immutable TarText; preserve invalid UTF-8 for legacy/GNU` |
| 72 | `PAXRecords` | `TarAttributes/records immutable list snapshot; last duplicate key wins on parse, deterministic sorted serialization; basic/header-derived and sparse protected precedence` |
| 73 | `Size` | `signed I64 logical size; reject negative body size except ignored header-only size; physical sparse size internal` |
| 74 | `Typeflag` | `TarEntryType including TarOther byte for unknown extension flags` |
| 75 | `Uid` | `TarPermissions/user-id signed I64, avoid host-width truncation` |
| 76 | `Uname` | `TarPermissions/user-name immutable raw bytes` |
| 77, 78 | `Xattrs` | `TarAttributes/xattrs compatibility input; map to SCHILY.xattr, precedence above explicit PAX key; expose parsed attributes as immutable snapshots` |
| 79 | `type Reader struct` | `TarReader (r :: Region)` |
| 80 | `type Writer struct` | `TarWriter (r :: Region)` |
| 81 | `var ErrFieldTooLong error` | `TarFieldTooLong` |
| 82 | `var ErrHeader error` | `TarInvalidHeader` |
| 83 | `var ErrInsecurePath error` | `TarInsecurePath` |
| 84 | `var ErrWriteAfterClose error` | `TarWriteAfterClose` |
| 85 | `var ErrWriteTooLong error` | `TarWriteTooLong` |

The detailed contracts cover nil/empty adaptation, metadata/provenance, format selection and rounding, advisory path errors, body boundaries, unread-body discard, partial read/write failures, sticky failures and finish state, PAX precedence, global records, long names, sparse formats and tree traversal. `design.md` gives the exact options and ownership plan.

Drafted components: immutable metadata, physical-block framing, checksums, field scans and octal/base-256 numeric codecs. Remaining: complete header materialization/selection, PAX framing, sparse decoder, entry controller, metadata/tree adapters, exact independent vector execution, maintained normal/poison/portable gates and public workload benchmarks.
