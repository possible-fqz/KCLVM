use super::super::message::message::{Message, MSG};
use super::base_checker::Check;
use super::base_checker::CheckerKind;
use indexmap::{IndexMap, IndexSet};
use kclvm_ast::ast::{Module, Program};
use kclvm_error::Diagnostic;
use kclvm_error::Position;
use kclvm_sema::resolver::scope::ProgramScope;
use once_cell::sync::Lazy;
use std::{fs::File, io::BufReader};

pub const MISC_MSGS: Lazy<IndexMap<String, MSG>> = Lazy::new(|| {
    let mut mapping = IndexMap::default();
    mapping.insert(
        String::from("E0501"),
        MSG {
            id: String::from("E0501"),
            short_info: String::from("Line too long."),
            long_info: String::from("Line too long ({} > {} characters)."),
            sarif_info: String::from("Line too long ('{0}' > '{1}' characters)."),
        },
    );
    mapping
});

#[derive(Debug, Clone)]
pub struct MiscChecker {
    kind: CheckerKind,
    MSGS: IndexMap<String, MSG>,
    msgs: IndexSet<Message>,
    file: Option<String>,
    module: Option<Module>,
    code_lines: Option<Vec<String>>,
    prog: Option<Program>,
    scope: Option<ProgramScope>,
    diagnostics: Option<IndexSet<Diagnostic>>,
}

impl MiscChecker {
    pub fn new() -> Self {
        Self {
            kind: CheckerKind::MiscChecker,
            MSGS: MISC_MSGS.clone(),
            msgs: IndexSet::new(),
            file: None,
            module: None,
            code_lines: None,
            prog: None,
            scope: None,
            diagnostics: None,
        }
    }

    fn set_contex(&mut self, ctx: &(String, Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>)) {
        self.file = Some(ctx.0.clone());
        self.code_lines = Some(ctx.1.clone());
        self.prog = Some(ctx.2.clone());
        self.scope = Some(ctx.3.clone());
        self.diagnostics = Some(ctx.4.clone());
    }

    fn check_line_too_long(&mut self, filename: String, code_lines: Vec<String>) {
        // let code_lines: Vec<&str> = code.split("\n").collect();
        let max_line_length = 50;
        for (i, code) in code_lines.iter().enumerate() {
            if code.len() > max_line_length {
                self.msgs.insert(Message {
                    msg_id: String::from("E0501"),
                    msg: format!(
                        "Line too long ({} > {} characters).",
                        code.len(),
                        max_line_length
                    ),
                    source_code: code.to_string(),
                    pos: Position {
                        filename: filename.clone(),
                        line: (i + 1) as u64,
                        column: Some(code.len() as u64),
                    },
                    arguments: (vec![code.len().to_string(), max_line_length.to_string()]),
                });
            }
        }
    }
}

impl Check for MiscChecker {
    fn check(
        self: &mut MiscChecker,
        ctx: &(String, Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>),
    ) {
        self.set_contex(ctx);
        let f = match &self.file {
            Some(f) => f.clone(),
            _ => "".to_string(),
        };
        let code_line = match &self.code_lines {
            Some(codes) => codes.clone(),
            _ => vec!["".to_string()],
        };
        self.check_line_too_long(f, code_line)
    }

    fn get_msgs(self: &MiscChecker) -> IndexSet<Message> {
        let msgs = &self.msgs;
        msgs.clone()
    }

    fn get_MSGS(self: &MiscChecker) -> IndexMap<String, MSG> {
        self.MSGS.clone()
    }

    fn get_kind(self: &MiscChecker) -> CheckerKind {
        let kind = self.kind.clone();
        kind
    }
}
