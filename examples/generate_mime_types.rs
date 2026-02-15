//! MIME Type Auto-Generator
//!
//! This tool fetches the latest MIME types from the jshttp/mime-db repository
//! and generates Rust module files for the mediatypes crate.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin generate_mime_types
//! ```
//!
//! This will:
//! 1. Fetch the latest mime-db.json from GitHub
//! 2. Parse and categorize all MIME types
//! 3. Generate Rust modules (application.rs, audio.rs, etc.)
//! 4. Create proper documentation for each constant

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const MIME_DB_URL: &str = "https://cdn.jsdelivr.net/gh/jshttp/mime-db@master/db.json";

#[derive(Debug, Deserialize)]
struct MimeEntry {
    #[serde(default)]
    source: String,
    #[serde(default)]
    compressible: Option<bool>,
    #[serde(default)]
    extensions: Option<Vec<String>>,
    #[serde(default)]
    charset: Option<String>,
}

#[derive(Clone)]
struct MimeType {
    full_name: String,
    category: String,
    subtype: String,
    extensions: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Fetching MIME database from {}", MIME_DB_URL);

    let response = reqwest::blocking::get(MIME_DB_URL)?;
    let mime_db: HashMap<String, MimeEntry> = response.json()?;

    println!("✓ Fetched {} MIME types", mime_db.len());

    // Parse and categorize MIME types
    let mut categorized: HashMap<String, Vec<MimeType>> = HashMap::new();

    for (mime_type, entry) in mime_db {
        if let Some((category, subtype)) = mime_type.split_once('/') {
            let mime = MimeType {
                full_name: mime_type.clone(),
                category: category.to_string(),
                subtype: subtype.to_string(),
                extensions: entry.extensions.unwrap_or_default(),
            };

            categorized
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push(mime);
        }
    }

    println!("✓ Categorized into {} categories", categorized.len());

    // Generate Rust modules
    let src_dir = Path::new("src");

