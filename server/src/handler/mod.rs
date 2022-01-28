use axum::{routing::get, Router};

pub mod sys_auto_code;
pub mod sys_initdb;

pub fn routers() -> Router {
    Router::new()
        .route("/", get(sys_initdb::str_response))
        .route("/autoCode/getDB", get(sys_auto_code::get_db)) // 获取数据库
        .route("/autoCode/getTables", get(sys_auto_code::get_tables)) // 获取对应数据库的表
}
