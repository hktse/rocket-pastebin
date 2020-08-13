#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_pastebin::PasteID;
use rocket::Data;
use rocket_contrib::serve::StaticFiles;
use std::fs;

#[get("/<id>")]
fn recieve(id: PasteID) -> Option<String> {
    fs::read_to_string(format!("uploads/{}.txt", id)).ok()
}

#[post("/", data = "<data>")]
fn upload(data: Data) -> String {
    let id = PasteID::new();
    data.stream_to_file(format!("uploads/{}.txt", id)).unwrap();

    format!("upload/{}", id)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("web/build"))
        .mount("/upload", routes![upload, recieve])
        .launch();
}
