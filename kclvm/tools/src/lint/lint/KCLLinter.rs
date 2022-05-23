// KCLLinter class controls all inspection processes of lint: loading config, checking and generating reports.

// The workflow of KCLLinter is as follows:
// 1. Load config.
// 2. Find all KCL files under the 'path' from CLI arguments, and parse them to ast.Program.
// 3. Register checker and reporter according to config
// 4. Distribute ast to each checker for checking, and generate Message，which represents the result of check.
// 5. Linter collects Messages from all checkers.
// 6. Distribute Message to each reporter as output
// ┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                   KCLLinter                                                                 │
// │                                                                                                             │
// │      ┌───────────┐                  ┌─────────────────────────────────────────────────────────────────┐     │
// │      │  KCL file │                  │                             Checker                             │     │
// │      └───────────┘                  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │     │
// │            ↓                        │  │  importChecker  │  │  schemaChecker  │  │       ...       │  │     │
// │      ┌───────────┐                  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │      │  ast.Prog │       →          │  │  │  Message  │  │  │  │  Message  │  │  │  │  Message  │  │  │     │
// │      └───────────┘                  │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │                                     │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │                                     │  │  │  Message  │  │  │  │  Message  │  │  │  │  Message  │  │  │     │
// │                                     │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │      ┌──────────────────────┐       │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │      │      Config          │       │  │  │    ...    │  │  │  │    ...    │  │  │  │    ...    │  │  │     │
// │      │                      │       │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
// │      │   1 config           │       │  └─────────────────┘  └─────────────────┘  └─────────────────┘  │     │
// │      │   2 .kcllint         │       └─────────────────────────────────────────────────────────────────┘     │
// │      │   3 default_config   │                                                                               │
// │      │                      │                                        ↓                                      │
// │      │                      │       msgs_map -> MessageID: count                                            │
// │      └──────────────────────┘       msgs ->    ┌────────────────────────────────────────────────────┐       │
// │                                                │  ┌───────────┐  ┌───────────┐  ┌───────────┐       │       │
// │                                                │  │  Message  │  │  Message  │  │  Message  │       │       │
// │                                                │  └───────────┘  └───────────┘  └───────────┘       │       │
// │                                                └────────────────────────────────────────────────────┘       │
// │                                                                                                             │
// │                                                                      ↓                                      │
// │                                     ┌─────────────────────────────────────────────────────────────────┐     │
// │                                     │                              Reporter                           │     │
// │                                     │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐     │     │
// │                                     │  │  stdout   │  │   sarif   │  │   file    │  │   ...     │     │     │
// │                                     │  └───────────┘  └───────────┘  └───────────┘  └───────────┘     │     │
// │                                     └─────────────────────────────────────────────────────────────────┘     │
// │                                                                                                             │
// │                                                                                                             │
// │                                                                                                             │
// └─────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

