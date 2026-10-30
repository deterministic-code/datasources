use regex::Regex;

#[derive(Clone)]
pub struct DefaultValueOptions {
    pub new_id: fn() -> String,
    pub now: fn() -> String,
    pub utc_now: fn() -> String,
    pub boolean_on: String,
    pub boolean_off: String,
    pub hex: fn(&str) -> String,
    pub hex_reverse: Regex,
    pub quote: Option<fn(&str) -> String>,
    pub quote_reverse: Option<Regex>,
}
