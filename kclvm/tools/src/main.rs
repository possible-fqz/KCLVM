use walkdir::WalkDir;
use kclvm_tools::lint::lint::Linter::Linter;

fn main() {
    // for entry in WalkDir::new("/Users/zz/code/test/rusr_test/src") {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }
    let mut lint = Linter::new();
    let file = "/Users/zz/code/KCLVM-ant/hello.k";
    println!("{}", &file);
    lint.run(&file);
}