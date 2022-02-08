use serde::Deserialize;
use std::io::Cursor;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::{AsEwkbPoint, EwkbRead, EwkbWrite, GeometryT, Point};

#[derive(Deserialize, Debug)]
pub struct Database {
    host: String,
    user: String,
    pw: String,
    port: String,
    name: String,
}

impl Database {
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pw, self.host, self.port, self.name
        )
    }
}

#[derive(SqlType, QueryId)]
#[postgres(type_name = "geometry")]
pub struct Geometry;

#[derive(Debug, AsExpression, FromSqlRow)]
#[sql_type = "Geometry"]
pub struct PointType(pub Point);

impl FromSql<Geometry, Pg> for PointType {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        let mut r = Cursor::new(bytes);
        let geom = GeometryT::read_ewkb(&mut r)?;
        return match geom {
            postgis::ewkb::GeometryT::Point(point) => Ok(PointType(point)),
            _ => Err("Geometry is not a point".into()),
        };
    }
}

impl<Db: Backend> ToSql<Geometry, Db> for PointType {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        self.0.as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
