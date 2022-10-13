use actix_web::{web::{self, get}, App, HttpServer, HttpResponse};

use std::process::exit;

mod error;
use error::InnerServerError;

mod templates;
use templates::{Templates, TemplateId as TID};


#[derive(Clone)]
#[allow(dead_code)]
struct ServerState {
    templates: Templates,
}


async fn index(data: web::Data<ServerState>) -> HttpResponse {
    HttpResponse::Ok().body(data.templates.get(TID::Index))
}

fn init_server_state() -> Result<ServerState, InnerServerError> {
    Ok(ServerState {
        templates: templates::load()?,
    })
}

fn server_state() -> web::Data<ServerState> {
    match init_server_state() {
        Ok(state) => web::Data::new(state),
        Err(e) => {
            println!("Failed to create server state: {}", e);
            exit(e.as_exit_code());
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = server_state();

    HttpServer::new(move || {
        App::new()
          .app_data(state.clone())
          .route("/", get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
