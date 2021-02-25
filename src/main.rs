#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;



use rocket_contrib::serve::{StaticFiles};


fn main() {


    use rocket::http::Header;

    let header = Header::new("X-Custom-Header", "custom value");

    //priority of mounted paths is in lowest to highest priority it seems?
    rocket::ignite()
    .mount("/", routes![ health_check ])
    .mount("/ccpgame", StaticFiles::from("static/chesscheckersgame_static"))
    .mount("/ccpfinder", StaticFiles::from("static/gamefinder_static"))
    .launch();

}




//use std::path::PathBuf;


use rocket::http::Status;

//catch all root paths
//respond to the health check and return a status of 200
#[get("/")]
fn health_check() -> Status{

    println!("health check performed");

    Status::Ok
}
