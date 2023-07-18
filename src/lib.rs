use chrono::{NaiveDateTime, Datelike, Utc, Weekday, DateTime};
use serde::{Serialize, Deserialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnixTimePayload {
    unixtime: i64,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    //Response::ok("test".to_string())
    if req.method() != Method::Post {
        return Response::ok("{\"error\": \"Method Not Allowed\"}".to_string());
    }
    //Response::ok("test".to_string())
    let full_path = req.path();
    let unixtime_str = full_path.trim_start_matches('/');
    let unixtime: i64 = unixtime_str.parse().unwrap();

    let naive = NaiveDateTime::from_timestamp(unixtime, 0);
    let datetime = DateTime::<Utc>::from_utc(naive, Utc);

    let day = match datetime.weekday() {
        Weekday::Mon => "月曜日",
        Weekday::Tue => "火曜日",
        Weekday::Wed => "水曜日",
        Weekday::Thu => "木曜日",
        Weekday::Fri => "金曜日",
        Weekday::Sat => "土曜日",
        Weekday::Sun => "日曜日",
    };

    let date_str = format!("{}/{}/{}({})", datetime.year(), datetime.month(), datetime.day(), day);

    Response::ok(date_str)
}
