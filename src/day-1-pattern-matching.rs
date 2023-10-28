/// An operation to perform on two subexpressions.
#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

/// The result of evaluating an expression.
#[derive(Debug, PartialEq, Eq)]
enum Res {
    /// Evaluation was successful, with the given result.
    Ok(i64),
    /// Evaluation failed, with the given error message.
    Err(String),
}
// Allow `Ok` and `Err` as shorthands for `Res::Ok` and `Res::Err`.
use Res::{Err, Ok};

fn eval(e: Expression) -> Res {
    match e {
        Expression::Op { op, left, right } => operate(op, eval(*left), eval(*right)),
        Expression::Value(v) => Ok(v),
    }
}

fn operate(op: Operation, left: Res, right: Res) -> Res {
    match (&op, left, right) {
        (Operation::Div, _, Ok(0)) => Err(String::from("division by zero")),
        (_, Ok(l), Ok(r)) => checked_operate(op, l, r),
        (_, Err(e), _) | (_, _, Err(e)) => Err(e),
    }
}

fn checked_operate(op: Operation, l: i64, r: i64) -> Res {
    match get_checked_fn(op)(l, r) {
        Some(v) => Ok(v),
        None => Err(String::from("overflow")),
    }
}

fn get_checked_fn(op: Operation) -> fn(i64, i64) -> Option<i64> {
    match op {
        Operation::Add => i64::checked_add,
        Operation::Sub => i64::checked_sub,
        Operation::Mul => i64::checked_mul,
        Operation::Div => i64::checked_div,
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}
