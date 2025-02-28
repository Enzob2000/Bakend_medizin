use super::login::config as config_login;
use super::Refresh_token::config as config_Refres_token;
use crate::Middleware::middleware::validador;
use crate::Websocket::Server::server::Server;
use crate::Websocket::Session::session::config as config_websocket;
use actix::Actor;
use actix_web::middleware::{Compress, Logger};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::extractors::bearer::Config as BearerConfig;
use actix_web_httpauth::middleware::HttpAuthentication;

#[get("/privado")]
async fn privado() -> HttpResponse {
    HttpResponse::Ok().body("privado")
}

pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::with_fn(validador);
        let ws = Server::new().start();

        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .app_data(BearerConfig::default().realm("jwt"))
            .app_data(web::Data::new(ws.clone()))
            .configure(config_login)
            .configure(config_Refres_token)
            .configure(config_websocket)
            .service(web::scope("/admin").wrap(auth).service(privado))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
