//! Binary interface packages borrowed from an immutable package artifact.

use crate::interface::{Iface, JsonValue, ParseError};
use crate::total::CheckedGraph;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

mod flat;
mod writer;
use flat::FlatBodies;
pub(crate) use flat::PackedClosure;
pub(crate) use writer::publish;

const HEADER: usize = 80;
const FLAT_HEADER: usize = 88;
const MODULE_ROW: usize = 36;
const BODY_ROW: usize = 32;
const MAX_DEPTH: usize = 256;

type Decode<T> = Result<T, ParseError>;
pub(crate) type Identities = BTreeMap<String, (u32, String)>;

fn invalid(reason: &str) -> ParseError {
    ParseError::from(reason.to_owned())
}

fn words<const N: usize>(data: &[u8], at: usize) -> Decode<[u32; N]> {
    let end = at
        .checked_add(N * 4)
        .ok_or_else(|| invalid("word offset overflow"))?;
    let data = data
        .get(at..end)
        .ok_or_else(|| invalid("truncated package word"))?;
    let mut words = [0; N];
    for (word, window) in words.iter_mut().zip(data.chunks_exact(4)) {
        *word = u32::from_le_bytes(window.try_into().unwrap());
    }
    Ok(words)
}

fn span(area: &[u8], start: u32, length: u32) -> Decode<&[u8]> {
    let start = start as usize;
    let end = start
        .checked_add(length as usize)
        .ok_or_else(|| invalid("span overflow"))?;
    area.get(start..end)
        .ok_or_else(|| invalid("package span outside its section"))
}

fn text(area: &[u8], start: u32, length: u32) -> Decode<&str> {
    std::str::from_utf8(span(area, start, length)?)
        .map_err(|_| invalid("invalid UTF-8 in interface package"))
}

#[derive(Clone, Copy)]
struct Strings<'a> {
    data: &'a [u8],
    spans: &'a [u8],
}

impl<'a> Strings<'a> {
    fn open(section: &'a [u8]) -> Decode<Self> {
        let [count] = words(section, 0)?;
        let end = (count as usize)
            .checked_mul(8)
            .and_then(|n| n.checked_add(4))
            .ok_or_else(|| invalid("invalid package string table length"))?;
        let data = section
            .get(end..)
            .ok_or_else(|| invalid("truncated package string table"))?;
        Ok(Self {
            data,
            spans: &section[4..end],
        })
    }

    fn get(self, id: u32) -> Decode<&'a str> {
        let row = (id as usize)
            .checked_mul(8)
            .ok_or_else(|| invalid("invalid package string index"))?;
        let [start, len] = words(self.spans, row)?;
        text(self.data, start, len)
    }

    fn prefix(self, count: u32) -> Decode<Self> {
        let bytes = (count as usize)
            .checked_mul(8)
            .and_then(|length| self.spans.get(..length))
            .ok_or_else(|| invalid("invalid structured ABI string boundary"))?;
        Ok(Self { spans: bytes, ..self })
    }
}

struct Package<'a> {
    strings: Strings<'a>,
    modules: &'a [u8],
    bodies: BodyTable<'a>,
    documents: &'a [u8],
}

enum BodyTable<'a> {
    Recipes { rows: &'a [u8], payload: &'a [u8] },
    Flat(FlatBodies<'a>),
}

enum Definitions {
    Recipes(Vec<JsonValue>),
    Flat(Arc<flat::CheckedTable>),
}

/// The decoder projects the checked graph from this exact wire root.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PackedBody {
    root: Box<str>,
    graph: CheckedGraph,
}

impl PackedBody {
    pub(crate) fn root(&self) -> &str {
        &self.root
    }

    pub(crate) fn graph(&self) -> &CheckedGraph {
        &self.graph
    }
}

