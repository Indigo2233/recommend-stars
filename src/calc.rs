use rusqlite::{Connection, params, Result};
use crate::utils::Star;


pub fn visible_stars(_longitude: f64, _latitude: f64, ways: usize, obj_type: String) -> Result<()> {
    let conn = Connection::open("stars.db")?;
    // 肉眼、双筒基于本人在仙林体验的极限星等，天文望远镜和深空摄影基于估测
    // TODO：根据观测方式和光污染等级，计算极限星等
    let limiting_mag = [0.0, 4.5, 8.5, 10.0, 12.0][ways];

    let mut stmt = conn.prepare(
        "SELECT * FROM stars WHERE
         Type = ? AND MGA < ?")?;
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