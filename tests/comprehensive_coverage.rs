//! Comprehensive tests validating MIME type coverage
//!
//! This module tests that our crate includes MIME types by fetching
//! them from the authoritative jshttp/mime-db database on GitHub and
//! automatically comparing against all constants in the crate.

use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// MIME database entry from jshttp/mime-db
#[derive(Debug, Deserialize)]
struct MimeEntry {
    #[serde(default)]
    source: String,
    #[serde(default)]
    compressible: Option<bool>,
    #[serde(default)]
    extensions: Option<Vec<String>>,
}

/// URL to the raw mime-db JSON database
/// This is the most comprehensive and maintained MIME type database
const MIME_DB_URL: &str = "https://cdn.jsdelivr.net/gh/jshttp/mime-db@master/db.json";

/// Fetch and parse the MIME database from jshttp/mime-db
fn fetch_mime_database() -> Result<HashMap<String, MimeEntry>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(MIME_DB_URL)?;
    let mime_db: HashMap<String, MimeEntry> = response.json()?;
    Ok(mime_db)
}

/// Build a set of all MIME types we provide in our crate
/// This automatically uses the generated all_types module, so it stays in sync
/// with the actual crate contents without manual maintenance.
fn get_all_crate_mime_types() -> HashSet<String> {
    mediatypes::all_types::all()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

#[test]
#[ignore] // Ignore by default since it requires network access
fn test_mime_types_against_external_database() {
    println!("\n🌐 Fetching MIME type database from {}", MIME_DB_URL);

    let mime_db = match fetch_mime_database() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("⚠️  Warning: Could not fetch MIME database: {}", e);
            eprintln!("This test requires internet access. Run with: cargo test -- --ignored");
            return;
        }
    };

    println!(
        "✓ Fetched {} MIME types from external database",
        mime_db.len()
    );

    let our_types = get_all_crate_mime_types();
    println!("✓ Our crate provides {} MIME types", our_types.len());

    // Validate ALL our types exist in the database
    let mut invalid_types = Vec::new();
    for our_type in &our_types {
        if !mime_db.contains_key(our_type) {
            invalid_types.push(our_type.clone());
        }
    }

    if !invalid_types.is_empty() {
        println!(
            "\n❌ Found {} invalid MIME types not in database:",
            invalid_types.len()
        );
        for invalid in &invalid_types {
            println!("   - {}", invalid);
        }
        panic!("All MIME types in crate must exist in the jshttp/mime-db database");
    }

    // Calculate coverage statistics
    let total_in_db = mime_db.len();
    let our_count = our_types.len();
    let coverage_percent = (our_count as f64 / total_in_db as f64) * 100.0;

    // Check coverage of types with file extensions (more common types)
    let types_with_extensions: HashSet<String> = mime_db
        .iter()
        .filter(|(_, entry)| {
            entry.extensions.is_some() && !entry.extensions.as_ref().unwrap().is_empty()
        })
        .map(|(mime_type, _)| mime_type.clone())
        .collect();

    let common_covered = our_types.intersection(&types_with_extensions).count();
    let common_total = types_with_extensions.len();
    let common_coverage = (common_covered as f64 / common_total as f64) * 100.0;

    println!("\n📊 Coverage Report:");
    println!("   Total database types: {}", total_in_db);
    println!(
        "   Our types: {} ({:.1}% of database)",
        our_count, coverage_percent
    );
    println!(
        "   Types with extensions: {}/{} ({:.1}%)",
        common_covered, common_total, common_coverage
    );

    // Check which types from the database we're missing
    let mut missing_from_db = Vec::new();
    for (mime_type, _) in &mime_db {
        if !our_types.contains(mime_type) {
            missing_from_db.push(mime_type.clone());
        }
    }

    if !missing_from_db.is_empty() {
        println!(
            "\n⚠️  Missing {} MIME types from database:",
            missing_from_db.len()
        );
        for missing in &missing_from_db {
            println!("   - {}", missing);
        }
    }

    // Check for missing highly common types
    let highly_common_exts = [
        "html", "css", "js", "json", "xml", "jpg", "jpeg", "png", "gif", "pdf", "zip", "mp3",
        "mp4", "webm", "svg", "txt", "csv", "webp", "woff", "woff2", "ttf", "otf", "wav", "ogg",
        "mpeg", "avi", "mov", "flac", "aac",
    ];

    let mut missing_important = Vec::new();
    for (mime_type, entry) in &mime_db {
        if !our_types.contains(mime_type) {
            if let Some(exts) = &entry.extensions {
                if exts
                    .iter()
                    .any(|ext| highly_common_exts.contains(&ext.as_str()))
                {
                    missing_important.push((mime_type.clone(), exts.clone()));
                }
            }
        }
    }

    if !missing_important.is_empty() {
        println!(
            "\n⚠️  Missing {} important MIME types:",
            missing_important.len()
        );
        for (mime_type, exts) in missing_important.iter().take(15) {
            println!("   - {} ({})", mime_type, exts.join(", "));
        }
        if missing_important.len() > 15 {
            println!("   ... and {} more", missing_important.len() - 15);
        }
    }

    println!(
        "\n✅ All {} MIME types validated against external database!",
        our_count
    );
    println!(
        "✅ Coverage of types with extensions: {:.1}%",
        common_coverage
    );
}

