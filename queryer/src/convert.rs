use std::sync::Arc;
use anyhow::anyhow;
use sqlparser::ast::{Expr, OrderByExpr, Select, SelectItem, SetExpr, Statement, TableWithJoins};

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

pub struct Expression(pub(crate) Box<SqlExpr>);
pub struct Operation(pub(crate) SqlBinaryOperator);
pub struct Projection<'a>(pub(crate) &'a SelectItem);
pub struct Source<'a>(pub(crate) &'a [TableWithJoins]);
pub struct Order<'a>(pub(crate) &'a OrderByExpr);
pub struct Offset<'a>(pub(crate) &'a SqlOffset);
pub struct Limit<'a>(pub(crate) &'a SqlExpr);
pub struct Value(pub(crate) SqlValue);

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(q) => {
                let offset = q.offset.as_ref();
                let limit = q.limit.as_ref();
                let orders = &q.order_by;
                let Select {
                    from: table_with_joins,
                    selection: where_clause,
                    projection,
                    group_by: _,
                    ..
                } = match &q.body {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("We only support Select Query at the moment")),
                };

                let source = Source(table_with_joins).try_into()?;

                let condition = match where_clause {
                    Some(expr) => Some(Expression(Box::new(expr.to_owned())).try_into()?),
                    None => None,
                };

                let mut selection = Vec::with_capacity(8);
                for p in projection {
                    let expr = Projection(p).try_into()?;
                    selection.push(expr);
                }

                let mut order_by = Vec::new();
                for expr in orders {
                    order_by.push(Order(expr).try_into()?);
                }

                let offset = offset.map(|v| Offset(v).into());
                let limit = limit.map(|v| Limit(v).into());

                Ok(Sql {
                    selection,
                    condition,
                    source,
                    order_by,
                    offset,
                    limit,
                })
            }
            _ => Err(anyhow!("We only support Query at the moment")),
        }
    }
}

/// 把 SqlParser 的 Expr 转换成 DataFrame 的 Expr
impl TryFrom<Expression> for Expr {
    type Error = anyhow::Error;

    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::BinaryOp { left, op, right } => Ok(Expr::BinaryExpr {
                left: Box::new(Expression(left).try_into()?),
                op: Operation(op).try_into()?,
                right: Box::new(Expression(right).try_into()?),
            }),
            SqlExpr::Wildcard => Ok(Self::Wildcard),
            SqlExpr::IsNull(expr) => Ok(Self::IsNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::IsNotNull(expr) => Ok(Self::IsNotNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::Identifier(id) => Ok(Self::Column(Arc::new(id.value))),
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expr {:#?} is not supported", v)),
        }
    }
}

