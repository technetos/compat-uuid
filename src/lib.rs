#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;

use diesel::{
    deserialize::{self, FromSql},
    not_none,
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Uuid as UuidDiesel,
};
use rocket_contrib::uuid::Uuid as rocketUuid;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use uuid;

#[derive(Clone, Debug, AsExpression, PartialEq, FromSqlRow, Serialize, Deserialize, Hash, Eq)]
#[sql_type = "UuidDiesel"]
pub struct Uuid(uuid::Uuid);

// All we need is a random uuid
impl Uuid {
    pub fn new() -> Uuid {
        Uuid(uuid::Uuid::new_v4())
    }
}

impl ToSql<UuidDiesel, Pg> for Uuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<UuidDiesel, Pg> for Uuid {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        Ok(Uuid(uuid::Uuid::from_slice(bytes)?))
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Uuid(uuid)
    }
}

impl From<rocketUuid> for Uuid {
    fn from(uuid: rocketUuid) -> Self {
        Uuid(uuid.into_inner())
    }
}
