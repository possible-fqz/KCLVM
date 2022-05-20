use super::super::message::message::{Message, MSG};
use indexmap::IndexSet;
use kclvm_ast::ast::{Program, Module};
use kclvm_sema::resolver::scope::ProgramScope;
use super::base_checker::{Check,Checker};
use once_cell::sync::Lazy;
use kclvm_error::Position;
use kclvm_error::Diagnostic;

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

// IMPORT_MSGS :[MSG; 2] = [
//     MSG{ 
//         id: String::from("E0401"), 
//         short_info: String::from("Module reimported."), 
//         long_info: String::from("{} is reimported multiple times."),
//     },
//     MSG{
//         id: String::from("E0404"),
//         short_info: String::from("Module reimported."), 
//         long_info: String::from("{} is reimported multiple times."), 
//     },
// ];
pub const IMPORT_MSGS: Lazy<Vec<MSG>> = Lazy::new(|| {
    vec![
        MSG{ 
            id: String::from("E0401"), 
            short_info: String::from("Unable to import."), 
            long_info: String::from("Unable to import {}."),
            sarif_info: String::from("Unable to import {0}."),
        },
        MSG{
            id: String::from("E0404"),
            short_info: String::from("Module reimported."), 
            long_info: String::from("{} is reimported multiple times."), 
            sarif_info: String::from("{} is reimported multiple times."),
        },
    ]
});

pub struct ImportChecker{
    kind: Checker,
    MSGS: Vec<MSG>,
    msgs: Vec<Message>,
    code: Option<Vec<String>>,
    prog: Option<Program>,
    scope: Option<ProgramScope>,
    diagnostic: Option<IndexSet<Diagnostic>>,
}

impl ImportChecker{
    pub fn new() -> Self{
        Self {
            kind: Checker::ImportCheck,
            MSGS: IMPORT_MSGS.to_vec(),
            msgs: vec![],
            prog: None, 
            code: None, 
            scope: None,
            diagnostic: None,
        }
    }
    fn set_contex(&mut self, ctx: &(Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>)){
        self.code = Some(ctx.0.clone());
        self.prog = Some(ctx.1.clone());
        self.scope= Some(ctx.2.clone());
        self.diagnostic = Some(ctx.3.clone());
    }
}

impl Check for ImportChecker{
    fn check(self: &mut ImportChecker, ctx: &(Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>)){
        self.set_contex(ctx);

    }

    fn get_msgs(self: &ImportChecker) -> Vec<Message>{
        let msgs = &self.msgs;
        msgs.to_vec()
    }
    
    fn get_kind(self: &ImportChecker) -> Checker{
        let kind = self.kind.clone();
        kind
    }
}
