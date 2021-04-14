use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};


pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Create Folder if not exist
    async_std::fs::create_dir_all("./tmp").await?;
    // iterate over multipart stream
    let mut file_name = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
        println!("field {:?}",field);
        let contenttype = field.content_type().to_string();

        if contenttype == "application/octet-stream".to_string() {
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                let str = std::str::from_utf8(&data)?;
                file_name = str.to_string();
                // println!("file name {:?}", file_name);
                // println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
            }
        }else{
            // let content_type = field
            //     .content_disposition()
            //     .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
            // let filename = content_type
            //     .get_filename()
            //     .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
            // println!("file name {:?}", &file_name);
            // let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
            let filepath: String = if file_name == "".to_string() {
                let content_type = field
                    .content_disposition()
                    .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
                let filename = content_type
                    .get_filename()
                    .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
                // println!("file name {:?}", &file_name);
                format!("./tmp/{}", sanitize_filename::sanitize(&filename))
            }else{
                format!("./tmp/{}", &file_name)
            };
            // let filepath = format!("./tmp/{}", &file_name);
            let mut f = async_std::fs::File::create(filepath).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).await?;
            }
        }
        
        
    }
    Ok(HttpResponse::Ok().into())
}

pub fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/form" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}