use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{Error, Result, web::AUTH_TOKEN};

// pub async fn mw_require_auth<B>(
//     cookies: Cookies,
//     req: Request<B>,
//     next: Next
// ) -> Result<Response> {
//     println!("->>> {:<12} - mw_require_auth", "MIDDLEWARE");
//     Ok(next.run(req).await)
// }

