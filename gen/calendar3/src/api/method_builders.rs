use super::*;
/// A builder providing access to all methods supported on *acl* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.acl();
/// # }
/// ```
pub struct AclMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for AclMethods<'a, S> {}

impl<'a, S> AclMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an access control rule.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `ruleId` - ACL rule identifier.
    pub fn delete(&self, calendar_id: &str, rule_id: &str) -> AclDeleteCall<'a, S> {
        AclDeleteCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _rule_id: rule_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an access control rule.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `ruleId` - ACL rule identifier.
    pub fn get(&self, calendar_id: &str, rule_id: &str) -> AclGetCall<'a, S> {
        AclGetCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _rule_id: rule_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an access control rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn insert(&self, request: AclRule, calendar_id: &str) -> AclInsertCall<'a, S> {
        AclInsertCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the rules in the access control list for the calendar.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn list(&self, calendar_id: &str) -> AclListCall<'a, S> {
        AclListCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _sync_token: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an access control rule. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `ruleId` - ACL rule identifier.
    pub fn patch(&self, request: AclRule, calendar_id: &str, rule_id: &str) -> AclPatchCall<'a, S> {
        AclPatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _rule_id: rule_id.to_string(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an access control rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `ruleId` - ACL rule identifier.
    pub fn update(&self, request: AclRule, calendar_id: &str, rule_id: &str) -> AclUpdateCall<'a, S> {
        AclUpdateCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _rule_id: rule_id.to_string(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes to ACL resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn watch(&self, request: Channel, calendar_id: &str) -> AclWatchCall<'a, S> {
        AclWatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _sync_token: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *calendarList* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.calendar_list();
/// # }
/// ```
pub struct CalendarListMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for CalendarListMethods<'a, S> {}

impl<'a, S> CalendarListMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a calendar from the user's calendar list.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn delete(&self, calendar_id: &str) -> CalendarListDeleteCall<'a, S> {
        CalendarListDeleteCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a calendar from the user's calendar list.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn get(&self, calendar_id: &str) -> CalendarListGetCall<'a, S> {
        CalendarListGetCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an existing calendar into the user's calendar list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: CalendarListEntry) -> CalendarListInsertCall<'a, S> {
        CalendarListInsertCall {
            hub: self.hub,
            _request: request,
            _color_rgb_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the calendars on the user's calendar list.
    pub fn list(&self) -> CalendarListListCall<'a, S> {
        CalendarListListCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _show_hidden: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _min_access_role: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing calendar on the user's calendar list. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn patch(&self, request: CalendarListEntry, calendar_id: &str) -> CalendarListPatchCall<'a, S> {
        CalendarListPatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _color_rgb_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing calendar on the user's calendar list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn update(&self, request: CalendarListEntry, calendar_id: &str) -> CalendarListUpdateCall<'a, S> {
        CalendarListUpdateCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _color_rgb_format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes to CalendarList resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn watch(&self, request: Channel) -> CalendarListWatchCall<'a, S> {
        CalendarListWatchCall {
            hub: self.hub,
            _request: request,
            _sync_token: Default::default(),
            _show_hidden: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _min_access_role: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *calendar* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `clear(...)`, `delete(...)`, `get(...)`, `insert(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.calendars();
/// # }
/// ```
pub struct CalendarMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for CalendarMethods<'a, S> {}

impl<'a, S> CalendarMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn clear(&self, calendar_id: &str) -> CalendarClearCall<'a, S> {
        CalendarClearCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn delete(&self, calendar_id: &str) -> CalendarDeleteCall<'a, S> {
        CalendarDeleteCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for a calendar.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn get(&self, calendar_id: &str) -> CalendarGetCall<'a, S> {
        CalendarGetCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a secondary calendar.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Calendar) -> CalendarInsertCall<'a, S> {
        CalendarInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates metadata for a calendar. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn patch(&self, request: Calendar, calendar_id: &str) -> CalendarPatchCall<'a, S> {
        CalendarPatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates metadata for a calendar.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn update(&self, request: Calendar, calendar_id: &str) -> CalendarUpdateCall<'a, S> {
        CalendarUpdateCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelMethods<'a, S> {}

impl<'a, S> ChannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a, S> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *color* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.colors();
/// # }
/// ```
pub struct ColorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for ColorMethods<'a, S> {}

impl<'a, S> ColorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the color definitions for calendars and events.
    pub fn get(&self) -> ColorGetCall<'a, S> {
        ColorGetCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *event* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `import(...)`, `insert(...)`, `instances(...)`, `list(...)`, `move_(...)`, `patch(...)`, `quick_add(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.events();
/// # }
/// ```
pub struct EventMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for EventMethods<'a, S> {}

impl<'a, S> EventMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an event.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `eventId` - Event identifier.
    pub fn delete(&self, calendar_id: &str, event_id: &str) -> EventDeleteCall<'a, S> {
        EventDeleteCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an event based on its Google Calendar ID. To retrieve an event using its iCalendar ID, call the events.list method using the iCalUID parameter.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `eventId` - Event identifier.
    pub fn get(&self, calendar_id: &str, event_id: &str) -> EventGetCall<'a, S> {
        EventGetCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _time_zone: Default::default(),
            _max_attendees: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports an event. This operation is used to add a private copy of an existing event to a calendar.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn import(&self, request: Event, calendar_id: &str) -> EventImportCall<'a, S> {
        EventImportCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _supports_attachments: Default::default(),
            _conference_data_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn insert(&self, request: Event, calendar_id: &str) -> EventInsertCall<'a, S> {
        EventInsertCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _supports_attachments: Default::default(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _max_attendees: Default::default(),
            _conference_data_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns instances of the specified recurring event.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `eventId` - Recurring event identifier.
    pub fn instances(&self, calendar_id: &str, event_id: &str) -> EventInstanceCall<'a, S> {
        EventInstanceCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _time_zone: Default::default(),
            _time_min: Default::default(),
            _time_max: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _original_start: Default::default(),
            _max_results: Default::default(),
            _max_attendees: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns events on the specified calendar.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn list(&self, calendar_id: &str) -> EventListCall<'a, S> {
        EventListCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _updated_min: Default::default(),
            _time_zone: Default::default(),
            _time_min: Default::default(),
            _time_max: Default::default(),
            _sync_token: Default::default(),
            _single_events: Default::default(),
            _show_hidden_invitations: Default::default(),
            _show_deleted: Default::default(),
            _shared_extended_property: Default::default(),
            _q: Default::default(),
            _private_extended_property: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _max_attendees: Default::default(),
            _i_cal_uid: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves an event to another calendar, i.e. changes an event's organizer.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier of the source calendar where the event currently is on.
    /// * `eventId` - Event identifier.
    /// * `destination` - Calendar identifier of the target calendar where the event is to be moved to.
    pub fn move_(&self, calendar_id: &str, event_id: &str, destination: &str) -> EventMoveCall<'a, S> {
        EventMoveCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _destination: destination.to_string(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an event. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `eventId` - Event identifier.
    pub fn patch(&self, request: Event, calendar_id: &str, event_id: &str) -> EventPatchCall<'a, S> {
        EventPatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _supports_attachments: Default::default(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _max_attendees: Default::default(),
            _conference_data_version: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an event based on a simple text string.
    /// 
    /// # Arguments
    ///
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `text` - The text describing the event to be created.
    pub fn quick_add(&self, calendar_id: &str, text: &str) -> EventQuickAddCall<'a, S> {
        EventQuickAddCall {
            hub: self.hub,
            _calendar_id: calendar_id.to_string(),
            _text: text.to_string(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    /// * `eventId` - Event identifier.
    pub fn update(&self, request: Event, calendar_id: &str, event_id: &str) -> EventUpdateCall<'a, S> {
        EventUpdateCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _event_id: event_id.to_string(),
            _supports_attachments: Default::default(),
            _send_updates: Default::default(),
            _send_notifications: Default::default(),
            _max_attendees: Default::default(),
            _conference_data_version: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes to Events resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `calendarId` - Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
    pub fn watch(&self, request: Channel, calendar_id: &str) -> EventWatchCall<'a, S> {
        EventWatchCall {
            hub: self.hub,
            _request: request,
            _calendar_id: calendar_id.to_string(),
            _updated_min: Default::default(),
            _time_zone: Default::default(),
            _time_min: Default::default(),
            _time_max: Default::default(),
            _sync_token: Default::default(),
            _single_events: Default::default(),
            _show_hidden_invitations: Default::default(),
            _show_deleted: Default::default(),
            _shared_extended_property: Default::default(),
            _q: Default::default(),
            _private_extended_property: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _max_attendees: Default::default(),
            _i_cal_uid: Default::default(),
            _always_include_email: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *freebusy* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`
/// // to build up your call.
/// let rb = hub.freebusy();
/// # }
/// ```
pub struct FreebusyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for FreebusyMethods<'a, S> {}

impl<'a, S> FreebusyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns free/busy information for a set of calendars.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn query(&self, request: FreeBusyRequest) -> FreebusyQueryCall<'a, S> {
        FreebusyQueryCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *setting* resources.
/// It is not used directly, but through the [`CalendarHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_calendar3 as calendar3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use calendar3::{CalendarHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.settings();
/// # }
/// ```
pub struct SettingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CalendarHub<S>,
}

impl<'a, S> client::MethodsBuilder for SettingMethods<'a, S> {}

impl<'a, S> SettingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a single user setting.
    /// 
    /// # Arguments
    ///
    /// * `setting` - The id of the user setting.
    pub fn get(&self, setting: &str) -> SettingGetCall<'a, S> {
        SettingGetCall {
            hub: self.hub,
            _setting: setting.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all user settings for the authenticated user.
    pub fn list(&self) -> SettingListCall<'a, S> {
        SettingListCall {
            hub: self.hub,
            _sync_token: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes to Settings resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn watch(&self, request: Channel) -> SettingWatchCall<'a, S> {
        SettingWatchCall {
            hub: self.hub,
            _request: request,
            _sync_token: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



