use std::f64::consts::FRAC_PI_2;
use crate::utils::{Star, AXIAL_TILT};
use chrono::{Datelike, Duration, TimeZone, Utc};
use rusqlite::{params, Connection, Result, Row};

pub fn visible_stars(_longitude: f64, latitude: f64, ways: usize, obj_type: String) -> Result<()> {
    let conn = Connection::open("stars.db")?;
    // 肉眼、双筒基于本人在仙林体验的极限星等，天文望远镜和深空摄影基于估测
    // TODO：根据观测方式和光污染等级，计算极限星等
    let limiting_mag = [0.0, 4.5, 8.5, 10.0, 12.0][ways];

    let lat_rad = latitude.to_radians();
    let (sun_ra, sun_dec) = sun_ra_dec();
    let get_row = |row: &Row| {
        // println!("{:?}", row);
        Ok(Star {
            id: row.get(0)?,
            name: row.get(1)?,
            longitude: row.get(3)?,
            latitude: row.get(4)?,
            obj_type: row.get(5)?,
        })
    };
    if (lat_rad - sun_dec).abs() > FRAC_PI_2 {
        // 极夜，只需要上中天高度足够高即可
        let mut stmt = conn.prepare(
            "SELECT * FROM stars WHERE
                Type = ?1 AND MGA < ?2 AND (DECL - ?3 < 87.5 OR DECL - ?3 > -87.5)",
        )?;
        let objects = stmt.query_map(params![&obj_type, limiting_mag, latitude], get_row)?;
        for obj in objects {
            print!("{} ", obj?.name);
        }
        return Ok(())
    }
    if (lat_rad + sun_dec).abs() > FRAC_PI_2  {
        // 极昼，只能看到太阳惹
        return Ok(())
    }
    // 当地纬线圈的半径
    let r = lat_rad.cos();
    let g = -lat_rad.sin() * sun_dec.tan();
    // 所在纬度太阳上中天到晨昏点在对应纬度圈上的弧度差
    // 由于只考虑太阳系外天体
    // 以日出前或日落后40分钟作为最晚/最早的观察时间
    let delta_dec = (g / r).acos() + 40.0 / (60.0 * 24.0) * 2.0 * std::f64::consts::PI;

    // let morning_point = sun_ra - delta_dec;
    // let evening_point = sun_ra + delta_dec;

    let stmt =
        "WITH
            MP AS (
                SELECT ?4 - ?5 AS val
            ),
            EP AS (
                SELECT ?4 + ?5 AS val
            ),
            LAT_RAD AS (
                SELECT radians(?3) AS val
            ),
            LAT AS (
                SELECT ?3 AS val
            )
        SELECT S.*, radians(S.RA * 360 / 24 - 360) > MP.val as c, radians(S.RA * 360 / 24 - 360) < MP.val as d
        FROM stars AS S, MP, EP, LAT_RAD, LAT
        WHERE
            S.Type = ?1 AND
            S.MGA < ?2 AND
            --若上中天不在白天，则计算上中天高度，否则判断晨星或昏星
            (NOT(radians(S.RA * 360 / 24) > MP.val AND radians(S.RA * 360 / 24) < EP.val OR
                --是否在晨昏点以内，或旋转一周再判断
                radians(S.RA * 360 / 24 - 360) > MP.val AND radians(S.RA * 360 / 24 - 360) < EP.val) AND
                 (S.DECL - LAT.val < 87.5 AND S.DECL - LAT.val > -87.5) OR  --上中天高于2.5度
                 (radians(S.RA * 360 / 24) > ?4 AND  --晨星or昏星
                    --计算入夜后或破晓前的高度
                     (cos(radians(S.DECL)) * cos(LAT_RAD.val) * cos(EP.val - radians(S.RA * 360 / 24)) + sin(radians(S.DECL)) * sin(LAT_RAD.val) > 0.04362) OR
                     (cos(radians(S.DECL)) * cos(LAT_RAD.val) * cos(MP.val - radians(S.RA * 360 / 24)) + sin(radians(S.DECL)) * sin(LAT_RAD.val) > 0.04362)
                 )
            )
        ";

    let mut stmt = conn.prepare(
        stmt,
    )?;
    // println!("{}, {}, {}", latitude, sun_ra, delta_dec);
    // 32.111296, 1.5458787328403245, 1.4699369466562318
    let objects = stmt.query_map(
        params![&obj_type, limiting_mag, latitude, sun_ra, delta_dec],
        get_row,
    )?;
    for obj in objects {
        print!("{} ", obj?.name);
    }
    Ok(())
}
fn spring_equinox() -> chrono::DateTime<chrono::Utc> {
    let now = Utc::now();
    let year = (now.year() - 2000) as f64;
    // 粗略计算春分日期，方法来源：
    // https://astronomy.stackexchange.com/questions/43283/accuracy-of-calculating-the-vernal-equinox
    let spring_equinox = Utc.with_ymd_and_hms(2000, 1, 1, 5, 40, 0).unwrap()
        + Duration::seconds((79.414 * 86400.0 + year * 86400.0 * 365.24228) as i64);
    spring_equinox
}

/// 计算太阳赤经赤纬，弧度制
fn sun_ra_dec() -> (f64, f64) {
    let spring_equinox = spring_equinox();
    let now = Utc::now();
    // println!("{:?}", spring_equinox);
    // 黄经，即当前日期与今年的春分日相隔多少年，极差为1 (大约在区间[-0.3,0.7])
    let t = (now - spring_equinox).num_seconds() as f64 / (86400.0 * 365.2422);
    let celestial_longitude_rad = t * 2.0 * std::f64::consts::PI;
    // 由黄经计算赤经
    // 方法来自https://tieba.baidu.com/p/6545878882
    let ra = (AXIAL_TILT.cos() * celestial_longitude_rad.sin())
        .atan2(celestial_longitude_rad.cos())
        + if celestial_longitude_rad > std::f64::consts::PI {
            std::f64::consts::PI
        } else {
            0.0
        };

    // 当前太阳赤纬，sind(23.4333°) = 0.39768173
    // 认为今日太阳赤纬为定值
    // 方法来自https://zhuanlan.zhihu.com/p/265303609
    let dec = ((2.0 * std::f64::consts::PI * t).sin() * 0.39768173).asin();

    (ra, dec)
}
