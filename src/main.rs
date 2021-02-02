#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;



use rocket_contrib::serve::{StaticFiles};


fn main() {

    rocket::ignite()
    .mount("/ccpgame", StaticFiles::from("static/chesscheckersgame_static"))
    .mount("/ccpfinder", StaticFiles::from("static/gamefinder_static"))
    //.mount("/", routes![ default_route ])
    .launch();

}



use std::path::PathBuf;



//catch every request not caught by another route
#[get("/<path..>")]
fn default_route(path: PathBuf) -> String{ 
    
    format!("{:?} cannot be routed to", path)

 }