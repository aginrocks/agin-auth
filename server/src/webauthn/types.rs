//! Types related to attestation (Registration)

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// <https://w3c.github.io/webauthn/#dictionary-makecredentialoptions>
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    /// The relying party
    pub rp: RelyingParty,
    /// The user.
    pub user: User,
    /// The one-time challenge for the credential to sign.
    #[schema(value_type = String)]
    pub challenge: base64urlsafedata::Base64UrlSafeData,
    /// The set of cryptographic types allowed by this server.
    pub pub_key_cred_params: Vec<PubKeyCredParams>,

    /// The timeout for the authenticator to stop accepting the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    /// Credential ID's that are excluded from being able to be registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_credentials: Option<Vec<PublicKeyCredentialDescriptor>>,

    /// Criteria defining which authenticators may be used in this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_selection: Option<AuthenticatorSelectionCriteria>,

    /// Hints defining which credentials may be used in this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<PublicKeyCredentialHints>>,

    /// The requested attestation level from the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation: Option<AttestationConveyancePreference>,

    /// The list of attestation formats that the RP will accept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_formats: Option<Vec<AttestationFormat>>,

    /// Non-standard extensions that may be used by the browser/authenticator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<RequestRegistrationExtensions>,
}

/// A JSON serializable challenge which is issued to the user's web browser
/// for handling. This is meant to be opaque, that is, you should not need
/// to inspect or alter the content of the struct - you should serialise it
/// and transmit it to the client only.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreationChallengeResponse {
    /// The options.
    pub public_key: PublicKeyCredentialCreationOptions,
}

/// <https://w3c.github.io/webauthn/#authenticatorattestationresponse>
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct AuthenticatorAttestationResponseRaw {
    /// <https://w3c.github.io/webauthn/#dom-authenticatorattestationresponse-attestationobject>
    #[serde(rename = "attestationObject")]
    #[schema(value_type = String)]
    pub attestation_object: base64urlsafedata::Base64UrlSafeData,

    /// <https://w3c.github.io/webauthn/#dom-authenticatorresponse-clientdatajson>
    #[serde(rename = "clientDataJSON")]
    #[schema(value_type = String)]
    pub client_data_json: base64urlsafedata::Base64UrlSafeData,

    /// <https://w3c.github.io/webauthn/#dom-authenticatorattestationresponse-gettransports>
    #[serde(default)]
    pub transports: Option<Vec<AuthenticatorTransport>>,
}

/// A client response to a registration challenge. This contains all required
/// information to assess and assert trust in a credential's legitimacy, followed
/// by registration to a user.
///
/// You should not need to handle the inner content of this structure - you should
/// provide this to the correctly handling function of Webauthn only.
/// <https://w3c.github.io/webauthn/#iface-pkcredential>
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct RegisterPublicKeyCredential {
    /// The id of the PublicKey credential, likely in base64.
    ///
    /// This is NEVER actually
    /// used in a real registration, because the true credential ID is taken from the
    /// attestation data.
    pub id: String,
    /// The id of the credential, as binary.
    ///
    /// This is NEVER actually
    /// used in a real registration, because the true credential ID is taken from the
    /// attestation data.
    #[serde(rename = "rawId")]
    #[schema(value_type = String)]
    pub raw_id: base64urlsafedata::Base64UrlSafeData,
    /// <https://w3c.github.io/webauthn/#dom-publickeycredential-response>
    pub response: AuthenticatorAttestationResponseRaw,
    /// The type of credential.
    #[serde(rename = "type")]
    pub type_: String,
    /// Unsigned Client processed extensions.
    #[serde(default)]
    pub extensions: RegistrationExtensionsClientOutputs,
}

// Extensions allowing certain types of authenticators to provide supplemental information.

/// Valid credential protection policies
#[derive(Debug, Serialize, Clone, Copy, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum CredentialProtectionPolicy {
    /// This reflects "FIDO_2_0" semantics. In this configuration, performing
    /// some form of user verification is optional with or without credentialID
    /// list. This is the default state of the credential if the extension is
    /// not specified.
    UserVerificationOptional = 0x1,
    /// In this configuration, credential is discovered only when its
    /// credentialID is provided by the platform or when some form of user
    /// verification is performed.
    UserVerificationOptionalWithCredentialIDList = 0x2,
    /// This reflects that discovery and usage of the credential MUST be
    /// preceded by some form of user verification.
    UserVerificationRequired = 0x3,
}

