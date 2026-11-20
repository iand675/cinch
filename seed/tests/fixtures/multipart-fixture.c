#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>

/** Exact emitted vector body, linked as a separate real kernel object. */
extern uint32_t multipart_equal16(uint64_t low, uint64_t high, uint8_t needle);

/** Scripted sink limits; negative entries accept the whole offer. */
static int64_t limits[128];
/** Arbitrary caller-error identities, independently of accepted prefix lengths. */
static int64_t failures[128];
/** Number of direct sink callbacks since the last explicit reset. */
static uint64_t calls;
/** Exact event fingerprint in specified little-endian word order. */
static uint64_t trace_hash;
/** Fingerprint of only confirmed output bytes. */
static uint64_t output_hash;
/** Number of confirmed output bytes. */
static uint64_t output_size;
/** The most recent simultaneous caller failure identity. */
static int64_t last_error;
/** Privately copied oracle wire, never a retained language pointer. */
static uint8_t *source_wire;
/** Actual copied source length and consumed cursor. */
static uint64_t source_length, source_position;
/** Maximum returned prefix per source callback, with zero meaning unrestricted. */
static uint64_t source_chunk;
/** Terminal status identity: one for EOF, two for the distinct caller failure. */
static int64_t source_terminal;
/** Most recent simultaneous source status. */
static int64_t source_status;
/** Exact callback count for detecting reads after sticky terminal status. */
static uint64_t source_calls;
/** Independent requested/prefix/status trace fingerprint. */
static uint64_t source_trace;
/** A single selected form source interrupts actual final scratch-close descriptors. */
static int source_interrupt_close;
/** Interrupt actual fixture-owned descriptors without marking logical handles closed. */
static void interrupt_scratch(void);

/** Extend a wrapping FNV fingerprint with one byte without vectorization. */
static uint64_t mix(uint64_t hash, uint8_t byte) {
    return (hash ^ byte) * UINT64_C(1099511628211);
}

/** Append a scalar word in an explicit target-independent wire order. */
static uint64_t word(uint64_t hash, uint64_t value) {
    for (unsigned shift = 0; shift != 64; shift += 8)
        hash = mix(hash, (uint8_t)(value >> shift));
    return hash;
}

/** Reset the fixture while retaining no borrowed language pointers. */
int64_t multipart_fixture_reset(int64_t ignored) {
    (void)ignored;
    for (size_t i = 0; i != 128; ++i) { limits[i] = -1; failures[i] = 0; }
    calls = output_size = 0;
    trace_hash = output_hash = UINT64_C(14695981039346656037);
    last_error = 0;
    return 0;
}

/** Set one count/error script position and fail closed on fixture overflow. */
int64_t multipart_fixture_script(int64_t at, int64_t count, int64_t failure) {
    if (at < 0 || at >= 128) return -1;
    limits[at] = count; failures[at] = failure;
    return 0;
}

/** Observe one actual borrowed sink offer and its exact accepted prefix. */
int64_t multipart_fixture_write(int64_t address, int64_t length) {
    if (length < 0 || calls >= 128) return -1;
    const uint8_t *input = (const uint8_t *)(uintptr_t)address;
    uint64_t count = (uint64_t)length;
    if (limits[calls] >= 0 && (uint64_t)limits[calls] < count)
        count = (uint64_t)limits[calls];
    last_error = failures[calls++];
    trace_hash = word(trace_hash, (uint64_t)length);
    for (int64_t i = 0; i < length; ++i) trace_hash = mix(trace_hash, input[i]);
    trace_hash = word(word(trace_hash, count), (uint64_t)last_error);
    for (uint64_t i = 0; i < count; ++i) output_hash = mix(output_hash, input[i]);
    output_size += count;
    return (int64_t)count;
}

/** Read the simultaneous failure identity without consuming more output. */
int64_t multipart_fixture_error(int64_t ignored) { (void)ignored; return last_error; }
/** Read exact direct callback count. */
int64_t multipart_fixture_calls(int64_t ignored) { (void)ignored; return (int64_t)calls; }
/** Read the exact offer/count/error trace fingerprint. */
uint64_t multipart_fixture_trace(int64_t ignored) { (void)ignored; return trace_hash; }
/** Read the confirmed output fingerprint. */
uint64_t multipart_fixture_output(int64_t ignored) { (void)ignored; return output_hash; }
/** Read the confirmed output length. */
int64_t multipart_fixture_size(int64_t ignored) { (void)ignored; return (int64_t)output_size; }

