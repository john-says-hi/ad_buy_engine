use crate::email_service::mailgun_user_init::send_invitation;
use crate::utils::authentication::hash;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use diesel::prelude::*;
use uuid::Uuid;

// Get
pub fn find(pool: &PgPool, _id: Uuid) -> Result<Invitation, ApiError> {
    use crate::schema::invitation::dsl::{id, invitation};

    let not_found = format!("User {} not found", _id);
    let conn = pool.get()?;

    let item = invitation
        .filter(id.eq(_id.to_string()))
        .first::<Invitation>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(item)
}

pub fn find_by_email(pool: &PgPool, _id: String) -> Result<Invitation, ApiError> {
    use crate::schema::invitation::dsl::{email, invitation};

    let not_found = format!("User {} not found", _id);
    let conn = pool.get().expect("Gw4esx");
    let item = invitation
        .filter(email.eq(_id.to_string()))
        .first::<Invitation>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(item)
}

pub fn new(pool: &PgPool, new: &Invitation) -> Result<Invitation, ApiError> {
    use crate::schema::invitation::dsl::{email, invitation};

    let conn = pool.get()?;

    // todo check if account already exists and handle
    diesel::delete(invitation)
        .filter(email.eq(&new.email))
        .execute(&conn)?;

    diesel::insert_into(invitation)
        .values(new)
        .execute(&conn)?;

    send_invitation(new)?;
    Ok(new.clone())
}

pub fn update(pool: &PgPool, item: &Invitation) -> Result<(), ApiError> {
    use crate::schema::invitation::dsl::{id, invitation};

    let conn = pool.get()?;
    diesel::update(invitation)
        .filter(id.eq(item.id.clone()))
        .set(item)
        .execute(&conn)?;
    Ok(())
}

pub fn remove(pool: &PgPool, _id: &String) -> Result<(), ApiError> {
    use crate::schema::invitation::dsl::{id, invitation};

    let conn = pool.get()?;
    diesel::delete(invitation)
        .filter(id.eq(_id))
        .execute(&conn)?;
    Ok(())
}

pub fn dedup(pool: &PgPool, _email: String) -> Result<(), ApiError> {
    use crate::schema::invitation::dsl::{email, invitation};

    let conn = pool.get()?;
    diesel::delete(invitation)
        .filter(email.eq(_email))
        .execute(&conn)?;
    Ok(())
}
