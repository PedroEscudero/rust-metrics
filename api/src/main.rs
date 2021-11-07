#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

extern crate job_scheduler;

mod schema;
mod models;

use crate::models::book::*;

use rocket::request::Request;
use rocket_contrib::databases::diesel::PgConnection;
use rocket_contrib::json::Json;
use std::thread;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

#[get("/health_check")]
fn health_check(_db_conn: RocketDbConn) -> &'static str {
    "{status:'ok'}"
}

#[get("/book")]
fn getter() -> Json<Book>{
    let book  = Book{
        id: 2,
        title: "Oscuralia".to_string(),
        author: "Pedro Escudero".to_string(),
        gender: "Terror".to_string(),
        year: 2018,
        price: 20
    };
    Json(book)
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service unavailable"
}

#[database("rocketdb")]
pub struct RocketDbConn(PgConnection);

fn cron_task(){
    let mut task = JobScheduler::new();
    task.add(Job::new("* 1/20 * * * *".parse().unwrap(), || {
        println!("Scheduled task executed");
    }));
    loop {
        task.tick();
        std::thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    thread::spawn(|| {
        cron_task();
      });

    rocket::ignite()
        .attach(RocketDbConn::fairing())
        .register(catchers![service_not_available])
        .mount("/api",
        routes![health_check,
        getter
        ])
        .launch();

}
