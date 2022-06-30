//! [kclvm_tools::format] module mainly contains some functions of language formatting,
//! the main API function is `format`, which accepts a path to be formatted and
//! formatted options.
//!
//! The basic principle is to call the [kclvm_parser::parse_file] function to parse the
//! AST Module, and then use the AST printer [kclvm_tools::printer::print_ast_module]
//! to print it as source code string.
use anyhow::{anyhow, Result};
use std::path::Path;

use crate::{printer::print_ast_module, util::get_kcl_files};
use kclvm_parser::parse_file;

#[cfg(test)]
mod tests;

/// FormatOptions contains two options:
/// - is_stdout: whether to output the formatted result to stdout.
/// - recursively: whether to recursively traverse a folder and format all KCL files in it.
#[derive(Debug, Default)]
pub struct FormatOptions {
    pub is_stdout: bool,
    pub recursively: bool,
}

/// Formats kcl file or directory path contains kcl files and
/// returns the changed file paths.
///
/// # Examples
///
/// ```no_run
/// use kclvm_tools::format::{format, FormatOptions};
///
/// // Format a single file.
/// format("path_to_a_single_file.k", &FormatOptions::default()).unwrap();
/// // Format a folder contains kcl files
/// format("path_to_a_folder", &FormatOptions::default()).unwrap();
/// ```
pub fn format<P: AsRef<Path>>(path: P, opts: &FormatOptions) -> Result<Vec<String>> {
    let mut changed_paths: Vec<String> = vec![];
    let path_ref = path.as_ref();
    if path_ref.is_dir() {
        for file in &get_kcl_files(path, opts.recursively)? {
            if format_file(file, opts)? {
                changed_paths.push(file.clone())
            }
        }
    } else if path_ref.is_file() {
        let file = path_ref.to_str().unwrap().to_string();
        if format_file(&file, opts)? {
            changed_paths.push(file)
        }
    }
    if !opts.is_stdout {
        let n = changed_paths.len();
        println!(
            "KCL format done and {} {} formatted:",
            n,
            if n <= 1 { "file was" } else { "files were" }
        );
        for p in &changed_paths {
            println!("{}", p);
        }
    }
    Ok(changed_paths)
}

/// Formats a file and returns whether the file has been formatted and modified.
fn format_file(file: &str, opts: &FormatOptions) -> Result<bool> {
    let src = std::fs::read_to_string(file)?;
    let (source, is_formatted) = format_source(&src)?;
    if opts.is_stdout {
        println!("{}", source);
    } else {
        std::fs::write(file, &source)?
    }
    Ok(is_formatted)
}

/// Formats a code source and returns the formatted source and
/// whether the source is changed.
fn format_source(src: &str) -> Result<(String, bool)> {
    let module = match parse_file("", Some(src.to_string())) {
        Ok(module) => module,
        Err(err) => return Err(anyhow!("{}", err)),
    };
    let formatted_src = print_ast_module(&module);
    let is_formatted = src != formatted_src;
    Ok((formatted_src, is_formatted))
}
