use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::Pattern;
use crate::span::Span;

mod builder;

pub use builder::SymbolTableBuilder;

#[derive(Debug, Default)]
pub struct Variable {
    // TODO: Implement Types
    // type_: Type,
    span: Span,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub parent: Option<Box<SymbolTable>>,
    variables: HashMap<Pattern, Variable>,
    pub scope_idx: usize,
    scopes: Vec<RefCell<SymbolTable>>,
}

impl SymbolTable {
    pub fn is_shadowing(&self, ident: &Pattern) -> bool {
        if self.variables.contains_key(ident) {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            parent.is_shadowing(ident)
        } else {
            false
        }
    }

    pub fn lookup_variable(&self, ident: &Pattern) -> Option<&Variable> {
        if let Some(var) = self.variables.get(ident) {
            Some(var)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_variable(ident)
        } else {
            None
        }
    }

    pub fn lookup_scope(&self, idx: usize) -> Option<&RefCell<SymbolTable>> {
        self.scopes.get(idx)
    }

    pub fn insert_scope(&mut self) -> usize {
        self.scopes.push(Default::default());
        let idx = self.scope_idx;
        self.scope_idx += 1;
        idx
    }

    pub fn insert_variable(&mut self, ident: Pattern, variable: Variable) {
        self.variables.insert(ident, variable);
    }
}