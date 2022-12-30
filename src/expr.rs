use console::style;
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::{Error, Parser, RbrepResult, CFG};

pub type ExprBranch = Vec<Expr>;

pub fn exec() -> anyhow::Result<()> {
    // the tree to apply
    let expr = Expr::tree_from(&CFG.expr)?;

    if CFG.dbg_expr_tree {
        expr.iter().for_each(|x| println!("{x}"));
    }

    // either use stdin, or match every file in the file list
    // TODO allow recursion for directories
    if !CFG.paths.is_empty() {
        // open each file and apply parsed tree
        for path in &CFG.paths {
            let f = File::open(path)?;
            Expr::apply(&expr, &mut BufReader::new(f), &mut std::io::stdout(), path)?
        }
        Ok(())
    } else {
        Expr::apply(
            &expr,
            &mut BufReader::new(std::io::stdin()),
            &mut std::io::stdout(),
            "stdin",
        )
    }
}

#[derive(Clone)]
pub enum ExprKind {
    // a single byte value
    Byte { value: u8 },
    // any string
    Any,
    // OR combination of expressions
    Group { nodes: Vec<Expr> },
    // a full string
    String { value: String },
    // a range from..to
    Range { from: u8, to: u8 },
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExprKind: [")?;
        match self {
            ExprKind::Byte { value } => write!(f, "[BYTE] value: {value}"),
            ExprKind::Any => write!(f, "[ANY]"),
            ExprKind::Group { nodes } => {
                write!(f, "[GROUP]")?;
                for node in nodes {
                    writeln!(f, "{node},")?;
                }
                write!(f, "")
            }
            ExprKind::String { value } => write!(f, "[STRING] value: {value}]"),
            ExprKind::Range { from, to } => write!(f, "[RANGE] from: {from}, to: {to}]"),
        }?;
        write!(f, "]")
    }
}