impl<'a> Package<'a> {
    fn open(file: &'a [u8]) -> Decode<Self> {
        let (version, header) = match file.get(..8) {
            Some(b"CINCHPK2") => (2, HEADER),
            Some(b"CINCHPK3") => (3, FLAT_HEADER),
            Some(b"CINCHPK4") => (4, FLAT_HEADER),
            _ => return Err(invalid("invalid interface package header")),
        };
        let [length, format] = words(file, 8)?;
        if length as usize != file.len() || format != version {
            return Err(invalid("invalid interface package header"));
        }
        let mut sections = [&file[..0]; 9];
        let mut next = header;
        for (i, section) in sections[..(header - 16) / 8].iter_mut().enumerate() {
            let [start, length] = words(file, 16 + 8 * i)?;
            if start as usize != next {
                return Err(invalid("invalid interface package section directory"));
            }
            *section = span(file, start, length)?;
            next += section.len();
        }
        let [strings, modules, values, bodies, edges, payload, operands_or_documents, documents_or_abi, abi] =
            sections;
        if next != file.len()
            || modules.len() % MODULE_ROW != 0
            || values.len() % 32 != 0
            || bodies.len() % BODY_ROW != 0
            || edges.len() % 4 != 0
        {
            return Err(invalid("mis-sized interface package section"));
        }
        let strings = Strings::open(strings)?;
        let strings = if version == 4 && !abi.is_empty() {
            let [contract_count] = words(abi, 0)?;
            strings.prefix(contract_count)?
        } else {
            strings
        };
        let (bodies, documents) = if version >= 3 {
            (
                BodyTable::Flat(FlatBodies::open(
                    strings,
                    bodies,
                    edges,
                    payload,
                    operands_or_documents,
                )?),
                documents_or_abi,
            )
        } else {
            (
                BodyTable::Recipes {
                    rows: bodies,
                    payload,
                },
                operands_or_documents,
            )
        };
        Ok(Self {
            strings,
            modules,
            bodies,
            documents,
        })
    }

    fn identities(&self, bytes: &Arc<PackageBytes>) -> Decode<Identities> {
        let definitions = match &self.bodies {
            BodyTable::Recipes { rows, payload } => Definitions::Recipes(
                rows.chunks_exact(BODY_ROW)
                    .map(|row| {
                        let [offset, length] = words(row, 16)?;
                        Document::decode(self.strings, span(payload, offset, length)?, None)
                    })
                    .collect::<Decode<_>>()?,
            ),
            BodyTable::Flat(table) => Definitions::Flat(table.check(Arc::clone(bytes))?),
        };
        let count = self.modules.len() / MODULE_ROW;
        let mut seen = vec![false; count];
        let mut interfaces = BTreeMap::new();
        for rank in self.modules.chunks_exact(MODULE_ROW) {
            let [id] = words(rank, 32)?;
            let Some(slot) = seen.get_mut(id as usize) else {
                return Err(invalid("invalid package manifest index"));
            };
            if std::mem::replace(slot, true) {
                return Err(invalid("duplicate package manifest index"));
            }
            let row = id as usize * MODULE_ROW;
            let [name_at, name_len, hash_at, hash_len, _, _, doc_at, doc_len, _] =
                words(self.modules, row)?;
            let name = text(self.strings.data, name_at, name_len)?;
            let hash = text(self.strings.data, hash_at, hash_len)?;
            let input = span(self.documents, doc_at, doc_len)?;
            let value = Document::decode(self.strings, input, Some(&definitions))?;
            let iface = Iface::from_document(&value)?;
            if iface.module != name || iface.iface_hash() != hash {
                return Err(invalid("package interface identity mismatch"));
            }
            if interfaces
                .insert(iface.module, (iface.runtime_abi, hash.into()))
                .is_some()
            {
                return Err(invalid("duplicate package module"));
            }
        }
        Ok(interfaces)
    }
}

struct Document<'a> {
    strings: Strings<'a>,
    source: &'a [u8],
    definitions: Option<&'a Definitions>,
    at: usize,
}

impl<'a> Document<'a> {
    fn decode(
        strings: Strings<'a>,
        source: &'a [u8],
        definitions: Option<&'a Definitions>,
    ) -> Decode<JsonValue> {
        let mut state = Document {
            strings,
            source,
            definitions,
            at: 0,
        };
        let value = state.value(MAX_DEPTH)?;
        if state.at != source.len() {
            return Err(invalid("trailing bytes in structured contract"));
        }
        Ok(value)
    }

    fn take<const N: usize>(&mut self) -> Decode<[u32; N]> {
        let values = words(self.source, self.at)?;
        self.at += N * 4;
        Ok(values)
    }

    fn count(&mut self, minimum_bytes: usize) -> Decode<usize> {
        let [count] = self.take()?;
        let count = count as usize;
        if count > (self.source.len() - self.at) / minimum_bytes {
            return Err(invalid("truncated structured contract entries"));
        }
        Ok(count)
    }

