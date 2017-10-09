//#![deny(warnings)]
#![feature(try_from)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate cfg_if;
extern crate r2d2;
extern crate url;
extern crate bigdecimal;
extern crate dao;
#[cfg(test)]
#[macro_use]
extern crate dao_codegen;
extern crate chrono;
extern crate uuid;

cfg_if! {if #[cfg(feature = "with-postgres")]{
    extern crate r2d2_postgres;
    extern crate postgres;
    #[macro_use]
    extern crate postgres_shared;
    mod pg;
}}
cfg_if! {if #[cfg(feature = "with-sqlite")]{
    extern crate r2d2_sqlite;
    extern crate rusqlite;
    mod sqlite;
}}

mod pool;
mod platform;
mod error;
mod database;
mod entity;


pub use pool::Pool;
pub use database::Database;
pub use dao::Dao;
pub use dao::Value;
pub use dao::Rows;
pub use error::DbError;
pub use dao::{ToDao,FromDao};
pub use dao::{ToTable};
pub use dao::{ToColumns};


#[cfg(test)]
mod test {
    use super::*;
    use dao::{FromDao, ToDao};

    #[test]
    fn derive_fromdao_and_todao() {
        #[derive(Debug, PartialEq, FromDao, ToDao)]
        struct User {
            id: i32,
            username: String,
            active: Option<bool>,
        }

        let user = User {
            id: 1,
            username: "ivanceras".into(),
            active: Some(true),
        };
        println!("user: {:#?}", user);
        let dao = user.to_dao();
        let mut expected_dao = Dao::new();
        expected_dao.insert("id", 1);
        expected_dao.insert("username", "ivanceras".to_string());
        expected_dao.insert("active", true);

        assert_eq!(expected_dao, dao);

        println!("dao: {:#?}", dao);
        let from_dao = User::from_dao(&dao);
        println!("from_dao: {:#?}", from_dao);
        assert_eq!(from_dao, user);
    }
}
