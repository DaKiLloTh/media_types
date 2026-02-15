# MIME Type Generation Tools

This directory contains documentation for the MIME type generation tools.

## generate_mime_types

**Location:** `examples/generate_mime_types.rs`

Auto-generates Rust modules containing MIME type constants from the authoritative [jshttp/mime-db](https://github.com/jshttp/mime-db) database.

### Usage

```bash
# Run the generator from the project root
cargo run --example generate_mime_types

# Or use the convenience script
./regenerate.sh
```

### What It Does

1. **Fetches** the latest MIME database from jshttp/mime-db (via CDN)
2. **Parses** all MIME types and categorizes them by top-level type
3. **Generates** Rust module files (e.g., `src/application.rs`, `src/audio.rs`, etc.)
4. **Documents** each constant with the MIME type and common file extensions

### Generated Output

Each MIME type becomes a constant like this:

```rust
/// `application/json`
///
/// Common file extensions: `.json`
pub const JSON: &str = "application/json";
```

### Setup

The generator is located in `examples/generate_mime_types.rs` and automatically has access to dev-dependencies.

No special Cargo.toml configuration needed - examples automatically work!

### Dependencies

The generator requires (as dev-dependencies):
- `reqwest` (with `blocking` feature) - for HTTP requests
- `serde` (with `derive` feature) - for JSON parsing

**Important:** These are dev-dependencies only. The library itself has ZERO dependencies!

### Benefits of Auto-Generation

- **Always up-to-date** with the official MIME database
- **Comprehensive coverage** of all registered MIME types
- **Consistent naming** and documentation
- **Zero manual maintenance** for new MIME types
- **File extension mappings** included automatically

### When to Regenerate

Regenerate the MIME type modules when:
- A new MIME type is standardized
- You want to update to the latest MIME database
- File extension mappings change
- Before major releases to ensure coverage

### Testing

After regenerating, run the comprehensive coverage test:

```bash
cargo test test_mime_types_against_external_database -- --ignored
```

This validates that generated types match the external database.

## Future Enhancements

Potential improvements to the generator:

- [ ] Generate additional metadata (compressibility, charset)
- [ ] Create extension-to-MIME-type reverse mapping
- [ ] Add deprecation warnings for obsolete types
