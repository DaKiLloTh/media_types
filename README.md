# mediatypes

[![Crates.io](https://img.shields.io/crates/v/mediatypes.svg)](https://crates.io/crates/mediatypes)
[![Documentation](https://docs.rs/mediatypes/badge.svg)](https://docs.rs/mediatypes)
[![License](https://img.shields.io/crates/l/mediatypes.svg)](LICENSE)

A comprehensive collection of MIME types (media types) as string constants for Rust.

**NEW in v0.2.0:** Now includes 100% coverage (2,595 MIME types) auto-generated from the authoritative [jshttp/mime-db](https://github.com/jshttp/mime-db) database!

This crate provides an organized, hierarchical structure of all standard MIME types, making it easy to reference media types in a type-safe manner without hardcoding strings throughout your application.

## Features

- **100% Coverage**: All 2,595 MIME types from jshttp/mime-db
- **Zero Dependencies**: Absolutely no dependencies - just string constants!
- **Type-Safe**: Constants prevent typos in MIME type strings
- **Well-Organized**: Grouped by top-level media type for easy navigation
- **Fully Documented**: Every constant includes the MIME type and file extensions
- **No Unsafe Code**: Built with `#![deny(unsafe_code)]`
- **Auto-Generated**: Kept up-to-date with the latest MIME database

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mediatypes = "0.2.0"
```

## Usage

```rust
use mediatypes::application;
use mediatypes::image;
use mediatypes::text;
use mediatypes::video;

// Use in HTTP headers
let content_type = application::JSON; // "application/json"

// File upload handling
match file_extension {
    "png" => image::PNG,
    "jpg" | "jpeg" => image::JPEG,
    "gif" => image::GIF,
    _ => application::OCTET_STREAM,
}

// API responses
response.header("Content-Type", text::HTML);

// Form data
let form_type = application::X_WWW_FORM_URLENCODED; // "application/x-www-form-urlencoded"
```

## Organization

MIME types are organized by their top-level type into separate modules:

- **`application`** - Application-specific data (JSON, XML, PDF, ZIP, etc.) - 1,953 types
- **`audio`** - Audio formats (MP3, WAV, OGG, FLAC, etc.) - 188 types
- **`chemical`** - Chemical data and molecular structures - 7 types
- **`font`** - Font formats (TTF, OTF, WOFF, WOFF2) - 6 types
- **`image`** - Image formats (PNG, JPEG, GIF, SVG, WebP, etc.) - 109 types
- **`message`** - Message protocols (RFC822, HTTP, etc.) - 24 types
- **`model`** - 3D model formats (GLTF, OBJ, STL, etc.) - 43 types
- **`multipart`** - Multi-part messages (form-data, mixed, etc.) - 16 types
- **`text`** - Human-readable text (HTML, CSS, JavaScript, etc.) - 134 types
- **`video`** - Video formats (MP4, WebM, MPEG, etc.) - 112 types
- **`x_conference`** - Conference-related experimental types - 1 type
- **`x_shader`** - Shader-related experimental types - 2 types

## Examples

### Web Server Response

```rust
use mediatypes::{application, text, image};

fn serve_file(path: &str) -> Response {
    let content_type = match path.split('.').last() {
        Some("html") => text::HTML,
        Some("css") => text::CSS,
        Some("js") => application::JAVASCRIPT,
        Some("json") => application::JSON,
        Some("png") => image::PNG,
        _ => application::OCTET_STREAM,
    };
    
    Response::new()
        .header("Content-Type", content_type)
        .body(read_file(path))
}
```

### API Client

```rust
use mediatypes::application;

async fn post_json(url: &str, data: &str) -> Result<Response> {
    client.post(url)
        .header("Content-Type", application::JSON)
        .header("Accept", application::JSON)
        .body(data)
        .send()
        .await
}
```

### Form Handling

```rust
use mediatypes::{application, multipart};

fn handle_upload(content_type: &str, body: Vec<u8>) {
    match content_type {
        multipart::FORM_DATA => parse_multipart(body),
        application::WWW_FORM_URLENCODED => parse_urlencoded(body),
        _ => Err("Unsupported content type"),
    }
}
```

## Contributing

Contributions are welcome! If you notice any missing MIME types or have suggestions for improvements, please open an issue or submit a pull request.

## Changelog

### Version 0.2.0 (2026-02-15)

**Major Update: 100% Coverage with Auto-Generation**

- **Complete Coverage**: Now includes all 2,595 MIME types from jshttp/mime-db (was ~160 types)
- **Auto-Generated**: MIME types are automatically generated from authoritative source
- **Zero Dependencies**: Removed all dependencies - library is pure constants
- **New Modules**: Added `chemical`, `x_conference`, `x_shader` modules
- **Better Naming**: `+` symbol converts to `_PLUS_` for better readability
- **100% Validated**: All types validated against external database
- **File Extensions**: Documentation now includes common file extensions

**Breaking Changes:**
- `application::WWW_FORM_URLENCODED` → `application::X_WWW_FORM_URLENCODED`
- Many new MIME types added with `_PLUS_` suffix (e.g., `ATOM_PLUS_XML`)

### Version 0.1.0 (Initial Release)

- Basic MIME type constants for common types

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

MIME types are sourced from:
- [jshttp/mime-db](https://github.com/jshttp/mime-db) - The authoritative MIME type database
- [IANA Media Types registry](https://www.iana.org/assignments/media-types/media-types.xhtml)

Special thanks to the jshttp/mime-db project for maintaining the comprehensive MIME type database.

## See Also

- [jshttp/mime-db](https://github.com/jshttp/mime-db) - Source database
- [IANA Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml)
- [MDN - MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
- [RFC 6838 - Media Type Specifications and Registration Procedures](https://tools.ietf.org/html/rfc6838)
