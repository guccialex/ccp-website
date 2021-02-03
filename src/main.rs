#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;



use rocket_contrib::serve::{StaticFiles};


fn main() {

    //priority of mounted paths is in lowest to highest priority it seems?
    rocket::ignite()
    .mount("/", routes![ health_check ])
    .mount("/ccpgame", StaticFiles::from("static/chesscheckersgame_static"))
    .mount("/ccpfinder", StaticFiles::from("static/gamefinder_static"))
    .launch();

}



//use std::path::PathBuf;



use rocket::http::Status;

//catch all paths without a slash in them
//respond to the health check and return a status of 200
#[get("/<catchall>")]
fn health_check(catchall: String) -> Status{

    println!("the path requested: {:?}", catchall);
    println!("health check performed");

    Status::Ok
}