/// The desired options for the client's use of the `credProtect` extension
///
/// <https://fidoalliance.org/specs/fido-v2.1-rd-20210309/fido-client-to-authenticator-protocol-v2.1-rd-20210309.html#sctn-credProtect-extension>
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CredProtect {
    /// The credential policy to enact
    pub credential_protection_policy: CredentialProtectionPolicy,
    /// Whether it is better for the authenticator to fail to create a
    /// credential rather than ignore the protection policy
    /// If no value is provided, the client treats it as `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_credential_protection_policy: Option<bool>,
}

/// Extension option inputs for PublicKeyCredentialCreationOptions.
///
/// Implements \[AuthenticatorExtensionsClientInputs\] from the spec.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestRegistrationExtensions {
    /// The `credProtect` extension options
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub cred_protect: Option<CredProtect>,

    /// ⚠️  - Browsers do not support this!
    /// Uvm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uvm: Option<bool>,

    /// ⚠️  - This extension result is always unsigned, and only indicates if the
    /// browser *requests* a residentKey to be created. It has no bearing on the
    /// true rk state of the credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_props: Option<bool>,

    /// CTAP2.1 Minumum pin length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pin_length: Option<bool>,

    /// ⚠️  - Browsers support the *creation* of the secret, but not the retrieval of it.
    /// CTAP2.1 create hmac secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmac_create_secret: Option<bool>,
}

// ========== Auth exten ============

/// The inputs to the hmac secret if it was created during registration.
///
/// <https://fidoalliance.22org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension>
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HmacGetSecretInput {
    /// Retrieve a symmetric secrets from the authenticator with this input.
    #[schema(value_type = String)]
    pub output1: base64urlsafedata::Base64UrlSafeData,
    /// Rotate the secret in the same operation.
    #[schema(value_type = Option<String>)]
    pub output2: Option<base64urlsafedata::Base64UrlSafeData>,
}

/// Extension option inputs for PublicKeyCredentialRequestOptions
///
/// Implements \[AuthenticatorExtensionsClientInputs\] from the spec
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestAuthenticationExtensions {
    /// The `appid` extension options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,

    /// ⚠️  - Browsers do not support this!
    /// Uvm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uvm: Option<bool>,

    /// ⚠️  - Browsers do not support this!
    /// <https://bugs.chromium.org/p/chromium/issues/detail?id=1023225>
    /// Hmac get secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmac_get_secret: Option<HmacGetSecretInput>,
}

/// The response to a hmac get secret request.
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HmacGetSecretOutput {
    /// Output of HMAC(Salt 1 || Client Secret)
    #[schema(value_type = String)]
    pub output1: base64urlsafedata::Base64UrlSafeData,
    /// Output of HMAC(Salt 2 || Client Secret)
    #[schema(value_type = Option<String>)]
    pub output2: Option<base64urlsafedata::Base64UrlSafeData>,
}

/// <https://w3c.github.io/webauthn/#dictdef-authenticationextensionsclientoutputs>
/// The default option here for Options are None, so it can be derived
#[derive(Debug, Deserialize, Serialize, Clone, Default, ToSchema)]
pub struct AuthenticationExtensionsClientOutputs {
    /// Indicates whether the client used the provided appid extension
    #[serde(default)]
    pub appid: Option<bool>,
    /// The response to a hmac get secret request.
    #[serde(default)]
    pub hmac_get_secret: Option<HmacGetSecretOutput>,
}

/// <https://www.w3.org/TR/webauthn-3/#sctn-authenticator-credential-properties-extension>
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CredProps {
    /// A user agent supplied hint that this credential *may* have created a resident key. It is
    /// retured from the user agent, not the authenticator meaning that this is an unreliable
    /// signal.
    ///
    /// Note that this extension is UNSIGNED and may have been altered by page javascript.
    pub rk: bool,
}

/// <https://w3c.github.io/webauthn/#dictdef-authenticationextensionsclientoutputs>
/// The default option here for Options are None, so it can be derived
#[derive(Debug, Deserialize, Serialize, Clone, Default, ToSchema)]
pub struct RegistrationExtensionsClientOutputs {
    /// Indicates whether the client used the provided appid extension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appid: Option<bool>,

