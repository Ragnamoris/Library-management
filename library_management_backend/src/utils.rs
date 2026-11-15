use time::format_description::well_known::Rfc3339;
use time::UtcDateTime;

pub fn encode_datetime(value: UtcDateTime) -> String {
    value.format(&Rfc3339).expect("datetime formatting failed")
}

pub fn encode_optional_datetime(value: Option<UtcDateTime>) -> Option<String> {
    value.map(encode_datetime)
}
