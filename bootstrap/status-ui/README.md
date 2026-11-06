# Cinch bootstrap status

Private progress dashboard: https://cinch-bootstrap-status.ian92821.chatgpt.site

Buildless static site. Serve `dist/` to preview. Hosting identity is retained in
`.openai/hosting.json`; use the Sites workflow to publish source changes.

## Updating compiler progress

The hosted page reads `bootstrap/progress.json` from `iand675/cinch` on `main` every 30
seconds while visible. During compiler work, update that file and push it when a
stage, blocker, verification result, or current activity materially changes.
Refresh `updatedAt` only for a real status update. Keep `commit` pointing at the
compiler checkpoint described, rather than a status-only commit.

Do not publish an estimated completion percentage. The schema2 feed includes the
full revision-aware checklist: five acceptance requirements, named engineering
gaps, all preserved test inventories and active validations. Display measured
counts separately for each scope. Record which candidate and compiler role each
result verified. Preserve earlier results as observations; they do not verify
changed compiler inputs. Never mark failed evidence as passed. A completed run
still requires independent audit and publication before closing its repair.

The page retains the last good checkpoint on connection errors, marks status older
than six hours, and bundles an initial snapshot for a first visit without network
access. Refresh `dist/snapshot.js` and `dist/snapshot.json` when republishing the site.
Status changes alone do not require a site deployment.

Local previews read `/snapshot.json` so they work without GitHub access. Search,
status filters and expanded groups persist across automatic data refreshes.
The checklist is generated from preserved run catalogs and evidence receipts,
including exact runtime input equivalence; it must never shrink the original
test scope or substitute selected controls for complete acceptance.
