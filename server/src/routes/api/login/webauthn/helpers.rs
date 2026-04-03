use color_eyre::eyre::{Context, Result};
use mongodb::bson::doc;
use webauthn_rs::prelude::{AuthenticationResult, Passkey};

use crate::{
    axum_error::AxumResult,
    database::{User, WebAuthnFactor},
    state::AppState,
};

pub async fn update_webauthn_credentials(
    state: &AppState,
    user: &User,
    auth_result: &AuthenticationResult,
) -> AxumResult<()> {
    let mut user_keys = user
        .auth_factors
        .webauthn
        .iter()
        .map(|f| -> Result<(WebAuthnFactor, Passkey)> {
            let passkey: Passkey = serde_json::from_str(&f.serialized_key)?;
            Ok((f.clone(), passkey))
        })
        .collect::<Result<Vec<(WebAuthnFactor, Passkey)>, _>>()?;

    user_keys.iter_mut().for_each(|(_, sk)| {
        sk.update_credential(auth_result);
    });

    let serialized_keys = user_keys
        .iter()
        .map(|(factor, sk)| {
            Ok(WebAuthnFactor {
                serialized_key: serde_json::to_string(&sk)
                    .wrap_err("Failed to serialize passkey")?,
                ..factor.clone()
            })
        })
        .collect::<Result<Vec<WebAuthnFactor>>>()?;

    state
        .database
        .collection::<User>("users")
        .find_one_and_update(
            doc! { "_id": user.id },
            doc! {
                "$set": {
                    "auth_factors.webauthn": serialized_keys,
                }
            },
        )
        .await?;

    Ok(())
}
