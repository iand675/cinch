#!/bin/sh
# Example-side schema introspection tool for 05-schema-generator.cinch.
#
# The schema probe's build inputs are the `SCHEMA_DATABASE_URL` secret
# and this tool (named by `CINCH_SCHEMA_TOOL`). `Stage.withDatabase`
# runs the tool over the subprocess capability with the credential in
# the tool's environment under its own name; the captured build input
# records the tool's output, never the credential. This stub emits a
# fixed billing schema document so the example checks without a live
# database. A real deployment points `CINCH_SCHEMA_TOOL` at a tool
# that introspects its database and prints the same document shape.
#
# Probe invocation (from the repository root):
#   SCHEMA_DATABASE_URL=postgresql://localhost/billing \
#   CINCH_SCHEMA_TOOL=./schema-tool.sh \
#   cargo run --manifest-path compiler/Cargo.toml -- \
#     check examples/source/05-schema-generator.cinch
#
# The tool path resolves against the example's directory (the stage
# working directory), so the example-side spelling is
# `CINCH_SCHEMA_TOOL=./schema-tool.sh`.

if [ -z "${SCHEMA_DATABASE_URL:-}" ]; then
  echo "schema-tool: SCHEMA_DATABASE_URL is not set in the tool environment" >&2
  exit 1
fi

cat <<'EOF'
{"dialect": "postgresql", "collation": "en_US.UTF-8", "snapshot": "0/16B3748", "extensions": ["pgcrypto"],
 "tables": [
   {"name": "billing_events", "columns": [
      {"name": "id", "type": "INTEGER", "nullable": false},
      {"name": "active", "type": "BOOLEAN", "nullable": false}]}]}
EOF
