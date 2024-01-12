use pavex::http::header::{ToStrError, USER_AGENT};
use pavex::request::RequestHead;
use pavex::response::Response;

pub enum UserAgent {
	Unknown,
	Known(String),
}

impl UserAgent {
	pub fn extract(request_head: &RequestHead) -> Result<Self, ToStrError> {
		let Some(user_agent) = request_head.headers.get(USER_AGENT) else {
			return Ok(Self::Unknown);
		};

		user_agent.to_str().map(|s| UserAgent::Known(s.into()))
	}
}

pub fn invalid_user_agent(_e: &ToStrError) -> Response {
    Response::bad_request()
        .set_typed_body("The `User-Agent` header value must be a valid UTF-8 string")
}
