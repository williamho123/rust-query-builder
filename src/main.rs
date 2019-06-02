

fn main() {
         let query = USERS
         .select((ID, LOGIN_COUNT))
         .filter(ID.equals(1))
         .finish();
        println!("{:?}", query.sql);
}

// Used when some type needs to remember some other type
// but without actually storing a value of that type.
use std::marker::PhantomData;

#[allow(non_camel_case_types)]
type sstr = &'static str;

/// Types that can be used to generate SQL.
pub trait ToSql {
    /// The result type. This is something that can be displayed, but
    /// not necessarily a string. This makes it possible to return
    /// structures with multiple types and display those parts in turn
    /// without allocating.
    type Sql: std::fmt::Display;

    /// Returns the SQL for this thing.
    fn sql(&self) -> Self::Sql;
}

/// Things that can be selected from. (Each table would
/// define its own type that implements this.
pub trait Selectable: ToSql + Default + Sized {
    /// Begins a query by providing the desired projection.
    /// This can be a tuple of fields for a particular table,
    /// or a custom struct for selecting from a particular table,
    /// or any combination thereof.
    fn select<P: Projection<Self>>(self, p: P) -> Selected<Self, P>;
}


///Example code
#[derive(Default, Debug)]
struct UserTable;

const USERS: UserTable = UserTable;

const ID: Column<UserTable, i64> = Column {
    name:     "id",
    position: 0,
    parse:    |s| s.parse().unwrap(),
    _table_marker:  PhantomData,
};

const LOGIN_COUNT: Column<UserTable, Option<u64>> = Column {
    name:     "login_count",
    position: 3,
    parse:    |s| s.parse().ok(),
    _table_marker:  PhantomData,
};

impl ToSql for UserTable {
    type Sql = sstr;
    fn sql(&self) -> Self::Sql {
        "UserTable"
    }
}

impl Selectable for UserTable {
    fn select<P: Projection<UserTable>>(self, p: P) -> Selected<Self, P> {
        Selected {
            source: self,
            projection: p,
        }
    }
}
///end of example code


//impl ToSql for all possible rust data types
impl ToSql for i64 {
    type Sql = i64;
    fn sql(&self) -> Self::Sql {
        *self
    }
}

/// Defines the types of things that can be projected from
/// a source `Src` (probably a table).
pub trait Projection<Src>: ToSql {
    /// The type that the projected thing gets in Rust. See
    /// `Column` below for an example of where this comes from.
    type Value;

}

/// A condition to put in a SQL `WHERE` clause. Pretty incomplete!
/// The parameter is the source of the data for the condition, which
/// prevents us from referring to columns that don't exist in the
/// given query's source table.
pub trait Condition<Src>: ToSql { }

/// An example of a `Condition` to check equality on a column or
/// other projection.
pub struct Equals<Src, Prj, Type> {
    source:     Src,
    projection: Prj,
    value:      Type,
}

pub struct Less<Src, Prj, Type> {
    source:     Src,
    projection: Prj,
    value:      Type,
}

pub struct Greater<Src, Prj, Type> {
    source:     Src,
    projection: Prj,
    value:      Type,
}

pub struct Leq<Src, Prj, Type> {
    source:     Src,
    projection: Prj,
    value:      Type,
}

pub struct Geq<Src, Prj, Type> {
    source:     Src,
    projection: Prj,
    value:      Type,
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

impl<Src, Prj, Type> Condition<Src> for Equals<Src, Prj, Type> 
where 
    Src: ToSql,
    Type: ToSql,
    Prj: Projection<Src, Value = Type>,
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
/// use super::Equals;
    
/// Represents some column of table `Table` whose values convert to
/// Rust type `Type`.
///
/// For example, suppose that there is a table type
/// `UserTable: Selectable`, with these two fields:
///
///   - `id` is a SQL `INTEGER NOT NULL`
///   - `login_count` is a SQL `INTEGER`
///
/// Then you would want to define columns like so:
///
/// ```
// / struct UserTable;
// /
// / const ID: Column<UserTable, i64> = Column {
// /     name:     "id",
// /     position: 0,
// /     parse:    |s| s.parse().unwrap(),
// /     _marker:  Default::default(),
// / };
///
/// const LOGIN_COUNT: Column<UserTable, Option<u64>> = Column {
///     name:     "login_count",
///     position: 3,
///     parse:    |s| s.parse().ok(),
///     _marker:  Default::default(),
/// };
/// ```
#[derive(Clone)]
pub struct Column<Table, Type> {
    pub name:     sstr,
    pub position: usize,
    pub parse:    for <'a> fn(&'a str) -> Type,
    pub _table_marker:  PhantomData<fn(&Table)>,
}

