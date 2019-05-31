// #[allow(unused)]

// tuna_connect! {
//     db = "localhost:9090";

//     struct Users = table "users";
//     struct Posts = table "posts";
// }

// pub mod tuna_library {
//     pub struct TunaError {}

//     pub type Result<T, E = TunaError> = std::result::Result<T, E>;

//     pub trait DbSession {}

//     pub trait SqlStorable {
//         fn serialize(&self) -> String;
//     }

//     pub trait SqlLoadable {
//         fn deserialize(s: &str) -> Result<Self>;
//     }

//     pub trait SqlUpdatable {
//         fn update(&self, session: impl DbSession, fields: Vec<String>);
//     }

//     macro_rules! expand_table {
//         (

//         $v:vis struct $n:ident {
//             $(
//                 $fv:vis $field:ident : $t:ty
//             ),*
//             $(,)?
//          }

//         ) => {

//             $v struct $n {
//                 id: usize,

//                 $(
//                     $fv $field: $t,
//                 )*
//             }

//             impl SqlStorable for $n {
//                 fn serialize(&self) {
//                     String::new()
//                 }
//             }

//             $(
//                 pub mod $field {
//                     pub struct LessThan($t);
//                     pub struct Equals($t);
//                 }
//             )*

//         }
//     }
// }

// #[derive(Tuna)]
// pub struct User {
//     name: String,
//     pub email: String,
//     select: usize,
// }

// pub struct Post {
//     title: String,
//     content: String,
//     user: tuna::WeakRef<User>,
// }

// #[derive(Tuna)]
// pub struct User {
//     name: String,
//     email: String,
//     #[tuna::column_name("select_")]
//     select: usize,
//     #[tuna::has_many(Post)]
//     post: tuna::Ref<Post>,
// }


// pub struct RealUser {
//     id: usize,
//     name: String,
//     email: String,
//     select: usize,
//     post: tuna::Ref<Post>,
// }

// pub trait Column<Table: Tuna> {
//     type Type;

//     fn get(table: &Table) -> Self::Type;
//     fn set(table: &mut Table, value: Self::Type);
// }

// fn foo () {

//     session.find::<User>()
//         .where_(user::Name).equals(someone)

//         .where_eq(user::Name("william".to_string()))
//         .where_like(user::Email("%northwestern%".to_string()))
//         .wherein(Lt(user::Age(10)))

// }

// pub mod user {
//     use super::{Column, RealUser};

//     pub struct Id(usize);
//     pub struct Name(String);
//     pub struct Email<T: Into<String>>(T);

//     impl Column<RealUser> for Id {
//         type Type = usize;

//         fn get(user: &RealUser) -> Self::Type {
//             user.id
//         }

//         fn set(user: &mut RealUser, value: Self::Type) {
//             user.id = value;
//         }
//     }

//     impl Column<RealUser> for Id {
//         type Type = usize;

//         fn get(user: &RealUser) -> Self::Type {
//             user.id
//         }

//         fn set(user: &mut RealUser, value: Self::Type) {
//             user.id = value;
//         }
//     }

//     impl Column<RealUser> for Name {
//         type Type = String;

//         fn get(table: &RealUser) -> Self::Type {
//             table.name.clone()
//         }

//         fn set(table: &mut RealUser, value: Self::Type) {
//             table.name = value;
//         }
//     }
// }