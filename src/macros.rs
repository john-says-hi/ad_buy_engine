#[cfg(feature = "backend")]
#[macro_export]
macro_rules! impl_database_communication {
	($($model:ty, $table:ident)*) => {
		$(
		impl DatabaseCommunication<$model> for $model {
			fn new(new: $model, conn:&PgConnection)->QueryResult<usize> {
				diesel::insert_into(crate::schema::$table::dsl::$table)
					.values(&new)
					.execute(conn)
			}

			fn delete(model_id: String, conn:&PgConnection)->QueryResult<usize> {
				diesel::delete(crate::schema::$table::dsl::$table.find(model_id))
					.execute(conn)
			}

			fn update(model_id: String, new: $model, conn:&PgConnection)->QueryResult<usize> {
			diesel::update(crate::schema::$table::dsl::$table.find(model_id))
				.set(new)
				.execute(conn)
			}

			fn get(model_id: String, conn:&PgConnection)->QueryResult<$model> {
				crate::schema::$table::dsl::$table.find(model_id)
					.get_result(conn)
			}

			fn update_and_get(model_id: String, new: $model, conn:&PgConnection)->QueryResult<$model> {
				diesel::update(crate::schema::$table::dsl::$table.find(model_id))
					.set(&new)
					.get_result(conn)
			}

			fn delete_all(conn:&PgConnection)->QueryResult<usize> {
				diesel::delete(crate::schema::$table::dsl::$table)
					.execute(conn)
			}

			fn all(conn:&PgConnection)->QueryResult<Vec<$model>> {
				crate::schema::$table::dsl::$table.load::<$model>(conn)
			}
		}
		)*
	};
}

#[cfg(feature = "backend")]
#[macro_export]
macro_rules! impl_accountable_database_communication {
	($($model:ty, $table:ident)*) => {
		$(
		impl AccountableDBComm<$model> for $model {
			fn all_by_last_updated(acc_id: String, conn:&PgConnection)->QueryResult<Vec<$model>> {
				crate::schema::$table::dsl::$table.filter(crate::schema::$table::dsl::account_id.eq(acc_id))
					.order(crate::schema::$table::dsl::last_updated.desc())
					.load::<$model>(conn)
			}

			fn all_for_account(acc_id: String, conn:&PgConnection)->QueryResult<Vec<$model>> {
				crate::schema::$table::dsl::$table.filter(crate::schema::$table::dsl::account_id.eq(acc_id))
					.load::<$model>(conn)
			}

			fn delete_all_for_account(acc_id: String, conn:&PgConnection)->QueryResult<usize> {
				diesel::delete(crate::schema::$table::dsl::$table.filter(crate::schema::$table::dsl::account_id.eq(acc_id)))
					.execute(conn)
			}
		}
		)*
	};
}

#[macro_export]
macro_rules! impl_accountable {
	($($model:ty)*) => {
		$(impl Accountable for $model {})*
	};
}

#[macro_export]
macro_rules! pr {
    ($str:expr) => {
        println!("{}", $str)
    };
    ($pre:expr, $str:expr) => {
        println!("{} {}", $pre, $str)
    };
    (:?$type:expr) => {
        println!("{:?}", $type)
    };
    ($pre:expr, :?$type:expr) => {
        println!("{} {:?}", $pre, $type)
    };
}

#[macro_export]
macro_rules! from_json_string {
    ($string:expr => $type:ty) => {
    let err_fmt = format!("expr: {}; 4dfg", &$string);
    serde_json::from_str::<$type>(&$string).expect(err_fmt.as_str());
    };

    ( $($name:ident; $string:expr => $type:ty)+ ) => {
	        $(
	        let err_fmt = format!("expr: {}; 453g", &$string);
	        let $name = serde_json::from_str::<$type>(&$string).expect(err_fmt.as_str());
	        )+
    };
}

#[macro_export]
macro_rules! to_json_string {
    ($type:expr) => {serde_json::to_string(&$type).expect("mac_vrt4")};

    ( $($name:ident; $type:expr)+ ) => {
        $(let $name = serde_json::to_string(&$type).expect("mac_vrt4");)+
    }
}

#[macro_export]
macro_rules! arc {
    ($type:expr) => {
        Arc::clone(&$type)
    };
}

#[macro_export]
macro_rules! wrap_arc {
    ($type:expr) => {
        Arc::new(RwLock::new($type))
    };
}
