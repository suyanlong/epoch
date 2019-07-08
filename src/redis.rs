//! Example of redis based session
//!
//! [User guide](https://actix.rs/book/actix-web/sec-9-middlewares.html#user-sessions)
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Result};

/// simple handler
pub fn index(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }

    Ok("Welcome!".into())
}
