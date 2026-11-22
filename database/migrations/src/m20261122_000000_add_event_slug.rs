use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Event {
	Table,
	Slug,
}

const SLUG_IDX: &str = "event_slug_idx";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(Event::Table)
					.add_column(ColumnDef::new(Event::Slug).text().null())
					.to_owned(),
			)
			.await?;

		manager
			.create_index(
				Index::create()
					.if_not_exists()
					.name(SLUG_IDX)
					.table(Event::Table)
					.col(Event::Slug)
					.unique()
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_index(Index::drop().name(SLUG_IDX).table(Event::Table).to_owned())
			.await?;
		manager
			.alter_table(
				Table::alter()
					.table(Event::Table)
					.drop_column(Event::Slug)
					.to_owned(),
			)
			.await
	}
}
