use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use axum::{
    extract::Json,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
    body::{Body, BodyDataStream},
    http::StatusCode
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;


use crate::{config::get_config, utils::get_id};


pub struct Fy {
    port: u32,
    fy_s: Arc<Sender<Task>>
}

#[derive(Deserialize)]
pub struct Input {
    pub atk_type: String,
    pub atk_mode: String,
    pub model_url: String,
    pub model_req: String
}

pub struct Task {
    pub id: u32,
    pub atk_type: Vec<String>,
    pub atk_mode: Vec<String>,
    pub target_url: String,
    pub reqfmt: String
}



#[derive(Serialize)]
struct Output {
    response: u32,
}

#[derive(Serialize)]
struct Opt {
    pub atk_type: Vec<String>,
    pub atk_mode: Vec<String>
}

impl Fy {
    pub fn new(p: u32, s: Sender<Task>) -> Self {
        Self {
            port: p,
            fy_s: Arc::new(s)
        }
    }


    pub async fn run(&self) -> Result<()> {
        let app = Router::new()
            .route("/", post({
                let s = Arc::clone(&self.fy_s);
                move |body| handle_post(body, s)
            }))
            .route("/option", get(handle_option))
            .route("/result", get(handle_file));

        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",self.port)).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

}


async fn handle_post(Json(payload): Json<Input>, s: Arc<Sender<Task>>) -> impl IntoResponse {
    println!("fy receive");
    
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let ts = payload.atk_type.split(',').map(|i|i.trim().to_owned()).collect::<Vec<String>>();
    let ms = payload.atk_mode.split(',').map(|i|i.trim().to_owned()).collect::<Vec<String>>();

    let id = get_id();
    let task = Task {
        id,
        atk_type: ts,
        atk_mode: ms,
        target_url: payload.model_url,
        reqfmt: payload.model_req
    };

    let response = Output {
        response: id
    };
    let _ = s.send(task).await;

    Json(response)
}


async fn handle_option() -> impl IntoResponse {
    let r = Opt {
        atk_type: get_config().atk_type.clone(),
        atk_mode: get_config().atk_mode.clone()
    };

    Json(r)
}

async fn handle_file() -> impl IntoResponse {

    
    let file_path = "";

    match File::open(file_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);

            let body = Body::from_stream(stream);
            (StatusCode::OK, body)
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            "File not found".into_response().into_body(),
        ),
    }
}
