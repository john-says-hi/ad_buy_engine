use actix_web::web::{Data, block};
use crate::utils::database::PoolType;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::EmailModel;
use diesel::prelude::*;

pub async fn email_is_unique(email_to_cmp: &String, pool: Data<PoolType>) -> Result<bool, ApiError> {
	use crate::schema::email_list_table::dsl::{email_list_table,email};
	let conn=pool.get()?;
	let eml=email_to_cmp.clone();
	if block(move || email_list_table
		.filter(email.eq(eml))
		.load::<EmailModel>(&conn)).await?.is_empty() {
		println!("email is uni");
		Ok(true)
	} else {
		println!("email is not uni");
		Ok(false)
	}
}

pub async fn add_email(email_to_cmp: &String, pool: Data<PoolType>) -> Result<(), ApiError> {
	use crate::schema::email_list_table::dsl::{email_list_table,email};
	let conn=pool.get()?;
	let eml=EmailModel{email:email_to_cmp.clone()};
	block(move || diesel::insert_into(email_list_table).values(&eml).execute(&conn)).await?;
	Ok(())
}

pub async fn delete_email(email_to_cmp: &String, pool: Data<PoolType>) -> Result<(), ApiError> {
	use crate::schema::email_list_table::dsl::{email_list_table,email};
	let conn=pool.get()?;
	let pk=email_to_cmp.clone();
	block(move || diesel::delete(email_list_table.find(pk)).execute(&conn)).await?;
	Ok(())
}
