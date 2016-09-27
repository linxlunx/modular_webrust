extern crate nickel;

use self::nickel::{Request, Response, MiddlewareResult, MediaType};

pub fn index_handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a>{
	res.set(MediaType::Json);
	res.send("{\"msg\": \"Hai\"}")
}