/** Copy exact source bytes before the library can mutate or retain a caller input. */
int64_t multipart_fixture_source_init(int64_t address, int64_t length, int64_t chunk, int64_t terminal) {
    if (length < 0 || chunk < 0) return -1;
    free(source_wire);
    source_wire = malloc(length ? (size_t)length : 1);
    if (!source_wire) return -1;
    if (length) memcpy(source_wire, (const void *)(uintptr_t)address, (size_t)length);
    source_length = (uint64_t)length;
    source_position = source_calls = 0;
    source_chunk = (uint64_t)chunk;
    source_terminal = terminal;
    source_status = 0;
    source_trace = UINT64_C(14695981039346656037);
    return 0;
}

/** Produce one bounded prefix with a simultaneous status, allowing repeated terminal calls. */
int64_t multipart_fixture_source_read(int64_t address, int64_t capacity) {
    if (!source_wire || capacity < 0) return -1;
    uint64_t count = (uint64_t)capacity;
    if (source_chunk && count > source_chunk) count = source_chunk;
    if (count > source_length - source_position) count = source_length - source_position;
    uint8_t *output = (uint8_t *)(uintptr_t)address;
    if (count) memcpy(output, source_wire + source_position, (size_t)count);
    source_position += count;
    if (source_interrupt_close && source_position == source_length) {
        interrupt_scratch();
        source_interrupt_close = 0;
    }
    source_status = source_position == source_length ?
        (source_terminal == 3 ? (count == 0 ? 1 : 0) : source_terminal) : 0;
    ++source_calls;
    source_trace = word(word(source_trace, (uint64_t)capacity), count);
    for (uint64_t at = 0; at < count; ++at) source_trace = mix(source_trace, output[at]);
    source_trace = word(source_trace, (uint64_t)source_status);
    return (int64_t)count;
}

/** Observe a language-owned callback's exact bytes/status without replacing its source behavior. */
int64_t multipart_fixture_observe_read(int64_t capacity, int64_t address, int64_t count, int64_t status) {
    if (capacity < 0 || count < 0 || count > capacity || status < 0 || status > 2) return -1;
    const uint8_t *output = (const uint8_t *)(uintptr_t)address;
    ++source_calls;
    source_trace = word(word(source_trace, (uint64_t)capacity), (uint64_t)count);
    for (int64_t at = 0; at < count; ++at) source_trace = mix(source_trace, output[at]);
    source_trace = word(source_trace, (uint64_t)status);
    return 0;
}

/** Observe the status accompanying the most recent exact source prefix. */
int64_t multipart_fixture_source_status(int64_t ignored) { (void)ignored; return source_status; }
/** Observe every actual source request/prefix/status without changing source state. */
uint64_t multipart_fixture_source_trace(int64_t ignored) { (void)ignored; return source_trace; }
/** Observe exact underlying callback count, including any invalid repeated terminal read. */
int64_t multipart_fixture_source_calls(int64_t ignored) { (void)ignored; return (int64_t)source_calls; }
/** Release fixture-owned wire storage after observations, including failed cases. */
int64_t multipart_fixture_source_finish(int64_t ignored) {
    (void)ignored;
    free(source_wire); source_wire = NULL;
    return 0;
}

/** One actual temporary file key independent of each opened handle. */
struct scratch_key {
    /** Full process-private filesystem path, never synthesized from a public filename. */
    char path[256];
    /** Whether cleanup still owes an unlink. */
    int live;
};

/** One independent open descriptor with retained logical close state. */
struct scratch_handle {
    /** Actual operating-system descriptor. */
    int fd;
    /** Matching private file key. */
    int key;
    /** Repeated close must not close a subsequently reused descriptor. */
    int closed;
};

/** Actual created file identities for this explicit fixture context. */
static struct scratch_key scratch_keys[128];
/** Actual independently opened descriptor identities. */
static struct scratch_handle scratch_handles[256];
/** Created key and handle counts, reserving token zero as invalid. */
static int scratch_key_count, scratch_handle_count;
/** Last actual created key for the tuple-valued creation callback. */
static int scratch_last_key;
/** Typed caller error identity from the most recent backend operation. */
static int scratch_error;
/** Most recent read status: zero more, one EOF, two caller error. */
static int scratch_status;
/** Backend callback count for independently checking advanced generic state. */
static int64_t scratch_calls;
/** Count of descriptors actually interrupted by the final-close oracle hook. */
static int64_t scratch_interrupted;
/** Private namespace created exclusively for this one fixture process/context. */
static char scratch_directory[192];

/** Clear only error/status observations at the beginning of one backend callback. */
static void begin_scratch(void) {
    ++scratch_calls; scratch_error = scratch_status = 0;
}

/** Look up a real handle without silently inventing metadata for an invalid token. */
static struct scratch_handle *get_scratch(int64_t token) {
    if (token <= 0 || token > scratch_handle_count) { scratch_error = 108; return NULL; }
    return &scratch_handles[token];
}

