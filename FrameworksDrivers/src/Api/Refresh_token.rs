use actix_web_httpauth::{extractors::bearer::BearerAuth};
use actix_web::{ post, HttpResponse,web};
use crate::JsonWebToken::jsonwebtoken::Claims;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RefreshResult {
    token: String,
}

#[post("/refresh-token")]
async fn refresh_token(refresh_jwt: Option<BearerAuth>) -> HttpResponse {
    let Some(refresh_jwt) = refresh_jwt else {
        return HttpResponse::Forbidden().body("Token no enviado");
    };

    let claims =Claims::validar_token(refresh_jwt.token().to_owned());

    match claims {
        Ok(c) => {
            // crear el nuevo token normal
            if c.tipo == "refresh" {
                let iss = c.iss.to_owned();
                let sub = c.sub.to_owned();
                let duracion_en_minutos: i64 = 5;
                let user_id = c.user_id;
                let tipo = "token-normal".to_owned();

                let token = Claims::generar_token(iss, sub, duracion_en_minutos, user_id, tipo);

                let resultado: RefreshResult = RefreshResult { token };

                HttpResponse::Ok().json(resultado)
            } else {
                HttpResponse::Unauthorized().body("")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body(""),
    }
}


            
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(refresh_token);
       
}
        