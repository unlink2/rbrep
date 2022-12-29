use std::io::BufReader;

use crate::{Error, Parser, RbrepResult, CFG};

pub fn exec() -> RbrepResult<()> {
    // the tree to apply
    let expr = Expr::tree_from(&CFG.expr)?;

    // either use stdin, or match every file in the file list
    // TODO allow recursion for directories
    if CFG.paths.len() > 0 {
        // TODO open each file and apply parsed tree
        todo!("Not implemented")
    } else {
        Expr::apply(&expr, &mut BufReader::new(std::io::stdin()))
    }
}

#[derive(Clone)]
pub enum ExprKind {
    Byte { value: u8 },
    Any,
    Group { nodes: Vec<Expr> },
    String { value: String },
    Range { from: u8, to: u8 },
}

#[derive(Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub mul: u32,
}

impl Expr {
    pub fn new(kind: ExprKind, mul: u32) -> Self {
        Self { kind, mul }
    }
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

    fn parse_byte(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.next();
        let second = parser.next();
        let value = u8::from_str_radix(&format!("{}{}", first, second), 16)
            .map_err(|_| Error::BadSyntax(parser.pos))?;

        Ok(Expr::new(ExprKind::Byte { value }, 0))
    }

    fn parse_any(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.next();
        let second = parser.next();

        if first == '?' && second == '?' {
            Ok(Expr::new(ExprKind::Any, 0))
        } else {
            Err(Error::BadSyntax(parser.pos))
        }
    }

    fn parse_mul(parser: &mut Parser, mut expr: Expr) -> RbrepResult<Expr> {
        // if not a mul return
        if parser.peek() != '*' {
            return Ok(expr);
        }
        // skip the * character
        parser.next();

        // now, get the slice of a numbers
        let num = parser.until(|x| x.is_digit(10));

        let num = u32::from_str_radix(num, 10).map_err(|_| Error::BadSyntax(parser.pos))?;

        expr.mul = num;

        Ok(expr)
    }

    fn parse(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.peek();

        let expr = match first {
            '?' => Self::parse_any(parser),
            _ => {
                if first.is_ascii_hexdigit() {
                    Self::parse_byte(parser)
                } else {
                    Err(Error::BadSyntax(parser.pos))
                }
            }
        }?;

        Self::parse_mul(parser, expr)
    }

    pub fn apply<T>(expr: &ExprBranch, f: &mut BufReader<T>) -> RbrepResult<()>
    where
        T: std::io::Read,
    {
        Err(Error::Unknown)
    }
}
