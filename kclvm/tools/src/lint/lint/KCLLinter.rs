// KCLLinter class controls all inspection processes of lint: loading config, checking and generating reports.

// The workflow of KCLLinter is as follows:
// 1. Load config.
// 2. Find all KCL files under the 'path' from CLI arguments, and get them context, i.e., source code, ast, scope, reslove diagnostic
// 3. Register checker and reporter according to config
// 4. Distribute context to each checker for checking, and generate Message，which represents the result of check.
// 5. Linter collects Messages from all checkers, and count the number.
// 6. Distribute Message to each reporter as output
// ┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
// │                                   KCLLinter                                                                 │
// │                                                                                                             │
// │      ┌───────────┐                  ┌─────────────────────────────────────────────────────────────────┐     │
// │      │  KCL file │                  │                             Checker                             │     │
// │      └───────────┘                  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │     │
// │            ↓                        │  │  importChecker  │  │  schemaChecker  │  │       ...       │  │     │
// │      ┌─────────------──┐            │  │  ┌───────────┐  │  │  ┌───────────┐  │  │  ┌───────────┐  │  │     │
// │      │  ast.Prog/scope │   →        │  │  │  Message  │  │  │  │  Message  │  │  │  │  Message  │  │  │     │
// │      └───────────------┘            │  │  └───────────┘  │  │  └───────────┘  │  │  └───────────┘  │  │     │
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
    message::message::{Message, MSG},
    reporter::base_reporter::BaseReporter,
};
use indexmap::{IndexMap, IndexSet};
use kclvm_ast::ast::Program;
use kclvm_error::Diagnostic;
use kclvm_parser::load_program;
use kclvm_sema::pre_process::pre_process_program;
use kclvm_sema::resolver::{scope::ProgramScope, Options, Resolver};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
pub const LINT_CONFIG_SUFFIX: &str = ".kcllint";
pub const PARSE_FAILED_MSG_ID: &str = "E0999";
use once_cell::sync::Lazy;
use walkdir::{WalkDir};

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
        let mut file_list: IndexSet<String> = IndexSet::new();
        for entry in WalkDir::new(paths) {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
            continue;
            }
            if entry.path().to_str().unwrap().ends_with(".k"){
                file_list.insert(entry.path().to_str().unwrap().to_string());
            }
        }

        Self {
            path: paths.clone(),
            file_list: file_list,
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
        file: &String,
    ) -> (
        String,
        Vec<String>,
        Program,
        ProgramScope,
        IndexSet<Diagnostic>,
    ) {
        println!("{}", file);
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        let mut code_line_list: Vec<String> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            code_line_list.push(line.clone());
        }

        //let file_list = vec![file];
        let mut prog = load_program(&vec![file.as_str()], None);
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
