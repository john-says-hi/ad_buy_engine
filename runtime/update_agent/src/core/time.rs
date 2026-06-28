use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

pub fn now_millis() -> Result<i64> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?;
    i64::try_from(elapsed.as_millis()).context("current time does not fit in i64 milliseconds")
}
