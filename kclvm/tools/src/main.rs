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
    // let files:Vec<Path> = glob("*").collect();
    // let walker = WalkDir::new("/Users/zz/code/kcl-lint-test/test_lint/test_checker/test_data").into_iter();
    // for entry in walker.filter_entry(|e| !is_kclfile(e)) {
    //     println!("{}", entry.path().display());
    // }
    println!("-----------------------------");
    let dir = "/Users/zz/code/kcl-lint-test/test_lint/test_checker/test_data";
    let dir1 = ".";
    let file = "/Users/zz/code/kcl-lint-test/test_lint/test_checker/test_data/import.k";

    let mut lint = Linter::new(&file.to_string(), None);
    
    // let filelist = vec![file];
    // let mut prog = load_program(&filelist, None);

    // let scope = resolve_program(&mut prog);

    // pre_process_program(&mut prog);
    // let mut resolver = Resolver::new(
    //     &prog,
    //     Options {
    //         raise_err: false,
    //         config_auto_fix: false,
    //     },
    // );
    // resolver.resolve_import();
    // let scope = resolver.check(kclvm_ast::MAIN_PKG);
    // resolver.handler.emit();

    println!("------");
    lint.run();
}
