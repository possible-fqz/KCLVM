use super::super::message::message::{Message, MSG};
use kclvm_ast::ast::{Program, Module};
use super::base_checker::Check;
use once_cell::sync::Lazy;
use kclvm_error::Position;

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
            short_info: String::from("Module reimported."), 
            long_info: String::from("{} is reimported multiple times."),
        },
        MSG{
            id: String::from("E0404"),
            short_info: String::from("Module reimported."), 
            long_info: String::from("{} is reimported multiple times."), 
        },
    ]
});

pub struct ImportChecker{
    MSGS: Vec<MSG>,
    msgs: Vec<Message>,
    prog: Option<Program>,
    module: Option<Module>,
    code: Option<String>,
    root: Option<String>,
    has_imported_module: Option<Vec<String>>,
    import_name_map: Option<Vec<String>>,
    import_position_check: bool
}

impl ImportChecker{
    pub fn new() -> Self{
        Self {
            MSGS: IMPORT_MSGS.to_vec(),
            msgs: vec![],
            prog: None, 
            module: None, 
            code: None, 
            root: None, 
            has_imported_module: None, 
            import_name_map: None, 
            import_position_check: true, 
        }
    }
    fn get_contex(&mut self, code: String){
        self.code = Some(code);
    }
}
impl Check for ImportChecker{
    fn check(self: &mut ImportChecker){
        let m = Message { 
            msg_id: (String::from("123")), 
            msg: (String::from("123")), 
            source_code: (String::from("123")), 
            pos: Position { filename: (String::from("123")), 
            line: (1 as u64), 
            column: None}, 
            arguments: (vec![String::from("123")]), 
        };
        self.msgs.push(m);
    }

    fn get_msgs(self: &ImportChecker) -> Vec<Message>{
        let msgs = &self.msgs;
        msgs.to_vec()
    } 
}
