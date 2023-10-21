use super::*;
/// Result message for VerifiedAccess.CreateChallenge.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create challenge](ChallengeCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Challenge {
    /// Challenge generated with the old signing key (this will only be present during key rotation)
    #[serde(rename="alternativeChallenge")]
    
    pub alternative_challenge: Option<SignedData>,
    /// Generated challenge
    
    pub challenge: Option<SignedData>,
}

impl client::ResponseResult for Challenge {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create challenge](ChallengeCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::RequestValue for Empty {}


/// The wrapper message of any data and its signature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignedData {
    /// The data to be signed.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// The signature of the data field.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
}

impl client::Part for SignedData {}


/// signed ChallengeResponse
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify challenge](ChallengeVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyChallengeResponseRequest {
    /// The generated response to the challenge
    #[serde(rename="challengeResponse")]
    
    pub challenge_response: Option<SignedData>,
    /// Service can optionally provide identity information about the device or user associated with the key. For an EMK, this value is the enrolled domain. For an EUK, this value is the user's email address. If present, this value will be checked against contents of the response, and verification will fail if there is no match.
    #[serde(rename="expectedIdentity")]
    
    pub expected_identity: Option<String>,
}

impl client::RequestValue for VerifyChallengeResponseRequest {}


/// Result message for VerifiedAccess.VerifyChallengeResponse.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify challenge](ChallengeVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyChallengeResponseResult {
    /// Device enrollment id is returned in this field (for the machine response only).
    #[serde(rename="deviceEnrollmentId")]
    
    pub device_enrollment_id: Option<String>,
    /// Device permanent id is returned in this field (for the machine response only).
    #[serde(rename="devicePermanentId")]
    
    pub device_permanent_id: Option<String>,
    /// Certificate Signing Request (in the SPKAC format, base64 encoded) is returned in this field. This field will be set only if device has included CSR in its challenge response. (the option to include CSR is now available for both user and machine responses)
    #[serde(rename="signedPublicKeyAndChallenge")]
    
    pub signed_public_key_and_challenge: Option<String>,
    /// For EMCert check, device permanent id is returned here. For EUCert check, signed_public_key_and_challenge [base64 encoded] is returned if present, otherwise empty string is returned. This field is deprecated, please use device_permanent_id or signed_public_key_and_challenge fields.
    #[serde(rename="verificationOutput")]
    
    pub verification_output: Option<String>,
}

impl client::ResponseResult for VerifyChallengeResponseResult {}


