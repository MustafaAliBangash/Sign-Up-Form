#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
extern crate rocket_cors;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Method; // 1.
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};
use rocket::State;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicUsize, Ordering};

type ID = usize;
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Message {
    id: ID,
    contents: String
}
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.      
      "http://127.0.0.1:5500/Form.html",
        " http://127.0.0.1:5500",
       "http://192.168.1.108:5500"
        // "chrome-extension://fhbjgbiflinjbdggehcddcbncdddomop",               
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]
fn hello() -> JsonValue {
      json!([
       {"THANK YOU":"FOR APPLYING"}
      ])
   }
type MessageMap = Mutex<HashMap<ID,String>>;

#[post("/add", data = "<user_input>")]
fn helloPost(user_input: Json<Message>, map: State<'_, MessageMap>) {
    println!("{:?}",user_input.0.contents);
}

fn rocket()-> rocket::Rocket{
    rocket::ignite()
    .mount("/", routes![hello,helloPost]).attach(make_cors())
    .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}