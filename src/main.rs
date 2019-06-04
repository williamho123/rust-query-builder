extern crate tuna;
use std::marker::PhantomData;
use tuna::{Column, Projection, Selected, ToSql, Selectable, sstr};

fn main() {
         let query = 
         USERS.select((ID, LOGIN_COUNT))
         .filter(NAME.equals("tov".to_owned()))
         .finish();
        println!("{:?}", query.sql);
}

///Example code
#[derive(Default, Debug)]
struct UserTable {
    id: PhantomData<fn(&i64)>,
    login_count: PhantomData<fn(&Option<i64>)>,
}

const USERS: UserTable = UserTable {
    id: PhantomData,
    login_count: PhantomData,
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