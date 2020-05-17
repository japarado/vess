use super::{ok_response, AppData, GenericRespnse};
use actix_web::{get, web, HttpResponse};

use crate::database::Conn;

#[get("")]
pub async fn index(data: AppData) -> GenericRespnse {
    let data = data.lock().unwrap();
    let conn: Conn = data.conn_pool.get().unwrap();
    Ok(HttpResponse::Ok().json("hello"))
}
