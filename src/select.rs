use crate::{from::FromClause, traits, r#where::WhereClause, ToQuery};
pub enum SetQuantifier {
    All,
    Distinct,
}

/// Select command
pub struct Select<
    Select: traits::SelectList,
    From: traits::FromClause,
    Where: traits::WhereClause,
    GroupBy: traits::GroupByClause,
    Having: traits::HavingClause,
    OrderBy: traits::OrderByClause,
    Limit: traits::LimitExpr,
> {
    pub quantifier: Option<SetQuantifier>,
    pub select_list: Select,
    pub from_clause: From,
    pub where_clause: Where,
    pub group_by_clause: GroupBy,
    pub having_clause: Having,
    pub order_by_clause: OrderBy,
    pub limit: Limit,
}

pub type BlankSelectStatement = Select<(), (), (), (), (), (), ()>;
pub type InitSelectStatement<S> = Select<S, (), (), (), (), (), ()>;

impl BlankSelectStatement {
    pub fn new<S: traits::SelectList>(select: S) -> InitSelectStatement<S> {
        InitSelectStatement {
            quantifier: None,
            select_list: select,
            from_clause: (),
            where_clause: (),
            group_by_clause: (),
            having_clause: (),
            order_by_clause: (),
            limit: (),
        }
    }
}

impl<SelectList, From, Where, GroupBy, Having, OrderBy, Limit>
    Select<SelectList, From, Where, GroupBy, Having, OrderBy, Limit>
where
    SelectList: traits::SelectList,
    From: traits::FromClause,
    Where: traits::WhereClause,
    GroupBy: traits::GroupByClause,
    Having: traits::HavingClause,
    OrderBy: traits::OrderByClause,
    Limit: traits::LimitExpr,
{
    #[inline]
    pub fn distinct(mut self) -> Self {
        self.quantifier = Some(SetQuantifier::Distinct);
        self
    }

    #[inline]
    pub fn all(mut self) -> Self {
        self.quantifier = Some(SetQuantifier::All);
        self
    }

    #[inline]
    pub fn from<TableRef: traits::TableReference>(
        self,
        table_ref: TableRef,
    ) -> Select<SelectList, FromClause<TableRef>, Where, GroupBy, Having, OrderBy, Limit> {
        Select {
            quantifier: None,
            select_list: self.select_list,
            from_clause: FromClause::from(table_ref),
            where_clause: self.where_clause,
            group_by_clause: self.group_by_clause,
            having_clause: self.having_clause,
            order_by_clause: self.order_by_clause,
            limit: self.limit,
        }
    }

    #[inline]
    pub fn r#where<SearchCond: traits::SearchCondition>(
        self,
        search_cond: SearchCond,
    ) -> Select<SelectList, From, WhereClause<SearchCond>, GroupBy, Having, OrderBy, Limit> {
        Select {
            quantifier: None,
            select_list: self.select_list,
            from_clause: self.from_clause,
            where_clause: WhereClause::from(search_cond),
            group_by_clause: self.group_by_clause,
            having_clause: self.having_clause,
            order_by_clause: self.order_by_clause,
            limit: self.limit,
        }
    }
}

impl<SelectList, From, Where, GroupBy, Having, OrderBy, Limit> ToQuery
    for Select<SelectList, From, Where, GroupBy, Having, OrderBy, Limit>
where
    SelectList: traits::SelectList,
    From: traits::FromClause,
    Where: traits::WhereClause,
    GroupBy: traits::GroupByClause,
    Having: traits::HavingClause,
    OrderBy: traits::OrderByClause,
    Limit: traits::LimitExpr,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "SELECT")?;
        
        if let Some(quantifier) = &self.quantifier {
            write!(stream, " ")?;

            match quantifier {
                SetQuantifier::All => write!(stream, "ALL"),
                SetQuantifier::Distinct => write!(stream, "DISTINCT"),
            }?;
        }

        if SelectList::IS_IMPL {
            write!(stream, " ")?;
            self.select_list.write(stream, ctx)?;         
        }

        if From::IS_IMPL {
            write!(stream, " ")?;
            self.from_clause.write(stream, ctx)?;
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
/// Wildcard (*)
pub struct All {}
pub const ALL: All = All {};
impl ToQuery for All {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "*")
    }
}
impl traits::SelectList for All {
    const IS_IMPL: bool = true;
}

#[cfg(test)]
mod tests {
    use crate::{identifier::id, traits::{DerivedColumn, SelectList}, ToQuery};

    use super::{Select, ALL};

    #[test]
    fn test_select_list_of_identifiers() {
        let selected_columns = id("col1")
            .chain(
                id("col2")
                    .r#as(id("aliased_column"))
            ).chain(id("col3"));

        let stmt = Select::new(selected_columns);
        let sql = stmt.to_string().unwrap();
        
        assert_eq!(sql, "SELECT col1, col2 AS aliased_column, col3");
    }

    #[test]
    fn test_select_with_from() {
        let stmt = Select::new(ALL)
            .from(id("my_table"));

        let sql = stmt.to_string().unwrap();
        assert_eq!(sql, "SELECT * FROM my_table")
    }
}
