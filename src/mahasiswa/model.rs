// use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
// use futures::future::{ready, Ready};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct MahasiswaL {
    id: i32,
    nim: String,
    nama: String,
    angkatan: String,
    sks: i32,
    ipk: f64,
    status_mhs: i16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mahasiswa {
    nim: String,
    nama: String,
    angkatan: String,
    sks: i32,
    ipk: f64
}

#[derive(Debug, Deserialize)]
pub struct PKMahasiswa {
    pub id: i32
}

// implementation of Actix Responder for MahasiswaL struct so we can return MahasiswaL from action handler
// impl Responder for MahasiswaL {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

impl MahasiswaL {
    pub async fn list(pool: &PgPool) -> Result<Vec<MahasiswaL>> {
        let result = sqlx::query_as!(MahasiswaL,
            "SELECT id, nim, nama, angkatan, sks, ipk, status_mhs FROM mahasiswa"
        ).fetch_all(pool).await?;

        Ok(result)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<MahasiswaL> {
        let result = sqlx::query_as!(MahasiswaL,
            "SELECT id, nim, nama, angkatan, sks, ipk, status_mhs FROM mahasiswa WHERE id = $1",id
        ).fetch_one(pool).await?;
        Ok(result)
    }

    pub async fn create(data: Mahasiswa, pool: &PgPool) -> Result<i32> {
        let insert = sqlx::query!(
            "INSERT INTO mahasiswa (nim, nama, angkatan, sks, ipk)
            VALUES ($1,$2,$3,$4,$5) RETURNING id",
            data.nim, data.nama, data.angkatan, data.sks, data.ipk
        ).fetch_one(pool).await?;
        Ok(insert.id)
    }

    pub async fn update(data: Mahasiswa, id: i32, pool: &PgPool) -> Result<i32> {
        let update = sqlx::query!(
            "UPDATE mahasiswa SET nim = $2, nama = $3, angkatan = $4, sks = $5, ipk = $6
            WHERE id = $1 RETURNING id",
            id, data.nim, data.nama, data.angkatan, data.sks, data.ipk
        ).fetch_one(pool).await?;
        Ok(update.id)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32> {
        let delete = sqlx::query!(
            "DELETE FROM mahasiswa WHERE id = $1 RETURNING id",
            id
        ).fetch_one(pool).await?;
        Ok(delete.id)
    }
    
}