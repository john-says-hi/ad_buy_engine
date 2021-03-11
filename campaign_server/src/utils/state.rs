use actix_web::web::Data;
use ad_buy_engine::data::elements::campaign::Campaign;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use uuid::Uuid;

// State is just a hashmap
pub type State<'a, T> = HashMap<&'a str, T>;
// pub type AppState<'a, T> = Data<RwLock<State<'a, T>>>;
pub type AppState = Data<Mutex<HashMap<Uuid, Campaign>>>;
/// Create a new state.app_state instance and wrap in a mutex.
/// Further wrap into an Actix Data instance.
// pub fn new_state<'a, T>() -> AppState<'a, T> {
//     let state = State::<T>::new();
//     Data::new(RwLock::new(state))
// Data::new(Mutex::new(state))
// }

pub fn init_state<'a>(campaigns: Vec<Campaign>) -> Data<Mutex<HashMap<Uuid, Campaign>>> {
    let mut hm = HashMap::new();
    for camp in campaigns {
        let id = camp.campaign_id;
        hm.insert(id, camp);
    }
    Data::new(Mutex::new(hm))
}

// /// Sets an entry in the application state.app_state by key.
// /// Returns Some(T) only if the entry exists (update operation).
// /// Returns None if the entry did not alreay exist (insert operation).
// #[allow(dead_code)]
// pub fn set<'a, T>(data: AppState<'a, T>, key: &'a str, value: T) -> Option<T> {
//     let mut hashmap = data.lock().expect("Could not acquire lock");
//     hashmap.insert(key, value)
// }
//
// /// Get a copy of an application state.app_state entry by key.
// /// Returns Some(T) only if the entry exists.
// #[allow(dead_code)]
// pub fn get<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T>
// where
//     T: Clone,
// {
//     let hashmap = data.lock().expect("Could not acquire lock");
//     Some(hashmap.get(key)?.to_owned())
// }
//
// /// Removes an entry in the application state.app_state by key.
// /// Returns Some(T) only if the entry existed before removal.
// #[allow(dead_code)]
// pub fn delete<'a, T>(data: AppState<'a, T>, key: &'a str) -> Option<T> {
//     let mut hashmap = data.lock().expect("Could not acquire lock");
//     hashmap.remove(key)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::tests::helpers::tests::app_state;
//
//     #[test]
//     fn it_creates_new_application_state_and_sets_and_reads_it() {
//         let data = app_state();
//         set(data.clone(), "testing", "123".into());
//         let value = get(data, "testing");
//         assert_eq!(value, Some("123".to_string()));
//     }
//
//     #[test]
//     fn it_removes_an_entry_in_application_state() {
//         let data = app_state();
//         set(data.clone(), "testing", "123".into());
//         let value = get(data.clone(), "testing");
//         assert_eq!(value, Some("123".to_string()));
//         delete(data.clone(), "testing");
//         let value = get(data, "testing");
//         assert_eq!(value, None);
//     }
// }
