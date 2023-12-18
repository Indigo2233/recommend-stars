use crate::utils::Star;
use chrono::LocalResult::Single;
use chrono::{Datelike, Duration, TimeZone};
use rusqlite::{params, Connection, Result};
use std::ops::{Add, AddAssign};

pub fn visible_stars(longitude: f64, latitude: f64, ways: usize, obj_type: String) -> Result<()> {
    let conn = Connection::open("stars.db")?;
    // 肉眼、双筒基于本人在仙林体验的极限星等，天文望远镜和深空摄影基于估测
    // TODO：根据观测方式和光污染等级，计算极限星等
    let limiting_mag = [0.0, 4.5, 8.5, 10.0, 12.0][ways];
    let (sunrise, sunset) = sun_rise_set_time(longitude, latitude);
    println!("日出时间：{:?}", sunrise);
    println!("日落时间：{:?}", sunset);

    let mut stmt = conn.prepare(
        "SELECT * FROM stars WHERE
         Type = ? AND MGA < ?",
    )?;
    let objects = stmt.query_map(params![&obj_type, limiting_mag], |row| {
        Ok(Star {
            id: row.get(0)?,
            name: row.get(1)?,
            longitude: row.get(3)?,
            latitude: row.get(4)?,
            obj_type: row.get(5)?,
        })
    })?;
    for obj in objects {
        println!("{:?}", obj?.name);
    }
    Ok(())
}

// 粗略计算日出日落时间
// 假设正午在时区基准经度上是12:00（实际不是）
// 误差在10min以内
fn sun_rise_set_time(longitude: f64, latitude: f64) -> (Option<f64>, Option<f64>) {
    use chrono::{Duration, Utc};
    let now = Utc::now();
    let year = (now.year() - 2000) as f64;
    // 粗略计算春分日期，方法来源：
    // https://astronomy.stackexchange.com/questions/43283/accuracy-of-calculating-the-vernal-equinox
    let spring_equinox = Utc.with_ymd_and_hms(2000, 1, 1, 5, 40, 0).unwrap()
        + Duration::seconds((79.411 * 86400.0 + 0.985647 * year * 86400.0) as i64);
    // 当前日期与春分日相隔多少天
    let t = (now - spring_equinox).num_seconds() as f64 / 86400.0 / 365.2422;

    // 今日太阳赤纬，sind(23.4333°) = 0.39768173
    // 方法来自https://zhuanlan.zhihu.com/p/265303609
    let phi_s = ((2.0 * std::f64::consts::PI * t).sin() * 0.39768173).asin();

    let latitude = latitude.to_radians();
    let r = latitude.cos();
    let h = latitude.sin() * phi_s.tan();

    // 极夜
    if h + r < 0.0 {
        return (None, Some(0.0));
    }
    // 极昼
    if h > r {
        return (Some(0.0), None);
    }

    let rad = (-h / r).acos();
    // 时区和正午时间
    let time_zone = (longitude + 7.5) / 15.0;
    let time_zone = time_zone.floor();
    let noon_time = 0.5 + (time_zone * 15.0 - longitude) / 360.0;

    let sunrise = noon_time - rad / 2.0 / std::f64::consts::PI;
    let sunset = noon_time + rad / 2.0 / std::f64::consts::PI;

    (Some(sunrise), Some(sunset))
}
