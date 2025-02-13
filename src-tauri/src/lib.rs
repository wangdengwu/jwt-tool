use base64::engine::general_purpose;
use base64::Engine as _;
use jwt_compact::alg::Rsa;
use jwt_compact::jwk::JsonWebKey;
use jwt_compact::{AlgorithmExt, Claims, Header, UntrustedToken};
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
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

#[tauri::command]
fn encode_token(header: String, payload: String, private_key: String) -> Result<String, String> {
    match general_purpose::STANDARD.decode(&private_key) {
        Ok(pk_der) => match RsaPrivateKey::from_pkcs8_der(&pk_der) {
            Ok(rpk) => match serde_json::from_str::<Header>(header.as_str()) {
                Ok(h) => match serde_json::from_str::<Claims<Value>>(payload.as_str()) {
                    Ok(c) => match Rsa::rs256().token(&h, &c, &rpk) {
                        Ok(t) => Ok(t),
                        Err(e) => Err(e.to_string()),
                    },
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn new_keys(bit_size: usize) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    match RsaPrivateKey::new(&mut rng, bit_size) {
        Ok(private_key) => {
            let private_der_encoded = private_key.to_pkcs8_der().unwrap();
            let private_base64_encoded =
                general_purpose::STANDARD.encode(private_der_encoded.as_bytes());
            let public_der_encoded = private_key.to_public_key().to_pkcs1_der().unwrap();
            let public_base64_encoded =
                general_purpose::STANDARD.encode(public_der_encoded.as_bytes());
            let b64 = Base64RsaKeys {
                public: public_base64_encoded,
                private: private_base64_encoded,
            };
            Ok(serde_json::to_string(&b64).unwrap())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_public(private_key: String) -> Result<String, String> {
    match general_purpose::STANDARD.decode(&private_key) {
        Ok(pk_der) => match RsaPrivateKey::from_pkcs8_der(&pk_der) {
            Ok(private_key) => {
                let public_der_encoded = private_key.to_public_key().to_pkcs1_der().unwrap();
                let public_base64_encoded =
                    general_purpose::STANDARD.encode(public_der_encoded.as_bytes());
                Ok(public_base64_encoded)
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn get_jwk(public_key: String) -> Result<String, String> {
    match general_purpose::STANDARD.decode(&public_key) {
        Ok(pk_der) => match RsaPublicKey::from_pkcs1_der(&pk_der) {
            Ok(public_key) => {
                let jwk = JsonWebKey::from(&public_key);
                Ok(serde_json::to_string(&jwk).unwrap())
            }
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            decode_token,
            encode_token,
            new_keys,
            get_public,
            get_jwk
        ])
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

#[derive(serde::Serialize)]
struct Base64RsaKeys {
    private: String,
    public: String,
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
