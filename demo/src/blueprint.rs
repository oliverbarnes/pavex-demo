use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::{constructor::Lifecycle, router::GET, Blueprint};
use pavex::request::{query::QueryParams, path::PathParams};
use pavex::request::body::{BodySizeLimit, BufferedBody, JsonBody};
use pavex::f;

/// The main blueprint, containing all the routes, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    register_common_constructors(&mut bp);

    add_telemetry_middleware(&mut bp);

    bp.constructor(
        f!(crate::user_agent::UserAgent::extract),
        Lifecycle::RequestScoped,
    )
    .error_handler(f!(crate::user_agent::invalid_user_agent));

    bp.route(GET, "/api/ping", f!(crate::routes::status::ping));
    bp.route(
        GET,
        "/api/greet/:name",
        f!(crate::routes::greet::greet),
    );
    bp
}

/// Common constructors used by all routes.
fn register_common_constructors(bp: &mut Blueprint) {
    PathParams::register(bp);
    QueryParams::register(bp);
    JsonBody::register(bp);
    BufferedBody::register(bp);
    BodySizeLimit::register(bp);
}

/// Add the telemetry middleware, as well as the constructors of its dependencies.
fn add_telemetry_middleware(bp: &mut Blueprint) {
    bp.constructor(
        f!(crate::telemetry::RootSpan::new),
        Lifecycle::RequestScoped,
    )
    .cloning(CloningStrategy::CloneIfNecessary);

    bp.wrap(f!(crate::telemetry::logger));
}
