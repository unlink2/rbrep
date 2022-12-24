use std::io::Read;

use crate::{Error, RbrepResult};

pub enum Expr {
    Byte,
    Any,
    Group,
    String,
}

pub type ExprBranch = Vec<Expr>;

impl Expr {
    pub fn expr_tree_from(src: &str) -> RbrepResult<ExprBranch> {
        Err(Error::Unknown)
    }

    pub fn apply(&self, f: &mut dyn Read) -> RbrepResult<()> {
        Err(Error::Unknown)
    }
}
