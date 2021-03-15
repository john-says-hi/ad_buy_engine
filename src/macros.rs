#[macro_export]
macro_rules! pr {
	($str:expr) => {println!("{}", $str)};
	($pre:expr, $str:expr) => {println!("{} {}", $pre, $str)};
	(:?$type:expr) => {println!("{:?}", $type)};
	($pre:expr, :?$type:expr) => {println!("{} {:?}", $pre, $type)};
}

#[macro_export]
macro_rules! from_json_string {
    ($string:expr => $type:ty) => {	serde_json::from_str::<$type>(&$string).expect("mac_f543rs")};
    
    ( $($name:ident; $string:expr => $type:ty)+ ) => {
	        $(let $name = serde_json::from_str::<$type>(&$string).expect("mac_f543rs");)+
    };
}

#[macro_export]
macro_rules! to_json_string {
    ($type:expr) => {serde_json::to_string(&$type).expect("mac_vrt4")};
    
    ( $($name:ident; $type:expr)+ ) => {
        $(let $name = serde_json::to_string(&$type).expect("mac_vrt4");)+
    }
}
