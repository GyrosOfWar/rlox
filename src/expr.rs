use crate::token::{Literal, Token};

#[derive(Debug)]
pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: &'a Token<'a>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct LiteralExpr<'a> {
    pub lit: Literal<'a>,
}

#[derive(Debug)]
pub struct GroupingExpr<'a> {
    pub expr: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct UnaryExpr<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Binary(BinaryExpr<'a>),
    Literal(LiteralExpr<'a>),
    Grouping(GroupingExpr<'a>),
    Unary(UnaryExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn binary(left: Expr<'a>, operator: &'a Token<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Binary(e) => {
                e.left.accept(visitor);
                e.right.accept(visitor);
                visitor.visit_binary(e)
            }
            Expr::Literal(e) => visitor.visit_literal(e),
            Expr::Grouping(e) => {
                e.expr.accept(visitor);
                visitor.visit_grouping(e)
            }
            Expr::Unary(e) => {
                e.right.accept(visitor);
                visitor.visit_unary(e)
            }
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary(&self, expr: &BinaryExpr) -> R;

    fn visit_literal(&self, expr: &LiteralExpr) -> R;

    fn visit_grouping(&self, expr: &GroupingExpr) -> R;

    fn visit_unary(&self, expr: &UnaryExpr) -> R;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Literal, Token, TokenType};

    #[derive(Default)]
    struct CountingVisitor {}

    impl Visitor<usize> for CountingVisitor {
        fn visit_binary(&self, _: &BinaryExpr) -> usize {
            // self.binary += 1;
            todo!()
        }

        fn visit_literal(&self, _: &LiteralExpr) -> usize {
            todo!()
            // self.literal += 1;
        }

        fn visit_grouping(&self, _: &GroupingExpr) -> usize {
            // self.grouping += 1;
            todo!()
        }

        fn visit_unary(&self, _: &UnaryExpr) -> usize {
            // self.unary += 1;
            todo!()
        }
    }

    fn literal_num(n: f64) -> Expr<'static> {
        Expr::Literal(LiteralExpr {
            lit: Literal::Number(n),
        })
    }

    #[test]
    fn test_visit_binary() {
        let token = Token::new(TokenType::Plus, "+", None, 1);
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                lit: Literal::Number(1.0),
            })),
            operator: &token,
            right: Box::new(Expr::Literal(LiteralExpr {
                lit: Literal::Number(2.0),
            })),
        });

        let mut visitor = CountingVisitor::default();

        expr.accept(&mut visitor);

        // assert_eq!(1, visitor.binary);
        // assert_eq!(2, visitor.literal);
        // assert_eq!(0, visitor.unary);
        // assert_eq!(0, visitor.grouping);
    }

    #[test]
    fn test_visit_unary() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenType::Minus, "-", None, 1),
            right: Box::new(literal_num(2.0)),
        });

        let mut visitor = CountingVisitor::default();

        expr.accept(&mut visitor);

        // assert_eq!(0, visitor.binary);
        // assert_eq!(1, visitor.literal);
        // assert_eq!(1, visitor.unary);
        // assert_eq!(0, visitor.grouping);
    }
}
