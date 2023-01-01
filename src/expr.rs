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
            Expr::apply(
                &expr,
                &mut BufReader::new(f),
                &mut std::io::stdout(),
                path.to_str().unwrap_or(""),
            )?
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
    // And expression
    And { value: u8 },
    // Not
    Not { expr: Box<Expr> },
    // any string
    Any,
    // OR combination of expressions
    Group { nodes: Vec<Expr>, and: bool },
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
            ExprKind::And { value } => write!(f, "[AND] value: {value}"),
            ExprKind::Not { expr } => write!(f, "[NOT] expr: {expr}"),
            ExprKind::Any => write!(f, "[ANY]"),
            ExprKind::Group { nodes, and } => {
                write!(f, "[GROUP] {and}")?;
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
            ExprKind::Group { nodes, and } => {
                if *and {
                    nodes.iter().fold(0, |i, n| i + n.kind.len())
                } else {
                    nodes.iter().fold(0, |i, n| i.max(n.kind.len()))
                }
            }
            ExprKind::String { value } => value.bytes().len(),
            _ => Expr::single_len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // FIXME return Result<Vec<ExprOutput>>, change IF to return Result and remove OF
    // An empty vec means no match was found (same as None right now)
    pub fn is_match<IF>(&self, offset: usize, read: &mut IF) -> RbrepResult<ExprMatchResponse>
    where
        IF: FnMut(usize) -> RbrepResult<u8>,
    {
        let mut res = ExprMatchResponse::default();
        let first = read(offset)?;
        match self {
            ExprKind::Byte { value } => {
                if first == *value {
                    res.push(ExprOutput::new(first, true));
                }
            }
            ExprKind::And { value } => {
                if first & value != 0 {
                    res.push(ExprOutput::new(first, true));
                }
            }
            ExprKind::Not { expr } => {
                // apply matcher to next function, but do not use the
                // callback. Only if the parser returns an error, call callback
                // for the next value
                if expr.is_match(offset, read)?.is_empty() {
                    res.push(ExprOutput::new(first, true));
                }
            }
            ExprKind::Any => {
                res.push(ExprOutput::new(first, false));
            }
            ExprKind::Group { nodes, and } => {
                if *and {
                    res.append(&mut Expr::match_all(nodes, offset, read)?);
                } else {
                    res.append(&mut Expr::match_any(nodes, offset, read)?);
                }
            }
            ExprKind::String { value } => {
                // compare to literal string
                for (idx, b) in value.as_bytes().iter().enumerate() {
                    if read(offset + idx)? != *b {
                        break;
                    }
                    res.push(ExprOutput::new(*b, true));
                }
            }
            ExprKind::Range { from, to } => {
                if (*from..*to).contains(&first) {
                    res.push(ExprOutput::new(first, true));
                }
            }
        }
        Ok(res)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ExprOutput {
    pub highlight: bool,
    pub value: u8,
}

impl ExprOutput {
    pub fn new(value: u8, highlight: bool) -> Self {
        Self { highlight, value }
    }
}

// FIXME change back to a larger buffer when improving performance
pub type ExprMatchResponse = tinyvec::TinyVec<[ExprOutput; 1]>;

#[derive(Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub mul: u32,

    // match until no more matches are found
    pub many: bool,

    // this will not cause a failure, even if it does not match
    pub optional: bool,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[kind: {} mul: {}]", self.kind, self.mul)
    }
}

impl Expr {
    pub fn new(kind: ExprKind, mul: u32) -> Self {
        Self {
            kind,
            mul,
            many: false,
            optional: false,
        }
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

        if num == 0 {
            expr.optional = true;
            expr.mul = 1;
        } else {
            expr.mul = num;
        }

        // if + follows mul set many flag
        if parser.adv_if_trim('+') {
            expr.many = true;
        }

        // ; is required after mul
        if !parser.adv_if_trim(';') {
            Err(Error::BadSyntax(parser.pos))
        } else {
            Ok(expr)
        }
    }

