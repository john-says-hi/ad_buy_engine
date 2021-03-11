use crate::appstate::app_state::{AppState, STATE};
use crate::components::page_utilities::crud_element::complex_sub_component::plus_button::PlusButton;
use crate::components::page_utilities::crud_element::crud_funnels::ActiveElement;
use crate::components::page_utilities::crud_element::dropdowns::condition_dropdown::ConditionDropdown;
use crate::components::page_utilities::crud_element::dropdowns::connection_type_dropdown::ConnectionTypeDropdown;
use crate::components::page_utilities::crud_element::dropdowns::country_dropdown::CountryDropdown;
use crate::components::page_utilities::crud_element::dropdowns::device_type_dropdown::DeviceTypeDropdown;
use crate::components::page_utilities::crud_element::dropdowns::iso_language_dropdown::ISOLanguageDropdown;
use crate::components::page_utilities::crud_element::dropdowns::referrer_handling_dropdown::ReferrerHandlingDropdown;
use crate::components::page_utilities::crud_element::dropdowns::time_zone_dropdown::TimeZoneDropdown;
use crate::components::page_utilities::crud_element::dropdowns::week_day_dropdown::WeekDayDropdown;
use crate::components::page_utilities::crud_element::notes::NotesComponent;
use crate::utils::javascript::js_bindings::{
    hide_uk_drop, hide_uk_modal, show_uk_drop, show_uk_modal, toggle_uk_dropdown,
};
use crate::{notify_danger, notify_warning};
use ad_buy_engine::data::elements::funnel::{ConditionalSequence, Sequence, SequenceType};
use ad_buy_engine::data::lists::condition::{
    Condition, ConditionDataType, ConnectionType, Weekdays,
};
use ad_buy_engine::data::lists::country::ISOCountry;
use ad_buy_engine::data::lists::referrer_handling::ReferrerHandling;
use ad_buy_engine::data::lists::time_zone::TimeZone;
use ad_buy_engine::data::lists::DeviceType;
use ad_buy_engine::{Country, ISOLanguage};
use boyer_moore_magiclen::BMByteSearchable;
use either::Either;
use std::cell::RefCell;
use std::io::Lines;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;
use web_sys::Element;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_material::MatSwitch;
use yew_services::storage::Area;
use yew_services::StorageService;

pub enum ConditionData {
    Text(InputData),
    ConnectionType(ConnectionType),
    Country(ISOCountry),
    Device(DeviceType),
    Language(ISOLanguage),
    TimeOfDay((Option<Either<InputData, InputData>>, Option<TimeZone>)),
    WeekDays((Option<Weekdays>, Option<TimeZone>)),
}

pub enum Msg {
    NewCondition,
    UpdateConditionalSequenceName(InputData),
    SubmitCondition,
    UpdateCondition(usize),
    UpdateActiveConditionData(ConditionData),
    RemoveCondition(usize),
    SelectCondition(ConditionDataType),
    ToggleInclusion,
    OnBurUpdateCondition,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub update_name: Callback<InputData>,
    pub update_conditions: Callback<Vec<Condition>>,
    pub conditional_sequence_name: String,
    pub conditions: Vec<Condition>,
}

#[derive(PartialEq)]
enum Mode {
    Edit,
    New,
}

pub struct ConditionView {
    link: ComponentLink<Self>,
    props: Props,
    conditions: Vec<Condition>,
    active_condition: Option<Condition>,
    tmp_buff: String,
    drop_node_ref: NodeRef,
    mode: Mode,
}

