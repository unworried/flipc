use std::collections::HashMap;
use std::fmt;

pub enum Error {
    UnknownToken(String),
    UnknownMacro(String),
    MacroEval(String, Box<Error>),
    BadMacroFormat,
    Unexpected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownToken(t) => write!(f, "unknown token: {}", t),
            Error::UnknownMacro(m) => write!(f, "unknown macro: {}", m),
            Error::MacroEval(name, e) => write!(f, "eval macro {}: {}", name, e),
            Error::BadMacroFormat => write!(f, "format <name> <value>"),
            Error::Unexpected(e) => write!(f, "{}", e),
        }
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        format!("{}", value)
    }
}

type MacroFn = fn(&mut PreProcessor, input: Vec<&str>) -> Result<Vec<String>, Error>;

pub struct PreProcessor {
    variables: HashMap<String, String>,
    macros: HashMap<String, MacroFn>,
}

impl PreProcessor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            macros: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, input: &str) -> Result<String, Error> {
        let parts: Vec<_> = input.split(' ').collect();
        if parts.is_empty() {
            return Ok(String::new());
        }

        if let Some(head) = parts.first() {
            match head.chars().nth(0) {
                Some(';') => return Ok(input.to_string()),
                Some('.') => {
                    let macro_name = &head[1..];
                    let func = self
                        .get_macro(macro_name)
                        .ok_or(Error::UnknownMacro(format!("macro: {}", head)))?;
                    let res = func(self, parts[1..].to_vec())
                        .map_err(|e| Error::MacroEval(macro_name.to_string(), Box::new(e)))?;
                    return Ok(res.join(" "));
                }
                _ => (),
            }
        }

        let resolved = parts.into_iter().map(|p| {
            match p.chars().nth(0) {
                Some('!') => self
                    .get_variable(&p[1..])
                    .ok_or(Error::UnknownToken((&p[1..]).to_string())),
                _ => Ok(p.to_string()),
            }
            /*if p.starts_with('!') {
                self.get_variable(&p[1..])
                    .ok_or(Error::Unknown(format!("token: {}", p)))
            } else {
                Ok(p.to_string())
            }*/
        });

        let st: Result<Vec<String>, Error> = resolved.collect();
        Ok(st?.join(" "))
    }

    fn get_variable(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }

    pub fn define_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    fn get_macro(&self, name: &str) -> Option<MacroFn> {
        self.macros.get(name).cloned()
    }

    pub fn define_macro(&mut self, name: &str, value: MacroFn) {
        self.macros.insert(name.to_string(), value);
    }
}
