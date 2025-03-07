use uuid::Uuid;

use corgi_database::{
    entities::library_directory,
    orm::{
        ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, NotSet,
        QueryFilter, Set,
    },
};

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<library_directory::Model>, DbErr> {
    library_directory::Entity::find_by_id(id).one(db).await
}

pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<library_directory::Model, crate::error::Error> {
    find_by_id(db, id)
        .await?
        .ok_or(crate::error::Error::NotFound("LibraryDirectory"))
}

pub async fn find_all_by_library_id(
    db: &DatabaseConnection,
    library_id: Uuid,
) -> Result<Vec<library_directory::Model>, crate::error::Error> {
    let items = library_directory::Entity::find()
        .filter(library_directory::Column::LibraryId.eq(library_id))
        .all(db)
        .await?;

    Ok(items)
}

pub async fn find_one_by_library_id_and_path(
    db: &DatabaseConnection,
    library_id: Uuid,
    path: &str,
) -> Result<Option<library_directory::Model>, DbErr> {
    library_directory::Entity::find()
        .filter(
            Condition::all()
                .add(library_directory::Column::LibraryId.eq(library_id))
                .add(library_directory::Column::Path.eq(path)),
        )
        .one(db)
        .await
}

pub async fn create(
    db: &DatabaseConnection,
    library_id: Uuid,
    path: String,
) -> Result<library_directory::Model, crate::error::Error> {
    let path = crate::filesystem::path::check_dir(&path)?;

    super::get_by_id(db, library_id).await?;

    let check = find_one_by_library_id_and_path(db, library_id, &path).await?;

    if check.is_some() {
        return Err(crate::error::Error::DuplicatedLibraryDirectory(path));
    }

    let model = library_directory::ActiveModel {
        id: NotSet,
        library_id: Set(library_id),
        path: Set(path),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(model)
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    path: String,
) -> Result<library_directory::Model, crate::error::Error> {
    let path = crate::filesystem::path::check_dir(&path)?;

    let model = get_by_id(db, id).await?;

    let check = find_one_by_library_id_and_path(db, model.library_id, &path).await?;

    if let Some(check) = check {
        if check.id != id {
            return Err(crate::error::Error::DuplicatedLibraryDirectory(path));
        }
    }

    let model: library_directory::ActiveModel = model.into();

    let model = library_directory::ActiveModel {
        path: Set(path),
        ..model
    }
    .update(db)
    .await?;

    Ok(model)
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), crate::error::Error> {
    library_directory::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}
