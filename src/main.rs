#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;



use rocket_contrib::serve::{StaticFiles};


fn main() {

    rocket::ignite()
    .mount("/ccpgame", StaticFiles::from("static/chesscheckersgame_static"))
    .mount("/ccpfinder", StaticFiles::from("static/gamefinder_static"))
    .launch();

}