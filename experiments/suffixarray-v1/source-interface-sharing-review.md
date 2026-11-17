# Sealed source/interface duplicate review

This is a proposal only. No sealed source or interface path has been linked,
renamed, compressed or removed.

The read-only inventory of 20 passing O0 and four passing O2 literal workdirs is
`build/suffixarray-v1-source-interface-duplicate-inventory.json`. Their plain
`.cinch` and `.cinchif` files total 52,346,708 bytes. Identical content accounts
for 48,694,541 redundant bytes in 82 hash groups. The inventory records each
path's complete content hash, size, mode, uid, gid, nanosecond mtime, device,
inode and link count. It excludes every failed project, oracle/SDK input,
compiler, provider, runtime, benchmark and active output.

There are **zero redundant bytes with identical complete metadata**. The
copies have different nanosecond mtimes. A hardlink shares one inode and its
mtime, so hardlinking those paths cannot preserve every original mtime in
place. Content equality alone does not establish that the epochs are identical
in metadata. No exact-metadata deduplication is presently eligible.

If the integration owner approves changing visible mtimes while keeping all
original metadata durably recoverable, the following narrower plan could reclaim
most of the 48.7 MB. It is not authorized or executed by this document.

1. Restrict candidates to report-linked, fully passed, closed functional
   workdirs. Require both normal and poison profiles and immutable file usage.
   Group only identical full content/size/mode/uid/gid on one device. Keep a
   representative plain file and explicitly record its chosen shared mtime.
2. Before any replacement, durably write an independent recovery index with
   every path's original bytes hash, size, mode, uid, gid and nanosecond mtime,
   source report/hash, representative path/hash and proposed resulting metadata.
   Fsync the index, atomic rename and its parent directory. Keep the earlier
   epoch reports unchanged; the index states the metadata adaptation separately.
3. For one file at a time, recheck both complete file hashes and metadata. Create
   a temporary hardlink to the representative, fsync it and verify its bytes.
   Atomically replace only the approved duplicate path, then fsync its parent
   and update/fsync the recovery index. Stop on any changed input or mismatch.
   Never chmod, chown or utime a shared inode to restore one path's metadata.
4. To restore independent original metadata, copy the shared bytes to a new
   temporary inode, verify the complete hash, apply owner/group before mode,
   restore the original nanosecond mtime, fsync, verify and atomically replace
   that one path. Fsync its parent and the recovery index. This restores bytes
   and recorded metadata; it does not claim restoration of the original inode.
5. Reopening a sealed directory for compilation would first require restoring
   independent files or copying it to a fresh epoch. In-place writes through a
   hardlink could otherwise corrupt every linked epoch. Future tool runs must
   not treat these sealed workdirs as writable caches.

Without approval for that explicit metadata adaptation, retain the current
plain copies. Verified functional ELF/IR recovery remains the existing source
of space, and qualification pauses at the normal phase guard.
