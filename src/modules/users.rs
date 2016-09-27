extern crate nickel;
extern crate rustc_serialize;
extern crate mysql;
extern crate pwhash;

use self::nickel::{Response, Request, MiddlewareResult, MediaType, FormBody};
use self::nickel::status::StatusCode;
use self::rustc_serialize::*;
use self::pwhash::bcrypt;

use structures::user::User;
use helpers::db_conn::request_connection;

pub fn user_login<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a>{
	let form_data = try_with!(res, req.form_body());
	let username = form_data.get("username").unwrap_or("");
	let password = form_data.get("password").unwrap_or("");

	let payload: String;

	if username == "" || password == "" {
		res.set(StatusCode::PreconditionFailed);
		payload = "{\"message\": \"Please fill username/password!\"}".to_string();
	} else {
		let pools = request_connection();

		let mut data = pools.prep_exec("SELECT password, fullname, email, username FROM users WHERE username = :username", params!{"username"=>username}).unwrap();

		let row = data.next();

		if (row.is_none()) {
			res.set(StatusCode::Unauthorized);
			payload = "{\"message\": \"Wrong username/password\"}".to_string();
		} else {
			let mut row = row.unwrap().unwrap();

			let fullname: String = row.take("fullname").unwrap();
			let email: String = row.take("email").unwrap();

			let hash_password: String = row.take("password").unwrap();

			if bcrypt::verify(password, hash_password.as_str()) {
				let user = User {
					username: username.to_string(),
					fullname: fullname,
					email: email
				};

				payload = json::encode(&user).unwrap();	
			} else {
				res.set(StatusCode::Unauthorized);
				payload = "{\"message\": \"Wrong username/password\"}".to_string();
			}
		}
	}

	res.set(MediaType::Json);
	res.send(payload)
}