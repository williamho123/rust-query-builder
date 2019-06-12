//! SQL table column.
//!
//! This module contains the structure and interface for dealing with columns in SQL tables.
//! `Condition`s can be called on `Column`s and logic dealing with parsing data coming back from
//! SQL is also handled here (not yet implemented).

use std::marker::PhantomData;
use super::{
  condition::*,
  builder::Selectable,
  common::{ToSql, Projection, sstr}
};

/// Represents some column of a table whose values convert to a
/// Rust type. The column keeps track of the data source (table)
/// through the use of `PhantomData`.
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
/// struct UserTable;
///
/// const ID: Column<UserTable, i64> = Column {
///     name:     "id",
///     position: 0,
///     parse:    |s| s.parse().unwrap(), // parsing for now
///     _marker:  PhantomData,
/// };
///
/// const LOGIN_COUNT: Column<UserTable, Option<u64>> = Column {
///     name:     "login_count",
///     position: 3,
///     parse:    |s| s.parse().ok(), // parsing for now
///     _marker:  PhantomData,
/// };
/// ```
#[derive(Clone)]
pub struct Column<Table, Type> {
    /// The name of the column.
    pub name: sstr,

    /// The zero based index of the column in the table.
    pub position: usize,

    /// How to data from this column into a Rust Type
    pub parse:    for <'a> fn(&'a str) -> Type,

    /// Keeps track of the source of data for this column
    pub _table_marker:  PhantomData<fn(&Table)>,
}

impl<Table, Type> Column<Table, Type>
where
    Table: Selectable {

    /// Checks to see if the column has data that equals some other value.
    pub fn equals(self, other: Type) -> Equals<Table, Self, Type> {
        Equals {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    /// Checks to see if the column has data that is null.
    pub fn null(self) -> IsNull<Table, Self> {
        IsNull {
            source: Table::default(),
            projection: self,
        }
    }

    /// Checks to see if the column has data that is not null.
    pub fn not_null(self) -> IsNotNull<Table, Self> {
        IsNotNull {
            source: Table::default(),
            projection: self,
        }
    }

    /// Checks to see if the column has data that is greater than some other value.
    pub fn greater_than(self, other: Type) -> Greater<Table, Self, Type> {
        Greater {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    /// Checks to see if the column has data that is less than some other value.
    pub fn less_than(self, other: Type) -> Less<Table, Self, Type> {
        Less {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    /// Checks to see if the column has data that is less than or equal to some other value.
    pub fn leq(self, other: Type) -> Leq<Table, Self, Type> {
        Leq {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    /// Checks to see if the column has data that is greater than or equal to some other value.
    pub fn geq(self, other: Type) -> Geq<Table, Self, Type> {
        Geq {
            source: Table::default(),
            projection: self,
            value: other,
        }
    }

    /// Checks to see if the column has data that is not equal to some other value.
    pub fn not_equals(self, other: Type) -> NotEq<Table, Self, Type> {
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