use crate::types::ParsingError;

#[derive(Debug, Clone)]
pub struct IncludeInvocation {
    included_items: Option<Vec<String>>,
    included_files: Vec<String>,
}

impl IncludeInvocation {
    pub fn parse_include_statement(stmt: &str) -> Result<IncludeInvocation, ParsingError> {
        let mut parts = stmt.trim().split(" ");
        let start = parts.next();

        if start.is_none_or(|s| s != "include!") {
            return Err(ParsingError::new(
                stmt,
                Some("include directive must have include!"),
                0..stmt.len(),
                None,
            ));
        }

        let included_files: Vec<_> = parts
            .by_ref()
            .take_while(|p| *p != "with")
            .map(|p| p.replace(|p: char| p.is_whitespace() || p == ',', ""))
            .collect();

        let included_items: Vec<_> = parts
            .map(|p| p.replace(|p: char| p.is_whitespace() || p == ',', ""))
            .collect();

        if included_files.is_empty() {
            return Err(ParsingError::new(
                stmt,
                Some("Must specify a file to include"),
                0..stmt.len(),
                None,
            ));
        }

        if included_files.len() > 1 && !included_items.is_empty() {
            return Err(ParsingError::new(
                stmt,
                Some("Cannot batch include files when using with clause."),
                0..stmt.len(),
                None,
            ));
        }

        Ok(IncludeInvocation {
            included_items: if included_items.is_empty() {
                None
            } else {
                Some(included_items)
            },
            included_files,
        })
    }
}
