use crate::utils::u64_;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, value},
    multi::many0,
    IResult,
};

pub fn star_1(data: String) {
    let exprs = parse(&data);
    let sum = exprs.map(|e| e.eval()).sum::<u64>();
    println!("{}", sum);
}

pub fn star_2(data: String) {
    let exprs = parse(&data);
    let sum = exprs.map(|e| e.eval_with_precedence()).sum::<u64>();
    println!("{}", sum);
}

fn parse<'a>(data: &'a str) -> impl Iterator<Item = Expr> + 'a {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| expr(s).unwrap().1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
    Plus,
    Times,
}

impl BinOp {
    fn apply(&self, a: &Expr, b: &Expr) -> u64 {
        let a = a.eval();
        let b = b.eval();
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
        }
    }
}

fn bin_op(input: &str) -> IResult<&str, BinOp> {
    alt((value(BinOp::Plus, tag("+")), value(BinOp::Times, tag("*"))))(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Num(u64),
    Paren(Box<Expr>),
    BinApp(Box<Expr>, BinOp, Box<Expr>),
}

impl Expr {
    fn paren(self) -> Self {
        Self::Paren(Box::new(self))
    }

    fn plus(self, r: Expr) -> Self {
        self.bin_app(BinOp::Plus, r)
    }

    fn times(self, r: Expr) -> Self {
        self.bin_app(BinOp::Times, r)
    }

    fn bin_app(self, op: BinOp, r: Expr) -> Self {
        Self::BinApp(Box::new(self), op, Box::new(r))
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Num(n) => *n,
            Self::Paren(e) => e.eval(),
            Self::BinApp(a, op, b) => op.apply(a, b),
        }
    }

    fn eval_with_precedence(&self) -> u64 {
        self.reorder_pluses().eval()
    }

    fn reorder_pluses(&self) -> Self {
        match self {
            Self::Num(n) => Self::Num(*n),
            Self::Paren(p) => p.reorder_pluses().paren(),
            Self::BinApp(a, op, b) => {
                let a = a.reorder_pluses();
                let b = b.reorder_pluses();
                match (op, a) {
                    (BinOp::Plus, Self::BinApp(a1, BinOp::Times, a2)) => a1.times(a2.plus(b)),
                    (op, a) => Self::bin_app(a, *op, b),
                }
            }
        }
    }
}

fn expr(input: &str) -> IResult<&str, Expr> {
    chained_ops(input)
}

fn simple_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = space0(input)?;
    alt((map(u64_, Expr::Num), paren))(input)
}

fn paren(input: &str) -> IResult<&str, Expr> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = space0(input)?;
    let (input, ex) = expr(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, ex.paren()))
}

fn chained_ops(input: &str) -> IResult<&str, Expr> {
    let (input, mut ex) = simple_expr(input)?;
    let (input, tails) = many0(bin_op_tail)(input)?;
    for (op, ex2) in tails {
        ex = Expr::BinApp(Box::new(ex), op, Box::new(ex2));
    }

    Ok((input, ex))
}

fn bin_op_tail(input: &str) -> IResult<&str, (BinOp, Expr)> {
    let (input, _) = space0(input)?;
    let (input, op) = bin_op(input)?;
    let (input, _) = space0(input)?;
    let (input, ex) = simple_expr(input)?;
    Ok((input, (op, ex)))
}

#[cfg(test)]
mod tests {
    use super::{expr, BinOp, Expr};

    #[test]
    fn expr_parses_single_num() {
        let raw = "12...";
        let expected = Expr::Num(12);
        assert_eq!(expr(raw), Ok(("...", expected)));
    }

    #[test]
    fn expr_parses_chain() {
        let raw = "1 + 2 * 3...";
        let expected = Expr::Num(1).plus(Expr::Num(2)).times(Expr::Num(3));
        assert_eq!(expr(raw), Ok(("...", expected)));
    }

    #[test]
    fn expr_parses_paren_in_chain() {
        let raw = "1 + (2 + 4) * 3...";
        let paren = Expr::Num(2).plus(Expr::Num(4)).paren();
        let expected = Expr::Num(1).plus(paren).times(Expr::Num(3));
        assert_eq!(expr(raw), Ok(("...", expected)));
    }

    #[test]
    fn eval_evals() {
        let raw = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let parsed = expr(raw).unwrap().1;
        assert_eq!(parsed.eval(), 13632);
    }

    #[test]
    fn eval_with_precedence_evals() {
        let raw = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let parsed = expr(raw).unwrap().1;
        assert_eq!(parsed.eval_with_precedence(), 23340);
    }

    #[test]
    fn eval_with_precedence_deals_with_plus_chains() {
        let raw = "8 * 3 + 0 + 9";
        let parsed = expr(raw).unwrap().1;
        assert_eq!(parsed.eval_with_precedence(), 96);
    }
}
