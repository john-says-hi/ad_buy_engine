use crate::utils::javascript::js_bindings::remove_element_by_id;

pub mod js_bindings;

pub fn clean_all_modals() {
	remove_element_by_id("campaigns");
	remove_element_by_id("funnels");
	remove_element_by_id("landing-pages");
	remove_element_by_id("offer");
	remove_element_by_id("offer-sources");
	remove_element_by_id("traffic-sources");
}