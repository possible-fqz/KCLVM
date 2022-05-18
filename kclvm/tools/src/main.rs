
use kclvm_tools::lint::lint::Linter::Linter;
use kclvm_sema::resolver::{resolve_program, scope::ProgramScope};
use kclvm_parser::parse_program;

fn main() {
    // for entry in WalkDir::new("/Users/zz/code/test/rusr_test/src") {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }
    let mut lint = Linter::new();
    let file = "/Users/zz/code/KCLVM-ant/test/integration/Konfig/sigma/base/samples/slo_configuration/single_error_budget_trigger/antmonitor/main.k";
    let mut prog = parse_program(file);
    let scope = resolve_program(&mut prog);
    println!("------");
    lint.run(&file);
}