impl Component for ConditionView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let conditions = props.conditions.clone();

        Self {
            link,
            props,
            conditions,
            active_condition: None,
            tmp_buff: "".to_string(),
            drop_node_ref: Default::default(),
            mode: Mode::New,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewCondition => self.mode = Mode::New,

            Msg::OnBurUpdateCondition => {
                if let Some(condi) = self.active_condition.as_mut() {
                    let c_dt = condi.condition_data_type.clone();
                    let mut dt = &mut condi.condition_data_type;
                    match c_dt {
                        ConditionDataType::Brand(_) => {
                            *dt = ConditionDataType::Brand(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Browser(_) => {
                            *dt = ConditionDataType::Browser(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::City(_) => {
                            *dt = ConditionDataType::City(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable1(_) => {
                            *dt = ConditionDataType::Variable1(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable2(_) => {
                            *dt = ConditionDataType::Variable2(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable3(_) => {
                            *dt = ConditionDataType::Variable3(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable4(_) => {
                            *dt = ConditionDataType::Variable4(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable5(_) => {
                            *dt = ConditionDataType::Variable5(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable6(_) => {
                            *dt = ConditionDataType::Variable6(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable7(_) => {
                            *dt = ConditionDataType::Variable7(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable8(_) => {
                            *dt = ConditionDataType::Variable8(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable9(_) => {
                            *dt = ConditionDataType::Variable9(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Variable10(_) => {
                            *dt = ConditionDataType::Variable10(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::MobileCarrier(_) => {
                            *dt = ConditionDataType::MobileCarrier(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::OperatingSystem(_) => {
                            *dt = ConditionDataType::OperatingSystem(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }

                        ConditionDataType::Proxy(_) => {
                            *dt = ConditionDataType::Proxy(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::ReferrerDomain(_) => {
                            let mut list = vec![];
                            for i in self.tmp_buff.lines() {
                                if let Ok(url) = Url::parse(i) {
                                    list.push(url)
                                } else {
                                    notify_warning(format!("URL Parse Err at: {}", i).as_str())
                                }
                            }
                            *dt = ConditionDataType::ReferrerDomain(list);
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Referrer(_) => {
                            let mut list = vec![];
                            for i in self.tmp_buff.lines() {
                                if let Ok(url) = Url::parse(i) {
                                    list.push(url)
                                } else {
                                    notify_warning(format!("URL Parse Err at: {}", i).as_str())
                                }
                            }
                            *dt = ConditionDataType::Referrer(list);
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::Region(_) => {
                            *dt = ConditionDataType::Region(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }
                        ConditionDataType::ISP(_) => {
                            *dt = ConditionDataType::ISP(
                                self.tmp_buff
                                    .lines()
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>(),
                            );
                            self.tmp_buff.clear();
                        }

                        _ => {}
                    }
                }
            }

            Msg::UpdateActiveConditionData(new_condition_data) => {
                if let Some(active_condition) = self.active_condition.as_mut() {
                    let condi_data_type = &mut active_condition.condition_data_type;
                    match condi_data_type.clone() {
                        ConditionDataType::Brand(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Browser(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::City(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::ConnectionType(mut extracted_condition_data) => {
                            if let ConditionData::ConnectionType(conn_type) = new_condition_data {
                                if let Some(pos) = extracted_condition_data
                                    .iter()
                                    .position(|s| s == &conn_type)
                                {
                                    extracted_condition_data.remove(pos);
                                    *condi_data_type =
                                        ConditionDataType::ConnectionType(extracted_condition_data);
                                } else {
                                    extracted_condition_data.push(conn_type);
                                    *condi_data_type =
                                        ConditionDataType::ConnectionType(extracted_condition_data);
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Country(mut extracted_condition_data) => {
                            if let ConditionData::Country(iso_country) = new_condition_data {
                                if let Some(pos) = extracted_condition_data
                                    .iter()
                                    .position(|s| s == &iso_country)
                                {
                                    extracted_condition_data.remove(pos);
                                    *condi_data_type =
                                        ConditionDataType::Country(extracted_condition_data);
                                } else {
                                    extracted_condition_data.push(iso_country);
                                    *condi_data_type =
                                        ConditionDataType::Country(extracted_condition_data);
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable1(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable2(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable3(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable4(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable5(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable6(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable7(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable8(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable9(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Variable10(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }
                        ConditionDataType::Proxy(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::DeviceType(mut extracted_condition_data) => {
                            if let ConditionData::Device(device) = new_condition_data {
                                if let Some(pos) =
                                    extracted_condition_data.iter().position(|s| s == &device)
                                {
                                    extracted_condition_data.remove(pos);
                                } else {
                                    extracted_condition_data.push(device);
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type =
                                ConditionDataType::DeviceType(extracted_condition_data);
                        }

                        ConditionDataType::IP(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                extracted_condition_data.clear();
                                for (line_idx, line) in i.value.lines().enumerate() {
                                    let mut is_left = true;
                                    let mut left_ip = None;
                                    let mut right_ip = None;

                                    for ip in line.split('-') {
                                        if is_left {
                                            if let Ok(res_ip) = IpAddr::from_str(&ip.trim()) {
                                                left_ip = Some(res_ip);
                                            } else {
                                                notify_warning(
                                                    format!("LHS IP Parse Error").as_str(),
                                                )
                                            }
                                            is_left = false;
                                        } else if let Ok(res_ip) = IpAddr::from_str(&ip.trim()) {
                                            right_ip = Some(res_ip);
                                        } else {
                                            notify_warning(format!("RHS IP Parse Error").as_str())
                                        }
                                    }

                                    if let Some(left_ip) = left_ip {
                                        if let Some(right_ip) = right_ip {
                                            extracted_condition_data.push((left_ip, right_ip));
                                        } else {
                                            notify_warning(
                                                "Right Hand IP Parse Failed: Please Try Again.",
                                            )
                                        }
                                    } else {
                                        notify_warning(
                                            "Left Hand IP Parse Failed: Please Try Again.",
                                        )
                                    }
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type = ConditionDataType::IP(extracted_condition_data);
                        }

                        ConditionDataType::ISP(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Language(mut extracted_condition_data) => {
                            if let ConditionData::Language(iso_lang) = new_condition_data {
                                if let Some(pos) =
                                    extracted_condition_data.iter().position(|s| s == &iso_lang)
                                {
                                    extracted_condition_data.remove(pos);
                                } else {
                                    extracted_condition_data.push(iso_lang);
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type =
                                ConditionDataType::Language(extracted_condition_data);
                        }

                        ConditionDataType::MobileCarrier(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::OperatingSystem(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::Referrer(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::ReferrerDomain(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type =
                                ConditionDataType::ReferrerDomain(extracted_condition_data);
                        }

                        ConditionDataType::Region(mut extracted_condition_data) => {
                            if let ConditionData::Text(i) = new_condition_data {
                                self.tmp_buff = i.value;
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                        }

                        ConditionDataType::TimeOfDay(mut extracted_condition_data) => {
                            if let ConditionData::TimeOfDay(tod_data) = new_condition_data {
                                match tod_data {
                                    (Some(Either::Left(i)), None) => {
                                        let parts = i.value.split(":").collect::<Vec<&str>>();
                                        let mut first_part_one: u8 = 00;
                                        let mut first_part_two: u8 = 00;

                                        if parts.len() > 2 {
                                            notify_warning(
                                                "More than 2 parts detected. Format by \"HH:MM\"",
                                            )
                                        } else if parts.len() != 2 {
                                            notify_warning(
                                                "2 parts note detected. Format by \"HH:MM\"",
                                            )
                                        } else if let Ok(res_first_part_one) =
                                            parts[0].parse::<u8>()
                                        {
                                            first_part_one = res_first_part_one;

                                            if let Ok(res_first_part_two) = parts[1].parse::<u8>() {
                                                first_part_two = res_first_part_two;

                                                extracted_condition_data.0 =
                                                    format!("{}:{}", first_part_one, first_part_two)
                                            } else {
                                                notify_warning("Second part (Hours(i.e. \":30\")) failed parse")
                                            }
                                        } else {
                                            notify_warning(
                                                "First part (Hours(i.e. \"22:\")) failed parse",
                                            )
                                        }
                                    }

                                    (Some(Either::Right(i)), None) => {
                                        let parts = i.value.split(":").collect::<Vec<&str>>();
                                        let mut second_part_one: u8 = 00;
                                        let mut second_part_two: u8 = 00;

                                        if parts.len() > 2 {
                                            notify_warning(
                                                "More than 2 parts detected. Format by \"HH:MM\"",
                                            )
                                        } else if parts.len() != 2 {
                                            notify_warning(
                                                "2 parts note detected. Format by \"HH:MM\"",
                                            )
                                        } else if let Ok(res_second_part_one) =
                                            parts[0].parse::<u8>()
                                        {
                                            second_part_one = res_second_part_one;

                                            if let Ok(res_second_part_two) = parts[1].parse::<u8>()
                                            {
                                                second_part_two = res_second_part_two;

                                                extracted_condition_data.1 = format!(
                                                    "{}:{}",
                                                    second_part_one, second_part_two
                                                )
                                            } else {
                                                notify_warning("Second part (Hours(i.e. \":30\")) failed parse")
                                            }
                                        } else {
                                            notify_warning(
                                                "First part (Hours(i.e. \"22:\")) failed parse",
                                            )
                                        }
                                    }

                                    (None, Some(tz)) => {
                                        extracted_condition_data.2 = tz;
                                    }

                                    _ => {}
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type =
                                ConditionDataType::TimeOfDay(extracted_condition_data);
                        }

                        ConditionDataType::Weekday(mut extracted_condition_data) => {
                            if let ConditionData::WeekDays(data) = new_condition_data {
                                match data {
                                    (None, Some(tz)) => {
                                        extracted_condition_data.1 = tz;
                                    }

                                    (Some(day), None) => {
                                        if let Some(pos) = extracted_condition_data
                                            .0
                                            .iter()
                                            .position(|s| s == &day)
                                        {
                                            extracted_condition_data.0.remove(pos);
                                        } else {
                                            extracted_condition_data.0.push(day);
                                        }
                                    }

                                    _ => {}
                                }
                            } else {
                                notify_danger("Err: Wrong Condition Data Type")
                            }
                            *condi_data_type = ConditionDataType::Weekday(extracted_condition_data);
                        }

                        _ => {}
                    }
                } else {
                    notify_danger("Err: No ActiveCondition Found")
                }
            }

            Msg::ToggleInclusion => {
                if let Some(mut condi) = self.active_condition.as_mut() {
                    condi.include = !condi.include
                }
            }

            Msg::SelectCondition(condi) => {
                self.active_condition = Some(Condition {
                    condition_data_type: condi,
                    include: true,
                });
            }

            Msg::RemoveCondition(pos) => {
                self.conditions.remove(pos);
                self.props.update_conditions.emit(self.conditions.clone());
            }

            Msg::UpdateCondition(pos) => {
                self.mode = Mode::Edit;
                self.active_condition = self.conditions.get(pos).cloned();
                let element = self.drop_node_ref.cast::<Element>().expect("getr");
                show_uk_drop(element);
            }

            Msg::SubmitCondition => {
                let element = self.drop_node_ref.cast::<Element>().expect("getr");

                if let Some(active_condi) = &self.active_condition {
                    if let Some(pos) = self.conditions.iter().position(|s| {
                        s.condition_data_type.to_string()
                            == active_condi.condition_data_type.to_string()
                    }) {
                        if Mode::New == self.mode {
                            self.conditions.push(active_condi.clone());
                        } else {
                            self.conditions.remove(pos);
                            self.conditions.insert(pos, active_condi.clone());
                        }
                    } else {
                        self.conditions.push(active_condi.clone());
                    }
                    self.active_condition=None;
                    hide_uk_drop(element);
                } else {
                    notify_danger("Err: No active condition")
                }

                self.props.update_conditions.emit(self.conditions.clone());
            }

            Msg::UpdateConditionalSequenceName(i) => self.props.update_name.emit(i),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.conditions = props.conditions.clone();
        self.active_condition = None;
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
                                <div class="uk-margin">
                                    <h2 class="uk-flex-left">{"Conditional Sequence Name"}</h2>
                                </div>

                                <div class="uk-margin-top-small uk-margin-bottom-large">
                                    <input type="text" class="uk-input" oninput=self.link.callback(Msg::UpdateConditionalSequenceName) />
                                </div>

                                <hr class="uk-divider" />

                                <div class="uk-margin-top-small uk-margin-bottom-large">
                                    <h4 class="uk-flex-left">{"Conditions"}</h4>
                                    <button class="uk-button uk-button-default uk-background-primary uk-light uk-flex-right" onclick=self.link.callback(|_| Msg::NewCondition) ><span class="fas fa-plus uk-margin-small-right"></span>{"New Condition"}</button>
                                    {self.render_drop()}
                                </div>

                                {self.render_conditions()}

        </>
        }
    }
}

impl ConditionView {
    pub fn render_condition_configuration(&self) -> VNode {
        if let Some(condition) = &self.active_condition {
            match &condition.condition_data_type {
                ConditionDataType::Brand(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Brands You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e.\nAndroid\niPhone 11"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Browser(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Browser/s and/or Version/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nChrome\nFirefox 86"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::City(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Cities You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nNew York\nBoston"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::ConnectionType(data) => {
                    let select_callback = self.link.callback(move |ct: ConnectionType| {
                        Msg::UpdateActiveConditionData(ConditionData::ConnectionType(ct))
                    });

                    let mut nodes = VList::new();

                    for item in data.iter() {
                        let condition_type = item.clone();
                        let btn_text = item.to_string();
                        let remove_callback = self.link.callback(move |_| {
                            Msg::UpdateActiveConditionData(ConditionData::ConnectionType(
                                condition_type,
                            ))
                        });

                        nodes.push(VNode::from(html!{
                            <button class="uk-button uk-margin-small" onclick=remove_callback>{btn_text}</button>
                        }))
                    }

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Select the Connection Type/s You Wish to Target"}</h4>
                        <div><ConnectionTypeDropdown onselect=select_callback /></div>
                        <div>{nodes}</div>
                    </div>
                    })
                }

                ConditionDataType::Country(data) => {
                    let value = data
                        .iter()
                        .map(|s| {
                            format!(
                                "{}
",
                                s
                            )
                        })
                        .collect::<String>();
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Countries You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nUS\nDE"} oninput=callback value=value ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable1(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var1\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nheadline_1\nheadline_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable2(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var2\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nimage_1\nimage_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable3(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var3\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable4(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var4\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable5(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var5\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable6(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var6\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable7(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var7\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable8(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var8\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable9(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var9\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Variable10(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) \"Var10\" You Wish to Match as a Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \ntarget_1\ntarget_2"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::DeviceType(data) => {
                    let select_callback = self.link.callback(move |dt: DeviceType| {
                        Msg::UpdateActiveConditionData(ConditionData::Device(dt))
                    });

                    let mut nodes = VList::new();

                    for item in data.iter() {
                        let condition_type = item.clone();
                        let btn_text = item.to_string();
                        let remove_callback = self.link.callback(move |_| {
                            Msg::UpdateActiveConditionData(ConditionData::Device(condition_type))
                        });

                        nodes.push(VNode::from(html!{
                            <button class="uk-button uk-margin-small" onclick=remove_callback>{btn_text}</button>
                        }))
                    }

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Select the Device Type/s You Wish to Target"}</h4>
                        <div><DeviceTypeDropdown onselect=select_callback /></div>
                        <div>{nodes}</div>
                    </div>
                    })
                }

                ConditionDataType::IP(data) => {
                    let value = data
                        .iter()
                        .map(|(lhs, rhs)| format!("{} - {}\n", lhs.to_string(), rhs.to_string()))
                        .collect::<String>();
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="">{"Type an IP Range (1 range per line) that You Wish to Target. (Format: START - END)"}</h4>
                        <h5 class="uk-margin-bottom-small">{"i.e. 192.123.0.1 - 192.123.1.69"}</h5>
                        <textarea class="uk-textarea" rows=11 placeholder={"192.168.0.1 - 192.168.1.88"} oninput=callback value=value ></textarea>
                    </div>
                    })
                }

                ConditionDataType::ISP(data) => {
                    let value = if self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) ISPs You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nComcast\nCharter"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Language(data) => {
                    let select_callback = self.link.callback(move |dt: ISOLanguage| {
                        Msg::UpdateActiveConditionData(ConditionData::Language(dt))
                    });

                    let mut nodes = VList::new();

                    for item in data.iter() {
                        let condition_type = item.clone();
                        let btn_text = item.name().to_string();
                        let remove_callback = self.link.callback(move |_| {
                            Msg::UpdateActiveConditionData(ConditionData::Language(condition_type))
                        });

                        nodes.push(VNode::from(html!{
                            <button class="uk-button uk-margin-small" onclick=remove_callback>{btn_text}</button>
                        }))
                    }

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Select the Language/s that You Wish to Target"}</h4>
                        <div><ISOLanguageDropdown onselect=select_callback /></div>
                        <div>{nodes}</div>
                    </div>
                    })
                }

                ConditionDataType::MobileCarrier(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Mobile Carrier/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nAT&T\nVerizon"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::OperatingSystem(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Operating System/s and/or Version/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nWindows 10\niOS"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Proxy(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Proxies/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nNord VPN\nHMA"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Referrer(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Referrer/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nhttp://xyz.com/profile_1.whatever\nhttps://something.com/juicy_target.okay"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::ReferrerDomain(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter()
                            .map(|s| {
                                format!(
                                    "{}
",
                                    s
                                )
                            })
                            .collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Referrer Domain/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nhttp://xyz.com/\nhttps://something.com/"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::Region(data) => {
                    let value = if !self.tmp_buff.is_empty() {
                        self.tmp_buff.clone()
                    } else {
                        data.iter().map(|s| format!("{}\n", s)).collect::<String>()
                    };
                    let callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::Text(i))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Type or Paste (from report) the Region/s You Wish to Target"}</h4>
                        <textarea class="uk-textarea" rows=11 placeholder={"i.e. \nOregon\nFrankfurt"} oninput=callback value=value onblur=self.link.callback(|_|Msg::OnBurUpdateCondition) ></textarea>
                    </div>
                    })
                }

                ConditionDataType::TimeOfDay((lhs, rhs, tz)) => {
                    let selected = tz.clone();
                    let lhs_value = lhs.clone();
                    let rhs_value = rhs.clone();
                    let lhs_callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::TimeOfDay((
                            Some(Either::Left(i)),
                            None,
                        )))
                    });
                    let rhs_callback = self.link.callback(move |i: InputData| {
                        Msg::UpdateActiveConditionData(ConditionData::TimeOfDay((
                            Some(Either::Right(i)),
                            None,
                        )))
                    });
                    let onselect = self.link.callback(move |tz: TimeZone| {
                        Msg::UpdateActiveConditionData(ConditionData::TimeOfDay((None, Some(tz))))
                    });

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Please Type a Time Range You Wish to Target, Formatted as \"HOUR:MINUTE - HOUR:MINUTE\", Military Time i.e. \"03:45 - 22:45\""}</h4>
                        <div><span>{"Between  "}</span><input type="text" class="uk-input" placeholder="00:00" oninput=lhs_callback value=lhs_value /><span>{" And  "}</span><input type="text" class="uk-input" placeholder="23:59" oninput=rhs_callback value=rhs_value /></div>
                        <div class="uk-margin-top"><TimeZoneDropdown onselect=onselect selected=selected /></div>
                    </div>
                    })
                }

                ConditionDataType::Weekday((days, tz)) => {
                    let selected = tz.clone();
                    let time_zone_select = self.link.callback(move |tz: TimeZone| {
                        Msg::UpdateActiveConditionData(ConditionData::WeekDays((None, Some(tz))))
                    });
                    let week_day_select = self.link.callback(move |day: Weekdays| {
                        Msg::UpdateActiveConditionData(ConditionData::WeekDays((Some(day), None)))
                    });

                    let mut nodes = VList::new();

                    for item in days.iter() {
                        let condition_type = item.clone();
                        let btn_text = item.to_string();
                        let remove_callback = self.link.callback(move |_| {
                            Msg::UpdateActiveConditionData(ConditionData::WeekDays((
                                Some(condition_type),
                                None,
                            )))
                        });

                        nodes.push(VNode::from(html! {
                            <button class="uk-button uk-margin-small" onclick=remove_callback>{btn_text}</button>
                        }))
                    }

                    VNode::from(html! {
                    <div class="uk-margin">
                        <h4 class="uk-margin-bottom-small">{"Please Select the Week Days You Wish to Target"}</h4>
                        <WeekDayDropdown onselect=week_day_select />
                        <h5>{"Selected Weekdays:"}</h5>
                        <div>{nodes}</div>
                        <div class="uk-margin-top"><TimeZoneDropdown onselect=time_zone_select selected=selected /></div>
                    </div>
                    })
                }

                _ => {
                    html! {}
                }
            }
        } else {
            VNode::from(html! {})
        }
    }

    pub fn render_drop(&self) -> VNode {
        let selected = if let Some(selected) = &self.active_condition {
            Some(selected.condition_data_type.clone())
        } else {
            None
        };

        VNode::from(html! {
        <div class="uk-flex-top uk-width-large" uk-drop="mode: click;pos: left-center" ref=self.drop_node_ref.clone() >
           <div class="uk-modal-dialog uk-margin-auto-vertical" uk-overflow-auto="">
              <div class="uk-modal-body" >

                   <div class="uk-grid-column-collapse uk-grid-collapse uk-child-width-1-1 uk-drop-grid" uk-grid="">

                        <div class="uk-margin">
                            <ConditionDropdown onselect=self.link.callback(Msg::SelectCondition) selected=selected />
                            {self.render_inclusion_btn()}
                        </div>

                            {self.render_condition_configuration()}

                   </div>

                 <div class="uk-modal-footer uk-text-right">
                 //    <button class="uk-button uk-button-default uk-modal-close" type="button">{"Cancel"}</button>
                    <button onclick=self.link.callback(|_|Msg::SubmitCondition) class="uk-button uk-button-primary" type="button">{"Save"}</button>
                 </div>
              </div>
           </div>
        </div>
        })
    }

    pub fn render_inclusion_btn(&self) -> VNode {
        let mut style = "";
        let mut btn_txt = "";

        if let Some(selected) = &self.active_condition {
            if selected.include {
                style = "border:2px solid green;";
                btn_txt = "Whitelist";
            } else {
                style = "border:2px solid red;";
                btn_txt = "Blacklist";
            }
        }

        if let Some(_) = self.active_condition {
            VNode::from(html! {
                <button onclick=self.link.callback(|_|Msg::ToggleInclusion) class="uk-button uk-margin-left" style=style>{btn_txt}</button>
            })
        } else {
            html! {}
        }
    }

    pub fn render_conditions(&self) -> VNode {
        let mut nodes = VList::new();

        for (idx, condition) in self.conditions.iter().enumerate() {
            let label_class = if condition.include {
                "uk-label uk-label-success uk-margin-small-right"
            } else {
                "uk-label uk-label-danger uk-margin-small-right"
            };
            let condi_type = condition.condition_data_type.to_string();

            nodes.push(html! {
             <div class="uk-margin-remove uk-grid-column-collapse uk-grid-collapse uk-child-width-1-2" uk-grid="">
             
                 <div class="uk-flex-left">
                    <span class={label_class} >{condi_type}</span>
                 </div>
                 
                 <div class="uk-flex-right">
                    <span onclick=self.link.callback(move |_| Msg::UpdateCondition(idx)) class="uk-margin-small-right" uk-icon="pencil"></span>
                    <span onclick=self.link.callback(move |_| Msg::RemoveCondition(idx)) uk-icon="trash"></span>
                 </div>
                
            </div>
            })
        }

        VNode::from(nodes)
    }
}
