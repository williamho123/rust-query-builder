use std::marker::PhantomData;
use super::{
  condition::*,
  builder::Selectable,
  common::{ToSql, Projection, sstr}
};

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

    pub fn is_null(self) -> IsNull<Table, Self> {
        IsNull {
            source: Table::default(),
            projection: self,
        }
    }

    pub fn is_not_null(self) -> IsNotNull<Table, Self> {
        IsNotNull {
            source: Table::default(),
            projection: self,
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

    pub fn neq(self, other: Type) -> NotEq<Table, Self, Type> {
        NotEq {
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