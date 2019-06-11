extern crate tuna;

use std::marker::PhantomData;
use tuna::*;

// #[derive(Tuna)]
// struct UserTable {
//    id: SqlType::Int
//    login_count: SqlType::Nullable(SqlType::Int),
//    name: SqlType::VarChar,
// }

fn main() {
    let cond = ID.geq(5)
        .and(LOGIN_COUNT.is_not_null().not());
    let query = USERTABLE
        .select((ID, LOGIN_COUNT))
        .filter(cond)
        .finish();

    println!("{:?}", query.sql);
    // let tst = ID.equals(5).and(LOGIN_COUNT.is_null());
    // println!("{}", tst.sql());
}

/// Macro generated code
#[derive(Default, Debug)]
struct UserTable {
    id: PhantomData<fn(&i64)>,
    login_count: PhantomData<fn(&Option<i64>)>,
    name: PhantomData<fn(&String)>,
}

const USERTABLE: UserTable = UserTable {
    id: PhantomData,
    login_count: PhantomData,
    name: PhantomData
};

const ID: Column<UserTable, i64> = Column {
    name:     "id",
    position: 0,
    parse:    |s| s.parse().unwrap(),
    _table_marker:  PhantomData,
};

const NAME: Column<UserTable, String> = Column {
    name:     "name",
    position: 0,
    parse:    |s| s.parse().unwrap(),
    _table_marker:  PhantomData,
};

const LOGIN_COUNT: Column<UserTable, Option<i64>> = Column {
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