use super::super::message::message::{Message, MSG};
use kclvm_ast::ast::{Program, Module};
use kclvm_sema::resolver::scope::ProgramScope;
use super::base_checker::Check;
use once_cell::sync::Lazy;
use kclvm_error::Position;
use super::base_checker::Checker;
use kclvm_error::Diagnostic;
use indexmap::{IndexSet, IndexMap};

pub const MISC_MSGS: Lazy<IndexMap<String, MSG>> = Lazy::new(|| {
    let mut mapping = IndexMap::default();
    mapping.insert(
        "E0501".to_string(),
        MSG{ 
            id: String::from("E0501"), 
            short_info: String::from("Line too long."), 
            long_info: String::from("Line too long ({} > {} characters)."),
            sarif_info: String::from("Line too long ('{0}' > '{1}' characters).")
        }
    );
    mapping
});




pub struct MiscChecker{
    kind: Checker,
    MSGS: IndexMap<String, MSG>,
    msgs: Vec<Message>,
    module: Option<Module>,
    code: Option<String>,
    prog: Option<Program>,
    scope: Option<ProgramScope>,
    diagnostic: Option<IndexSet<Diagnostic>>,
}

impl MiscChecker{
    pub fn new() -> Self{
        Self {
            kind: Checker::MiscChecker,
            MSGS: MISC_MSGS.clone(),
            msgs: vec![],
            module: None, 
            code: None,
            prog: None, 
            scope: None,
            diagnostic: None,
        }
    }

    fn set_contex(&mut self,  ctx: &(String, Program, ProgramScope, IndexSet<Diagnostic>)){
        self.code = Some(ctx.0.clone());
        self.prog = Some(ctx.1.clone());
        self.scope = Some(ctx.2.clone());
        self.diagnostic = Some(ctx.3.clone())
    }

    fn check_line_too_long(&mut self, code: Option<String>){
        if let Some(c) = code {
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
}

impl Check for MiscChecker{
    fn check(self: &mut MiscChecker, ctx: &(String, Program, ProgramScope, IndexSet<Diagnostic>)){
        let code = "123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123".to_string();
        self.set_contex(ctx);
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