/** Allocate a distinct logical handle after the actual OS open succeeded. */
static int64_t add_scratch_handle(int fd, int key) {
    if (scratch_handle_count + 1 >= 256) { close(fd); scratch_error = 108; return 0; }
    struct scratch_handle *entry = &scratch_handles[++scratch_handle_count];
    entry->fd = fd; entry->key = key; entry->closed = 0;
    return scratch_handle_count;
}

/** Verify no outstanding resources before creating a new private scratch namespace. */
int64_t multipart_fixture_fs_begin(int64_t ignored) {
    (void)ignored;
    if (scratch_directory[0]) return -1;
    memset(scratch_keys, 0, sizeof scratch_keys);
    memset(scratch_handles, 0, sizeof scratch_handles);
    strcpy(scratch_directory, "/tmp/cinch-multipart-fixture-XXXXXX");
    if (!mkdtemp(scratch_directory)) { scratch_directory[0] = 0; return -1; }
    scratch_key_count = scratch_handle_count = scratch_last_key = 0;
    scratch_error = scratch_status = 0;
    scratch_calls = scratch_interrupted = 0;
    source_interrupt_close = 0;
    return 0;
}

/** Enable the actual deferred-close fault for the next final source-prefix callback. */
int64_t multipart_fixture_fs_interrupt(int64_t enabled) { source_interrupt_close = enabled != 0; return 0; }

/** Close actual live temporary descriptors while retaining their logical open state. */
static void interrupt_scratch(void) {
    for (int index = 1; index <= scratch_handle_count; ++index) {
        struct scratch_handle *entry = &scratch_handles[index];
        if (!entry->closed && close(entry->fd) == 0) ++scratch_interrupted;
    }
}

/** Create a real exclusive scratch file and publish its actual handle/key tuple. */
int64_t multipart_fixture_fs_create(int64_t ignored) {
    (void)ignored; begin_scratch();
    if (scratch_key_count + 1 >= 128) { scratch_error = 109; return 0; }
    struct scratch_key *key = &scratch_keys[scratch_key_count + 1];
    snprintf(key->path, sizeof key->path, "%s/multipart-XXXXXX", scratch_directory);
    int fd = mkstemp(key->path);
    if (fd < 0) { scratch_error = 109; return 0; }
    key->live = 1; scratch_last_key = ++scratch_key_count;
    return add_scratch_handle(fd, scratch_last_key);
}

/** Observe the already-created key without incrementing backend callback state. */
int64_t multipart_fixture_fs_key(int64_t ignored) { (void)ignored; return scratch_last_key; }
/** Observe the original typed backend failure identity. */
int64_t multipart_fixture_fs_error(int64_t ignored) { (void)ignored; return scratch_error; }
/** Observe a read's simultaneous EOF/error status. */
int64_t multipart_fixture_fs_status(int64_t ignored) { (void)ignored; return scratch_status; }
/** Observe actual backend callback count independently of the language state cell. */
int64_t multipart_fixture_fs_calls(int64_t ignored) { (void)ignored; return scratch_calls; }
/** Observe actual interrupted descriptors, never an invented close diagnostic. */
int64_t multipart_fixture_fs_interrupted(int64_t ignored) { (void)ignored; return scratch_interrupted; }

/** Make one real descriptor write with exact confirmed-prefix/error observations. */
int64_t multipart_fixture_fs_write(int64_t token, int64_t address, int64_t length) {
    begin_scratch(); struct scratch_handle *entry = get_scratch(token);
    if (!entry || length < 0) { scratch_error = 108; return 0; }
    if (entry->closed) { scratch_error = 107; return 0; }
    ssize_t result;
    do { result = write(entry->fd, (const void *)(uintptr_t)address, (size_t)length); } while (result < 0 && errno == EINTR);
    if (result < 0) { scratch_error = 108; return 0; }
    if (result < length) scratch_error = 110;
    return result;
}

/** Read sequentially using the real descriptor's independent current position. */
int64_t multipart_fixture_fs_read(int64_t token, int64_t address, int64_t length) {
    begin_scratch(); struct scratch_handle *entry = get_scratch(token);
    if (!entry || length < 0) { scratch_error = 108; scratch_status = 2; return 0; }
    if (entry->closed) { scratch_error = 100; scratch_status = 2; return 0; }
    ssize_t result;
    do { result = read(entry->fd, (void *)(uintptr_t)address, (size_t)length); } while (result < 0 && errno == EINTR);
    if (result < 0) { scratch_error = 108; scratch_status = 2; return 0; }
    if (!result && length) scratch_status = 1;
    return result;
}

