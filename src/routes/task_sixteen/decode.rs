use std::collections::HashSet;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::Value;
use tracing::{error, info, instrument};

#[instrument(skip(payload))]
pub async fn decode(payload: String) -> Result<String, Response> {
    info!("payload" = payload);

    let decoding_key = DecodingKey::from_rsa_pem(
        "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAs5BlLjDtKuEY2NV3+xhH
WWlKrZDWkIOV+HoLURIBEpAHa11xU+wL9sySR17j4bL9MJawlCJAGArW8vnDiAv8
366PfOhCqZsD9N2iG28y7vf5q1PhoXl/Vfuelykw0k+r4054h0uCg9Olal0Nm/V8
vsdPEC3wjNLBi86oYESkW43/7lbBWPBti1POCVJDuBEASZFhIR2+mfz6AFWQwmqO
zzhP1Yli/7EtNMELWezQJXnVLQ3JvjT2btWWwKYT468YX/NtQgMC7SLvIRBuWb/Z
ayfoi/9rGndqW0YPE1xwJEQA415w5HbfTneyAIxDy7TC8/+dFaKRcoPiEQA1T5bk
OQIDAQAB
-----END PUBLIC KEY-----"
            .as_bytes(),
    )
    .unwrap();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.required_spec_claims = HashSet::new();
    validation.algorithms.push(Algorithm::RS512);
    let token = jsonwebtoken::decode::<Value>(&payload, &decoding_key, &validation);

    match token {
        Ok(token) => {
            info!("token" = format!("{:?}", token));
            Ok(token.claims.to_string())
        }
        Err(e) => {
            if e.to_string().contains("InvalidSignature") {
                error!(
                    "error" = format!("{}", e.to_string()),
                    "status" = StatusCode::UNAUTHORIZED.to_string()
                );

                return Err((StatusCode::UNAUTHORIZED, "").into_response());
            }
            error!(
                "error" = format!("{}", e.to_string()),
                "status" = StatusCode::BAD_REQUEST.to_string()
            );

            return Err((StatusCode::BAD_REQUEST, "Token decoding error").into_response());
        }
    }
}
