use actix_web::web::{Data, block};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use ad_buy_engine::data::backend_models::EmailModel;
use diesel::prelude::*;
use actix_web::HttpResponse;
use crate::management::db;

pub async fn get_email_list(pool: Data<PgPool>) -> Result<HttpResponse, ApiError> {
	let res =block(move || db::email::get_all_emails(&pool)).await?;
	Ok(HttpResponse::Ok().json(&res))
}
	
	
	pub async fn email_is_unique(email_to_cmp: &String, pool: Data<PgPool>) -> Result<bool, ApiError> {
	use crate::schema::emails::dsl::{emails,email};
	let conn=pool.get()?;
	let eml=email_to_cmp.clone();
	if block(move || emails
		.filter(email.eq(eml))
		.load::<EmailModel>(&conn)).await?.is_empty() {
		println!("email is uni");
		Ok(true)
	} else {
		println!("email is not uni");
		Ok(false)
	}
}

pub async fn add_email(new_eml: &String, pool: Data<PgPool>) -> Result<(), ApiError> {
	use crate::schema::emails::dsl::{emails,email};
	let conn=pool.get()?;
	let eml=EmailModel{email:new_eml.clone()};
	block(move || diesel::insert_into(emails).values(&eml).execute(&conn)).await?;
	println!("Eml added");
	Ok(())
}

pub async fn delete_email(email_to_cmp: &String, pool: Data<PgPool>) -> Result<(), ApiError> {
	use crate::schema::emails::dsl::{emails,email};
	let conn=pool.get()?;
	let pk=email_to_cmp.clone();
	block(move || diesel::delete(emails.find(pk)).execute(&conn)).await?;
	Ok(())
}