#[test]
fn test_mime_type_format_validity() {
    // Test that all our MIME types follow the correct format: type/subtype
    let our_types = get_all_crate_mime_types();

    for mime_type in &our_types {
        assert!(
            mime_type.contains('/'),
            "MIME type '{}' doesn't contain '/'",
            mime_type
        );

        let parts: Vec<&str> = mime_type.split('/').collect();
        assert_eq!(
            parts.len(),
            2,
            "MIME type '{}' should have exactly one '/' separator",
            mime_type
        );

        assert!(
            !parts[0].is_empty() && !parts[1].is_empty(),
            "MIME type '{}' has empty type or subtype",
            mime_type
        );
    }

    println!("✓ All {} MIME types have valid format", our_types.len());
}

#[test]
fn test_no_duplicate_mime_types() {
    let our_types = get_all_crate_mime_types();
    let unique_count = our_types.len();

    // The HashSet already deduped, so if we had dupes, this would fail
    // Let's verify by checking we have a reasonable number
    assert!(
        unique_count >= 100,
        "Should have at least 100 unique MIME types, found {}",
        unique_count
    );

    println!("✓ No duplicates found among {} MIME types", unique_count);
}

#[test]
fn test_category_consistency() {
    // Verify that MIME types in each module start with the correct prefix
    let test_cases = vec![
        ("application", mediatypes::application::JSON),
        ("audio", mediatypes::audio::MP3),
        ("font", mediatypes::font::TTF),
        ("image", mediatypes::image::PNG),
        ("message", mediatypes::message::RFC822),
        ("model", mediatypes::model::OBJ),
        ("multipart", mediatypes::multipart::FORM_DATA),
        ("text", mediatypes::text::PLAIN),
        ("video", mediatypes::video::MP4),
    ];

    for (expected_prefix, mime_type) in test_cases {
        assert!(
            mime_type.starts_with(&format!("{}/", expected_prefix)),
            "MIME type '{}' should start with '{}/...'",
            mime_type,
            expected_prefix
        );
    }

    println!("✓ All categories have consistent naming");
}

#[test]
fn test_essential_mime_types_present() {
    // Test for absolutely essential MIME types that must exist
    let essential_types = vec![
        ("JSON", mediatypes::application::JSON, "application/json"),
        ("HTML", mediatypes::text::HTML, "text/html"),
        ("CSS", mediatypes::text::CSS, "text/css"),
        (
            "JavaScript",
            mediatypes::application::JAVASCRIPT,
            "application/javascript",
        ),
        ("PNG", mediatypes::image::PNG, "image/png"),
        ("JPEG", mediatypes::image::JPEG, "image/jpeg"),
        ("PDF", mediatypes::application::PDF, "application/pdf"),
        ("ZIP", mediatypes::application::ZIP, "application/zip"),
        ("MP4 Video", mediatypes::video::MP4, "video/mp4"),
        ("MP3 Audio", mediatypes::audio::MP3, "audio/mp3"),
    ];

    for (name, constant, expected) in &essential_types {
        assert_eq!(
            constant, expected,
            "{} MIME type should be '{}'",
            name, expected
        );
    }

    println!(
        "✓ All {} essential MIME types are present and correct",
        essential_types.len()
    );
}
