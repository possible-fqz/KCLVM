use super::super::checker::base_checker::CheckerKind;
pub struct Config {
    pub check_list: Vec<CheckerKind>,
    pub ignore: Vec<String>,
    pub max_line_length: usize,
    pub output: Vec<String>,
    pub output_path: Option<String>,
    pub module_naming_style: String,
    pub package_naming_style: String,
    pub schema_naming_style: String,
    pub mixin_naming_style: String,
    pub protocol_naming_style: String,
    pub argument_naming_style: String,
    pub variable_naming_style: String,
    pub schema_attribute_naming_style: String,
    pub module_rgx: Option<String>,
    pub package_rgx: Option<String>,
    pub schema_rgx: Option<String>,
    pub mixin_rgx: Option<String>,
    pub protocol_rgx: Option<String>,
    pub argument_rgx: Option<String>,
    pub variable_rgx: Option<String>,
    pub schema_attribute_rgx: Option<String>,
    pub bad_names: Vec<String>,
}
impl Config {
    pub fn DEFAULT_CONFIG() -> Config {
        Self {
            check_list: vec![
                CheckerKind::ImportCheck,
                CheckerKind::BasicChecker,
                CheckerKind::MiscChecker,
            ],
            ignore: vec![],
            max_line_length: 200,
            output: vec![String::from("stdout")],
            output_path: None,
            module_naming_style: String::from("ANY"),
            package_naming_style: String::from("ANY"),
            schema_naming_style: String::from("PascalCase"),
            mixin_naming_style: String::from("PascalCase"),
            protocol_naming_style: String::from("PascalCase"),
            argument_naming_style: String::from("camelCase"),
            variable_naming_style: String::from("ANY"),
            schema_attribute_naming_style: String::from("ANY"),
            module_rgx: None,
            package_rgx: None,
            schema_rgx: None,
            mixin_rgx: None,
            protocol_rgx: None,
            argument_rgx: None,
            variable_rgx: None,
            schema_attribute_rgx: None,
            bad_names: vec![
                String::from("foo"),
                String::from("bar"),
                String::from("baz"),
                String::from("toto"),
                String::from("tutu"),
                String::from("I"),
                String::from("l"),
                String::from("O"),
            ],
        }
    }
    pub fn update(&mut self, config: Config) {
        // todo
    }
}
