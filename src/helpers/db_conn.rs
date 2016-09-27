extern crate mysql;
extern crate ini;

use self::mysql::Pool;
use self::mysql::conn::OptsBuilder;
use self::ini::Ini;

pub fn request_connection() -> Pool {
	let conf = Ini::load_from_file("env.ini").unwrap();
	let section = conf.section(Some("Database".to_owned())).unwrap();

	let db_name = section.get("name").unwrap().as_str();
	let db_user = section.get("username").unwrap().as_str();
	let db_password = section.get("password").unwrap().as_str();

	let mut options = OptsBuilder::default();
	options.user(Some(db_user))
			.pass(Some(db_password))
			.db_name(Some(db_name));
	Pool::new(options).unwrap()
}