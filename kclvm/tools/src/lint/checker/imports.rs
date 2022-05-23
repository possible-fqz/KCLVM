use std::borrow::{Borrow, BorrowMut};
use std::path::Path;

use crate::lint::lint::config::Config;

use super::super::message::message::{Message, MSG};
use super::base_checker::{Check, CheckerKind};
use indexmap::{IndexMap, IndexSet};
use kclvm_ast::ast::{Module, Program};
use kclvm_ast::token::LitKind::Integer;
use kclvm_error::{Diagnostic, DiagnosticId, WarningKind};
use kclvm_error::{ErrorKind, Position};
use kclvm_sema::resolver::scope::ProgramScope;
use once_cell::sync::Lazy;
use rustc_span::source_map::FilePathMapping;

pub const IMPORT_POSITION_CHECK_LIST: [&str; 7] = [
    "AssignStmt",
    "AugAssignStmt",
    "AssertStmt",
    "IfStmt",
    "TypeAliasStmt",
    "SchemaStmt",
    "RuleStmt",
];

pub const IMPORT_MSGS: Lazy<IndexMap<String, MSG>> = Lazy::new(|| {
    let mut mapping = IndexMap::default();
    mapping.insert(
        String::from("E0401"),
        MSG {
            id: String::from("E0401"),
            short_info: String::from("Unable to import."),
            long_info: String::from("Unable to import {}."),
            sarif_info: String::from("Unable to import {0}."),
        },
    );
    mapping.insert(
        String::from("W0404"),
        MSG {
            id: String::from("W0404"),
            short_info: String::from("Module reimported."),
            long_info: String::from("{} is reimported multiple times."),
            sarif_info: String::from("{0} is reimported multiple times."),
        },
    );
    mapping.insert(
        String::from("W0411"),
        MSG {
            id: String::from("W0411"),
            short_info: String::from("Module imported but unused."),
            long_info: String::from("{} is imported but unused."),
            sarif_info: String::from("{0} is imported but unused."),
        },
    );
    mapping
});

#[derive(Debug, Clone)]
pub struct ImportChecker {
    kind: CheckerKind,
    MSGS: IndexMap<String, MSG>,
    msgs: IndexSet<Message>,
    code_lines: Option<Vec<String>>,
    prog: Option<Program>,
    scope: Option<ProgramScope>,
    diagnostics: Option<IndexSet<Diagnostic>>,
}

impl ImportChecker {
    pub fn new() -> Self {
        Self {
            kind: CheckerKind::ImportCheck,
            MSGS: IMPORT_MSGS.clone(),
            msgs: IndexSet::new(),
            prog: None,
            code_lines: None,
            scope: None,
            diagnostics: None,
        }
    }
    fn set_contex(
        &mut self,
        ctx: &(
            String,
            Vec<String>,
            Program,
            ProgramScope,
            IndexSet<Diagnostic>,
        ),
    ) {
        self.code_lines = Some(ctx.1.clone());
        self.prog = Some(ctx.2.clone());
        self.scope = Some(ctx.3.clone());
        self.diagnostics = Some(ctx.4.clone());
    }

    fn check_importstmt(&mut self, diagnostics: IndexSet<Diagnostic>) {
        for diagnostic in diagnostics {
            if let Some(code_lines) = &self.code_lines {
                if let Some(msg) = ImportChecker::diagnostic_to_msg(self, diagnostic) {
                    self.msgs.insert(msg);
                }
            }
        }
    }

    fn diagnostic_to_msg(&self, diag: Diagnostic) -> Option<Message> {
        let sm = rustc_span::SourceMap::new(FilePathMapping::empty());
        let filename = &diag.messages[0].pos.filename;
        let line = diag.messages[0].pos.line.clone() as usize - 1;
        let mut line_source = "".to_string();
        if let Ok(source_file) = sm.load_file(Path::new(&filename)) {
            if let Some(line) = source_file.get_line(line) {
                line_source = line.to_string();
            }
        }

        let mut msg: Option<Message> = None;
        let mut pos = diag.messages[0].pos.clone();
        pos.column = match pos.column {
            Some(col) => Some(col + 1),
            None => Some(1),
        };
        if let Some(id) = &diag.code {
            msg = match id {
                DiagnosticId::Error(kind) => match kind {
                    ErrorKind::CannotFindModule => Some(Message {
                        msg_id: "E0401".to_string(),
                        msg: diag.messages[0].message.clone(),
                        source_code: line_source,
                        pos: pos,
                        arguments: diag.args.clone(),
                    }),
                    _ => None,
                },
                DiagnosticId::Warning(kind) => match kind {
                    WarningKind::UnusedImportWarning => Some(Message {
                        msg_id: "W0411".to_string(),
                        msg: diag.messages[0].message.clone(),
                        source_code: line_source,
                        pos: pos,
                        arguments: diag.args.clone(),
                    }),
                    WarningKind::ReimportWarning => Some(Message {
                        msg_id: "W0404".to_string(),
                        msg: diag.messages[0].message.clone(),
                        source_code: line_source,
                        pos: pos,
                        arguments: diag.args.clone(),
                    }),
                },
            };
        }
        msg
    }
}

impl Check for ImportChecker {
    fn check(
        self: &mut ImportChecker,
        ctx: &(
            String,
            Vec<String>,
            Program,
            ProgramScope,
            IndexSet<Diagnostic>,
        ),
        cfg: &Config,
    ) {
        self.set_contex(ctx);
        if let Some(diagnostics) = &self.diagnostics {
            self.check_importstmt(diagnostics.clone());
        }
    }

    fn get_msgs(self: &ImportChecker) -> IndexSet<Message> {
        self.msgs.clone()
    }

    fn get_MSGS(self: &ImportChecker) -> IndexMap<String, MSG> {
        self.MSGS.clone()
    }

    fn get_kind(self: &ImportChecker) -> CheckerKind {
        self.kind.clone()
    }
}
