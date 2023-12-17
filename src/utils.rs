use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug)]
pub enum ObjType {
    Sun,
    Twins,
    Galaxy,
    Cluster,
    Nebula,
}

impl FromSql for ObjType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "sun" => Ok(ObjType::Sun),
            "twin" => Ok(ObjType::Twins),
            "galaxy" => Ok(ObjType::Galaxy),
            "cluster" => Ok(ObjType::Cluster),
            "nebula" => Ok(ObjType::Nebula),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
#[derive(Debug)]
pub struct Star {
    pub id: i32,
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub obj_type: ObjType,
}