    /// Indicates if the client believes it created a resident key. This
    /// property is managed by the webbrowser, and is NOT SIGNED and CAN NOT be trusted!
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cred_props: Option<CredProps>,

    /// Indicates if the client successfully applied a HMAC Secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac_secret: Option<bool>,

    /// Indicates if the client successfully applied a credential protection policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cred_protect: Option<CredentialProtectionPolicy>,

    /// Indicates the current minimum PIN length
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_pin_length: Option<u32>,
}

/// The result state of an extension as returned from the authenticator.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub enum ExtnState<T>
where
    T: Clone + std::fmt::Debug + ToSchema,
{
    /// This extension was not requested, and so no result was provided.
    #[default]
    NotRequested,
    /// The extension was requested, and the authenticator did NOT act on it.
    Ignored,
    /// The extension was requested, and the authenticator correctly responded.
    Set(T),
    /// The extension was not requested, and the authenticator sent an unsolicited extension value.
    Unsolicited(T),
    /// ⚠️  WARNING: The data in this extension is not signed cryptographically, and can not be
    /// trusted for security assertions. It MAY be used for UI/UX hints.
    Unsigned(T),
}

/// The set of extensions that were registered by this credential.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct RegisteredExtensions {
    // ⚠️  It's critical we place serde default here so that we
    // can deserialise in the future as we add new types!
    /// The state of the cred_protect extension
    #[serde(default)]
    pub cred_protect: ExtnState<CredentialProtectionPolicy>,
    /// The state of the hmac-secret extension, if it was created
    #[serde(default)]
    pub hmac_create_secret: ExtnState<bool>,
    /// The state of the client appid extensions
    #[serde(default)]
    pub appid: ExtnState<bool>,
    /// The state of the client credential properties extension
    #[serde(default)]
    pub cred_props: ExtnState<CredProps>,
}

/// The set of extensions that were provided by the client during authentication
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthenticationExtensions {}

// Types that define options as to how an authenticator may interact with
// with the server.

/// Defines the User Authenticator Verification policy. This is documented
/// <https://w3c.github.io/webauthn/#enumdef-userverificationrequirement>, and each
/// variant lists it's effects.
///
/// To be clear, Verification means that the Authenticator perform extra or supplementary
/// interaction with the user to verify who they are. An example of this is Apple Touch Id
/// required a fingerprint to be verified, or a yubico device requiring a pin in addition to
/// a touch event.
///
/// An example of a non-verified interaction is a yubico device with no pin where touch is
/// the only interaction - we only verify a user is present, but we don't have extra details
/// to the legitimacy of that user.
///
/// As UserVerificationPolicy is *only* used in credential registration, this stores the
/// verification state of the credential in the persisted credential. These persisted
/// credentials define which UserVerificationPolicy is issued during authentications.
///
/// **IMPORTANT** - Due to limitations of the webauthn specification, CTAP devices, and browser
/// implementations, the only secure choice as an RP is *required*.
///
/// > ⚠️  **WARNING** - discouraged is marked with a warning, as some authenticators
/// > will FORCE verification during registration but NOT during authentication.
/// > This makes it impossible for a relying party to *consistently* enforce user verification,
/// > which can confuse users and lead them to distrust user verification is being enforced.
///
/// > ⚠️  **WARNING** - preferred can lead to authentication errors in some cases due to browser
/// > peripheral exchange allowing authentication verification bypass. Webauthn RS is not vulnerable
/// > to these bypasses due to our
/// > tracking of UV during registration through authentication, however preferred can cause
/// > legitimate credentials to not prompt for UV correctly due to browser perhipheral exchange
/// > leading Webauthn RS to deny them in what should otherwise be legitimate operations.
#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema,
)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "lowercase")]
pub enum UserVerificationPolicy {
    /// Require user verification bit to be set, and fail the registration or authentication
    /// if false. If the authenticator is not able to perform verification, it will not be
    /// usable with this policy.
    ///
    /// This policy is the default as it is the only secure and consistent user verification option.
    #[serde(rename = "required")]
    #[default]
    Required,
    /// Prefer UV if possible, but ignore if not present. In other webauthn deployments this is bypassable
    /// as it implies the library will not check UV is set correctly for this credential. Webauthn-RS
    /// is *not* vulnerable to this as we check the UV state always based on it's presence at registration.
    ///
    /// However, in some cases use of this policy can lead to some credentials failing to verify
    /// correctly due to browser peripheral exchange bypasses.
    #[serde(rename = "preferred")]
    Preferred,
    /// Discourage - but do not prevent - user verification from being supplied. Many CTAP devices
    /// will attempt UV during registration but not authentication leading to user confusion.
    #[serde(rename = "discouraged")]
    Discouraged_DO_NOT_USE,
}

