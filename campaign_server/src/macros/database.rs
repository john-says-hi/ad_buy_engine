
#[macro_export]
macro_rules! database_functions {
	($($name:ident, $model:tt)*) => {
		$(
			pub mod $name {
				use crate::utils::errors::ApiError;
				use ad_buy_engine::data::backend_models::{
					$name::$model,
					DatabaseCommunication
				};
				use crate::utils::database::{get_conn, PgPool};
				use std::ops::Deref;
				
				pub fn all(pool: &PgPool) -> Result<Vec<$model>, ApiError> {
					Ok(<$model>::all(get_conn(pool)?.deref())?)
				}
				
				pub fn new(new: $model, pool: &PgPool) -> Result<usize, ApiError> {
					Ok(<$model>::new(new, get_conn(pool)?.deref())?)
				}
				
			pub fn delete(model_id: String, pool: &PgPool) -> Result<usize, ApiError> {
				Ok(<$model>::delete(model_id, get_conn(pool)?.deref())?)
			}
			
			pub fn update(model_id: String, new: $model, pool: &PgPool) -> Result<usize, ApiError> {
				Ok(<$model>::update(model_id, new, get_conn(pool)?.deref())?)
			}
			
			pub fn get(model_id: String, pool: &PgPool) -> Result<$model, ApiError> {
				Ok(<$model>::get(model_id, get_conn(pool)?.deref())?)
			}
			
			pub fn update_and_get(model_id: String, new: $model, pool: &PgPool) -> Result<$model, ApiError> {
				Ok(<$model>::update_and_get(model_id, new, get_conn(pool)?.deref())?)
			}
			
			pub fn delete_all(pool: &PgPool) -> Result<usize, ApiError> {
				Ok(<$model>::delete_all(get_conn(pool)?.deref())?)
			}
			}
		)*
	};
}

#[macro_export]
macro_rules! accountable_database_functions {
	() => {};
}