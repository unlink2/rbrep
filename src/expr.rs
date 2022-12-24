use std::io::Read;

use crate::{Error, Parser, RbrepResult};

pub enum Expr {
    Byte { value: u8 },
    Any,
    Group,
    String,
}

pub type ExprBranch = Vec<Expr>;

impl Expr {
    pub fn tree_from(src: &str) -> RbrepResult<ExprBranch> {
        let mut parser = Parser::new(src);
        Self::tree_from_parser(&mut parser)
    }

    fn tree_from_parser(parser: &mut Parser) -> RbrepResult<ExprBranch> {
        let mut branch: ExprBranch = vec![];
        branch.push(Self::parse(parser)?);
        Ok(branch)
    }

    fn parse_byte(parser: &mut Parser, first: char) -> RbrepResult<Expr> {
        let second = parser.next();
        let value = u8::from_str_radix(&format!("{}{}", first, second), 16)
            .map_err(|_| Error::BadSyntax)?;

        Ok(Expr::Byte { value })
    }

    fn parse(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.next();

        if first.is_ascii_hexdigit() {
            Self::parse_byte(parser, first)
        } else {
            Err(Error::Unknown)
        }
    }

    pub fn apply(&self, f: &mut dyn Read) -> RbrepResult<()> {
        Err(Error::Unknown)
    }
}
