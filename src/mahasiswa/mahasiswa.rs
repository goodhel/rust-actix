use actix_web::{web, HttpResponse, Responder, Error};
use sqlx::PgPool;
use crate::mahasiswa::model::{MahasiswaL, PKMahasiswa, Mahasiswa};
use crate::ws_response;

// #[get("/mahasiswa")]
pub async fn list(req: web::Data<PgPool>) -> impl Responder {
    // let result = Todo::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    let result = MahasiswaL::list(req.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Mahasiswa not found"),
    }
}


// #[get("/mahasiswa/find")]
pub async fn find_by_id(
    pk: web::Query<PKMahasiswa>,
    req: web::Data<PgPool>
) -> impl Responder {
    let result = MahasiswaL::find_by_id(pk.id.into(), req.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Mahasiswa not found"),
    }
}

// #[post("/mahasiswa")]
pub async fn create(
    data: web::Json<Mahasiswa>,
    req: web::Data<PgPool>
) -> impl Responder {
    let result = MahasiswaL::create(data.into_inner(), req.get_ref()).await;

    match result {
        Ok(rows) => {
            if rows > 0 {
                let json = serde_json::json!({
                    "id": rows,
                    "status": "OK",
                    "info": "Data Berhasil Disimpan"
                });
                HttpResponse::Ok().json(json)
            } else {
                HttpResponse::BadRequest().body("Mahasiswa not found")
            }
        }
        _ => HttpResponse::BadRequest().body("Mahasiswa not found"),
    }
    
}

pub async fn update(
    data: web::Json<Mahasiswa>,
    query: web::Query<PKMahasiswa>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, Error> {
    let result = MahasiswaL::update(data.into_inner(), query.id.into(), pool.get_ref()).await;

    match result {
        Ok(rows) => {
            if rows > 0 {
                ws_response("OK", "Data Berhasil di Update")
            }else{
                ws_response("OK", "Data Tidak Ditemukan")
            }
        }
        _ => {
            ws_response("OK", "Data Tidak Ditemukan")
        }
    }
}

pub async fn delete(
    data: web::Query<PKMahasiswa>,
    req: web::Data<PgPool>) -> Result<HttpResponse, Error>{
        let result = MahasiswaL::delete(data.id, req.get_ref()).await;
        match result {
            Ok(rows) => {
                if rows > 0 {
                    ws_response("OK", "Data Berhasil di Update")
                }else{
                    ws_response("OK", "Data Tidak Ditemukan")
                }
            }
            _ => ws_response("OK", "Data Tidak Ditemukan")
        }
        
}


