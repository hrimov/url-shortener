use sea_query::{Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use sqlx::PgPool;

use crate::database::models::{Url, UrlRow};

pub struct UrlGateway<'a> {
    pool: &'a PgPool,
}

impl<'a> UrlGateway<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_url(&self, long_url: &str, short_url: &str) -> Result<UrlRow, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(Url::Table)
            .columns([Url::LongUrl, Url::ShortUrl])
            .values_panic([long_url.into(), short_url.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with::<_, UrlRow, _>(&sql, values)
            .fetch_one(self.pool)
            .await
    }

    pub async fn get_url_by_short_url(
        &self,
        short_url: &str,
    ) -> Result<Option<UrlRow>, sqlx::Error> {
        let (sql, values) = Query::select()
            .from(Url::Table)
            .columns([
                Url::Id,
                Url::LongUrl,
                Url::ShortUrl,
                Url::Status,
                Url::CreatedAt,
                Url::UpdatedAt,
            ])
            .cond_where(
                Expr::col(Url::ShortUrl).binary(sea_query::BinOper::Equal, Expr::val(short_url)),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with::<_, UrlRow, _>(&sql, values)
            .fetch_optional(self.pool)
            .await
    }
}
