#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq)]
pub struct User {
	pub name: String,
	pub nickname: Option<String>,
	pub level: i64,
	pub is_admin: bool,
	pub account: super::Account
}
