use base64::engine::general_purpose;
use base64::Engine as _;
use jwt_compact::alg::Rsa;
use jwt_compact::jwk::JsonWebKey;
use jwt_compact::{AlgorithmExt, Claims, Header, UntrustedToken};
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use serde_json::Value;
use std::collections::HashMap;

#[tauri::command]
fn decode_token(jwt_token: String, public_key: String) -> Result<String, String> {
    match UntrustedToken::new(&jwt_token) {
        Ok(token) => {
            let claims = token
                .deserialize_claims_unchecked::<HashMap<String, Value>>()
                .unwrap();
            let algorithm = token.algorithm();
            if algorithm != "RS256" {
                return Err(String::from("Only support RS256 now!"));
            }
            let mut is_valid = false;
            if !public_key.is_empty() {
                if is_valid_base64(&public_key) {
                    let rsa_public_byte = general_purpose::STANDARD.decode(&public_key).unwrap();
                    match RsaPublicKey::from_public_key_der(&rsa_public_byte) {
                        Ok(rpk) => {
                            if rs256_validate(&rpk, &token) {
                                is_valid = true;
                            }
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                } else {
                    match serde_json::from_str::<JsonWebKey<'_>>(public_key.as_str()) {
                        Ok(jwk) => match RsaPublicKey::try_from(&jwk) {
                            Ok(rpk) => {
                                if rs256_validate(&rpk, &token) {
                                    is_valid = true;
                                }
                            }
                            Err(e) => return Err(e.to_string()),
                        },
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            let decoded_token = DecodedToken {
                header: MyHeader {
                    header: token.header().clone(),
                    algorithm: token.algorithm().to_string(),
                },
                payload: claims,
                is_valid,
            };
            Ok(serde_json::to_string(&decoded_token).unwrap())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![decode_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize)]
struct DecodedToken {
    header: MyHeader,
    payload: Claims<HashMap<String, Value>>,
    #[serde(rename = "isValid")]
    is_valid: bool,
}

#[derive(serde::Serialize)]
struct MyHeader {
    #[serde(flatten)]
    header: Header,
    #[serde(rename = "alg")]
    algorithm: String,
}

fn is_valid_base64(input: &str) -> bool {
    let result = general_purpose::STANDARD.decode(input);
    result.is_ok()
}

fn rs256_validate(rpk: &RsaPublicKey, token: &UntrustedToken) -> bool {
    match Rsa::rs256()
        .validator::<HashMap<String, Value>>(&rpk)
        .validate(&token)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
