//! Builder library.
//!
//! This module is the main mechanism through which a user can build queries. It defines
//! the beginning, chaining, and finishing of a query. All other modules from this crate
//! are re-exported to allow for ease of access.
//!
//! For example, suppose that there is a table type
//! `UserTable: Selectable`, with these two fields:
//!
//!   - `id` is a SQL `INTEGER NOT NULL`
//!   - `login_count` is a SQL `INTEGER`
//!
//! ```
//! const USERS: UserTable = UserTable;
//!
//! const ID: Column<UserTable, i64> = Column {
//!     name:     "id",
//!     position: 0,
//!     parse:    |s| s.parse().unwrap(),
//!     _marker:  PhantomData,
//! };
//!
//! const LOGIN_COUNT: Column<UserTable, Option<u64>> = Column {
//!     name:     "login_count",
//!     position: 3,
//!     parse:    |s| s.parse().ok(),
//!     _marker:  PhantomData,
//! };
//! ```
//!
//! Then a query would be constructed like so:
//! ```
//! let query = USERS
//!     .select((ID, LOGIN_COUNT))
//!     .filter(ID.geq(5)
//!               .and(LOGIN_COUNT.not_null()))
//!     .finish();
//! ```

pub use super::{
    common::*,
    column::*,
    condition::*
};

// Used when some type needs to remember some other type
// but without actually storing a value of that type.
use std::marker::PhantomData;

/// Things that can be selected from. (Each table would
/// define its own type that implements this).
pub trait Selectable: ToSql + Default + Sized {
    /// Begins a query by providing the desired projection.
    /// This can be a tuple of fields for a particular table,
    /// or a custom struct for selecting from a particular table,
    /// or any combination thereof.
    fn select<P: Projection<Self>>(self, p: P) -> Selected<Self, P>;
}

/// The result of a selection that has yet to be filtered. The query at this
/// point is basically `SELECT self.projection FROM self.source`.
pub struct Selected<Src, Prj> {
    pub source: Src,
    pub projection: Prj,
}

impl<Src, Prj> Selected<Src, Prj> {
    /// Filters a selection by some given condition.
    pub fn filter<Cond>(self, condition: Cond) -> Filtered<Src, Prj, Cond>
    where
        Cond: Condition<Src> {

        Filtered {
            source:     self.source,
            projection: self.projection,
            condition,
        }
    }
}

/// The result of applying filtering. The query at this
/// point is `SELECT self.projection FROM self.source WHERE self.condition`.
pub struct Filtered<Src, Prj, Cond> {
    source:     Src,
    projection: Prj,
    condition:  Cond,
}

impl<Src, Prj, Cond> Filtered<Src, Prj, Cond> {
    /// Finishes constructing a query.
    pub fn finish(self) -> Query<Src, Prj>
    where
        Src: Selectable,
        Prj: Projection<Src>,
        Cond: Condition<Src> {

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
    pub sql:    String,
    conversion: Prj,
    _marker:    PhantomData<fn(&Src)>,
}