#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

extern crate jsonwebtoken as jwt;

//#[macro_use]
extern crate clap;

use std::fs::File;
use std::io;
use std::io::Read;

use actix_redis::RedisSession;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

use bytes::BytesMut;
use clap::{App as cli, Arg};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use futures::future::{err, Either};
use futures::{Future, Stream};
use toml;

mod config;
mod models;
mod redis;
mod schema;
mod token;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// Diesel query
fn query(nm: String, pool: web::Data<Pool>) -> Result<models::User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: nm.as_str(),
    };
    let conn = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

/// Async request handler
fn add(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || query(name.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_add(
    pl: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        //
        // Douman NOTE:
        // The return value in this closure helps, to clarify result for compiler
        // as otheriwse it cannot understand it
        .and_then(move |body| {
            // body is loaded, now we can deserialize serde-json
            let r_obj = serde_json::from_slice::<MyUser>(&body);

            // Send to the db for create
            match r_obj {
                Ok(obj) => Either::A(web::block(move || query(obj.name, pool)).then(
                    |res| match res {
                        Ok(user) => Ok(HttpResponse::Ok().json(user)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                    },
                )),
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
            }
        })
}

fn add2(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || query(item.into_inner().name, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=trace,actix_web=trace");
    env_logger::init();

    // Finally we call get_matches() to start the parsing process. We use the matches just as we
    // normally would
    let matches = cli::new("epoch")
        .version("1.0")
        .author("suyanlong <yanlong.su@goscn.io>")
        .about("epoch welcome you")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("epoch.toml")
                .help("Sets a config file")
                .takes_value(true),
        )
        .get_matches();

    let val = if let Some(val) = matches.value_of("config") {
        val
    } else {
        ""
    };
    let mut input = String::new();
    File::open(&val)
        .and_then(|mut f| f.read_to_string(&mut input))
        .unwrap();
    let val: config::Config = toml::from_str(&input).unwrap();

    dotenv::dotenv().ok();
    let database_url = if let Ok(ok) = std::env::var("DATABASE_URL") {
        ok
    } else {
        val.mysql.unwrap()
    };

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // redis session middleware
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            // register simple route, handle all methods
            .service(web::resource("/").to(redis::index))
            // This can be called with:
            // curl -S --header "Content-Type: application/json" --request POST --data '{"name":"xyz"}'  http://127.0.0.1:8080/add
            // Use of the extractors makes some post conditions simpler such
            // as size limit protections and built in json validation.
            .service(web::resource("/add2").route(web::post().to_async(add2)))
            //  Manual parsing would allow custom error construction, use of
            //  other parsers *beside* json (for example CBOR, protobuf, xml), and allows
            //  an application to standardise on a single parser implementation.
            .service(web::resource("/add").route(web::post().to_async(index_add)))
            .service(web::resource("/add/{name}").route(web::get().to_async(add)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
}
