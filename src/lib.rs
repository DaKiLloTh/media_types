//! # mediatypes
//!
//! A comprehensive collection of MIME types (media types) as string constants.
//!
//! This crate provides an organized, hierarchical structure of all standard MIME types
//! registered with IANA, making it easy to reference media types in a type-safe manner.
//!
//! ## Usage
//!
//! ```rust
//! use mediatypes::application::WWW_FORM_URLENCODED;
//! use mediatypes::image::PNG;
//! use mediatypes::text::HTML;
//!
//! assert_eq!(WWW_FORM_URLENCODED, "application/x-www-form-urlencoded");
//! assert_eq!(PNG, "image/png");
//! assert_eq!(HTML, "text/html");
//! ```
//!
//! ## Organization
//!
//! MIME types are organized by their top-level type:
//! - `application` - Application-specific data
//! - `audio` - Audio data
//! - `font` - Font data
//! - `image` - Image data
//! - `message` - Message protocol data
//! - `model` - 3D model data
//! - `multipart` - Multi-part data
//! - `text` - Human-readable text
//! - `video` - Video data

#![warn(missing_docs)]
#![deny(unsafe_code)]

/// Application-specific MIME types
pub mod application;
/// Audio MIME types
pub mod audio;
/// Font MIME types
pub mod font;
/// Image MIME types
pub mod image;
/// Message protocol MIME types
pub mod message;
/// 3D model MIME types
pub mod model;
/// Multi-part MIME types
pub mod multipart;
/// Text MIME types
pub mod text;
/// Video MIME types
pub mod video;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_types() {
        assert_eq!(application::JSON, "application/json");
        assert_eq!(application::XML, "application/xml");
        assert_eq!(application::PDF, "application/pdf");
    }

    #[test]
    fn test_text_types() {
        assert_eq!(text::PLAIN, "text/plain");
        assert_eq!(text::HTML, "text/html");
        assert_eq!(text::CSS, "text/css");
    }

    #[test]
    fn test_image_types() {
        assert_eq!(image::PNG, "image/png");
        assert_eq!(image::JPEG, "image/jpeg");
        assert_eq!(image::GIF, "image/gif");
    }

    #[test]
    fn test_video_types() {
        assert_eq!(video::MP4, "video/mp4");
        assert_eq!(video::WEBM, "video/webm");
    }

    #[test]
    fn test_audio_types() {
        assert_eq!(audio::MPEG, "audio/mpeg");
        assert_eq!(audio::WAV, "audio/wav");
    }
}
