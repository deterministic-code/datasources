use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Clock {
    now: Arc<Mutex<Arc<dyn Fn() -> String + Send + Sync>>>,
    uuid: Arc<Mutex<Arc<dyn Fn() -> String + Send + Sync>>>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            now: Arc::new(Mutex::new(Arc::new(system_now))),
            uuid: Arc::new(Mutex::new(Arc::new(|| {
                "00000000-0000-0000-0000-000000000000".into()
            }))),
        }
    }

    pub fn now(&self) -> String {
        let now = self.now.lock().expect("clock").clone();
        now()
    }

    pub fn uuid(&self) -> String {
        let uuid = self.uuid.lock().expect("clock").clone();
        uuid()
    }

    pub fn set_now(&self, now: impl Fn() -> String + Send + Sync + 'static) {
        *self.now.lock().expect("clock") = Arc::new(now);
    }

    pub fn set_uuid(&self, uuid: impl Fn() -> String + Send + Sync + 'static) {
        *self.uuid.lock().expect("clock") = Arc::new(uuid);
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

fn system_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let days = (duration.as_secs() / 86_400) as i64;
    let rem = duration.as_secs() % 86_400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;
    let millis = duration.subsec_millis();
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}.{millis:03}Z")
}

fn civil_from_days(days: i64) -> (i64, u64, u64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
