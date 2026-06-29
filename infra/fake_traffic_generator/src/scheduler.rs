use std::time::Duration;

use crate::config::RunConfig;
use crate::profiles::DeterministicRng;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScheduledSession {
    pub session_index: u64,
    pub user_index: u32,
    pub start_after: Duration,
}

pub fn build_schedule(config: &RunConfig) -> Vec<ScheduledSession> {
    let mut rng = DeterministicRng::new(config.seed ^ 0x5252_aaaa_1919_efef);
    let mut schedule = Vec::new();
    let interval_ms = duration_millis(config.interval);
    let jitter_window_ms = interval_ms * u128::from(config.jitter_percent) / 100;

    for session_index in 0..config.total_requested_sessions() {
        let user_index = (session_index % u64::from(config.users)) as u32;
        let base_delay_ms = u128::from(session_index) * interval_ms;
        let delay_ms = jittered_delay(base_delay_ms, jitter_window_ms, &mut rng);
        let start_after = duration_from_millis(delay_ms);
        if let Some(limit) = config.duration_limit
            && start_after > limit
        {
            continue;
        }
        schedule.push(ScheduledSession {
            session_index,
            user_index,
            start_after,
        });
    }

    schedule
}

fn jittered_delay(base_delay_ms: u128, jitter_window_ms: u128, rng: &mut DeterministicRng) -> u128 {
    if jitter_window_ms == 0 {
        return base_delay_ms;
    }
    let window = bounded_u128(jitter_window_ms.saturating_mul(2).saturating_add(1));
    let offset = i128::from(rng.bounded_u64(window)) - i128::from(bounded_u128(jitter_window_ms));
    base_delay_ms.saturating_add_signed(offset)
}

fn bounded_u128(value: u128) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn duration_millis(duration: Duration) -> u128 {
    duration.as_millis()
}

fn duration_from_millis(milliseconds: u128) -> Duration {
    let milliseconds = u64::try_from(milliseconds).unwrap_or(u64::MAX);
    Duration::from_millis(milliseconds)
}
