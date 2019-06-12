use super::common::*;

/// A condition to put in a SQL `WHERE` clause. Pretty incomplete!
/// The parameter is the source of the data for the condition, which
/// prevents us from referring to columns that don't exist in the
/// given query's source table.
pub trait Condition<Src>: ToSql {
    fn and<Other: Condition<Src>>(self, other: Other) -> Both<Self, Other>
    where
        Self: Sized {
        Both(self, other)
    }

    fn or<Other: Condition<Src>>(self, other: Other) -> Either<Self, Other>
    where
        Self: Sized {
        Either(self, other)
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized {
        Not(self)
    }
}

impl <Src, A, B> Condition<Src> for Both<A, B>
where
    Src: ToSql,
    A: Condition<Src>,
    B: Condition<Src> { }

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

impl<Src, A, B> Condition<Src> for Either<A, B>
where
    Src: ToSql,
    A: Condition<Src>,
    B: Condition<Src> { }

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

impl<Src, A> Condition<Src> for Not<A>
where
    A: Condition<Src> { }

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

pub struct Both<A, B>(A, B);
pub struct Either<A, B>(A, B);
pub struct Not<A>(A);

/// An example of a `Condition` to check equality on a column or
/// other projection.
pub struct Equals<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

pub struct IsNull<Src, Prj> {
    pub source: Src,
    pub projection: Prj,
}

pub struct IsNotNull<Src, Prj> {
    pub source: Src,
    pub projection: Prj,
}

pub struct Less<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

pub struct Greater<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

pub struct Leq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

pub struct Geq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type,
}

pub struct NotEq<Src, Prj, Type> {
    pub source:     Src,
    pub projection: Prj,
    pub value:      Type
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

// How to turn an equality condition into SQL:
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

// How to turn an equality condition into SQL:
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

// How to turn an equality condition into SQL:
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

// How to turn an equality condition into SQL:
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

// How to turn an equality condition into SQL:
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

// How to turn an equality condition into SQL:
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

impl<Src, Prj, Type> Condition<Src> for Equals<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}

impl<Src, Prj> Condition<Src> for IsNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src>,
{

}

impl<Src, Prj> Condition<Src> for IsNotNull<Src, Prj>
where
    Src: ToSql,
    Prj: Projection<Src>,
{

}

impl<Src, Prj, Type> Condition<Src> for Less<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}

impl<Src, Prj, Type> Condition<Src> for Greater<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}

impl<Src, Prj, Type> Condition<Src> for Leq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}

impl<Src, Prj, Type> Condition<Src> for Geq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}

impl<Src, Prj, Type> Condition<Src> for NotEq<Src, Prj, Type>
where
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
{

}