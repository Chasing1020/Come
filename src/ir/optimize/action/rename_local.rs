use std::fmt::Display;

use crate::ir::{quantity::Quantity, RegisterName};

use super::{Action, IsAction};
use crate::ir::function::statement::IsIRStatement;

#[derive(Debug, Clone)]
pub struct RenameLocal {
    pub from: RegisterName,
    pub to: Quantity,
}

impl RenameLocal {
    pub fn new(from: RegisterName, to: impl Into<Quantity>) -> Self {
        Self {
            from,
            to: to.into(),
        }
    }
}

impl Display for RenameLocal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rename `{}` to `{}`", self.from, self.to)
    }
}

impl IsAction for RenameLocal {
    fn perform(self, ir: &mut crate::ir::FunctionDefinition) {
        ir.iter_mut().for_each(|statement| {
            statement.on_register_change(&self.from, self.to.clone());
        });
    }

    fn affect_others<'a>(&self, _others: impl Iterator<Item = &'a mut Action>) {
        // nothing to do here
    }
}