    fn string(&mut self) -> Decode<&'a str> {
        let [id] = self.take()?;
        self.strings.get(id)
    }

    fn value(&mut self, depth: usize) -> Decode<JsonValue> {
        if depth == 0 {
            return Err(invalid("over-nested structured contract"));
        }
        let [tag] = self.take()?;
        let depth = depth - 1;
        Ok(match tag {
            0 => JsonValue::Null,
            1 => JsonValue::Bool(false),
            2 => JsonValue::Bool(true),
            3 => {
                let [lo, hi] = self.take()?;
                JsonValue::Num(u64::from(lo) | u64::from(hi) << 32)
            }
            4 => JsonValue::Str(self.string()?.into()),
            5 => {
                let count = self.count(4)?;
                JsonValue::Arr(
                    (0..count)
                        .map(|_| self.value(depth))
                        .collect::<Decode<_>>()?,
                )
            }
            6 => {
                let count = self.count(8)?;
                let mut fields = BTreeMap::new();
                let mut previous = None;
                for _ in 0..count {
                    let name = self.string()?;
                    if previous.is_some_and(|earlier| earlier >= name) {
                        return Err(invalid("unordered or duplicate structured contract field"));
                    }
                    previous = Some(name);
                    fields.insert(name.to_owned(), self.value(depth)?);
                }
                JsonValue::Obj(fields)
            }
            7 => JsonValue::Arr(self.definition_rows()?),
            9 => {
                let Some(Definitions::Flat(table)) = self.definitions else {
                    return Err(invalid("unexpected packed body root"));
                };
                let [root_id] = self.take()?;
                let root = table.definition_name(root_id)?;
                let graph = CheckedGraph::from_packed(table.closure_by_id(root_id)?);
                JsonValue::PackedBody(PackedBody { root: root.into(), graph })
            }
            _ => return Err(invalid("unknown or unsupported structured contract tag")),
        })
    }

    fn definition_rows(&mut self) -> Decode<Vec<JsonValue>> {
        let Some(Definitions::Recipes(definitions)) = self.definitions else {
            return Err(invalid("unexpected shared definition table"));
        };
        let count = self.count(8)?;
        let mut rows = Vec::new();
        for _ in 0..count {
            let [start, len] = self.take()?;
            let (start, len) = (start as usize, len as usize);
            let end = start
                .checked_add(len)
                .ok_or_else(|| invalid("definition range overflow"))?;
            if len == 0 || end > definitions.len() || len > definitions.len() - rows.len() {
                return Err(invalid("invalid definition range"));
            }
            rows.extend_from_slice(&definitions[start..end]);
        }
        Ok(rows)
    }
}

#[cfg(unix)]
mod mapping {
    use std::os::fd::AsRawFd;
    use std::ptr::NonNull;

    pub(super) struct Mapping {
        address: NonNull<u8>,
        len: usize,
    }

    // The mapping is read only and its owner keeps the inode immutable.
    // Shared references can read its bytes, and only drop unmaps it.
    unsafe impl Send for Mapping {}
    unsafe impl Sync for Mapping {}

    impl Mapping {
        /// The package publisher writes a fresh inode and replaces by rename.
        /// A caller must keep the mapped inode immutable until drop.
        pub(super) unsafe fn open(file: &std::fs::File) -> std::io::Result<Self> {
            let meta = file.metadata()?;
            if !meta.is_file() || meta.len() == 0 || meta.len() > u32::MAX as u64 {
                return Err(std::io::Error::other(
                    "invalid interface package length or file kind",
                ));
            }
            let len = meta.len() as usize;
            let raw = unsafe {
                libc::mmap(
                    std::ptr::null_mut(),
                    len,
                    libc::PROT_READ,
                    libc::MAP_PRIVATE,
                    file.as_raw_fd(),
                    0,
                )
            };
            if raw == libc::MAP_FAILED {
                return Err(std::io::Error::last_os_error());
            }
            let address = NonNull::new(raw.cast()).ok_or_else(|| {
                unsafe { libc::munmap(raw, len) };
                std::io::Error::other("could not map interface package at a non-null address")
            })?;
            Ok(Self { address, len })
        }

        pub(super) fn bytes(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.address.as_ptr(), self.len) }
        }
    }

    impl Drop for Mapping {
        fn drop(&mut self) {
            unsafe { libc::munmap(self.address.as_ptr().cast(), self.len) };
        }
    }
}

#[cfg(unix)]
use mapping::Mapping as PackageBytes;

#[cfg(not(unix))]
struct PackageBytes(Vec<u8>);

#[cfg(not(unix))]
impl PackageBytes {
    fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Read every verified interface. Checked body views share this file's backing.
/// The caller verifies the same name and digest against the object manifest.
pub(crate) fn read_verified_identities(path: &Path) -> Result<Identities, crate::Error> {
    let error =
        |error: &dyn std::fmt::Display| crate::Error::plain(format!("{}: {error}", path.display()));
    let file = std::fs::File::open(path).map_err(|e| error(&e))?;
    #[cfg(unix)]
    let bytes = Arc::new(unsafe { PackageBytes::open(&file) }.map_err(|e| error(&e))?);
    #[cfg(not(unix))]
    let bytes = {
        let mut owned = Vec::new();
        std::io::Read::read_to_end(&mut &file, &mut owned).map_err(|e| error(&e))?;
        Arc::new(PackageBytes(owned))
    };
    Package::open(bytes.bytes())
        .and_then(|package| package.identities(&bytes))
        .map_err(|e| error(&e))
}