    for (category, mimes) in &categorized {
        // Convert category name to valid Rust module name (replace - with _)
        let module_name = category.replace('-', "_");

        // Sort by MIME type name for consistent output
        let mut sorted_mimes = mimes.clone();
        sorted_mimes.sort_by(|a, b| a.full_name.cmp(&b.full_name));

        let module_path = src_dir.join(format!("{}.rs", module_name));
        let mut file = File::create(&module_path)?;

        // Write module header
        writeln!(file, "//! {} MIME types", capitalize_category(category))?;
        writeln!(file, "//!")?;
        writeln!(
            file,
            "//! This module contains MIME types for {} data.",
            get_category_description(category)
        )?;
        writeln!(file)?;

        // Track used constant names to avoid duplicates
        let mut used_names: HashMap<String, usize> = HashMap::new();

        // Write constants
        for mime in sorted_mimes.iter() {
            let mut const_name = generate_constant_name(&mime.subtype);

            // Handle duplicate constant names
            if let Some(count) = used_names.get_mut(&const_name) {
                *count += 1;
                const_name = format!("{}_{}", const_name, count);
            } else {
                used_names.insert(const_name.clone(), 1);
            }

            // Write documentation
            writeln!(file, "/// `{}`", mime.full_name)?;
            if !mime.extensions.is_empty() {
                writeln!(file, "///")?;
                writeln!(
                    file,
                    "/// Common file extensions: {}",
                    mime.extensions
                        .iter()
                        .take(5)
                        .map(|s| format!("`.{}`", s))
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }

            // Write constant
            writeln!(
                file,
                "pub const {}: &str = \"{}\";",
                const_name, mime.full_name
            )?;
            writeln!(file)?;
        }

        println!(
            "✓ Generated {} with {} MIME types",
            module_path.display(),
            sorted_mimes.len()
        );
    }

    // Generate a helper module that exports all MIME types for testing
    generate_all_types_module(&src_dir, &categorized)?;

    println!("\n🎉 Successfully generated all MIME type modules!");
    println!("\n⚠️  Note: You may need to update lib.rs to include new modules.");

    Ok(())
}

/// Generate the all_types.rs module that exports all MIME types as a vector
fn generate_all_types_module(
    src_dir: &Path,
    categorized: &HashMap<String, Vec<MimeType>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let all_types_path = src_dir.join("all_types.rs");
    let mut file = File::create(&all_types_path)?;

    writeln!(
        file,
        "//! All MIME types exported as a vector for testing and validation"
    )?;
    writeln!(file, "//!")?;
    writeln!(
        file,
        "//! This module is automatically generated by the generate_mime_types tool."
    )?;
    writeln!(file, "//! Do not edit manually.")?;
    writeln!(file)?;
    writeln!(file, "#![allow(missing_docs)]")?;
    writeln!(file)?;

    // Count total types
    let mut total_count = 0;
    for (category, mimes) in categorized {
        if !category.starts_with("x-") {
            total_count += mimes.len();
        }
    }

    writeln!(file, "/// Get all MIME types as a vector of string slices")?;
    writeln!(file, "///")?;
    writeln!(
        file,
        "/// Returns a vector containing all {} MIME type constants defined in this crate.",
        total_count
    )?;
    writeln!(file, "pub fn all() -> Vec<&'static str> {{")?;
    writeln!(file, "    vec![")?;

    // Collect all types from all categories
    let mut all_entries: Vec<(String, String)> = Vec::new();

    for (category, mimes) in categorized {
        // Convert category name to valid Rust module name
        let module_name = category.replace('-', "_");

        let mut used_names: HashMap<String, usize> = HashMap::new();

        for mime in mimes.iter() {
            let mut const_name = generate_constant_name(&mime.subtype);

            if let Some(count) = used_names.get_mut(&const_name) {
                *count += 1;
                const_name = format!("{}_{}", const_name, count);
            } else {
                used_names.insert(const_name.clone(), 1);
            }

            all_entries.push((module_name.clone(), const_name));
        }
    }

    // Sort for consistent output
    all_entries.sort();

    // Write all entries
    for (category, const_name) in &all_entries {
        writeln!(file, "        crate::{}::{},", category, const_name)?;
    }

    writeln!(file, "    ]")?;
    writeln!(file, "}}")?;

    println!(
        "✓ Generated {} with {} MIME types",
        all_types_path.display(),
        total_count
    );

    Ok(())
}

/// Capitalize the first letter of a category name
fn capitalize_category(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Get a human-readable description for a category
fn get_category_description(category: &str) -> &str {
    match category {
        "application" => "application-specific",
        "audio" => "audio",
        "font" => "font",
        "image" => "image",
        "message" => "message protocol",
        "model" => "3D model",
        "multipart" => "multi-part",
        "text" => "text",
        "video" => "video",
        _ => category,
    }
}

/// Generate a Rust constant name from a MIME subtype
fn generate_constant_name(subtype: &str) -> String {
    // Replace special characters with meaningful names
    let mut result = String::new();
    let mut prev_was_underscore = false;
    let chars: Vec<char> = subtype.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                result.push(c.to_ascii_uppercase());
                prev_was_underscore = false;
            }
            '+' => {
                // Plus sign becomes _PLUS_ (or _PLUS if at end)
                if !result.is_empty() && !prev_was_underscore {
                    result.push('_');
                }
                result.push_str("PLUS");

                // Add trailing underscore if there are more non-separator chars after this
                let has_more_content = chars
                    .iter()
                    .skip(i + 1)
                    .any(|&ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9'));

                if has_more_content {
                    result.push('_');
                    prev_was_underscore = true;
                } else {
                    prev_was_underscore = false;
                }
            }
            '-' | '.' | '/' | '_' => {
                // Standard separators become underscore
                if !prev_was_underscore && !result.is_empty() {
                    result.push('_');
                    prev_was_underscore = true;
                }
            }
            _ => {
                // Other special characters become underscore
                if !prev_was_underscore && !result.is_empty() {
                    result.push('_');
                    prev_was_underscore = true;
                }
            }
        }
    }

    // Remove trailing underscore if any
    if result.ends_with('_') {
        result.pop();
    }

    // Handle special cases
    if result.is_empty() {
        result = "UNKNOWN".to_string();
    }

    // Ensure it doesn't start with a number
    if result.chars().next().unwrap().is_numeric() {
        result = format!("_{}", result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_constant_name() {
        assert_eq!(generate_constant_name("json"), "JSON");
        assert_eq!(generate_constant_name("atom+xml"), "ATOM_PLUS_XML");
        assert_eq!(generate_constant_name("vnd.ms-excel"), "VND_MS_EXCEL");
        assert_eq!(
            generate_constant_name("x-www-form-urlencoded"),
            "X_WWW_FORM_URLENCODED"
        );
        assert_eq!(generate_constant_name("problem+json"), "PROBLEM_PLUS_JSON");
        assert_eq!(generate_constant_name("amr-wb+"), "AMR_WB_PLUS");
        assert_eq!(generate_constant_name("x3d+binary"), "X3D_PLUS_BINARY");
    }

    #[test]
    fn test_capitalize_category() {
        assert_eq!(capitalize_category("application"), "Application");
        assert_eq!(capitalize_category("audio"), "Audio");
        assert_eq!(capitalize_category(""), "");
    }
}
