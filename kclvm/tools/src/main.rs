
use kclvm_tools::lint::lint::Linter::Linter;
use kclvm_sema::resolver::{resolve_program, scope::ProgramScope};
use kclvm_parser::load_program;

fn main() {
    // for entry in WalkDir::new("/Users/zz/code/test/rusr_test/src") {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }
    // let mut lint = Linter::new();
    let file = "/Users/zz/code/kcl-lint-test/a.k";
    let filelist = vec![file];
    let mut prog = load_program(&filelist, None);
    let scope = resolve_program(&mut prog);
    println!("------");
    // lint.run(&file);
}