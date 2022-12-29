use std::{fmt::Display, io::BufReader};

use crate::{Error, Parser, RbrepResult, CFG};

pub fn exec() -> RbrepResult<()> {
    // the tree to apply
    let expr = Expr::tree_from(&CFG.expr)?;

    expr.iter().for_each(|x| println!("{}", x));

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

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExprKind: [")?;
        match self {
            ExprKind::Byte { value } => write!(f, "[BYTE] value: {}", value),
            ExprKind::Any => write!(f, "[ANY]"),
            ExprKind::Group { nodes } => {
                write!(f, "[GROUP]")?;
                for node in nodes {
                    write!(f, "{},\n", node)?;
                }
                write!(f, "")
            }
            ExprKind::String { value } => write!(f, "[STRING] value: {}]", value),
            ExprKind::Range { from, to } => write!(f, "[RANGE] from: {}, to: {}]", from, to),
        }?;
        write!(f, "]")
    }
}

#[derive(Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub mul: u32,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[kind: {} mul: {}]", self.kind, self.mul)
    }
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
        while !parser.is_end() {
            branch.push(Self::parse(parser)?);
        }
        Ok(branch)
    }

    fn parse_byte_value(parser: &mut Parser) -> RbrepResult<u8> {
        let first = parser.next();
        let second = parser.next();
        u8::from_str_radix(&format!("{}{}", first, second), 16)
            .map_err(|_| Error::BadSyntax(parser.pos))
    }

    fn parse_byte_or_range(parser: &mut Parser) -> RbrepResult<Expr> {
        let value = Self::parse_byte_value(parser)?;
        if parser.next_if_trim('-') {
            let value2 = Self::parse_byte_value(parser)?;
            Ok(Expr::new(
                ExprKind::Range {
                    // TODO should we allow this behaviour?
                    from: value.min(value2),
                    to: value2.max(value),
                },
                0,
            ))
        } else {
            Ok(Expr::new(ExprKind::Byte { value }, 0))
        }
    }

    fn parse_any(parser: &mut Parser) -> RbrepResult<Expr> {
        if parser.next_if_trim('?') && parser.next_if_trim('?') {
            Ok(Expr::new(ExprKind::Any, 0))
        } else {
            Err(Error::BadSyntax(parser.pos))
        }
    }

    fn parse_mul(parser: &mut Parser, mut expr: Expr) -> RbrepResult<Expr> {
        // if not a mul return
        if !parser.next_if_trim('*') {
            return Ok(expr);
        }

        // now, get the slice of a numbers
        let num = parser.until(|x| x.is_digit(10));

        let num = u32::from_str_radix(num, 10).map_err(|_| Error::BadSyntax(parser.pos))?;

        expr.mul = num;

        // ; is required after mul
        if !parser.next_if_trim(';') {
            return Err(Error::BadSyntax(parser.pos));
        } else {
            Ok(expr)
        }
    }

    fn parse_group(parser: &mut Parser) -> RbrepResult<Expr> {
        if !parser.next_if_trim('(') {
            return Err(Error::BadSyntax(parser.pos));
        }

        let mut nodes = vec![];

        while !parser.next_if_trim(')') {
            if parser.is_end() {
                return Err(Error::BadSyntax(parser.pos));
            }
            nodes.push(Self::parse(parser)?);
        }

        Ok(Expr::new(ExprKind::Group { nodes }, 0))
    }

    fn parse(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.peek_trim();

        let expr = match first {
            '?' => Self::parse_any(parser),
            '(' => Self::parse_group(parser),
            _ => {
                if first.is_ascii_hexdigit() {
                    Self::parse_byte_or_range(parser)
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
