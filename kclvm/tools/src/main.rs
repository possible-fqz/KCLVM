use kclvm_parser::load_program;
use kclvm_sema::pre_process::pre_process_program;
use kclvm_sema::resolver::{resolve_program, scope::ProgramScope, Options, Resolver};
use kclvm_tools::lint::lint::KCLLinter::Linter;

fn main() {
    // for entry in WalkDir::new("/Users/zz/code/test/rusr_test/src") {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }
    let mut lint = Linter::new();
    let file = "/Users/zz/code/kcl-lint-test/test_lint/test_checker/test_data/import.k";
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
    lint.run(&file);
}