    fn parse_group(parser: &mut Parser, and: bool) -> RbrepResult<Expr> {
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

        Ok(Expr::new(ExprKind::Group { nodes, and }, 1))
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

    fn parse_and(parser: &mut Parser) -> RbrepResult<Expr> {
        if !parser.adv_if_trim('&') {
            return Err(Error::BadSyntax(parser.pos));
        }

        // is it an and group?
        if parser.peek_trim() == '(' {
            Self::parse_group(parser, true)
        } else {
            let value = Self::parse_byte_value(parser)?;

            Ok(Expr::new(ExprKind::And { value }, 1))
        }
    }

    fn parse_not(parser: &mut Parser) -> RbrepResult<Expr> {
        if !parser.adv_if_trim('!') {
            return Err(Error::BadSyntax(parser.pos));
        }
        Ok(Expr::new(
            ExprKind::Not {
                expr: Box::new(Self::parse(parser)?),
            },
            1,
        ))
    }

    fn parse(parser: &mut Parser) -> RbrepResult<Expr> {
        let first = parser.peek_trim();

        let expr = match first {
            '?' => Self::parse_any(parser),
            '(' => Self::parse_group(parser, false),
            '"' => Self::parse_string(parser),
            '&' => Self::parse_and(parser),
            '!' => Self::parse_not(parser),
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

    pub fn is_match<IF>(&self, offset: usize, i: &mut IF) -> RbrepResult<ExprMatchResponse>
    where
        IF: FnMut(usize) -> RbrepResult<u8>,
    {
        let mut res = ExprMatchResponse::new();
        let mut total = 0;
        for _ in 0..self.mul {
            let mut result = self.kind.is_match(offset + total, i)?;
            let amount = if result.is_empty() && self.optional {
                0
            } else if result.is_empty() {
                return Ok(ExprMatchResponse::default());
            } else {
                result.len()
            };
            res.append(&mut result);

            total += amount;

            // call again if many flag is set and we had a result
            if (amount > 0 || self.optional) && self.many {
                loop {
                    let mut result = self.kind.is_match(offset + total, i)?;
                    if result.is_empty() {
                        break;
                    }
                    total += result.len();
                    res.append(&mut result);
                }
            }
        }
        Ok(res)
    }

    // match all
    // the buffer should match the lenght of the expr
    // the i callback gets called with the byte offset the matcher is attempting to read
    // i should return None if the read failed
    // and a byte value if the read was ok
    // i can be implemented in any way required to provide data to the matcher
    fn match_all<IF>(expr: &ExprBranch, offset: usize, i: &mut IF) -> RbrepResult<ExprMatchResponse>
    where
        IF: FnMut(usize) -> RbrepResult<u8>,
    {
        let mut total = 0;
        let mut res = ExprMatchResponse::default();
        for e in expr {
            let mut result = e.is_match(offset + total, i)?;
            total += result.len();
            // no match => return empty
            if result.is_empty() && !e.optional {
                return Ok(ExprMatchResponse::default());
            }
            res.append(&mut result);
        }
        // got to end without fail => match found!
        Ok(res)
    }

    // match any
    // matches any of the group and if it ends up matching, returns
    fn match_any<IF>(expr: &ExprBranch, offset: usize, i: &mut IF) -> RbrepResult<ExprMatchResponse>
    where
        IF: FnMut(usize) -> RbrepResult<u8>,
    {
        for e in expr {
            let res = e.is_match(offset, i)?;
            if !res.is_empty() {
                return Ok(res);
            }
        }
        // got to end without success => no match found!
        Ok(ExprMatchResponse::default())
    }

    // here we read the data and manage the buffer
    pub fn apply(
        expr: &ExprBranch,
        i: &mut dyn Read,
        o: &mut dyn Write,
        name: &str,
    ) -> anyhow::Result<()> {
        let mut next = [0; 1];
        let mut buffer = Vec::with_capacity(expr.len());

        let mut total = 0;
        let mut first_in_file = true;
        let mut matches = 0;

        // read initial byte
        let res = i.read_exact(&mut next);
        // same here, if it is eof
        // we have simply reached the onf of the file!
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            _ => res?,
        }

        // add next to vec
        buffer.push(next[0]);
        loop {
            if let Some(stop_after) = CFG.stop_after {
                if matches >= stop_after {
                    break;
                }
            }

            // no matter what, we always advance a single byte
            // to check all possible combinations
            let output = Self::match_all(expr, 0, &mut |offset| {
                if let Some(val) = buffer.get(offset) {
                    Ok(*val)
                } else if i.read_exact(&mut next).is_err() {
                    // FIXME this may not be correct
                    Err(Error::EndOfFile)
                } else {
                    buffer.push(next[0]);
                    Ok(next[0])
                }
            })?;
            if !output.is_empty() {
                if first_in_file {
                    if CFG.pretty {
                        writeln!(o, "{}", style(name).magenta())?;
                    } else {
                        writeln!(o, "{name}")?;
                    }
                    first_in_file = false;
                }

                // print current buffer if match
                // and count is not set
                if !CFG.count {
                    if CFG.pretty {
                        write!(o, "{:08x}\t", style(total).green())?;
                    } else {
                        write!(o, "{total:08x}\t")?;
                    }
                    for (i, b) in output.iter().enumerate() {
                        if CFG.space != 0 && i != 0 && i as u32 % CFG.space == 0 {
                            write!(o, " ")?;
                        }

                        if CFG.pretty {
                            if !b.highlight {
                                write!(o, "{:02x}", style(b.value))?;
                            } else {
                                write!(o, "{:02x}", style(b.value).red())?;
                            }
                        } else {
                            write!(o, "{:02x}", b.value)?;
                        }
                    }
                    writeln!(o)?;
                }
                matches += 1;
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

        if CFG.count {
            writeln!(o, "{matches}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn validate(expected: &str, expr: &str, input: &str) {
        let input: Vec<u8> = input.bytes().collect();
        let mut output = Vec::new();
        let expr = Expr::tree_from(&expr).unwrap();
        Expr::apply(&expr, &mut input.as_slice(), &mut output, "stdin").unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(expected, &output);
    }

    #[test]
    fn values() {
        validate("stdin\n00000000\t30\n", "30", "01");
        validate("stdin\n00000001\t31\n", "31", "01");
    }

    #[test]
    fn any() {
        validate("stdin\n00000000\t30\n00000001\t31\n", "??", "01");
    }

    #[test]
    fn string() {
        validate("stdin\n00000002\t48656c6c6f\n", "\"Hello\"", "12Hello34");
    }

    #[test]
    fn range() {
        validate("stdin\n00000000\t30\n00000001\t31\n", "30-32", "01234");
    }

    #[test]
    fn or_group() {
        validate("stdin\n00000000\t30\n00000002\t32\n", "(3032)", "01234");
    }

    #[test]
    fn and_group() {
        validate("stdin\n00000000\t3032\n", "&(3032)", "02134");
    }

    #[test]
    fn and() {
        validate(
            "stdin\n00000000\t30\n00000001\t31\n00000002\t32\n00000003\t33\n00000004\t34\n",
            "&30",
            "01234ABC",
        );
    }

    #[test]
    fn not() {
        validate(
            "stdin\n00000005\t41\n00000006\t42\n00000007\t43\n",
            "!&30",
            "01234ABC",
        );
    }

    #[test]
    fn mul() {
        validate("stdin\n00000002\t3131\n", "31*2;", "00112233");
    }

    #[test]
    fn nesting() {
        validate(
            "stdin\n00000001\t31323342\n00000006\t31323441\n",
            "3132(3334)41-43",
            "0123B0124A",
        );
    }

    #[test]
    fn optional() {
        validate("stdin\n00000000\t30\n00000001\t3031\n", "3031*0;", "0012")
    }

    #[test]
    fn many() {
        validate("stdin\n00000002\t3031313132\n", "3031*1+;32", "0001112");
    }

    #[test]
    fn many_optional() {
        validate("stdin\n00000002\t3032\n", "3031*0+;32", "0002");
        validate("stdin\n00000002\t3031313132\n", "3031*0+;32", "0001112");
    }
}
