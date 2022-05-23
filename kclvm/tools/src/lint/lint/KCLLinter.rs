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
use std::fs;

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
    pub path: String,
    pub file_list: IndexSet<String>,
    pub checkers: Vec<BaseChecker>,
    pub reporters: Vec<BaseReporter>,
    pub config: Config,
    pub msgs: IndexSet<Message>,
    pub MSGS: IndexMap<String, MSG>,
    pub msgs_map: HashMap<String, u32>,
}

impl Linter {
    pub fn new(paths: &String, config: Option<Config>) -> Self {
        let mut filelist: IndexSet<String> = IndexSet::new();
        let meta = std::fs::symlink_metadata(&paths);
        let file_type = meta.unwrap().file_type();
        if file_type.is_dir() {
            let ps = fs::read_dir(&paths).unwrap();
            for path in ps {
                if let Some(filepath) = path.unwrap().path().to_str() {
                    if filepath.ends_with(".k") {
                        filelist.insert(filepath.to_string());
                        println!("{}", filepath);
                    }
                }
            }
        } else {
            filelist.insert(paths.clone());
        }

        Self {
            path: paths.clone(),
            file_list: filelist,
            checkers: vec![],
            reporters: vec![],
            config: Linter::load_config(config),
            msgs: IndexSet::new(),
            MSGS: LINTER_MSGS.clone(),
            msgs_map: HashMap::new(),
        }
    }

    fn load_config(config: Option<Config>) -> Config {
        let mut cfg = Config::DEFAULT_CONFIG();
        match config {
            Some(config) => cfg.update(config),
            None => {}
        }
        cfg
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
            for (id, M) in MSGS {
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

    fn get_ctx(
        &self,
        file: &str,
    ) -> (
        String,
        Vec<String>,
        Program,
        ProgramScope,
        IndexSet<Diagnostic>,
    ) {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        let mut code_line_list: Vec<String> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            code_line_list.push(line.clone());
        }

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

    pub fn run(&mut self) {
        self.reset();
        self.register_checkers(vec![ImportCheck, MiscChecker]);
        self.register_reporters(vec![ReporterKind::Stdout]);
        for file in &self.file_list {
            let ctx = self.get_ctx(file);
            for c in &mut self.checkers {
                c.check(&ctx, &self.config);
                let msgs = c.get_msgs();
                // collect lint msgs
                for m in &msgs {
                    self.msgs.insert(m.clone());
                    let id = m.msg_id.clone();
                    match self.msgs_map.get_mut(&id) {
                        Some(v) => *v += 1 as u32,
                        None => {
                            self.msgs_map.insert(id, 1 as u32);
                        }
                    }
                }
            }
        }
        for r in &self.reporters {
            r.print_msg(&self);
        }
    }
}
