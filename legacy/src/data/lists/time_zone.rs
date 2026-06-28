use strum::IntoEnumIterator;

#[derive(Deserialize, Serialize, Copy, Clone, EnumString, ToString, EnumIter, PartialEq, Debug)]
pub enum TimeZone {
    UTC,
    #[strum(serialize = "UTC")]
    UTCPlus1,
    #[strum(serialize = "UTC + 1")]
    UTCPlus2,
    #[strum(serialize = "UTC + 2")]
    UTCPlus3,
    #[strum(serialize = "UTC + 3")]
    UTCPlus4,
    #[strum(serialize = "UTC + 4")]
    UTCPlus5,
    #[strum(serialize = "UTC + 5")]
    UTCPlus6,
    #[strum(serialize = "UTC + 6")]
    UTCPlus7,
    #[strum(serialize = "UTC + 7")]
    UTCPlus8,
    #[strum(serialize = "UTC + 8")]
    UTCPlus9,
    #[strum(serialize = "UTC + 9")]
    UTCPlus10,
    #[strum(serialize = "UTC + 10")]
    UTCPlus11,
    #[strum(serialize = "UTC + 12")]
    UTCPlus12,
    #[strum(serialize = "UTC - 11")]
    UTCMinus11,
    #[strum(serialize = "UTC - 10")]
    UTCMinus10,
    #[strum(serialize = "UTC - 9")]
    UTCMinus9,
    #[strum(serialize = "UTC - 8")]
    UTCMinus8,
    #[strum(serialize = "UTC - 7")]
    UTCMinus7,
    #[strum(serialize = "UTC - 6")]
    UTCMinus6,
    #[strum(serialize = "UTC - 5")]
    UTCMinus5,
    #[strum(serialize = "UTC - 4")]
    UTCMinus4,
    #[strum(serialize = "UTC - 3")]
    UTCMinus3,
    #[strum(serialize = "UTC - 2")]
    UTCMinus2,
    #[strum(serialize = "UTC - 1")]
    UTCMinus1,
}

impl Default for TimeZone {
    fn default() -> Self {
        TimeZone::UTC
    }
}
