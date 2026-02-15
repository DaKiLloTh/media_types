# mediatypes

[![Crates.io](https://img.shields.io/crates/v/mediatypes.svg)](https://crates.io/crates/mediatypes)
[![Documentation](https://docs.rs/mediatypes/badge.svg)](https://docs.rs/mediatypes)
[![License](https://img.shields.io/crates/l/mediatypes.svg)](LICENSE)

A comprehensive collection of MIME types (media types) as string constants for Rust.

This crate provides an organized, hierarchical structure of all standard MIME types registered with IANA, making it easy to reference media types in a type-safe manner without hardcoding strings throughout your application.

## Features

- **Comprehensive**: Includes all major MIME types from the IANA registry
- **Zero dependencies**: No external dependencies required
- **Type-safe**: Constants prevent typos in MIME type strings
- **Well-organized**: Grouped by top-level media type for easy navigation
- **Fully documented**: Every constant includes the actual MIME type string
- **No unsafe code**: Built with `#![deny(unsafe_code)]`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mediatypes = "0.1.0"
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
let form_type = application::WWW_FORM_URLENCODED; // "application/x-www-form-urlencoded"
```

## Organization

MIME types are organized by their top-level type into separate modules:

- **`application`** - Application-specific data (JSON, XML, PDF, ZIP, etc.)
- **`audio`** - Audio formats (MP3, WAV, OGG, FLAC, etc.)
- **`font`** - Font formats (TTF, OTF, WOFF, WOFF2, etc.)
- **`image`** - Image formats (PNG, JPEG, GIF, SVG, WebP, etc.)
- **`message`** - Message protocols (RFC822, HTTP, SIP, etc.)
- **`model`** - 3D model formats (GLTF, OBJ, STL, etc.)
- **`multipart`** - Multi-part messages (form-data, mixed, etc.)
- **`text`** - Human-readable text (HTML, CSS, JavaScript, plain text, etc.)
- **`video`** - Video formats (MP4, WebM, MPEG, etc.)

## Examples

### Web Server Response

```rust
use mediatypes::{application, text};

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

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

MIME types are based on the [IANA Media Types registry](https://www.iana.org/assignments/media-types/media-types.xhtml).

## See Also

- [IANA Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml)
- [MDN - MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
- [RFC 6838 - Media Type Specifications and Registration Procedures](https://tools.ietf.org/html/rfc6838)







