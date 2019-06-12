#[allow(non_camel_case_types)]
pub type sstr = &'static str;

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

/// Defines the types of things that can be projected from
/// a source `Src` (probably a table).
pub trait Projection<Src>: ToSql {
    /// The type that the projected thing gets in Rust. See
    /// `Column` below for an example of where this comes from.
    type Value;
}

//impl ToSql for all possible rust data types
impl ToSql for i64 {
    type Sql = i64;
    fn sql(&self) -> Self::Sql {
        *self
    }
}

impl ToSql for String {
    type Sql = String;
    fn sql(&self) -> Self::Sql {
        //TODO: add sanitization
        self.clone()
    }
}

impl ToSql for f64 {
    type Sql = f64;
    fn sql(&self) -> Self::Sql {
        *self
    }
}

impl<T: ToSql + Clone> ToSql for Option<T> {
    type Sql = OptionT<T>;
    fn sql(&self) -> Self::Sql {
        match self {
            Some(a) => {
                return OptionT {
                    0: Some(a.clone()),
                }
            },
            None => {
                return OptionT{
                    0: None,
                }
            }
        }
    }
}

pub struct OptionT<T: ToSql>(Option<T>);

impl<T: ToSql> std::fmt::Display for OptionT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self.0 {
            Some(a) => a.sql().fmt(f),
            None => write!(f, "NULL"),
        }
    }
}

pub mod join {
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

macro_rules! impl_projection_for_tuple {
    ($($tv:ident),* $(,)?) => {
        impl<Src, $($tv),*> $crate::common::Projection<Src> for ($($tv,)*)
        where
            $( $tv: $crate::common::Projection<Src>, )* {
                type Value = ( $($tv::Value),* );
        }

        impl<$($tv),*> $crate::common::ToSql for ($($tv,)*)
        where
            $( $tv: $crate::common::ToSql, )* {

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
        A B;
        // then adds each of these in order:
        C D E F G H I J K L M N O P Q R S T U V W X Y Z
    }
}

macro_rules! impl_display_join_for_tuple {
    ($tv1:ident $(, $tv:ident)* $(,)?) => {
        impl<Sep, $tv1 $(,$tv)*> ::std::fmt::Display
            for $crate::common::join::Join<Sep, ($tv1 $(,$tv)*)>
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

// The following impls let you select tuples.

// impl<Src, A, B> Projection<Src> for (A, B)
// where
//     A: Projection<Src>,
//     B: Projection<Src>, {

//     type Value = (A::Value, B::Value);
// }

// impl<A, B> ToSql for (A, B)
// where
//     A: ToSql,
//     B: ToSql, {

//     type Sql = join::Join<sstr, (A::Sql, B::Sql)>;

//     #[allow(non_snake_case)]
//     fn sql(&self) -> Self::Sql {
//         join::Join {
//             sep: ", ",
//             tup: (self.0.sql(), self.1.sql()),
//         }
//     }
// }

// impl<Src, A, B, C> Projection<Src> for (A, B, C)
// where
//     A: Projection<Src>,
//     B: Projection<Src>,
//     C: Projection<Src>, {

//     type Value = (A::Value, B::Value, C::Value);
// }

// impl<A, B, C> ToSql for (A, B, C)
// where
//     A: ToSql,
//     B: ToSql,
//     C: ToSql, {

//     type Sql = join::Join<sstr, (A::Sql, B::Sql, C::Sql)>;

//     #[allow(non_snake_case)]
//     fn sql(&self) -> Self::Sql {
//         join::Join {
//             sep: ", ",
//             tup: (self.0.sql(), self.1.sql(), self.2.sql()),
//         }
//     }
// }

// This is gonna get annoying.

// impl_projection_for_tuple!(A, B, C, D);
// impl_projection_for_tuple!(A, B, C, D, E);

// That too.
