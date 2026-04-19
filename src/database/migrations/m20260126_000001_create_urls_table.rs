use crate::database::models::Url;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Url::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Url::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Url::LongUrl).string_len(2048).not_null())
                    .col(
                        ColumnDef::new(Url::ShortUrl)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Url::Status)
                            .string_len(20)
                            .not_null()
                            .default("active")
                            .check(Expr::col(Url::Status).is_in(["active", "inactive", "expired"])),
                    )
                    .col(
                        ColumnDef::new(Url::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Url::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_urls_short_url")
                    .table(Url::Table)
                    .col(Url::ShortUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_urls_created_at")
                    .table(Url::Table)
                    .col((Url::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_urls_created_at")
                    .table(Url::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_urls_short_url")
                    .table(Url::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Url::Table).to_owned())
            .await
    }
}
