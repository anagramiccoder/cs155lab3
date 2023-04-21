use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}
pub fn eval(expr: Expr) -> Value {
    match expr{
        ArithExpr(arith_expr)=> IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr)=>BoolValue(eval_bool_expr(bool_expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr{
        BinArithExpr{left,right,op}=> match op{
            AddOp => eval_arith_expr(*left)+eval_arith_expr(*right),
            SubOp => eval_arith_expr(*left)-eval_arith_expr(*right),
            MulOp => eval_arith_expr(*left)*eval_arith_expr(*right),
            IntDivOp => eval_arith_expr(*left)/eval_arith_expr(*right),
        },
        IntLit(num)=> num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr{
        ArithCmpExpr{left, right, op} => match op{
            LtOp => eval_arith_expr (*left) < eval_arith_expr (*right),
            LteOp => eval_arith_expr (*left) <= eval_arith_expr (*right),
            GtOp => eval_arith_expr (*left) > eval_arith_expr (*right),
            GteOp => eval_arith_expr (*left) >= eval_arith_expr (*right),
            ArithEqOp => eval_arith_expr (*left) == eval_arith_expr (*right),
            ArithNeqOp => eval_arith_expr (*left) != eval_arith_expr (*right),
        },
        BinBoolExpr{left,right,op}=>match op{
            AndOp => eval_bool_expr(*left) && eval_bool_expr(*right),
            OrOp => eval_bool_expr(*left) || eval_bool_expr(*right),
            BoolEqOp => eval_bool_expr(*left) == eval_bool_expr(*right),
            BoolNeqOp => eval_bool_expr(*left) != eval_bool_expr(*right),
        },
        NotExpr(expr)=>!eval_bool_expr(*expr),
        BoolLit(val)=> val,
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
        let add_test=ArithExpr(
            BinArithExpr{
                left: Box::new(IntLit(5)),
                right: Box::new(IntLit(6)),
                op:AddOp,
            }
        );
        
        let sub_test=ArithExpr(
            BinArithExpr{
                left: Box::new(IntLit(6)),
                right: Box::new(IntLit(5)),
                op:SubOp,
            }
        );
        let mult_test=ArithExpr(
            BinArithExpr{
                left: Box::new(IntLit(5)),
                right: Box::new(IntLit(6)),
                op:MulOp,
            }
        );
        let div_test=ArithExpr(
            BinArithExpr{
                left: Box::new(IntLit(10)),
                right: Box::new(IntLit(2)),
                op:IntDivOp,
            }
        );
        //assertion for arith exprs
        assert_eq!(eval(add_test),IntValue(11));
        assert_eq!(eval(sub_test),IntValue(1));
        assert_eq!(eval(mult_test),IntValue(30));
        assert_eq!(eval(div_test),IntValue(5));


        //test case for arith comparisons
        let le_test=BoolExpr(
            ArithCmpExpr{
                left: Box::new(
                    BinArithExpr{
                        left: Box::new(IntLit(12)),
                        right: Box::new(IntLit(155)),
                        op:AddOp,
                    }
                ),
                right:Box::new(IntLit(167)),
                op:LtOp,  
        });
        let leq_test=BoolExpr(
            ArithCmpExpr{
                left: Box::new(
                    BinArithExpr{
                        left: Box::new(IntLit(12)),
                        right: Box::new(IntLit(155)),
                        op:AddOp,
                    }
                ),
                right:Box::new(IntLit(167)),
                op:LteOp,
        });
        assert_eq!(eval(le_test),BoolValue(false));
        assert_eq!(eval(leq_test),BoolValue(true));
        //assertion for ge, geq, and
        let ge_test=
            ArithCmpExpr{
                left: Box::new(
                    BinArithExpr{
                        left: Box::new(IntLit(12)),
                        right: Box::new(IntLit(155)),
                        op:AddOp,
                    }
                ),
                right:Box::new(IntLit(167)),
                op:GteOp,  
        };
        let geq_test=
            ArithCmpExpr{
                left: Box::new(
                    BinArithExpr{
                        left: Box::new(IntLit(12)),
                        right: Box::new(IntLit(155)),
                        op:AddOp,
                    }
                ),
                right:Box::new(IntLit(167)),
                op:GtOp,  
        };
        let and_test= BoolExpr(
            BinBoolExpr{
                left:Box::new(ge_test),
                right:Box::new(geq_test),
                op:AndOp,
            }
        );
        assert_eq!(eval(and_test),BoolValue(false));
        //eq,neq,or
        let or_test=BoolExpr(
            BinBoolExpr{
                left:Box::new(
                    ArithCmpExpr{
                        left:Box::new(
                            IntLit(5),
                        ),
                        right:Box::new(
                            IntLit(6)
                        ),
                        op:ArithEqOp,
                    }
                ),
                right:Box::new(
                    ArithCmpExpr{
                        left:Box::new(
                            IntLit(5),
                        ),
                        right:Box::new(
                            IntLit(6)
                        ),
                        op:ArithNeqOp,
                    }
                ),
                op: OrOp
            }
        );
        
        assert_eq!(eval(or_test),BoolValue(true));
        //eq neq not test
        let not_test=BoolExpr(
            NotExpr(Box::new (
                BinBoolExpr{
                    left:Box::new(BoolLit(true)),
                    right:Box::new(BinBoolExpr{
                        left:Box::new(BoolLit(true)),
                        right:Box::new(BoolLit(false)),
                        op:BoolNeqOp,
                    }),
                    op:BoolEqOp
                }
            ))
        );
        assert_eq!(eval(not_test),BoolValue(false));
    }
}