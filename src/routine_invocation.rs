use crate::{grammar as G, ToQuery};
use sqlx::Database;
use std::fmt::Write;

pub struct RoutineInvocation<Name, Args>
where
    Name: G::RoutineName,
    Args: G::SQLArgumentList,
{
    name: Name,
    args: Args,
}

impl<Name, Args> RoutineInvocation<Name, Args>
where
    Name: G::RoutineName,
    Args: G::SQLArgumentList,
{
    pub fn new(name: Name, args: Args) -> Self {
        Self { name, args }
    }
}

impl<Name, Args> std::fmt::Display for RoutineInvocation<Name, Args>
where
    Name: G::RoutineName + std::fmt::Display,
    Args: G::SQLArgumentList + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.args)
    }
}

impl<'q, DB, Name, Args> ToQuery<'q, DB> for RoutineInvocation<Name, Args>
where
    DB: Database,
    Name: G::RoutineName + ToQuery<'q, DB>,
    Args: G::SQLArgumentList + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.name.write(ctx)?;
        write!(ctx, "(")?;
        self.args.write(ctx)?;
        write!(ctx, ")")
    }
}
