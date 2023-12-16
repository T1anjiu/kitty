use sea_orm::{entity::prelude::*, FromJsonQueryResult};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "hysteria")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub server: String,
    pub auth: String,
    #[sea_orm(column_type = "Text")]
    tls: Tls,
    #[sea_orm(column_type = "Text")]
    bandwidth: Bandwidth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]

pub struct Tls {
    sni: String,
    insecure: bool,
    #[serde(rename = "pinSHA256")]
    pin_sha256: Option<String>,
    ca: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Bandwidth {
    up: String,
    down: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HysteriaModelWithoutName {
    #[serde(skip)]
    pub name: String,
    pub server: String,
    pub auth: String,
    tls: Tls,
    bandwidth: Bandwidth,
}

impl<'a> From<&'a Model> for HysteriaModelWithoutName {
    fn from(source: &'a Model) -> Self {
        HysteriaModelWithoutName {
            name: source.name.clone(),
            server: source.server.clone(),
            auth: source.auth.clone(),
            tls: source.tls.clone(),
            bandwidth: source.bandwidth.clone(),
        }
    }
}

impl Model {
    fn serde_string(&self) -> Result<String, serde_json::Error> {
        let tmp_struct = HysteriaModelWithoutName::from(self);
        let value = serde_json::to_string(&tmp_struct);
        value
    }
}
