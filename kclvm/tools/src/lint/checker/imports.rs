use crate::lint::lint::config::Config;

use super::base_checker::BaseChecker;
use super::super::message::message::{Message, MSG};
use kclvm_ast::ast::{Program, Module};
use super::super::lint::Linter::Linter;
use super::base_checker::Check;


pub const IMPORT_POSITION_CHECK_LIST:[&str;7] = [ 
    "AssignStmt",
    "AugAssignStmt",
    "AssertStmt",
    "IfStmt",
    "TypeAliasStmt",
    "SchemaStmt",
    "RuleStmt",
];

// pub IMPORT_MSGS:Vec<MSG> = vec![
//     MSG{ 
//         id: "E0401", 
//         short_info: "Module reimported.", 
//         long_info: "{} is reimported multiple times.",
//     },
//     MSG{
//         id: "E0404", 
//         short_info: "Module reimported.", 
//         long_info: "{} is reimported multiple times.", 
//     },
// ];

pub const IMPORT_MSGS :[MSG; 2] = [
    MSG{ 
        id: "E0401", 
        short_info: "Module reimported.", 
        long_info: "{} is reimported multiple times.",
    },
    MSG{
        id: "E0404", 
        short_info: "Module reimported.", 
        long_info: "{} is reimported multiple times.", 
    },
];

pub struct ImportCheck<'c>{
    base_checker:  BaseChecker<'c>,
    prog: Option<Program>,
    module: Option<Module>,
    code: Option<String>,
    root: Option<String>,
    has_imported_module: Option<Vec<String>>,
    import_name_map: Option<Vec<String>>,
    import_position_check: bool
}

impl<'c> ImportCheck<'c>{
    pub fn new(option: &'c Config) -> Self{
        Self { 
            base_checker: BaseChecker::new(
                String::from("ImportCheck"), 
                IMPORT_MSGS.to_vec(),
                Vec::new(), 
                &option,
            ), 
            prog: None, 
            module: None, 
            code: None, 
            root: None, 
            has_imported_module: None, 
            import_name_map: None, 
            import_position_check: true
        }
    }
}
impl<'c> Check for ImportCheck<'c>{
    fn check(&self) -> &Config {
        print!("test");
        &self.base_checker.options
    }
}