/// Relying Party Entity
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelyingParty {
    /// The name of the relying party.
    pub name: String,
    /// The id of the relying party.
    pub id: String,
    // Note: "icon" is deprecated: https://github.com/w3c/webauthn/pull/1337
}

/// User Entity
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The user's id in base64 form. This MUST be a unique id, and
    /// must NOT contain personally identifying information, as this value can NEVER
    /// be changed. If in doubt, use a UUID.
    #[schema(value_type = String)]
    pub id: base64urlsafedata::Base64UrlSafeData,
    /// A detailed name for the account, such as an email address. This value
    /// **can** change, so **must not** be used as a primary key.
    pub name: String,
    /// The user's preferred name for display. This value **can** change, so
    /// **must not** be used as a primary key.
    pub display_name: String,
    // Note: "icon" is deprecated: https://github.com/w3c/webauthn/pull/1337
}

/// Public key cryptographic parameters
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct PubKeyCredParams {
    /// The type of public-key credential.
    #[serde(rename = "type")]
    pub type_: String,
    /// The algorithm in use defined by COSE.
    pub alg: i64,
}

/// <https://www.w3.org/TR/webauthn/#enumdef-attestationconveyancepreference>
#[derive(Debug, Serialize, Clone, Deserialize, Default, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AttestationConveyancePreference {
    /// Do not request attestation.
    /// <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-none>
    #[default]
    None,

    /// Request attestation in a semi-anonymized form.
    /// <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-indirect>
    Indirect,

    /// Request attestation in a direct form.
    /// <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-direct>
    Direct,
}

/// <https://www.w3.org/TR/webauthn/#enumdef-authenticatortransport>
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum AuthenticatorTransport {
    /// <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-usb>
    Usb,
    /// <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-nfc>
    Nfc,
    /// <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-ble>
    Ble,
    /// <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-internal>
    Internal,
    /// Hybrid transport, formerly caBLE. Part of the level 3 draft specification.
    /// <https://w3c.github.io/webauthn/#dom-authenticatortransport-hybrid>
    Hybrid,
    /// Test transport; used for Windows 10.
    Test,
    /// An unknown transport was provided - it will be ignored.
    #[serde(other)]
    Unknown,
}

/// The type of attestation on the credential
///
/// <https://www.iana.org/assignments/webauthn/webauthn.xhtml>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, ToSchema)]
pub enum AttestationFormat {
    /// Packed attestation
    #[serde(rename = "packed", alias = "Packed")]
    Packed,
    /// TPM attestation (like Microsoft)
    #[serde(rename = "tpm", alias = "Tpm", alias = "TPM")]
    Tpm,
    /// Android hardware attestation
    #[serde(rename = "android-key", alias = "AndroidKey")]
    AndroidKey,
    /// Older Android Safety Net
    #[serde(
        rename = "android-safetynet",
        alias = "AndroidSafetyNet",
        alias = "AndroidSafetynet"
    )]
    AndroidSafetyNet,
    /// Old U2F attestation type
    #[serde(rename = "fido-u2f", alias = "FIDOU2F")]
    FIDOU2F,
    /// Apple touchID/faceID
    #[serde(rename = "apple", alias = "AppleAnonymous")]
    AppleAnonymous,
    /// No attestation
    #[serde(rename = "none", alias = "None")]
    None,
}

/// <https://www.w3.org/TR/webauthn/#dictdef-publickeycredentialdescriptor>
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
pub struct PublicKeyCredentialDescriptor {
    /// The type of credential
    #[serde(rename = "type")]
    pub type_: String,
    /// The credential id.
    #[schema(value_type = String)]
    pub id: base64urlsafedata::Base64UrlSafeData,
    /// The allowed transports for this credential. Note this is a hint, and is NOT
    /// enforced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<AuthenticatorTransport>>,
}

