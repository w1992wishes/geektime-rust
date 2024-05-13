use axum::{handler::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 创建路由
    let app = Router::new().route("/", get(handler));

    // 绑定服务器地址
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 运行服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 处理请求的处理程序
async fn handler() -> &'static str {
    "Hello, Axum!"
}
