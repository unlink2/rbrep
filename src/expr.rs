use std::{
    fmt::Display,
    io::{BufReader, Read, Write},
};

use crate::{Error, Parser, RbrepResult, CFG};

pub type ExprBranch = Vec<Expr>;

pub fn exec() -> anyhow::Result<()> {
    // the tree to apply
    let expr = Expr::tree_from(&CFG.expr)?;

    expr.iter().for_each(|x| println!("{}", x));

    // either use stdin, or match every file in the file list
    // TODO allow recursion for directories
    if CFG.paths.len() > 0 {
        // TODO open each file and apply parsed tree
        todo!("Not implemented")
    } else {
        Expr::apply(
            &expr,
            &mut BufReader::new(std::io::stdin()),
            &mut std::io::stdout(),
        )
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

impl ExprKind {
    pub fn len(&self) -> usize {
        match self {
            ExprKind::Group { nodes } => Expr::len(&nodes),
            ExprKind::String { value } => value.bytes().len(),
            _ => 1,
        }
    }

    pub fn is_match(&self, buffer: &[u8]) -> Option<usize> {
        None
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

    // calculates how many bytes this expression tree may match
    pub fn len(tree: &ExprBranch) -> usize {
        tree.iter()
            .fold(0, |i, n| i + n.kind.len() * n.mul as usize)
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
                1,
            ))
        } else {
            Ok(Expr::new(ExprKind::Byte { value }, 1))
        }
    }

    fn parse_any(parser: &mut Parser) -> RbrepResult<Expr> {
        if parser.next_if_trim('?') && parser.next_if_trim('?') {
            Ok(Expr::new(ExprKind::Any, 1))
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

        Ok(Expr::new(ExprKind::Group { nodes }, 1))
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

    pub fn is_match(&self, buffer: &[u8]) -> Option<usize> {
        let mut total = 0;
        for _ in 0..self.mul {
            total += self.kind.is_match(&buffer[total..])?
        }
        Some(total)
    }

    fn match_all(expr: &ExprBranch, buffer: &[u8]) -> anyhow::Result<bool> {
        let mut total = 0;
        for e in expr {
            match e.is_match(&buffer[total..]) {
                Some(amount) => total += amount,
                // no match => return
                _ => return Ok(false),
            }
        }
        // got to end without fail => match found!
        Ok(true)
    }

    // here we read the data and manage the buffer
    pub fn apply(expr: &ExprBranch, i: &mut dyn Read, o: &mut dyn Write) -> anyhow::Result<()> {
        let mut next = [0; 1];
        let mut buffer = vec![0; Expr::len(expr)];

        // read initial buffer
        let res = i.read_exact(&mut buffer);

        // if the result was an error of Eof
        // there can never be a match
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            _ => res?,
        }

        loop {
            // no matter what, we always advance a single byte
            // to check all possible combinations
            if Self::match_all(expr, &buffer)? {
                // TODO print current buffer if match
            }

            // remove fisrt
            buffer.remove(0);

            // read a new byte
            let res = i.read_exact(&mut next);
            // same here, if it is eof
            // we have simply reached the onf of the file!
            match res {
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                _ => res?,
            }

            // add next to vec
            buffer.push(next[0]);
        }

        Ok(())
    }
}
