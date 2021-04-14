use actix_web::{web};
use crate::mahasiswa::mahasiswa;
use crate::multi::multi;
use crate::client::client;

pub fn init(app: &mut web::ServiceConfig) {
    // app.service(mahasiswa::list);
    // app.service(mahasiswa::find_by_id);

    // Mahasiswa
    app.service(
        web::resource("/mahasiswa")
        .route(web::get().to(mahasiswa::list))
        .route(web::post().to(mahasiswa::create))
        .route(web::delete().to(mahasiswa::delete))
        .route(web::patch().to(mahasiswa::update))
    );

    // Mahasiswa find by id
    app.service(
        web::resource("/mahasiswa/find")
        .route(web::get().to(mahasiswa::find_by_id))
    );

    // Form Data
    app.service(
        web::resource("/form")
        .route(web::get().to(multi::index))
        .route(web::post().to(multi::save_file))
    );

    // Email
    app.service(
        web::resource("/email")
        .route(web::post().to(client::insert))
    );
}