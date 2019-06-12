//! Tuna Query Builder
//!
//! This crate is a general purpose library for building "fluent" SQL queries in Rust. Importantly, the idea
//! is to have the library prevent SQL injections and allow queried and returned data to be typed.
//!
//! In an ideal world, this is how the library would be used by the client.
//! ```
//! // The client provides the DB schema using `tuna::SqlType` that map Sql types to Rust types.
//! // Deriving Tuna generates a lot of utility code from macros, such as defining columns and
//! // default trait implementations.
//! #[derive(Tuna)]
//! struct UserTable {
//!     id: SqlType::Int
//!     login_count: SqlType::Int,
//!     name: SqlType::VarChar,
//!     deleted_at: SqlType::Nullable(SqlType::DateTime)
//! }
//! ```
//!
//! Then a query would be constructed like this:
//! ```
//! let query = USERS
//!     .select((ID, LOGIN_COUNT))
//!     .filter(ID.geq(5)
//!               .and(LOGIN_COUNT.not_null()))
//!     .finish();
//!
//! // `query` can now be executed with the DB credentials.
//! ```
//!
//! Right now, the library supports the "query building" part, but automatic code generation from macros
//! has not been fully implemented. As such, the user will have to manually define the columns. Likewise,
//! `tuna::SqlType` is a work in a progress so a user will have to manually define the Rust types he expects
//! to get back. See `main.rs` for an example of current functionality.

pub mod builder;

pub mod common;
pub mod column;
pub mod condition;
