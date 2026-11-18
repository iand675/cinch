"""Read exact completed buffered qualification bytes from plain files or verified recovery archives."""
from pathlib import Path
import hashlib
import json
import tarfile


def streamed_digest(stream):
    """Hash a closed artifact without extracting it or retaining its complete body in host memory."""
    result = hashlib.sha256()
    for chunk in iter(lambda: stream.read(1024*1024), b''):
        result.update(chunk)
    return result.hexdigest()


def file_digest(path):
    """Identify exact plain artifact or report bytes."""
    with path.open('rb') as stream:
        return streamed_digest(stream)


def artifact_digest(root, path):
    """Verify a qualified ELF/IR in place or stream its indexed archive member without restoring disk-heavy outputs."""
    root, path = Path(root).resolve(), Path(path).resolve()
    if path.is_file():
        return file_digest(path)
    relative = str(path.relative_to(root))
    for index_path in sorted((root/'build').glob('buffered-completed-functional-v*-index.json')):
        index = json.loads(index_path.read_text())
        entry = next((entry for entry in index['entries'] if entry['path'] == relative), None)
        if entry is None:
            continue
        assert index['deleted_originals'] is True
        archive_path = (root/index['archive']).resolve()
        assert archive_path.is_relative_to(root/'build')
        assert file_digest(archive_path) == index['compressed_sha256']
        assert file_digest(root/entry['report']) == entry['report_sha256']
        with tarfile.open(archive_path, 'r:gz') as archive:
            member = archive.getmember(relative)
            assert member.isfile() and member.size == entry['bytes'] and member.mode == entry['mode']
            stream = archive.extractfile(member)
            assert stream is not None
            with stream:
                observed = streamed_digest(stream)
        assert observed == entry['sha256']
        return observed
    raise FileNotFoundError('No plain or indexed archived qualification artifact: '+relative)
