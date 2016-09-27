#[derive(RustcEncodable)]
pub struct User {
	pub username: String,
	pub fullname: String,
	pub email: String
}