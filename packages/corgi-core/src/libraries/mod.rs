pub mod directories;

use uuid::Uuid;

use corgi_database::{
    entities::library,
    orm::{
        ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryOrder,
        QuerySelect, Set,
    },
};
use corgi_types::{LibraryPayload, Paginated, Pagination};

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<Option<library::Model>, DbErr> {
    library::Entity::find_by_id(id).one(db).await
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<library::Model, crate::error::Error> {
    find_by_id(db, id)
        .await?
        .ok_or(crate::error::Error::NotFound("Library"))
}

pub async fn create(
    db: &DatabaseConnection,
    payload: LibraryPayload,
) -> Result<library::Model, crate::error::Error> {
    let model = library::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(payload.name),
        category: Set(payload.category.to_string()),
        language: Set(payload.language),
        region: Set(payload.region),
        metadata_providers: Set(payload.metadata_providers.into()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(model)
}

pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    payload: LibraryPayload,
) -> Result<library::Model, crate::error::Error> {
    let model: library::ActiveModel = get_by_id(db, id).await?.into();

    let model = library::ActiveModel {
        id: model.id,
        name: Set(payload.name),
        category: Set(payload.category.to_string()),
        language: Set(payload.language),
        region: Set(payload.region),
        metadata_providers: Set(payload.metadata_providers.into()),
        ..model
    }
    .update(db)
    .await?;

    Ok(model)
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
    library::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

pub async fn query(
    db: &DatabaseConnection,
    pagination: Pagination,
) -> Result<Paginated<library::Model>, DbErr> {
    let items = library::Entity::find()
        .order_by_asc(library::Column::Id)
        .limit(pagination.limit)
        .offset(pagination.offset)
        .all(db)
        .await?;

    let count = library::Entity::find().count(db).await?;

    Ok(Paginated::new(
        items,
        count,
        pagination.limit,
        pagination.offset,
    ))
}