impl ExprKind {
    pub fn len(&self) -> usize {
        match self {
            ExprKind::Group { nodes } => nodes.iter().fold(0, |i, n| i.max(n.kind.len())),
            ExprKind::String { value } => value.bytes().len(),
            _ => Expr::single_len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_match<F>(&self, buffer: &[u8], f: &mut F) -> Option<usize>
    where
        F: FnMut(ExprOutput),
    {
        let first = buffer.first()?;
        match self {
            ExprKind::Byte { value } => {
                if first == value {
                    f(ExprOutput::new(*first, true));
                    Some(self.len())
                } else {
                    None
                }
            }
            ExprKind::Any => {
                f(ExprOutput::new(*first, false));
                Some(self.len())
            }
            ExprKind::Group { nodes } => Expr::match_any(nodes, buffer, f),
            ExprKind::String { value } => {
                // compare to literal string
                if buffer[0..self.len()] == *value.as_bytes() {
                    buffer[0..self.len()]
                        .iter()
                        .for_each(|b| f(ExprOutput::new(*b, true)));
                    Some(self.len())
                } else {
                    None
                }
            }
            ExprKind::Range { from, to } => {
                if (*from..*to).contains(first) {
                    f(ExprOutput::new(*first, true));
                    Some(self.len())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct ExprOutput {
    pub highlight: bool,
    pub value: u8,
}

impl ExprOutput {
    pub fn new(value: u8, highlight: bool) -> Self {
        Self { highlight, value }
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

    pub fn single_len() -> usize {
        1
    }

    // calculates how many bytes this expression tree may match
    pub fn len(tree: &ExprBranch) -> usize {
        tree.iter()
            .fold(0, |i, n| i + n.kind.len() * n.mul as usize)
    }

    fn parse_byte_value(parser: &mut Parser) -> RbrepResult<u8> {
        let first = parser.adv();
        let second = parser.adv();
        u8::from_str_radix(&format!("{first}{second}"), 16)
            .map_err(|_| Error::BadSyntax(parser.pos))
    }

    fn parse_byte_or_range(parser: &mut Parser) -> RbrepResult<Expr> {
        let value = Self::parse_byte_value(parser)?;
        if parser.adv_if_trim('-') {
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
        if parser.adv_if_trim('?') && parser.adv_if_trim('?') {
            Ok(Expr::new(ExprKind::Any, 1))
        } else {
            Err(Error::BadSyntax(parser.pos))
        }
    }

    fn parse_mul(parser: &mut Parser, mut expr: Expr) -> RbrepResult<Expr> {
        // if not a mul return
        if !parser.adv_if_trim('*') {
            return Ok(expr);
        }

        // now, get the slice of a numbers
        let num = parser.until(|x| x.is_ascii_digit());

        let num = num
            .parse::<u32>()
            .map_err(|_| Error::BadSyntax(parser.pos))?;

        expr.mul = num;

        // ; is required after mul
        if !parser.adv_if_trim(';') {
            Err(Error::BadSyntax(parser.pos))
        } else {
            Ok(expr)
        }
    }

    fn parse_group(parser: &mut Parser) -> RbrepResult<Expr> {
        if !parser.adv_if_trim('(') {
            return Err(Error::BadSyntax(parser.pos));
        }

        let mut nodes = vec![];

        while !parser.adv_if_trim(')') {
            if parser.is_end() {
                return Err(Error::BadSyntax(parser.pos));
            }
            nodes.push(Self::parse(parser)?);
        }

        Ok(Expr::new(ExprKind::Group { nodes }, 1))
    }

    fn parse_string(parser: &mut Parser) -> RbrepResult<Expr> {
        if !parser.adv_if_trim('"') {
            return Err(Error::BadSyntax(parser.pos));
        }

        let start = parser.pos;
        // FIXME maybe allow escaping "s
        // but for now the user could just insert the ascii value...
        while !parser.adv_if_trim('"') {
            if parser.is_end() {
                return Err(Error::BadSyntax(parser.pos));
            }
            parser.adv();
        }
        let end = parser.pos - 1;
        let string = parser.src[start..end].to_owned();
        Ok(Expr::new(ExprKind::String { value: string }, 1))
    }

    fn parse(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.peek_trim();

        let expr = match first {
            '?' => Self::parse_any(parser),
            '(' => Self::parse_group(parser),
            '"' => Self::parse_string(parser),
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

    pub fn is_match<F>(&self, buffer: &[u8], f: &mut F) -> Option<usize>
    where
        F: FnMut(ExprOutput),
    {
        let mut total = 0;

        for _ in 0..self.mul {
            total += self.kind.is_match(&buffer[total..], f)?
        }
        Some(total)
    }

    // match all
    // the buffer should match the lenght of the expr
    // the f callback gets called for each successful matched byte
    // such a call does not mean that the overall match was successful though!
    fn match_all<F>(expr: &ExprBranch, buffer: &[u8], f: &mut F) -> Option<usize>
    where
        F: FnMut(ExprOutput),
    {
        if buffer.len() < Expr::len(expr) {
            return None;
        }

        let mut total = 0;
        for e in expr {
            match e.is_match(&buffer[total..], f) {
                Some(amount) => total += amount,
                // no match => return
                _ => return None,
            }
        }
        // got to end without fail => match found!
        Some(total)
    }

    // match any
    // matches any of the group and if it ends up matching, returns
    fn match_any<F>(expr: &ExprBranch, buffer: &[u8], f: &mut F) -> Option<usize>
    where
        F: FnMut(ExprOutput),
    {
        // find the shortest lenght in all nodes
        // this is the lenght we will at least require at this stage
        if buffer.len() < expr.iter().fold(0, |i, n| usize::min(n.kind.len(), i)) {
            return None;
        }

        for e in expr {
            if let Some(amount) = e.is_match(buffer, f) {
                return Some(amount);
            }
        }
        // got to end without success => no match found!
        None
    }

    // here we read the data and manage the buffer
    pub fn apply(
        expr: &ExprBranch,
        i: &mut dyn Read,
        o: &mut dyn Write,
        name: &str,
    ) -> anyhow::Result<()> {
        let mut next = [0; 1];
        let mut buffer = vec![0; Expr::len(expr)];

        let mut total = 0;
        let mut first_in_file = true;
        // let mut matches = 0;

        // read initial buffer
        let res = i.read_exact(&mut buffer);

        let mut output = vec![];

        // if the result was an error of Eof
        // there can never be a match
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            _ => res?,
        }

        loop {
            output.clear();
            // no matter what, we always advance a single byte
            // to check all possible combinations
            if Self::match_all(expr, &buffer, &mut |out| output.push(out)).is_some() {
                if first_in_file {
                    writeln!(o, "{}", style(name).magenta())?;
                    first_in_file = false;
                }

                // print current buffer if match
                write!(o, "{:08x}\t", style(total).green())?;
                for (i, b) in output.iter().enumerate() {
                    if CFG.space != 0 && i != 0 && i as u32 % CFG.space == 0 {
                        write!(o, " ")?;
                    }

                    if !b.highlight {
                        write!(o, "{:02x}", style(b.value))?;
                    } else {
                        write!(o, "{:02x}", style(b.value).red())?;
                    }
                }
                writeln!(o)?;
                // matches += 1;
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
            total += 1;
        }

        Ok(())
    }
}
