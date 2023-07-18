use chrono::{DateTime, Datelike, NaiveDateTime, Utc, Weekday, NaiveTime};
use chrono::Timelike;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnixTimePayload {
    unixtime: i64,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::ok("{\"error\": \"Method Not Allowed\"}".to_string());
    }
    let full_path = req.path();
    let unixtime_str = full_path.trim_start_matches('/');
    let unixtime: i64 = unixtime_str.parse().unwrap();

    let naive = NaiveDateTime::from_timestamp_opt(unixtime, 0).unwrap();
    let datetime = DateTime::<Utc>::from_utc(naive, Utc);
    let naive_time = datetime.time();

    let day = match datetime.weekday() {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    };

    let hours = naive_time.hour();
    let minutes = naive_time.minute();
    let seconds = naive_time.second();


    let date_str = format!("{}/{}/{}({}) {:02}:{:02}:{:02}", 
                           datetime.year(), 
                           datetime.month(), 
                           datetime.day(), 
                           day, 
                           hours, 
                           minutes, 
                           seconds);

    Response::ok(date_str)
}
