#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241213_131911_articles;
mod m20241213_140246_comments;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241213_131911_articles::Migration),
            Box::new(m20241213_140246_comments::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}