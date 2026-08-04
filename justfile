#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

# run ry.dev python repl
repl:
    uv run python -m ry.dev

# dev run build + tests
dev: test

# cargo test
test:
    cargo test

# ci rust checks
ci:
    cargo fmt -- --check
    cargo clippy --all-targets -- -D warnings
    cargo test

# ===========================================================================
# FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT
# ===========================================================================

# cargo format
cargo-fmt:
    cargo +nightly fmt --all -- --unstable-features --config group_imports=StdExternalCrate,imports_granularity=Module,reorder_imports=true
    cargo fmt --all

# cargo format check
cargo-fmtc:
    cargo fmt --all -- --check

# justfile format
justfilefmt:
    just --fmt --unstable

# justfile format check
justfilefmtc:
    just --check --fmt --unstable

# format markdown
fmt-md:
    pnpm dlx prettier@latest --cache --prose-wrap=always -w '*.md'

# format markdown (check)
fmt-mdc:
    pnpm dlx prettier@latest --cache --prose-wrap=always --check '*.md'

# format
fmt: cargo-fmt justfilefmt fmt-md

# format check
fmtc: cargo-fmtc justfilefmtc fmt-mdc

# run clippy
clippy:
    cargo clippy --all-targets --features mimalloc -- -W warnings

# =====================================================================
# docs
# =====================================================================

# generate cargo docs for all crates (in workspace)
cargo-doc:
    cargo doc --no-deps --workspace
