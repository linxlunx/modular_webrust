#[macro_use]
extern crate modular_webrust;
extern crate nickel;
extern crate mysql;

use nickel::{Nickel, HttpRouter};
use modular_webrust::modules;


fn main() {
	let mut server = Nickel::new();

	server.get("/", modules::index::index_handler);
	server.post("/users/login", modules::users::user_login);

	server.listen("127.0.0.1:3000");
}
