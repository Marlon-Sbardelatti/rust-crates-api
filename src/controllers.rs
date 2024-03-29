use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RustaceanController;
impl RustaceanController {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table::find(rustaceans::table, id)
            .get_result(c)
            .await
    }

    pub async fn find_many(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table::find(rustaceans::table, rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        diesel::delete(rustaceans::table.find(id))
            .get_result(c)
            .await
    }
}

pub struct CrateController;
impl CrateController {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table::find(crates::table, id)
            .get_result(c)
            .await
    }

    pub async fn find_many(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_crate: NewCrate,
    ) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table::find(crates::table, a_crate.id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        diesel::delete(crates::table.find(id))
            .get_result(c)
            .await
    }

}
