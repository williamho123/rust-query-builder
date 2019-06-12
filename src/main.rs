extern crate tuna;

use std::marker::PhantomData;
use tuna::builder::*;

fn main() {

    let query = USERTABLE
        .select((ID, LOGIN_COUNT))
        .filter(ID.geq(1)
                  .and(LOGIN_COUNT.less_than(Some(5))
                  .and(NAME.equals("tuna".to_owned()))))
        .finish();

    println!("{:?}", query.sql);
}

// The user would only need to define something like this.
// #[derive(Tuna)]
// struct UserTable {
//    id: SqlType::Int
//    login_count: SqlType::Nullable(SqlType::Int),
//    name: SqlType::VarChar,
// }

// Macro generated code all below here.
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