/** Fill a positioned request using real pread calls without moving the descriptor cursor. */
int64_t multipart_fixture_fs_read_at(int64_t token, int64_t address, int64_t length, int64_t offset) {
    begin_scratch(); struct scratch_handle *entry = get_scratch(token);
    if (!entry || length < 0) { scratch_error = 108; scratch_status = 2; return 0; }
    if (offset < 0) { scratch_error = 101; scratch_status = 2; return 0; }
    if (!length) return 0;
    if (entry->closed) { scratch_error = 100; scratch_status = 2; return 0; }
    int64_t total = 0;
    while (total < length) {
        ssize_t result = pread(entry->fd, (uint8_t *)(uintptr_t)address + total,
                               (size_t)(length - total), (off_t)(offset + total));
        if (result < 0 && errno == EINTR) continue;
        if (result < 0) { scratch_error = 108; scratch_status = 2; return total; }
        if (!result) { scratch_status = 1; return total; }
        total += result;
    }
    return total;
}

/** Seek a whole disk descriptor while keeping section seek errors a library concern. */
int64_t multipart_fixture_fs_seek(int64_t token, int64_t offset, int64_t whence) {
    begin_scratch(); struct scratch_handle *entry = get_scratch(token);
    if (!entry) { scratch_error = 108; return 0; }
    if (entry->closed) { scratch_error = 103; return 0; }
    off_t result = lseek(entry->fd, (off_t)offset, (int)whence);
    if (result < 0) { scratch_error = 102; return 0; }
    return (int64_t)result;
}

/** Preserve repeated-close state and actual bad-descriptor failures after interruption. */
int64_t multipart_fixture_fs_close(int64_t token) {
    begin_scratch(); struct scratch_handle *entry = get_scratch(token);
    if (!entry) { scratch_error = 108; return -1; }
    if (entry->closed) { scratch_error = 104; return -1; }
    entry->closed = 1;
    if (close(entry->fd) < 0) { scratch_error = errno == EBADF ? 106 : 108; return -1; }
    return 0;
}

/** Reopen the actual key as a new independent OS descriptor and logical handle. */
int64_t multipart_fixture_fs_open(int64_t key) {
    begin_scratch();
    if (key <= 0 || key > scratch_key_count) { scratch_error = 108; return 0; }
    int fd = open(scratch_keys[key].path, O_RDONLY);
    if (fd < 0) { scratch_error = errno == ENOENT ? 105 : 108; return 0; }
    return add_scratch_handle(fd, (int)key);
}

/** Unlink the actual scratch object; repeated missing-key attempts are successful no-ops. */
int64_t multipart_fixture_fs_remove(int64_t key) {
    begin_scratch();
    if (key <= 0 || key > scratch_key_count) { scratch_error = 108; return -1; }
    if (unlink(scratch_keys[key].path) == 0) { scratch_keys[key].live = 0; return 1; }
    if (errno == ENOENT) { scratch_keys[key].live = 0; return 0; }
    scratch_error = 108; return -1;
}

/** Check all library-owned resources before fixture teardown, without masking a leak. */
int64_t multipart_fixture_fs_finish(int64_t ignored) {
    (void)ignored;
    int leaked = 0;
    for (int at = 1; at <= scratch_handle_count; ++at) if (!scratch_handles[at].closed) ++leaked;
    for (int at = 1; at <= scratch_key_count; ++at) if (scratch_keys[at].live) ++leaked;
    if (leaked) return leaked;
    if (rmdir(scratch_directory) < 0) return -1;
    scratch_directory[0] = 0;
    return 0;
}

/** Exercise every byte/needle/lane through the unmodified emitted LLVM body. */
int32_t multipart_fixture_kernels(int64_t ignored) {
    (void)ignored;
    for (unsigned needle = 0; needle != 256; ++needle)
        for (unsigned byte = 0; byte != 256; ++byte)
            for (unsigned selected = 0; selected != 16; ++selected) {
                uint64_t low = 0, high = 0;
                uint32_t expected = 0;
                for (unsigned lane = 0; lane != 16; ++lane) {
                    uint8_t value = lane == selected ? (uint8_t)byte : (uint8_t)(needle + lane + 1);
                    if (value == needle) expected |= UINT32_C(1) << lane;
                    if (lane < 8) low |= (uint64_t)value << (lane * 8);
                    else high |= (uint64_t)value << ((lane - 8) * 8);
                }
                if (multipart_equal16(low, high, (uint8_t)needle) != expected) return 1;
            }
    return 0;
}

/** Retain a precise failed observation label without changing resource callback counts. */
int64_t multipart_fixture_checkpoint(int64_t label, int64_t matched) {
    if (!matched) fprintf(stderr, "multipart-fixture: checkpoint %lld failed\n", (long long)label);
    return matched;
}

/** Emit exact bytes only for a failed high-level fixture observation. */
int64_t multipart_fixture_diagnostic_bytes(int64_t label, int64_t address, int64_t length) {
    fprintf(stderr, "multipart-fixture: bytes %lld ", (long long)label);
    for (int64_t at = 0; at < length; ++at) fprintf(stderr, "%02x", ((const uint8_t *)(uintptr_t)address)[at]);
    fputc('\n', stderr);
    return 0;
}
