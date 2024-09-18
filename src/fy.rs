use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};


pub struct Fy {
    port: u32,
    fy_s: Arc<Sender<Input>>
}

// 定义接收 POST 请求的 JSON 结构
#[derive(Deserialize)]
pub struct Input {
    pub id: u32,
    pub url: String,
    pub reqfmt: String
}

// 定义返回响应的 JSON 结构
#[derive(Serialize)]
struct Output {
    response: String,
}


impl Fy {
    pub fn new(p: u32, s: Sender<Input>) -> Self {
        Self {
            port: p,
            fy_s: Arc::new(s)
        }
    }


    pub async fn run(&self) -> Result<()> {
        let app = Router::new()
            .route("/", post({
                let s = Arc::clone(&self.fy_s);
                move |body| handle_post(body, s);
            }))
            .route("/option", get(handle_option));

        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",self.port)).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

}


async fn handle_post(Json(payload): Json<Input>, s: Arc<Sender<Input>>) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let response = Output {
        response: "错！".to_string(),
    };

    s.send(payload).await;

    Json(response)
}


async fn handle_option() {
    println!("fy get");
}