impl<Table, Type> Column<Table, Type>
where
    Table: Selectable {
    
    pub fn equals(self, other: Type) -> Equals<Table, Self, Type> {
        Equals {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    pub fn greater(self, other: Type) -> Greater<Table, Self, Type> {
        Greater {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    pub fn less(self, other: Type) -> Less<Table, Self, Type> {
        Less {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    pub fn leq(self, other: Type) -> Leq<Table, Self, Type> {
        Leq {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    pub fn geq(self, other: Type) -> Geq<Table, Self, Type> {
        Geq {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }
}

// You can project a column from its table, and it gives you the
// columns type:
impl<Table, Type> Projection<Table> for Column<Table, Type> {
    type Value = Type;
}

// The SQL to project a column is just its name.
// (This might not be right---might need to include the
// source/table name here.)
impl<Table, Type> ToSql for Column<Table, Type> {
    type Sql = sstr;

    fn sql(&self) -> Self::Sql {
        self.name
    }
}

/// The result of a selection that has yet to be filtered. The query at this
/// point is basically `SELECT self.projection FROM self.source`.
pub struct Selected<Src, Prj> {
    source: Src,
    projection: Prj,
}

impl<Src, Prj> Selected<Src, Prj> {
    /// Filters a selection by some given condition.
    pub fn filter<Cond>(self, condition: Cond) -> Filtered<Src, Prj, Cond>
    where
        Cond: Condition<Src>, {

        Filtered {
            source:     self.source,
            projection: self.projection,
            condition,
        }
    }
}

/// The result of applying filtering.
pub struct Filtered<Src, Prj, Cond> {
    source:     Src,
    projection: Prj,
    condition:  Cond,
}

impl<Src, Prj, Cond> Filtered<Src, Prj, Cond>
{
    /// Finishes constructing a query.
    ///
    /// ```
    /// use super::Equals;
    ///
    /// struct UserTable;
    ///
    /// const USERS: UserTable = UserTable;
    ///
    /// const ID: Column<UserTable, i64> = Column {
    ///     name:     "id",
    ///     position: 0,
    ///     parse:    |s| s.parse().unwrap(),
    ///     _marker:  Default::default(),
    /// };
    ///
    /// const LOGIN_COUNT: Column<UserTable, Option<u64>> = Column {
    ///     name:     "login_count",
    ///     position: 3,
    ///     parse:    |s| s.parse().ok(),
    ///     _marker:  Default::default(),
    /// };
    ///
    /// let query = USERS
    ///     .select::<(ID, LOGIN_COUNT)>()
    ///     .filter(LOGIN_COUNT.equals(Some(1)))
    ///     .finish();
    /// ```
    pub fn finish(self) -> Query<Src, Prj>
    where
        Src: Selectable,
        Prj: Projection<Src>,
        Cond: Condition<Src>,
    {
        let sql = format!("SELECT {} FROM {} WHERE {}",
                          self.projection.sql(),
                          self.source.sql(),
                          self.condition.sql());

        Query {
            sql,
            conversion: self.projection,
            _marker:    PhantomData,
        }
    }
}

/// A query that’s ready to execute. It no longer stores any data
/// pointing to the source table, but it’s still tied by type so that
/// you can only execute it on a connection that has that table.

#[derive(Debug)]
pub struct Query<Src, Prj> {
    sql:        String,
    conversion: Prj,
    _marker:    PhantomData<fn(&Src)>,
}

// The following impls let you select tuples.

impl<Src, A, B> Projection<Src> for (A, B)
where
    A: Projection<Src>,
    B: Projection<Src>, {

    type Value = (A::Value, B::Value);
}

impl<A, B> ToSql for (A, B)
where
    A: ToSql,
    B: ToSql, {

    type Sql = join::Join<sstr, (A::Sql, B::Sql)>;

    #[allow(non_snake_case)]
    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: ", ",
            tup: (self.0.sql(), self.1.sql()),
        }
    }
}

impl<Src, A, B, C> Projection<Src> for (A, B, C)
where
    A: Projection<Src>,
    B: Projection<Src>,
    C: Projection<Src>, {

    type Value = (A::Value, B::Value, C::Value);
}

impl<A, B, C> ToSql for (A, B, C)
where
    A: ToSql,
    B: ToSql,
    C: ToSql, {

    type Sql = join::Join<sstr, (A::Sql, B::Sql, C::Sql)>;

    #[allow(non_snake_case)]
    fn sql(&self) -> Self::Sql {
        join::Join {
            sep: ", ",
            tup: (self.0.sql(), self.1.sql(), self.2.sql()),
        }
    }
}

// This is gonna get annoying.

macro_rules! impl_projection_for_tuple {
    ($($tv:ident),* $(,)?) => {
        impl<Src, $($tv),*> $crate::Projection<Src> for ($($tv,)*)
        where
            $( $tv: $crate::Projection<Src>, )* {
                type Value = ( $($tv::Value),* );
        }

        impl<$($tv),*> $crate::ToSql for ($($tv,)*)
        where
            $( $tv: $crate::ToSql, )* {

            type Sql = join::Join<sstr, ($($tv::Sql,)*)>;

            #[allow(non_snake_case)]
            fn sql(&self) -> Self::Sql {
                let ($($tv,)*) = self;
                join::Join {
                    sep: ", ",
                    tup: ($($tv.sql(),)*),
                }
            }
        }
    };
}

impl_projection_for_tuple!(A, B, C, D);
impl_projection_for_tuple!(A, B, C, D, E);

// That too.

macro_rules! apply_macro_for_tuples {
    ($op:ident!{ $($now:ident)* ;}) => {
        $op!{$($now),*}
    };
    ($op:ident!{$($now:ident)*; $next:ident $($later:ident)*}) => {
        $op!{$($now),*}
        apply_macro_for_tuples!{$op!{$($now)* $next; $($later)*}}
    };
}

apply_macro_for_tuples! {
    impl_projection_for_tuple! {
        // starts with these:
        A B C D E F;
        // then adds each of these in order:
        G H I J K L M N O P Q R S T U V W X Y Z
    }
}

mod join {
    use std::fmt::{Display, Formatter, Result};

    pub struct Join<Sep, Tup> {
        pub sep: Sep,
        pub tup: Tup,
    }

    impl<Sep, A, B> Display for Join<Sep, (A, B)>
    where
        Sep: Display,
        A: Display,
        B: Display, {

        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{}{}{}", self.tup.0, self.sep, self.tup.1)
        }
    }

}

macro_rules! impl_display_join_for_tuple {
    ($tv1:ident $(, $tv:ident)* $(,)?) => {
        impl<Sep, $tv1 $(,$tv)*> ::std::fmt::Display
            for $crate::join::Join<Sep, ($tv1 $(,$tv)*)>
        where
            Sep: ::std::fmt::Display,
            $tv1: ::std::fmt::Display,
            $( $tv: ::std::fmt::Display, )* {

            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #[allow(non_snake_case)]
                let (ref $tv1 $(, ref $tv)*) = self.tup;
                ::std::write!(f, "{}", $tv1)?;
                $(
                    ::std::write!(f, "{}{}", self.sep, $tv)?;
                )*
                ::std::result::Result::Ok(())
            }
        }
    };
}

apply_macro_for_tuples! {
    impl_display_join_for_tuple! {
        A B C;
        D E F G H I J K L M N O P Q R S T U V W X Y Z
    }
}
