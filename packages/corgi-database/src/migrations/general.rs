use sea_orm_migration::{prelude::*, schema::*};

#[derive(Iden)]
enum GeneralTableIds {
    CreatedAt,
    UpdatedAt,
    Deleted,
    DeletedAt,
}

/// Add general columns to a table: (`created_at`, `updated_at`, `deleted`, `deleted_at`).
pub fn general_table_cols(t: TableCreateStatement) -> TableCreateStatement {
    let mut t = t;
    t.col(date_time(GeneralTableIds::CreatedAt).default(Expr::current_time()))
        .col(date_time(GeneralTableIds::UpdatedAt).default(Expr::current_time()))
        .col(boolean(GeneralTableIds::Deleted).default(false).take())
        .col(date_time_null(GeneralTableIds::DeletedAt))
        .take()
}
