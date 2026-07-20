use getset::Getters;

use crate::{
    Lambda, LambdaKind, UnzipExpressions,
    interpreter::InterpretingError,
    invocations::include::IncludeInvocation,
    types::{Node, VariableNode},
};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

#[derive(Debug, Getters)]
pub struct Module {
    name: String,
    file: File,
    filepath: PathBuf,
    assignments: HashMap<VariableNode, Node>,
    inclusions: Vec<IncludeInvocation>,
}

impl Module {
    pub fn new(filepath: PathBuf) -> Result<Module, InterpretingError> {
        let file = File::open(&filepath)?;
        Ok(Module {
            name: filepath
                .file_name()
                .unwrap_or_else(|| panic!("Failed to get file name for filepath: ${:?}!", filepath))
                .to_string_lossy()
                .to_string(),
            file,
            filepath,
            assignments: HashMap::new(),
            inclusions: Vec::new(),
        })
    }

    pub fn generate_inclusion_map(&mut self) -> Result<(), InterpretingError> {
        let content = fs::read_to_string(&self.filepath)?;

        for line in content.lines() {
            if line.contains("include!") {
                let include = IncludeInvocation::parse_include_statement(line)?;
                self.inclusions.push(include);
            }
        }

        Ok(())
    }

    pub fn generate_assignment_map(&mut self) -> Result<(), InterpretingError> {
        let content = fs::read_to_string(&self.filepath)?;
        let statements = Lambda::parse(content).unzip_expressions()?;

        statements.0.iter().for_each(|asgn| {
            let LambdaKind::Assignment {
                ref ident,
                ref body,
            } = asgn.kind
            else {
                unreachable!();
            };

            self.assignments.insert(ident.clone(), body.clone());
        });

        Ok(())
    }

    pub fn interpret(
        &mut self,
        global_assignments: &HashMap<VariableNode, Node>,
        global_modules: &mut HashMap<String, Module>,
    ) -> Result<&Module, InterpretingError> {
        todo!();
    }
}
