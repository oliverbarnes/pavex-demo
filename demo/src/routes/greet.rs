use pavex::request::path::PathParams;
use pavex::response::Response;

#[PathParams]
pub struct GreetParams {
	pub name: String,
}

pub fn greet(params: PathParams<GreetParams>) -> Response {
	let GreetParams { name } = params.0;

	Response::ok()
		.set_typed_body(format!("Hello, {name}!"))
}