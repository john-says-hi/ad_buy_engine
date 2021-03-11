use crate::email_service::mailgun_user_init::send_invitation;
use crate::utils::authentication::hash;
use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::invitation::Invitation;
use diesel::prelude::*;
use uuid::Uuid;

// Get
pub fn find(pool: &PoolType, _id: Uuid) -> Result<Invitation, ApiError> {
    use crate::schema::invitation_table::dsl::{invitation_id, invitation_table};

    let not_found = format!("User {} not found", _id);
    let conn = pool.get()?;

    let item = invitation_table
        .filter(invitation_id.eq(_id.to_string()))
        .first::<Invitation>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(item)
}

//todo

pub fn find_by_email(pool: &PoolType, _id: String) -> Result<Invitation, ApiError> {
    use crate::schema::invitation_table::dsl::{email, invitation_table};

    let not_found = format!("User {} not found", _id);
    let conn = pool.get().expect("Gw4esx");
    let item = invitation_table
        .filter(email.eq(_id.to_string()))
        .first::<Invitation>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;
    Ok(item)
}

/// Create
pub fn new(pool: &PoolType, new: &Invitation) -> Result<Invitation, ApiError> {
    use crate::schema::invitation_table::dsl::{email, invitation_table};

    let conn = pool.get()?;

    //dedup
    diesel::delete(invitation_table)
        .filter(email.eq(&new.email))
        .execute(&conn)?;

    diesel::insert_into(invitation_table)
        .values(new)
        .execute(&conn)?;

    send_invitation(new)?;
    Ok(new.clone())
}

pub fn update(pool: &PoolType, item: &Invitation) -> Result<(), ApiError> {
    use crate::schema::invitation_table::dsl::{invitation_id, invitation_table};

    let conn = pool.get()?;
    diesel::update(invitation_table)
        .filter(invitation_id.eq(item.invitation_id.clone()))
        .set(item)
        .execute(&conn)?;
    Ok(())
}

pub fn remove(pool: &PoolType, _id: &String) -> Result<(), ApiError> {
    use crate::schema::invitation_table::dsl::{invitation_id, invitation_table};

    let conn = pool.get()?;
    diesel::delete(invitation_table)
        .filter(invitation_id.eq(_id))
        .execute(&conn)?;
    Ok(())
}

pub fn dedup(pool: &PoolType, _email: String) -> Result<(), ApiError> {
    use crate::schema::invitation_table::dsl::{email, invitation_table};

    let conn = pool.get()?;
    diesel::delete(invitation_table)
        .filter(email.eq(_email))
        .execute(&conn)?;
    Ok(())
}
