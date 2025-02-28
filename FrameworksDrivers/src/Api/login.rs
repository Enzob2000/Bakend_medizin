use crate::JsonWebToken::jsonwebtoken::Claims;
use serde::{Deserialize, Serialize};
use actix_web::{ post, web, HttpResponse, };
use InterfaceAdapters::DTO::login::Login;


#[derive(Serialize, Deserialize)]
struct LoginResult {
    token: String,
    refresh: String,
}


#[post("/login")]
async fn login(form: web::Query<Login>) -> HttpResponse {
    if form.usuario != "rusty" && form.password != "fullstack" {

        return HttpResponse::Unauthorized().body("Login invalido")
    }

        let iss = "Rusty Full Stack".to_owned();
        let sub = "Prueba".to_owned();
        let duracion_en_minutos: i64 = 5;
        let duracion_dia: i64 = 1440;
        let user_id = 1;

        let token = Claims::generar_token(
            iss.clone(),
            sub.clone(),
            duracion_en_minutos,
            user_id,
            "token-normal".to_owned(),
        );
        let refresh =Claims::generar_token(
            iss.clone(),
            sub.clone(),
            duracion_dia,
            user_id,
            "refresh".to_owned(),
        );

        let respuesta = LoginResult { token, refresh };

        HttpResponse::Ok().json(respuesta)
   

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
       
}