extern crate ipnetwork;
use diesel::FromSqlRow;
use diesel::QueryResult;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Inet;
use ipnetwork::IpNetwork;
use std::io::Write;
use std::net::IpAddr;

use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Inet)]
pub struct DbIpNetwork(pub IpNetwork);

impl ToSql<Inet, Pg> for DbIpNetwork {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        write!(out, "{}", self.0)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
impl From<IpAddr> for DbIpNetwork {
    fn from(ip: IpAddr) -> Self {
        DbIpNetwork(ip.into())
    }
}
impl FromSql<Inet, Pg> for DbIpNetwork {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        s.parse::<IpNetwork>()
            .map(DbIpNetwork)
            .map_err(|e| e.into())
    }
}
