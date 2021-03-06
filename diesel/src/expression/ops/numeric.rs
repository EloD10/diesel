use backend::Backend;
use expression::{Expression, NonAggregate};
use query_builder::*;
use result::QueryResult;
use types;

macro_rules! numeric_operation {
    ($name:ident, $op:expr) => {
        #[derive(Debug, Copy, Clone, QueryId)]
        pub struct $name<Lhs, Rhs> {
            lhs: Lhs,
            rhs: Rhs,
        }

        impl<Lhs, Rhs> $name<Lhs, Rhs> {
            pub fn new(left: Lhs, right: Rhs) -> Self {
                $name {
                    lhs: left,
                    rhs: right,
                }
            }
        }

        impl<Lhs, Rhs> Expression for $name<Lhs, Rhs> where
            Lhs: Expression,
            Lhs::SqlType: types::ops::$name,
            Rhs: Expression,
        {
            type SqlType = <Lhs::SqlType as types::ops::$name>::Output;
        }

        impl<Lhs, Rhs, DB> QueryFragment<DB> for $name<Lhs, Rhs> where
            DB: Backend,
            Lhs: QueryFragment<DB>,
            Rhs: QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                self.lhs.walk_ast(out.reborrow())?;
                out.push_sql($op);
                self.rhs.walk_ast(out.reborrow())?;
                Ok(())
            }
        }

        impl_selectable_expression!($name<Lhs, Rhs>);

        impl<Lhs, Rhs> NonAggregate for $name<Lhs, Rhs> where
            Lhs: NonAggregate,
            Rhs: NonAggregate,
            $name<Lhs, Rhs>: Expression,
        {
        }

        generic_numeric_expr!($name, A, B);
    }
}

numeric_operation!(Add, " + ");
numeric_operation!(Sub, " - ");
numeric_operation!(Mul, " * ");
numeric_operation!(Div, " / ");
