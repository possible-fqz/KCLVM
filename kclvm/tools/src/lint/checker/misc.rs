use super::super::message::message::{Message, MSG};
use kclvm_ast::ast::{Program, Module};
use super::base_checker::Check;
use once_cell::sync::Lazy;
use kclvm_error::Position;
use super::base_checker::Checker;

pub const IMPORT_MSGS: Lazy<Vec<MSG>> = Lazy::new(|| {
    vec![
        MSG{ 
            id: String::from("E0501"), 
            short_info: String::from("Line too long."), 
            long_info: String::from("line too long ('{}' > '{}' characters)."),
        },
    ]
});

pub struct MiscChecker{
    kind: Checker,
    MSGS: Vec<MSG>,
    msgs: Vec<Message>,
    prog: Option<Program>,
    module: Option<Module>,
    code: Option<String>,
}

impl MiscChecker{
    pub fn new() -> Self{
        Self {
            kind: Checker::MiscChecker,
            MSGS: IMPORT_MSGS.to_vec(),
            msgs: vec![],
            prog: None, 
            module: None, 
            code: None,
        }
    }
    fn get_contex(&mut self, code: Option<String>){
        self.code = code;
    }
    fn check_line_too_long(&mut self, code: Option<String>){
        let c = match code{
            Some(c) => c,
            None => "".to_string(),
        };
        let code_lines: Vec<&str> = c.split("\n").collect();
        let max_line_length = 50;
        for (i, v) in code_lines.iter().enumerate(){
            if v.len() > max_line_length{
                let filename = match &self.module{
                    Some(m) => m.filename.clone(),
                    None => "".to_string(),
                };
                self.msgs.push(Message { 
                    msg_id: String::from("E0501"), 
                    msg: String::from("Line too long."), 
                    source_code: v.to_string(), 
                    pos: Position { 
                        filename: filename, 
                        line: (i + 1) as u64, 
                        column: Some(1) }, 
                    arguments: (vec![v.len().to_string(), max_line_length.to_string()]) 
                })
            }
        }
    }
}

impl Check for MiscChecker{
    fn check(self: &mut MiscChecker){
        let code = "123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123".to_string();
        self.get_contex(Some(code));
        self.check_line_too_long(self.code.clone())
    }

    fn get_msgs(self: &MiscChecker) -> Vec<Message>{
        let msgs = &self.msgs;
        msgs.to_vec()
    } 
    fn get_kind(self: &MiscChecker) -> Checker{
        let kind = self.kind.clone();
        kind
    }
}
