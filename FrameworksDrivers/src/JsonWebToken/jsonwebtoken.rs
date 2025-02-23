use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Duration, Utc};


const LLAVE: &str = "12345";

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub tipo: String,
    pub user_id: usize,
}

impl  Claims {
    

pub fn  generar_token(
    iss: String,
    sub: String,
    duracion_en_minutos: i64,
    user_id: usize,
    tipo: String,
) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(LLAVE.as_ref());

    let exp = (Utc::now() + Duration::minutes(duracion_en_minutos)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;

    let my_claims = Claims {
        iss,
        sub,
        exp,
        iat,
        tipo,
        user_id,
    };

    encode(&header, &my_claims, &encoding_key).unwrap()
}

pub fn validar_token(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validacion = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(LLAVE.as_ref());

    let resultado = decode::<Claims>(&token, &decoding_key, &validacion);

    match resultado {
        Ok(c) => {
            println!("Token es valido");
            Ok(c.claims)
        }
        Err(e) => {
            println!("Token es invalido");
            Err(e)
        }
    }
}
}