/// The authenticator attachment hint. This is NOT enforced, and is only used
/// to help a user select a relevant authenticator type.
///
/// <https://www.w3.org/TR/webauthn/#attachment>
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub enum AuthenticatorAttachment {
    /// Request a device that is part of the machine aka inseperable.
    /// <https://www.w3.org/TR/webauthn/#attachment>
    #[serde(rename = "platform")]
    Platform,
    /// Request a device that can be seperated from the machine aka an external token.
    /// <https://www.w3.org/TR/webauthn/#attachment>
    #[serde(rename = "cross-platform")]
    CrossPlatform,
}

/// A hint as to the class of device that is expected to fufil this operation.
///
/// <https://www.w3.org/TR/webauthn-3/#enumdef-publickeycredentialhints>
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub enum PublicKeyCredentialHints {
    /// The credential is a removable security key
    SecurityKey,
    /// The credential is a platform authenticator
    ClientDevice,
    /// The credential will come from an external device
    Hybrid,
}

/// The Relying Party's requirements for client-side discoverable credentials.
///
/// <https://www.w3.org/TR/webauthn-2/#enumdef-residentkeyrequirement>
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ResidentKeyRequirement {
    /// <https://www.w3.org/TR/webauthn-2/#dom-residentkeyrequirement-discouraged>
    Discouraged,
    /// ⚠️  In all major browsers preferred is identical in behaviour to required.
    /// You should use required instead.
    /// <https://www.w3.org/TR/webauthn-2/#dom-residentkeyrequirement-preferred>
    Preferred,
    /// <https://www.w3.org/TR/webauthn-2/#dom-residentkeyrequirement-required>
    Required,
}

/// <https://www.w3.org/TR/webauthn/#dictdef-authenticatorselectioncriteria>
#[derive(Debug, Default, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorSelectionCriteria {
    /// How the authenticator should be attached to the client machine.
    /// Note this is only a hint. It is not enforced in anyway shape or form.
    /// <https://www.w3.org/TR/webauthn/#attachment>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment: Option<AuthenticatorAttachment>,

    /// Hint to the credential to create a resident key. Note this value should be
    /// a member of ResidentKeyRequirement, but client must ignore unknown values,
    /// treating an unknown value as if the member does not exist.
    /// <https://www.w3.org/TR/webauthn-2/#dom-authenticatorselectioncriteria-residentkey>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resident_key: Option<ResidentKeyRequirement>,

    /// Hint to the credential to create a resident key. Note this can not be enforced
    /// or validated, so the authenticator may choose to ignore this parameter.
    /// <https://www.w3.org/TR/webauthn/#resident-credential>
    pub require_resident_key: bool,

    /// The user verification level to request during registration. Depending on if this
    /// authenticator provides verification may affect future interactions as this is
    /// associated to the credential during registration.
    pub user_verification: UserVerificationPolicy,
}

/// A descriptor of a credential that can be used.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct AllowCredentials {
    #[serde(rename = "type")]
    /// The type of credential.
    pub type_: String,
    /// The id of the credential.
    #[schema(value_type = String)]
    pub id: base64urlsafedata::Base64UrlSafeData,
    /// <https://www.w3.org/TR/webauthn/#transport>
    /// may be usb, nfc, ble, internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<AuthenticatorTransport>>,
}

/// The data collected and hashed in the operation.
/// <https://www.w3.org/TR/webauthn-2/#dictdef-collectedclientdata>
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct CollectedClientData {
    /// The credential type
    #[serde(rename = "type")]
    pub type_: String,
    /// The challenge.
    #[schema(value_type = String)]
    pub challenge: base64urlsafedata::Base64UrlSafeData,
    /// The rp origin as the browser understood it.
    pub origin: url::Url,
    /// The inverse of the sameOriginWithAncestors argument value that was
    /// passed into the internal method.
    #[serde(rename = "crossOrigin", skip_serializing_if = "Option::is_none")]
    pub cross_origin: Option<bool>,
    /// tokenBinding.
    #[serde(rename = "tokenBinding")]
    pub token_binding: Option<TokenBinding>,
    /// This struct be extended, so it's important to be tolerant of unknown
    /// keys.
    #[serde(flatten)]
    pub unknown_keys: std::collections::BTreeMap<String, serde_json::value::Value>,
}

/// Token binding
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct TokenBinding {
    /// status
    pub status: String,
    /// id
    pub id: Option<String>,
}
