use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    extract::{FromRef, Path},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use axum_template::{engine::Engine, Key, RenderHtml};
use tera::Tera;

use std::collections::HashMap;

// Type alias for our engine. For this example, we are using Tera
type AppEngine = Engine<Tera>; 

#[macro_use]
extern crate lazy_static;


// Define your application shared state
#[derive(Clone,Debug,FromRef)]
struct AppState {
    engine: AppEngine,
}

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    // build our application with a route
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     // `POST /users` goes to `create_user`
    //     .route("/users", post(create_user));



    let user_routes = Router::new()
    .route("/view_users/:name", get(view_user))
    .route("/create_users", post(create_user));

    let home_routes = Router::new()
    .route("/main", get(root));

    let app = Router::new()
    .nest("/users", user_routes)
    .nest("/home", home_routes)
    .with_state(AppState{
        engine: Engine::from(tera)
    })
    ;

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn view_user(
    engine: AppEngine,
    Key(key): Key,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };
    RenderHtml("hello.html", engine, person)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}