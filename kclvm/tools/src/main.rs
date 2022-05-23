use kclvm_parser::load_program;
use kclvm_sema::pre_process::pre_process_program;
use kclvm_sema::resolver::{resolve_program, scope::ProgramScope, Options, Resolver};
use kclvm_tools::lint::lint::KCLLinter::Linter;

use walkdir::{DirEntry, WalkDir};
// use self::glob::glob;

use std::fs;

fn is_kclfile(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".k"))
        .unwrap_or(false)
}
fn main() {



    let dir = "/Users/zz/code/kcl-lint-test/test_lint/test_checker";
    let dir1 = ".";
    let file = "/Users/zz/code/kcl-lint-test/test_lint/test_checker/test_data/import.k";
    let base = "/Users/zz/code/Konfig-ant/sigma/base";

    println!("-----------------------------");
    let mut lint = Linter::new(&dir.to_string(), None);


    println!("------");
    lint.run();
}
