use base64::Engine;
use color_eyre::eyre::Context;
use mongodb::bson::doc;
use webauthn_rs::prelude::{AuthenticationResult, Passkey};

use crate::{axum_error::AxumResult, database::User, state::AppState};

pub async fn update_webauthn_credentials(
    state: &AppState,
    user: &User,
    auth_result: &AuthenticationResult,
) -> AxumResult<()> {
    let cred_id =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(auth_result.cred_id().as_ref());

    let factor = user
        .auth_factors
        .webauthn
        .iter()
        .find(|f| f.credential_id == cred_id);

    let Some(factor) = factor else {
        return Ok(());
    };

    let mut passkey: Passkey =
        serde_json::from_str(&factor.serialized_key).wrap_err("Failed to deserialize passkey")?;
    passkey.update_credential(auth_result);

    let updated_key = serde_json::to_string(&passkey).wrap_err("Failed to serialize passkey")?;

    state
        .database
        .collection::<User>("users")
        .update_one(
            doc! { "_id": user.id, "auth_factors.webauthn.credential_id": &cred_id },
            doc! {
                "$set": {
                    "auth_factors.webauthn.$.serialized_key": updated_key,
                }
            },
        )
        .await?;

    Ok(())
}
