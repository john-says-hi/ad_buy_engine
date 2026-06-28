use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{ServerError, ServerResult};

pub fn now_millis() -> ServerResult<i64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ServerError::internal(format!("system clock is before epoch: {error}")))?;
    i64::try_from(duration.as_millis())
        .map_err(|error| ServerError::internal(format!("timestamp overflow: {error}")))
}
