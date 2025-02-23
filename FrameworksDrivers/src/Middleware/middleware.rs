use crate::JsonWebToken::jsonwebtoken::Claims;
use actix_web_httpauth::{extractors::bearer::BearerAuth};
use actix_web::{dev::ServiceRequest, error, Error};

pub async fn validador(
    req: ServiceRequest,
    credenciales: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credenciales) = credenciales else {
        return Err((error::ErrorBadRequest("no se especifico el token"), req));
    };

    let token = credenciales.token();

    let resultado = Claims::validar_token(token.to_owned());

    match resultado {
        Ok(claims) => {
        

            if claims.tipo != "refresh" {
                return Ok(req);
            } else {
                return Err((error::ErrorForbidden("no tiene acceso"), req));
            }
        }
        Err(e) => {
            println!("el token no es valido: {:?}", e);
            return Err((error::ErrorForbidden("no tiene acceso"), req));
        }
    }
}