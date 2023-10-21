use super::*;
/// A builder providing access to all methods supported on *relyingparty* resources.
/// It is not used directly, but through the [`IdentityToolkit`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_identitytoolkit3 as identitytoolkit3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use identitytoolkit3::{IdentityToolkit, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = IdentityToolkit::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create_auth_uri(...)`, `delete_account(...)`, `download_account(...)`, `email_link_signin(...)`, `get_account_info(...)`, `get_oob_confirmation_code(...)`, `get_project_config(...)`, `get_public_keys(...)`, `get_recaptcha_param(...)`, `reset_password(...)`, `send_verification_code(...)`, `set_account_info(...)`, `set_project_config(...)`, `sign_out_user(...)`, `signup_new_user(...)`, `upload_account(...)`, `verify_assertion(...)`, `verify_custom_token(...)`, `verify_password(...)` and `verify_phone_number(...)`
/// // to build up your call.
/// let rb = hub.relyingparty();
/// # }
/// ```
pub struct RelyingpartyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a IdentityToolkit<S>,
}

impl<'a, S> client::MethodsBuilder for RelyingpartyMethods<'a, S> {}

impl<'a, S> RelyingpartyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates the URI used by the IdP to authenticate the user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create_auth_uri(&self, request: IdentitytoolkitRelyingpartyCreateAuthUriRequest) -> RelyingpartyCreateAuthUriCall<'a, S> {
        RelyingpartyCreateAuthUriCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete user account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn delete_account(&self, request: IdentitytoolkitRelyingpartyDeleteAccountRequest) -> RelyingpartyDeleteAccountCall<'a, S> {
        RelyingpartyDeleteAccountCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch download user accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn download_account(&self, request: IdentitytoolkitRelyingpartyDownloadAccountRequest) -> RelyingpartyDownloadAccountCall<'a, S> {
        RelyingpartyDownloadAccountCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reset password for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn email_link_signin(&self, request: IdentitytoolkitRelyingpartyEmailLinkSigninRequest) -> RelyingpartyEmailLinkSigninCall<'a, S> {
        RelyingpartyEmailLinkSigninCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the account info.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_account_info(&self, request: IdentitytoolkitRelyingpartyGetAccountInfoRequest) -> RelyingpartyGetAccountInfoCall<'a, S> {
        RelyingpartyGetAccountInfoCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a code for user action confirmation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_oob_confirmation_code(&self, request: Relyingparty) -> RelyingpartyGetOobConfirmationCodeCall<'a, S> {
        RelyingpartyGetOobConfirmationCodeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get project configuration.
    pub fn get_project_config(&self) -> RelyingpartyGetProjectConfigCall<'a, S> {
        RelyingpartyGetProjectConfigCall {
            hub: self.hub,
            _project_number: Default::default(),
            _delegated_project_number: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get token signing public key.
    pub fn get_public_keys(&self) -> RelyingpartyGetPublicKeyCall<'a, S> {
        RelyingpartyGetPublicKeyCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get recaptcha secure param.
    pub fn get_recaptcha_param(&self) -> RelyingpartyGetRecaptchaParamCall<'a, S> {
        RelyingpartyGetRecaptchaParamCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reset password for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reset_password(&self, request: IdentitytoolkitRelyingpartyResetPasswordRequest) -> RelyingpartyResetPasswordCall<'a, S> {
        RelyingpartyResetPasswordCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Send SMS verification code.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn send_verification_code(&self, request: IdentitytoolkitRelyingpartySendVerificationCodeRequest) -> RelyingpartySendVerificationCodeCall<'a, S> {
        RelyingpartySendVerificationCodeCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set account info for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn set_account_info(&self, request: IdentitytoolkitRelyingpartySetAccountInfoRequest) -> RelyingpartySetAccountInfoCall<'a, S> {
        RelyingpartySetAccountInfoCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set project configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn set_project_config(&self, request: IdentitytoolkitRelyingpartySetProjectConfigRequest) -> RelyingpartySetProjectConfigCall<'a, S> {
        RelyingpartySetProjectConfigCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sign out user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn sign_out_user(&self, request: IdentitytoolkitRelyingpartySignOutUserRequest) -> RelyingpartySignOutUserCall<'a, S> {
        RelyingpartySignOutUserCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Signup new user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn signup_new_user(&self, request: IdentitytoolkitRelyingpartySignupNewUserRequest) -> RelyingpartySignupNewUserCall<'a, S> {
        RelyingpartySignupNewUserCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch upload existing user accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn upload_account(&self, request: IdentitytoolkitRelyingpartyUploadAccountRequest) -> RelyingpartyUploadAccountCall<'a, S> {
        RelyingpartyUploadAccountCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the assertion returned by the IdP.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify_assertion(&self, request: IdentitytoolkitRelyingpartyVerifyAssertionRequest) -> RelyingpartyVerifyAssertionCall<'a, S> {
        RelyingpartyVerifyAssertionCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the developer asserted ID token.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify_custom_token(&self, request: IdentitytoolkitRelyingpartyVerifyCustomTokenRequest) -> RelyingpartyVerifyCustomTokenCall<'a, S> {
        RelyingpartyVerifyCustomTokenCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the user entered password.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify_password(&self, request: IdentitytoolkitRelyingpartyVerifyPasswordRequest) -> RelyingpartyVerifyPasswordCall<'a, S> {
        RelyingpartyVerifyPasswordCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies ownership of a phone number and creates/updates the user account accordingly.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn verify_phone_number(&self, request: IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest) -> RelyingpartyVerifyPhoneNumberCall<'a, S> {
        RelyingpartyVerifyPhoneNumberCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



