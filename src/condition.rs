//! SQL conditions.
//!
//! This module contains definitions and implementations related to SQL conditions.
//! That is, things that go after the `WHERE` clause.

use super::common::*;

/// A trait to represent a condition to put in a SQL `WHERE` clause.
/// The parameter is the source of the data for the condition, which
/// prevents us from referring to columns that don't exist in the
/// given query's source table.
pub trait Condition<Src>: ToSql {
    /// Chains two conditions from the same souce together using the SQL `AND` clause.
    fn and<Other: Condition<Src>>(self, other: Other) -> Both<Self, Other>
    where
        Self: Sized {
        Both(self, other)
    }

    /// Chains two conditions from the same source together using the SQL `OR` clause.
    fn or<Other: Condition<Src>>(self, other: Other) -> Either<Self, Other>
    where
        Self: Sized {
        Either(self, other)
    }

    /// Negates a condition using the SQL `NOT` clause.
    fn not(self) -> Not<Self>
    where
        Self: Sized {
        Not(self)
    }
}

/// A condition representing a pair of conditions grouped together by `AND`.
pub struct Both<A, B>(A, B);

/// A condition representing a pair of conditions grouped together by `OR`.
pub struct Either<A, B>(A, B);

/// A condition prefixed with `NOT`.
pub struct Not<A>(A);

/// A condition to check if the column or other projection is equal to some other value.
pub struct Equals<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

/// A condition to check if the column or other projection is not equal to some other value.
pub struct NotEq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type
}

/// A condition to check if the column or other projection is null.
pub struct IsNull<Src, Prj> {
    pub source: Src,
    pub projection: Prj,
}

/// A condition to check if the column or other projection is not null.
pub struct IsNotNull<Src, Prj> {
    pub source: Src,
    pub projection: Prj,
}

/// A condition to check if the column or other projection is less than some other value.
pub struct Less<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

/// A condition to check if the column or other projection is greater than some other value.
pub struct Greater<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

/// A condition to check if the column or other projection is less than or equal to some other value.
pub struct Leq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

/// A condition to check if the column or other projection is greater than or equal to some other value.
pub struct Geq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

impl<A, B> ToSql for Both<A, B>
where
    A: ToSql,
    B: ToSql {

    type Sql = join::Join<sstr, (A::Sql, sstr, B::Sql)>;

    fn sql(&self) -> Self::Sql {
        join::Join{
            sep: "",
            tup: (self.0.sql(),
                  " AND ",
                  self.1.sql())
        }
    }
}

impl<A, B> ToSql for Either<A, B>
where
    A: ToSql,
    B: ToSql {

    type Sql = join::Join<sstr, (A::Sql, sstr, B::Sql)>;

    fn sql(&self) -> Self::Sql {
        join::Join{
            sep: "",
            tup: (self.0.sql(),
                  " OR ",
                  self.1.sql())
        }
    }
}

impl<A> ToSql for Not<A>
where
    A: ToSql {

    type Sql = join::Join<sstr, (sstr, A::Sql)>;

    fn sql(&self) -> Self::Sql {
        join::Join{
            sep: "",
            tup: ("NOT ",
                  self.0.sql())
        }
    }
}

impl<Src, Prj, Type> ToSql for Equals<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " = '",
                  self.value.sql(),      "'"),
        }
    }
}

impl<Src, Prj, Type> ToSql for NotEq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " <> '",
                  self.value.sql(),      "'"),
        }
    }
}

impl<Src, Prj> ToSql for IsNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src>,
{
    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " IS NULL"),
        }
    }
}

impl<Src, Prj> ToSql for IsNotNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src>,
{
    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " IS NOT NULL"),
        }
    }
}

impl<Src, Prj, Type> ToSql for Less<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " < '",
                  self.value.sql(),      "'"),
        }
    }
}

impl<Src, Prj, Type> ToSql for Greater<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " > '",
                  self.value.sql(),      "'"),
        }
    }
}

impl<Src, Prj, Type> ToSql for Leq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " <= '",
                  self.value.sql(),      "'"),
        }
    }
}

impl<Src, Prj, Type> ToSql for Geq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> {

    type Sql = join::Join<sstr, (Src::Sql, sstr, Prj::Sql, sstr, Type::Sql, sstr)>;

    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: "",
            tup: (self.source.sql(),     ".",
                  self.projection.sql(), " >= '",
                  self.value.sql(),      "'"),
        }
    }
}

impl <Src, A, B> Condition<Src> for Both<A, B>
where
    Src: ToSql,
    A: Condition<Src>,
    B: Condition<Src> { }

impl<Src, A, B> Condition<Src> for Either<A, B>
where
    Src: ToSql,
    A: Condition<Src>,
    B: Condition<Src> { }

impl<Src, A> Condition<Src> for Not<A>
where
    Src: ToSql,
    A: Condition<Src> { }

impl<Src, Prj, Type> Condition<Src> for Equals<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }

impl<Src, Prj, Type> Condition<Src> for NotEq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }

impl<Src, Prj> Condition<Src> for IsNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src> { }

impl<Src, Prj> Condition<Src> for IsNotNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src> { }

impl<Src, Prj, Type> Condition<Src> for Less<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }

impl<Src, Prj, Type> Condition<Src> for Greater<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }

impl<Src, Prj, Type> Condition<Src> for Leq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }

impl<Src, Prj, Type> Condition<Src> for Geq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type> { }