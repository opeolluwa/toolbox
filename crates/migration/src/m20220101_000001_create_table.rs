use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table("bookmarks")
                    .if_not_exists()
                    .if_not_exists()
                    .col(
                        ColumnDef::new("identifier")
                            .string()
                            .string_len(26)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new("title").string().not_null().unique_key())
                    .col(ColumnDef::new("description").string().not_null())
                    .col(
                        ColumnDef::new("sensitive")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new("created_at")
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new("updated_at").timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table("bookmarks").to_owned())
            .await
    }
}
