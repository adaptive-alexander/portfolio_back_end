use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web};
use futures::{StreamExt, TryStreamExt};

pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
    // iterate over multipart stream
    // File::create is blocking operation, use threadpool
    let filepath = file_path.clone();
    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();
    while let Ok(Some(mut field)) = payload.try_next().await {

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.as_ref().unwrap().write_all(&data).map(|_| f).unwrap())
                .await
                .unwrap();
        }
    }

    Some(true)
}