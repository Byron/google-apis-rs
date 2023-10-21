use super::*;
/// A builder providing access to all methods supported on *jwk* resources.
/// It is not used directly, but through the [`Firebaseappcheck`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebaseappcheck1_beta as firebaseappcheck1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebaseappcheck1_beta::{Firebaseappcheck, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Firebaseappcheck::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.jwks();
/// # }
/// ```
pub struct JwkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Firebaseappcheck<S>,
}

impl<'a, S> client::MethodsBuilder for JwkMethods<'a, S> {}

impl<'a, S> JwkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a public JWK set as specified by [RFC 7517](https://tools.ietf.org/html/rfc7517) that can be used to verify App Check tokens. Exactly one of the public keys in the returned set will successfully validate any App Check token that is currently valid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name to the public JWK set. Must always be exactly the string `jwks`.
    pub fn get(&self, name: &str) -> JwkGetCall<'a, S> {
        JwkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Firebaseappcheck`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_firebaseappcheck1_beta as firebaseappcheck1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use firebaseappcheck1_beta::{Firebaseappcheck, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Firebaseappcheck::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apps_app_attest_config_batch_get(...)`, `apps_app_attest_config_get(...)`, `apps_app_attest_config_patch(...)`, `apps_debug_tokens_create(...)`, `apps_debug_tokens_delete(...)`, `apps_debug_tokens_get(...)`, `apps_debug_tokens_list(...)`, `apps_debug_tokens_patch(...)`, `apps_device_check_config_batch_get(...)`, `apps_device_check_config_get(...)`, `apps_device_check_config_patch(...)`, `apps_exchange_app_attest_assertion(...)`, `apps_exchange_app_attest_attestation(...)`, `apps_exchange_custom_token(...)`, `apps_exchange_debug_token(...)`, `apps_exchange_device_check_token(...)`, `apps_exchange_play_integrity_token(...)`, `apps_exchange_recaptcha_enterprise_token(...)`, `apps_exchange_recaptcha_token(...)`, `apps_exchange_recaptcha_v3_token(...)`, `apps_exchange_safety_net_token(...)`, `apps_generate_app_attest_challenge(...)`, `apps_generate_play_integrity_challenge(...)`, `apps_play_integrity_config_batch_get(...)`, `apps_play_integrity_config_get(...)`, `apps_play_integrity_config_patch(...)`, `apps_recaptcha_config_batch_get(...)`, `apps_recaptcha_config_get(...)`, `apps_recaptcha_config_patch(...)`, `apps_recaptcha_enterprise_config_batch_get(...)`, `apps_recaptcha_enterprise_config_get(...)`, `apps_recaptcha_enterprise_config_patch(...)`, `apps_recaptcha_v3_config_batch_get(...)`, `apps_recaptcha_v3_config_get(...)`, `apps_recaptcha_v3_config_patch(...)`, `apps_safety_net_config_batch_get(...)`, `apps_safety_net_config_get(...)`, `apps_safety_net_config_patch(...)`, `services_batch_update(...)`, `services_get(...)`, `services_list(...)` and `services_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Firebaseappcheck<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the AppAttestConfigs for the specified list of apps.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all AppAttestConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_app_attest_config_batch_get(&self, parent: &str) -> ProjectAppAppAttestConfigBatchGetCall<'a, S> {
        ProjectAppAppAttestConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the AppAttestConfig for the specified app.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the AppAttestConfig, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ```
    pub fn apps_app_attest_config_get(&self, name: &str) -> ProjectAppAppAttestConfigGetCall<'a, S> {
        ProjectAppAppAttestConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the AppAttestConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange AppAttest tokens for App Check tokens.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the App Attest configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/appAttestConfig ```
    pub fn apps_app_attest_config_patch(&self, request: GoogleFirebaseAppcheckV1betaAppAttestConfig, name: &str) -> ProjectAppAppAttestConfigPatchCall<'a, S> {
        ProjectAppAppAttestConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new DebugToken for the specified app. For security reasons, after the creation operation completes, the `token` field cannot be updated or retrieved, but you can revoke the debug token using DeleteDebugToken. Each app can have a maximum of 20 debug tokens.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the parent app in which the specified DebugToken will be created, in the format: ``` projects/{project_number}/apps/{app_id} ```
    pub fn apps_debug_tokens_create(&self, request: GoogleFirebaseAppcheckV1betaDebugToken, parent: &str) -> ProjectAppDebugTokenCreateCall<'a, S> {
        ProjectAppDebugTokenCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified DebugToken. A deleted debug token cannot be used to exchange for an App Check token. Use this method when you suspect the secret `token` has been compromised or when you no longer need the debug token.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the DebugToken to delete, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ```
    pub fn apps_debug_tokens_delete(&self, name: &str) -> ProjectAppDebugTokenDeleteCall<'a, S> {
        ProjectAppDebugTokenDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified DebugToken. For security reasons, the `token` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ```
    pub fn apps_debug_tokens_get(&self, name: &str) -> ProjectAppDebugTokenGetCall<'a, S> {
        ProjectAppDebugTokenGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all DebugTokens for the specified app. For security reasons, the `token` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the parent app for which to list each associated DebugToken, in the format: ``` projects/{project_number}/apps/{app_id} ```
    pub fn apps_debug_tokens_list(&self, parent: &str) -> ProjectAppDebugTokenListCall<'a, S> {
        ProjectAppDebugTokenListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified DebugToken. For security reasons, the `token` field cannot be updated, nor will it be populated in the response, but you can revoke the debug token using DeleteDebugToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the debug token, in the format: ``` projects/{project_number}/apps/{app_id}/debugTokens/{debug_token_id} ```
    pub fn apps_debug_tokens_patch(&self, request: GoogleFirebaseAppcheckV1betaDebugToken, name: &str) -> ProjectAppDebugTokenPatchCall<'a, S> {
        ProjectAppDebugTokenPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the DeviceCheckConfigs for the specified list of apps. For security reasons, the `private_key` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all DeviceCheckConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_device_check_config_batch_get(&self, parent: &str) -> ProjectAppDeviceCheckConfigBatchGetCall<'a, S> {
        ProjectAppDeviceCheckConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the DeviceCheckConfig for the specified app. For security reasons, the `private_key` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the DeviceCheckConfig, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ```
    pub fn apps_device_check_config_get(&self, name: &str) -> ProjectAppDeviceCheckConfigGetCall<'a, S> {
        ProjectAppDeviceCheckConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the DeviceCheckConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange DeviceCheck tokens for App Check tokens. For security reasons, the `private_key` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the DeviceCheck configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/deviceCheckConfig ```
    pub fn apps_device_check_config_patch(&self, request: GoogleFirebaseAppcheckV1betaDeviceCheckConfig, name: &str) -> ProjectAppDeviceCheckConfigPatchCall<'a, S> {
        ProjectAppDeviceCheckConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the PlayIntegrityConfigs for the specified list of apps.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all PlayIntegrityConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_play_integrity_config_batch_get(&self, parent: &str) -> ProjectAppPlayIntegrityConfigBatchGetCall<'a, S> {
        ProjectAppPlayIntegrityConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the PlayIntegrityConfig for the specified app.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the PlayIntegrityConfig, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ```
    pub fn apps_play_integrity_config_get(&self, name: &str) -> ProjectAppPlayIntegrityConfigGetCall<'a, S> {
        ProjectAppPlayIntegrityConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the PlayIntegrityConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange Play Integrity tokens for App Check tokens.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the Play Integrity configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/playIntegrityConfig ```
    pub fn apps_play_integrity_config_patch(&self, request: GoogleFirebaseAppcheckV1betaPlayIntegrityConfig, name: &str) -> ProjectAppPlayIntegrityConfigPatchCall<'a, S> {
        ProjectAppPlayIntegrityConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the RecaptchaConfigs for the specified list of apps. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all RecaptchaConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_recaptcha_config_batch_get(&self, parent: &str) -> ProjectAppRecaptchaConfigBatchGetCall<'a, S> {
        ProjectAppRecaptchaConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the RecaptchaConfig for the specified app. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the RecaptchaConfig, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ```
    pub fn apps_recaptcha_config_get(&self, name: &str) -> ProjectAppRecaptchaConfigGetCall<'a, S> {
        ProjectAppRecaptchaConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the RecaptchaConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA tokens for App Check tokens. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaConfig ```
    pub fn apps_recaptcha_config_patch(&self, request: GoogleFirebaseAppcheckV1betaRecaptchaConfig, name: &str) -> ProjectAppRecaptchaConfigPatchCall<'a, S> {
        ProjectAppRecaptchaConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the RecaptchaEnterpriseConfigs for the specified list of apps.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all RecaptchaEnterpriseConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_recaptcha_enterprise_config_batch_get(&self, parent: &str) -> ProjectAppRecaptchaEnterpriseConfigBatchGetCall<'a, S> {
        ProjectAppRecaptchaEnterpriseConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the RecaptchaEnterpriseConfig for the specified app.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the RecaptchaEnterpriseConfig, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ```
    pub fn apps_recaptcha_enterprise_config_get(&self, name: &str) -> ProjectAppRecaptchaEnterpriseConfigGetCall<'a, S> {
        ProjectAppRecaptchaEnterpriseConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the RecaptchaEnterpriseConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA Enterprise tokens for App Check tokens.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the reCAPTCHA Enterprise configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaEnterpriseConfig ```
    pub fn apps_recaptcha_enterprise_config_patch(&self, request: GoogleFirebaseAppcheckV1betaRecaptchaEnterpriseConfig, name: &str) -> ProjectAppRecaptchaEnterpriseConfigPatchCall<'a, S> {
        ProjectAppRecaptchaEnterpriseConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the RecaptchaV3Configs for the specified list of apps. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all RecaptchaV3Configs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_recaptcha_v3_config_batch_get(&self, parent: &str) -> ProjectAppRecaptchaV3ConfigBatchGetCall<'a, S> {
        ProjectAppRecaptchaV3ConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the RecaptchaV3Config for the specified app. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the RecaptchaV3Config, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ```
    pub fn apps_recaptcha_v3_config_get(&self, name: &str) -> ProjectAppRecaptchaV3ConfigGetCall<'a, S> {
        ProjectAppRecaptchaV3ConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the RecaptchaV3Config for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange reCAPTCHA V3 tokens for App Check tokens. For security reasons, the `site_secret` field is never populated in the response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the reCAPTCHA v3 configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/recaptchaV3Config ```
    pub fn apps_recaptcha_v3_config_patch(&self, request: GoogleFirebaseAppcheckV1betaRecaptchaV3Config, name: &str) -> ProjectAppRecaptchaV3ConfigPatchCall<'a, S> {
        ProjectAppRecaptchaV3ConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically gets the SafetyNetConfigs for the specified list of apps.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project name shared by all SafetyNetConfigs being retrieved, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being retrieved must match this field, or the entire batch fails.
    pub fn apps_safety_net_config_batch_get(&self, parent: &str) -> ProjectAppSafetyNetConfigBatchGetCall<'a, S> {
        ProjectAppSafetyNetConfigBatchGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the SafetyNetConfig for the specified app.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the SafetyNetConfig, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ```
    pub fn apps_safety_net_config_get(&self, name: &str) -> ProjectAppSafetyNetConfigGetCall<'a, S> {
        ProjectAppSafetyNetConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the SafetyNetConfig for the specified app. While this configuration is incomplete or invalid, the app will be unable to exchange SafetyNet tokens for App Check tokens.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the SafetyNet configuration object, in the format: ``` projects/{project_number}/apps/{app_id}/safetyNetConfig ```
    pub fn apps_safety_net_config_patch(&self, request: GoogleFirebaseAppcheckV1betaSafetyNetConfig, name: &str) -> ProjectAppSafetyNetConfigPatchCall<'a, S> {
        ProjectAppSafetyNetConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts an App Attest assertion and an artifact previously obtained from ExchangeAppAttestAttestation and verifies those with Apple. If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_app_attest_assertion(&self, request: GoogleFirebaseAppcheckV1betaExchangeAppAttestAssertionRequest, app: &str) -> ProjectAppExchangeAppAttestAssertionCall<'a, S> {
        ProjectAppExchangeAppAttestAssertionCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts an App Attest CBOR attestation and verifies it with Apple using your preconfigured team and bundle IDs. If valid, returns an attestation artifact that can later be exchanged for an AppCheckToken using ExchangeAppAttestAssertion. For convenience and performance, this method's response object will also contain an AppCheckToken (if the verification is successful).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_app_attest_attestation(&self, request: GoogleFirebaseAppcheckV1betaExchangeAppAttestAttestationRequest, app: &str) -> ProjectAppExchangeAppAttestAttestationCall<'a, S> {
        ProjectAppExchangeAppAttestAttestationCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a custom token signed using your project's Admin SDK service account credentials. If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_custom_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeCustomTokenRequest, app: &str) -> ProjectAppExchangeCustomTokenCall<'a, S> {
        ProjectAppExchangeCustomTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a debug token secret that you have previously created using CreateDebugToken. If valid, returns an AppCheckToken. Note that a restrictive quota is enforced on this method to prevent accidental exposure of the app to abuse.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_debug_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeDebugTokenRequest, app: &str) -> ProjectAppExchangeDebugTokenCall<'a, S> {
        ProjectAppExchangeDebugTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts a [`device_token`](https://developer.apple.com/documentation/devicecheck/dcdevice) issued by DeviceCheck, and attempts to validate it with Apple. If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_device_check_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeDeviceCheckTokenRequest, app: &str) -> ProjectAppExchangeDeviceCheckTokenCall<'a, S> {
        ProjectAppExchangeDeviceCheckTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates an [integrity verdict response token from Play Integrity](https://developer.android.com/google/play/integrity/verdict#decrypt-verify). If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the Android app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_play_integrity_token(&self, request: GoogleFirebaseAppcheckV1betaExchangePlayIntegrityTokenRequest, app: &str) -> ProjectAppExchangePlayIntegrityTokenCall<'a, S> {
        ProjectAppExchangePlayIntegrityTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a [reCAPTCHA Enterprise response token](https://cloud.google.com/recaptcha-enterprise/docs/create-assessment#retrieve_token). If valid, returns an App Check token AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the web app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_recaptcha_enterprise_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeRecaptchaEnterpriseTokenRequest, app: &str) -> ProjectAppExchangeRecaptchaEnterpriseTokenCall<'a, S> {
        ProjectAppExchangeRecaptchaEnterpriseTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a [reCAPTCHA v3 response token](https://developers.google.com/recaptcha/docs/v3). If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the web app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_recaptcha_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeRecaptchaTokenRequest, app: &str) -> ProjectAppExchangeRecaptchaTokenCall<'a, S> {
        ProjectAppExchangeRecaptchaTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a [reCAPTCHA v3 response token](https://developers.google.com/recaptcha/docs/v3). If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the web app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_recaptcha_v3_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeRecaptchaV3TokenRequest, app: &str) -> ProjectAppExchangeRecaptchaV3TokenCall<'a, S> {
        ProjectAppExchangeRecaptchaV3TokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates a [SafetyNet token](https://developer.android.com/training/safetynet/attestation#request-attestation-step). If valid, returns an AppCheckToken.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the Android app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_exchange_safety_net_token(&self, request: GoogleFirebaseAppcheckV1betaExchangeSafetyNetTokenRequest, app: &str) -> ProjectAppExchangeSafetyNetTokenCall<'a, S> {
        ProjectAppExchangeSafetyNetTokenCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a challenge that protects the integrity of an immediately following call to ExchangeAppAttestAttestation or ExchangeAppAttestAssertion. A challenge should not be reused for multiple calls.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the iOS app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_generate_app_attest_challenge(&self, request: GoogleFirebaseAppcheckV1betaGenerateAppAttestChallengeRequest, app: &str) -> ProjectAppGenerateAppAttestChallengeCall<'a, S> {
        ProjectAppGenerateAppAttestChallengeCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a challenge that protects the integrity of an immediately following integrity verdict request to the Play Integrity API. The next call to ExchangePlayIntegrityToken using the resulting integrity token will verify the presence and validity of the challenge. A challenge should not be reused for multiple calls.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `app` - Required. The relative resource name of the app, in the format: ``` projects/{project_number}/apps/{app_id} ``` If necessary, the `project_number` element can be replaced with the project ID of the Firebase project. Learn more about using project identifiers in Google's [AIP 2510](https://google.aip.dev/cloud/2510) standard.
    pub fn apps_generate_play_integrity_challenge(&self, request: GoogleFirebaseAppcheckV1betaGeneratePlayIntegrityChallengeRequest, app: &str) -> ProjectAppGeneratePlayIntegrityChallengeCall<'a, S> {
        ProjectAppGeneratePlayIntegrityChallengeCall {
            hub: self.hub,
            _request: request,
            _app: app.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Atomically updates the specified Service configurations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent project name shared by all Service configurations being updated, in the format ``` projects/{project_number} ``` The parent collection in the `name` field of any resource being updated must match this field, or the entire batch fails.
    pub fn services_batch_update(&self, request: GoogleFirebaseAppcheckV1betaBatchUpdateServicesRequest, parent: &str) -> ProjectServiceBatchUpdateCall<'a, S> {
        ProjectServiceBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Service configuration for the specified service name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The relative resource name of the Service to retrieve, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore)
    pub fn services_get(&self, name: &str) -> ProjectServiceGetCall<'a, S> {
        ProjectServiceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Service configurations for the specified project. Only Services which were explicitly configured using UpdateService or BatchUpdateServices will be returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative resource name of the parent project for which to list each associated Service, in the format: ``` projects/{project_number} ```
    pub fn services_list(&self, parent: &str) -> ProjectServiceListCall<'a, S> {
        ProjectServiceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Service configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The relative resource name of the service configuration object, in the format: ``` projects/{project_number}/services/{service_id} ``` Note that the `service_id` element must be a supported service ID. Currently, the following service IDs are supported: * `firebasestorage.googleapis.com` (Cloud Storage for Firebase) * `firebasedatabase.googleapis.com` (Firebase Realtime Database) * `firestore.googleapis.com` (Cloud Firestore)
    pub fn services_patch(&self, request: GoogleFirebaseAppcheckV1betaService, name: &str) -> ProjectServicePatchCall<'a, S> {
        ProjectServicePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



