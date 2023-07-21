use chrono::{DateTime, Datelike, NaiveDateTime, Utc, Weekday};
use chrono::{FixedOffset, Timelike};
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnixTimePayload {
    unixtime: i64,
}

pub enum TimeZone {
    UTC,
    JST,
}

fn format_weekday(weekday: Weekday, timezone: &TimeZone) -> String {
    match timezone {
        TimeZone::UTC => match weekday {
            Weekday::Mon => "Mon".to_string(),
            Weekday::Tue => "Tue".to_string(),
            Weekday::Wed => "Wed".to_string(),
            Weekday::Thu => "Thu".to_string(),
            Weekday::Fri => "Fri".to_string(),
            Weekday::Sat => "Sat".to_string(),
            Weekday::Sun => "Sun".to_string(),
        },
        TimeZone::JST => match weekday {
            Weekday::Mon => "月".to_string(),
            Weekday::Tue => "火".to_string(),
            Weekday::Wed => "水".to_string(),
            Weekday::Thu => "木".to_string(),
            Weekday::Fri => "金".to_string(),
            Weekday::Sat => "土".to_string(),
            Weekday::Sun => "日".to_string(),
        },
    }
}

fn format_datetime(datetime: DateTime<Utc>, timezone: TimeZone) -> String {
    let formatted_datetime: DateTime<FixedOffset> = match timezone {
        TimeZone::UTC => datetime.with_timezone(&FixedOffset::east(0)),
        TimeZone::JST => {
            let tokyo = FixedOffset::east_opt(9 * 3600).unwrap_or(FixedOffset::east(0));
            datetime.with_timezone(&tokyo)
        }
    };
    let time = formatted_datetime.time();
    let (hour, minute, second) = (time.hour(), time.minute(), time.second());

    format!(
        "{}/{}/{}({}) {:02}:{:02}:{:02}",
        formatted_datetime.year(),
        formatted_datetime.month(),
        formatted_datetime.day(),
        format_weekday(formatted_datetime.weekday(), &timezone),
        hour,
        minute,
        second
    )
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::ok("{\"error\": \"Method Not Allowed\"}".to_string());
    }
    let full_path = req.path();
    let unixtime_str = full_path.trim_start_matches('/');
    let unixtime = if let Ok(value) = unixtime_str.parse::<i64>() {
        value
    } else {
        return Response::ok("{\"error\": \"Invalid unixtime\"}".to_string());
    };

    let naive = if let Some(value) = NaiveDateTime::from_timestamp_opt(unixtime, 0) {
        value
    } else {
        return Response::ok("{\"error\": \"Invalid timestamp\"}".to_string());
    };
    let datetime = DateTime::<Utc>::from_utc(naive, Utc);

    let date_str = format_datetime(datetime, TimeZone::UTC);

    Response::ok(date_str)
}
