use sql_builder_macros::Insert;

use crate::{
    grammar::{self, InsertColumnsAndSources, InsertionTarget},
    ToQuery,
};

use crate::grammar as G;
use crate::helpers as H;

#[derive(Insert)]
pub struct Insert<Target, Values>
where
    Target: G::InsertionTarget,
    Values: G::InsertColumnsAndSources,
{
    target: Target,
    values: Values,
}

impl<Target, Values> H::Insert for Insert<Target, Values>
where
    Target: InsertionTarget,
    Values: InsertColumnsAndSources,
{
    type Target = Target;
    type ColumnsAndSources = Values;

    fn transform_target<NewTarget: InsertionTarget>(
        self,
        transform: impl FnOnce(Self::Target) -> NewTarget,
    ) -> impl grammar::Insert {
        Insert {
            target: transform(self.target),
            values: self.values,
        }
    }

    fn transform_columns_and_sources<NewColumnsAndSources: InsertColumnsAndSources>(
        self,
        transform: impl FnOnce(Self::ColumnsAndSources) -> NewColumnsAndSources,
    ) -> impl grammar::Insert {
        Insert {
            target: self.target,
            values: transform(self.values),
        }
    }
}

impl<Target, Values> ToQuery for Insert<Target, Values>
where
    Target: InsertionTarget,
    Values: InsertColumnsAndSources,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "INSERT INTO ")?;
        self.target.write(stream, ctx)?;
        write!(stream, " ")?;
        self.values.write(stream, ctx)
    }
}
