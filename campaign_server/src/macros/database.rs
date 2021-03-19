
#[macro_export]
macro_rules! database_functions {
	($($name:ident, $modal:tt)*) => {
		$(
			pub mod $name {
			use crate::utils::errors::ApiError;
			use ad_buy_engine::data::backend_models::{
				$name::$modal,
				DatabaseCommunication
			};
			use crate::utils::database::{get_conn, PgPool};
			use std::ops::Deref;
			
			pub fn all(pool: &PgPool) -> Result<Vec<$modal>, ApiError> {
				Ok(<$modal>::all(get_conn(pool)?.deref())?)
			}
			}
		)*
	};
}

#[macro_export]
macro_rules! accountable_database_functions {
	() => {};
}