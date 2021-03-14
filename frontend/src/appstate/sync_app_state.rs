use crate::appstate::app_state::AppState;
use crate::notify_primary;
use ad_buy_engine::constant::browser_storage_keys::{
    CAMPAIGNS_KEY, FUNNELS_KEY, LANDING_PAGES_KEY, OFFERS_KEY, OFFER_SOURCES, TRAFFIC_SOURCES_KEY,
};
use ad_buy_engine::data::elements::crud::{
    CRUDElementResponse, CreatableElement, PrimeElementBuild,
};
use yew::format::Json;
use yew_services::storage::Area;
use yew_services::StorageService;
use ad_buy_engine::data::elements::sync::{SyncElementResponse, SyncFlagData, SyncElementRequest, SyncElementUnit};

impl AppState {
    pub fn request_sync_elements(&self)->SyncElementRequest {
        let mut request=  SyncElementRequest {
            offer_sources: vec![],
            offers: vec![],
            landers: vec![],
            funnels: vec![],
            traffic_sources: vec![],
            campaigns: vec![]
        };
        
        for item in self.offer_sources.borrow().iter() {
            request.offer_sources.push(SyncElementUnit{
                id: item.offer_source_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
    
        for item in self.offers.borrow().iter() {
            request.offers.push(SyncElementUnit{
                id: item.offer_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
    
        for item in self.traffic_sources.borrow().iter() {
            request.traffic_sources.push(SyncElementUnit{
                id: item.traffic_source_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
    
        for item in self.landing_pages.borrow().iter() {
            request.landers.push(SyncElementUnit{
                id: item.landing_page_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
    
        for item in self.funnels.borrow().iter() {
            request.funnels.push(SyncElementUnit{
                id: item.funnel_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
    
        for item in self.campaigns.borrow().iter() {
            request.campaigns.push(SyncElementUnit{
                id: item.campaign_id.clone(),
                last_updated: item.last_updated.clone(),
            })
        }
        
        request
    }
    
    pub fn sync_update(&self, sync_response: SyncElementResponse) {
        
        for flag in sync_response.offer_sources {
            let mut local_state = self.offer_sources.borrow_mut();
            
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.offer_source_id== item.offer_source_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.offer_source_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(OFFER_SOURCES, Json(&*local_state))
        }
    
        for flag in sync_response.offers {
            let mut local_state = self.offers.borrow_mut();
        
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.offer_id== item.offer_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.offer_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(OFFERS_KEY, Json(&*local_state))
        }
    
        for flag in sync_response.traffic_sources {
            let mut local_state = self.traffic_sources.borrow_mut();
        
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.traffic_source_id== item.traffic_source_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.traffic_source_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(TRAFFIC_SOURCES_KEY, Json(&*local_state))
        }
    
        for flag in sync_response.landers {
            let mut local_state = self.landing_pages.borrow_mut();
        
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.landing_page_id== item.landing_page_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.landing_page_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(LANDING_PAGES_KEY, Json(&*local_state))
        }
    
        for flag in sync_response.funnels {
            let mut local_state = self.funnels.borrow_mut();
        
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.funnel_id== item.funnel_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.funnel_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(FUNNELS_KEY, Json(&*local_state))
        }
    
        for flag in sync_response.campaigns {
            let mut local_state = self.campaigns.borrow_mut();
        
            match flag {
                SyncFlagData::Update(item)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.campaign_id== item.campaign_id) {
                        local_state.remove(pos);
                        local_state.insert(pos, item);
                    }
                }
                SyncFlagData::Insert(item)=>{
                    local_state.push(item);
                }
                SyncFlagData::Remove(id)=>{
                    if let Some(pos)=local_state.iter().position(|s|s.campaign_id == id){
                        local_state.remove(pos);
                    }
                }
                SyncFlagData::Nothing =>{}
            }
            StorageService::new(Area::Local)
                .expect("f43sa")
                .store(CAMPAIGNS_KEY, Json(&*local_state))
        }

    }
}