use super::super::checker::base_checker::{
    BaseChecker, CheckerKind,
    CheckerKind::{ImportCheck, MiscChecker},
};
use super::config::Config;
use crate::lint::reporter::base_reporter::ReporterKind;
use crate::lint::{
    checker::imports::{ImportChecker, IMPORT_MSGS},
    message::message::{Message, MSG},
    reporter::base_reporter::BaseReporter,
};
use indexmap::{IndexMap, IndexSet};
use kclvm_ast::ast::Program;
use kclvm_error::Diagnostic;
use kclvm_parser::load_program;
use kclvm_sema::pre_process::pre_process_program;
use kclvm_sema::resolver::{scope::ProgramScope, Options, Resolver};
use rustc_span::source_map::FilePathMapping;
use std::{
    borrow::Borrow,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub const LINT_CONFIG_SUFFIX: &str = ".kcllint";
pub const PARSE_FAILED_MSG_ID: &str = "E0999";
use once_cell::sync::Lazy;

pub const LINTER_MSGS: Lazy<IndexMap<String, MSG>> = Lazy::new(|| {
    let mut mapping = IndexMap::default();
    mapping.insert(
        "E0999".to_string(),
        MSG {
            id: String::from("E0999"),
            short_info: String::from("Parse failed."),
            long_info: String::from("Parse failed: {}."),
            sarif_info: String::from("Parse failed: '{0}'."),
        },
    );
    mapping
});

pub struct Linter {
    path: Option<String>,
    file_list: IndexSet<String>,
    checkers: Vec<BaseChecker>,
    reporters: Vec<BaseReporter>,
    config: Config,
    msgs: IndexSet<Message>,
    MSGS: IndexMap<String, MSG>,
    msgs_map: HashMap<String, u32>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            path: None,
            file_list: IndexSet::new(),
            checkers: vec![],
            reporters: vec![],
            config: Config::DEFAULT_CONFIG(),
            msgs: IndexSet::new(),
            MSGS: LINTER_MSGS.clone(),
            msgs_map: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.reporters = vec![];
        self.checkers = vec![];
        self.MSGS = LINTER_MSGS.clone();
        self.msgs = IndexSet::new();
        self.msgs_map = HashMap::new();
    }

    fn register_checkers(&mut self, checkers: Vec<CheckerKind>) {
        for c in checkers {
            let checker = BaseChecker::new(c.clone());
            let MSGS = checker.get_MSGS();
            for (id, M) in MSGS{
                self.MSGS.insert(id, M);
            }
            self.checkers.push(checker);
        }
    }

    fn register_reporters(&mut self, reporters: Vec<ReporterKind>) {
        for r in reporters {
            let reporter = BaseReporter::new(r);
            self.reporters.push(reporter);
        }
    }

    fn get_ctx(&self, file: &str) -> (String ,Vec<String>, Program, ProgramScope, IndexSet<Diagnostic>) {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        let mut code_line_list: Vec<String> = vec![];
        for line in reader.lines() {
            // line 是 std::result::Result<std::string::String, std::io::Error> 类型
            // line 不包含换行符
            let line = line.unwrap();
            code_line_list.push(line.clone());
        }
        // let mut src_code: Vec<&str> = vec![];
        // let sm = rustc_span::SourceMap::new(FilePathMapping::empty());
        // if let Ok(source_file) = sm.load_file(Path::new(&file)) {
        //     if let Some(src) = source_file.src.clone() {
        //         src_code = src.split("\n").collect();
        //     }
        // }

        let file_list = vec![file];
        let mut prog = load_program(&file_list, None);
        // apply_overrides(&mut program, &args.overrides, &[]);
        pre_process_program(&mut prog);
        let mut resolver = Resolver::new(
            &prog,
            Options {
                raise_err: false,
                config_auto_fix: false,
            },
        );
        resolver.resolve_import();
        let scope = resolver.check(kclvm_ast::MAIN_PKG);
        resolver.handler.emit();
        let diagnostics = resolver.handler.diagnostics;
        (file.to_string(), code_line_list, prog, scope, diagnostics)
    }

    pub fn run(&mut self, file: &str) {
        self.register_checkers(vec![ImportCheck, MiscChecker]);
        self.register_reporters(vec![ReporterKind::Stdout]);
        let ctx = self.get_ctx(file);
        for c in &mut self.checkers {
            c.check(&ctx);
            let msgs = c.get_msgs();
            // collect lint error
            for m in &msgs {
                self.msgs.insert(m.clone());
                let id = m.msg_id.clone();
                match self.msgs_map.get_mut(&id){
                    Some(v) => {*v += 1 as u32},
                    None => {self.msgs_map.insert(id, 1 as u32);},

                }


                // if msg in self.msgs{
                //     self.msgs.append(msg);
                //     self.msgs_map[msg.msg_id] = (
                //         self.msgs_map.setdefault(msg.msg_id, 0) + 1
                //     )
                // }

            }
        }
        for r in &self.reporters {
            r.print_msg(&self.msgs, &self.msgs_map, self.MSGS.clone());
        }
    }
}
