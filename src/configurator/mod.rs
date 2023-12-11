use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alice_server_conf {
	#[serde(rename = "run")]
	pub run: Option<bool>,

	#[serde(rename = "host")]
	pub host: Option<String>,

	#[serde(rename = "port")]
	pub port: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Backer_conf {
	#[serde(rename = "run")]
	pub run: Option<bool>,

	#[serde(rename = "path_to_database_bk")]
	pub path_to_database_bk: Option<String>,

	#[serde(rename = "path_to_cache_bk")]
	pub path_to_cache_bk: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Journalist_conf {
	#[serde(rename = "run")]
	pub run: Option<bool>,

	#[serde(rename = "path_to_logs")]
	pub path_to_logs: Option<String>,

	#[serde(rename = "level")]
	pub level: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
	#[serde(rename = "run")]
	pub run: Option<bool>,

	#[serde(rename = "host")]
	pub host: Option<String>,

	#[serde(rename = "port")]
	pub port: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cacher_conf {
	#[serde(rename = "capacity")]
	pub capacity: Option<String>,

	#[serde(rename = "server")]
	pub server: Option<Server>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alicedb_conf {
	#[serde(rename = "alice_server_conf")]
	pub alice_server_conf: Option<Alice_server_conf>,

	#[serde(rename = "backer_conf")]
	pub backer_conf: Option<Backer_conf>,

	#[serde(rename = "journalist_conf")]
	pub journalist_conf: Option<Journalist_conf>,

	#[serde(rename = "cacher_conf")]
	pub cacher_conf: Option<Cacher_conf>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
	#[serde(rename = "alicedb_conf")]
	pub alicedb_conf: Option<Alicedb_conf>,
}
