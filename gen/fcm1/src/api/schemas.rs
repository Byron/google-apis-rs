use super::*;
/// Android specific options for messages sent through [FCM connection server](https://goo.gl/4GLdUl).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidConfig {
    /// An identifier of a group of messages that can be collapsed, so that only the last message gets sent when delivery can be resumed. A maximum of 4 different collapse keys is allowed at any given time.
    #[serde(rename="collapseKey")]
    
    pub collapse_key: Option<String>,
    /// Arbitrary key/value payload. If present, it will override google.firebase.fcm.v1.Message.data.
    
    pub data: Option<HashMap<String, String>>,
    /// If set to true, messages will be allowed to be delivered to the app while the device is in direct boot mode. See [Support Direct Boot mode](https://developer.android.com/training/articles/direct-boot).
    #[serde(rename="directBootOk")]
    
    pub direct_boot_ok: Option<bool>,
    /// Options for features provided by the FCM SDK for Android.
    #[serde(rename="fcmOptions")]
    
    pub fcm_options: Option<AndroidFcmOptions>,
    /// Notification to send to android devices.
    
    pub notification: Option<AndroidNotification>,
    /// Message priority. Can take "normal" and "high" values. For more information, see [Setting the priority of a message](https://goo.gl/GjONJv).
    
    pub priority: Option<String>,
    /// Package name of the application where the registration token must match in order to receive the message.
    #[serde(rename="restrictedPackageName")]
    
    pub restricted_package_name: Option<String>,
    /// How long (in seconds) the message should be kept in FCM storage if the device is offline. The maximum time to live supported is 4 weeks, and the default value is 4 weeks if not set. Set it to 0 if want to send the message immediately. In JSON format, the Duration type is encoded as a string rather than an object, where the string ends in the suffix "s" (indicating seconds) and is preceded by the number of seconds, with nanoseconds expressed as fractional seconds. For example, 3 seconds with 0 nanoseconds should be encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should be expressed in JSON format as "3.000000001s". The ttl will be rounded down to the nearest second.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::Part for AndroidConfig {}


/// Options for features provided by the FCM SDK for Android.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidFcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    
    pub analytics_label: Option<String>,
}

impl client::Part for AndroidFcmOptions {}


/// Notification to send to android devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidNotification {
    /// The notification's body text. If present, it will override google.firebase.fcm.v1.Notification.body.
    
    pub body: Option<String>,
    /// Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the body text to the user's current localization. See [Formatting and Styling](https://goo.gl/MalYE3) for more information.
    #[serde(rename="bodyLocArgs")]
    
    pub body_loc_args: Option<Vec<String>>,
    /// The key to the body string in the app's string resources to use to localize the body text to the user's current localization. See [String Resources](https://goo.gl/NdFZGI) for more information.
    #[serde(rename="bodyLocKey")]
    
    pub body_loc_key: Option<String>,
    /// If set, display notifications delivered to the device will be handled by the app instead of the proxy.
    #[serde(rename="bypassProxyNotification")]
    
    pub bypass_proxy_notification: Option<bool>,
    /// The [notification's channel id](https://developer.android.com/guide/topics/ui/notifiers/notifications#ManageChannels) (new in Android O). The app must create a channel with this channel ID before any notification with this channel ID is received. If you don't send this channel ID in the request, or if the channel ID provided has not yet been created by the app, FCM uses the channel ID specified in the app manifest.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The action associated with a user click on the notification. If specified, an activity with a matching intent filter is launched when a user clicks on the notification.
    #[serde(rename="clickAction")]
    
    pub click_action: Option<String>,
    /// The notification's icon color, expressed in #rrggbb format.
    
    pub color: Option<String>,
    /// If set to true, use the Android framework's default LED light settings for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml). If `default_light_settings` is set to true and `light_settings` is also set, the user-specified `light_settings` is used instead of the default value.
    #[serde(rename="defaultLightSettings")]
    
    pub default_light_settings: Option<bool>,
    /// If set to true, use the Android framework's default sound for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml).
    #[serde(rename="defaultSound")]
    
    pub default_sound: Option<bool>,
    /// If set to true, use the Android framework's default vibrate pattern for the notification. Default values are specified in [config.xml](https://android.googlesource.com/platform/frameworks/base/+/master/core/res/res/values/config.xml). If `default_vibrate_timings` is set to true and `vibrate_timings` is also set, the default value is used instead of the user-specified `vibrate_timings`.
    #[serde(rename="defaultVibrateTimings")]
    
    pub default_vibrate_timings: Option<bool>,
    /// Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time. A point in time is represented using [protobuf.Timestamp](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/Timestamp).
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The notification's icon. Sets the notification icon to myicon for drawable resource myicon. If you don't send this key in the request, FCM displays the launcher icon specified in your app manifest.
    
    pub icon: Option<String>,
    /// Contains the URL of an image that is going to be displayed in a notification. If present, it will override google.firebase.fcm.v1.Notification.image.
    
    pub image: Option<String>,
    /// Settings to control the notification's LED blinking rate and color if LED is available on the device. The total blinking time is controlled by the OS.
    #[serde(rename="lightSettings")]
    
    pub light_settings: Option<LightSettings>,
    /// Set whether or not this notification is relevant only to the current device. Some notifications can be bridged to other devices for remote display, such as a Wear OS watch. This hint can be set to recommend this notification not be bridged. See [Wear OS guides](https://developer.android.com/training/wearables/notifications/bridger#existing-method-of-preventing-bridging)
    #[serde(rename="localOnly")]
    
    pub local_only: Option<bool>,
    /// Sets the number of items this notification represents. May be displayed as a badge count for launchers that support badging.See [Notification Badge](https://developer.android.com/training/notify-user/badges). For example, this might be useful if you're using just one notification to represent multiple new messages but you want the count here to represent the number of total new messages. If zero or unspecified, systems that support badging use the default, which is to increment a number displayed on the long-press menu each time a new notification arrives.
    #[serde(rename="notificationCount")]
    
    pub notification_count: Option<i32>,
    /// Set the relative priority for this notification. Priority is an indication of how much of the user's attention should be consumed by this notification. Low-priority notifications may be hidden from the user in certain situations, while the user might be interrupted for a higher-priority notification. The effect of setting the same priorities may differ slightly on different platforms. Note this priority differs from `AndroidMessagePriority`. This priority is processed by the client after the message has been delivered, whereas [AndroidMessagePriority](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority) is an FCM concept that controls when the message is delivered.
    #[serde(rename="notificationPriority")]
    
    pub notification_priority: Option<String>,
    /// The sound to play when the device receives the notification. Supports "default" or the filename of a sound resource bundled in the app. Sound files must reside in /res/raw/.
    
    pub sound: Option<String>,
    /// When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel. When set to true, the notification persists even when the user clicks it.
    
    pub sticky: Option<bool>,
    /// Identifier used to replace existing notifications in the notification drawer. If not specified, each request creates a new notification. If specified and a notification with the same tag is already being shown, the new notification replaces the existing one in the notification drawer.
    
    pub tag: Option<String>,
    /// Sets the "ticker" text, which is sent to accessibility services. Prior to API level 21 (`Lollipop`), sets the text that is displayed in the status bar when the notification first arrives.
    
    pub ticker: Option<String>,
    /// The notification's title. If present, it will override google.firebase.fcm.v1.Notification.title.
    
    pub title: Option<String>,
    /// Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the title text to the user's current localization. See [Formatting and Styling](https://goo.gl/MalYE3) for more information.
    #[serde(rename="titleLocArgs")]
    
    pub title_loc_args: Option<Vec<String>>,
    /// The key to the title string in the app's string resources to use to localize the title text to the user's current localization. See [String Resources](https://goo.gl/NdFZGI) for more information.
    #[serde(rename="titleLocKey")]
    
    pub title_loc_key: Option<String>,
    /// Set the vibration pattern to use. Pass in an array of [protobuf.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration) to turn on or off the vibrator. The first value indicates the `Duration` to wait before turning the vibrator on. The next value indicates the `Duration` to keep the vibrator on. Subsequent values alternate between `Duration` to turn the vibrator off and to turn the vibrator on. If `vibrate_timings` is set and `default_vibrate_timings` is set to `true`, the default value is used instead of the user-specified `vibrate_timings`.
    #[serde(rename="vibrateTimings")]
    
    #[serde_as(as = "Option<Vec<::client::serde::duration::Wrapper>>")]
    pub vibrate_timings: Option<Vec<client::chrono::Duration>>,
    /// Set the [Notification.visibility](https://developer.android.com/reference/android/app/Notification.html#visibility) of the notification.
    
    pub visibility: Option<String>,
}

impl client::Part for AndroidNotification {}


/// [Apple Push Notification Service](https://goo.gl/MXRTPa) specific options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApnsConfig {
    /// Options for features provided by the FCM SDK for iOS.
    #[serde(rename="fcmOptions")]
    
    pub fcm_options: Option<ApnsFcmOptions>,
    /// HTTP request headers defined in Apple Push Notification Service. Refer to [APNs request headers](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/sending_notification_requests_to_apns) for supported headers such as `apns-expiration` and `apns-priority`. The backend sets a default value for `apns-expiration` of 30 days and a default value for `apns-priority` of 10 if not explicitly set.
    
    pub headers: Option<HashMap<String, String>>,
    /// APNs payload as a JSON object, including both `aps` dictionary and custom payload. See [Payload Key Reference](https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification). If present, it overrides google.firebase.fcm.v1.Notification.title and google.firebase.fcm.v1.Notification.body.
    
    pub payload: Option<HashMap<String, json::Value>>,
}

impl client::Part for ApnsConfig {}


/// Options for features provided by the FCM SDK for iOS.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApnsFcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    
    pub analytics_label: Option<String>,
    /// Contains the URL of an image that is going to be displayed in a notification. If present, it will override google.firebase.fcm.v1.Notification.image.
    
    pub image: Option<String>,
}

impl client::Part for ApnsFcmOptions {}


/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0).
    
    pub alpha: Option<f32>,
    /// The amount of blue in the color as a value in the interval [0, 1].
    
    pub blue: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    
    pub red: Option<f32>,
}

impl client::Part for Color {}


/// Platform independent options for features provided by the FCM SDKs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    
    pub analytics_label: Option<String>,
}

impl client::Part for FcmOptions {}


/// Settings to control notification LED.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LightSettings {
    /// Required. Set `color` of the LED with [google.type.Color](https://github.com/googleapis/googleapis/blob/master/google/type/color.proto).
    
    pub color: Option<Color>,
    /// Required. Along with `light_on_duration `, define the blink rate of LED flashes. Resolution defined by [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)
    #[serde(rename="lightOffDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub light_off_duration: Option<client::chrono::Duration>,
    /// Required. Along with `light_off_duration`, define the blink rate of LED flashes. Resolution defined by [proto.Duration](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)
    #[serde(rename="lightOnDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub light_on_duration: Option<client::chrono::Duration>,
}

impl client::Part for LightSettings {}


/// Message to send by Firebase Cloud Messaging Service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages send projects](ProjectMessageSendCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Input only. Android specific options for messages sent through [FCM connection server](https://goo.gl/4GLdUl).
    
    pub android: Option<AndroidConfig>,
    /// Input only. [Apple Push Notification Service](https://goo.gl/MXRTPa) specific options.
    
    pub apns: Option<ApnsConfig>,
    /// Condition to send a message to, e.g. "'foo' in topics && 'bar' in topics".
    
    pub condition: Option<String>,
    /// Input only. Arbitrary key/value payload, which must be UTF-8 encoded. The key should not be a reserved word (“from”, “message_type”, or any word starting with “google” or “gcm”). When sending payloads containing only data fields to iOS devices, only normal priority (`"apns-priority": "5"`) is allowed in [`ApnsConfig`](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsconfig).
    
    pub data: Option<HashMap<String, String>>,
    /// Input only. Template for FCM SDK feature options to use across all platforms.
    #[serde(rename="fcmOptions")]
    
    pub fcm_options: Option<FcmOptions>,
    /// Output Only. The identifier of the message sent, in the format of `projects/*/messages/{message_id}`.
    
    pub name: Option<String>,
    /// Input only. Basic notification template to use across all platforms.
    
    pub notification: Option<Notification>,
    /// Registration token to send a message to.
    
    pub token: Option<String>,
    /// Topic name to send a message to, e.g. "weather". Note: "/topics/" prefix should not be provided.
    
    pub topic: Option<String>,
    /// Input only. [Webpush protocol](https://tools.ietf.org/html/rfc8030) options.
    
    pub webpush: Option<WebpushConfig>,
}

impl client::ResponseResult for Message {}


/// Basic notification template to use across all platforms.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// The notification's body text.
    
    pub body: Option<String>,
    /// Contains the URL of an image that is going to be downloaded on the device and displayed in a notification. JPEG, PNG, BMP have full support across platforms. Animated GIF and video only work on iOS. WebP and HEIF have varying levels of support across platforms and platform versions. Android has 1MB image size limit. Quota usage and implications/costs for hosting image on Firebase Storage: https://firebase.google.com/pricing
    
    pub image: Option<String>,
    /// The notification's title.
    
    pub title: Option<String>,
}

impl client::Part for Notification {}


/// Request to send a message to specified target.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages send projects](ProjectMessageSendCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// Required. Message to send.
    
    pub message: Option<Message>,
    /// Flag for testing the request without actually delivering the message.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for SendMessageRequest {}


/// [Webpush protocol](https://tools.ietf.org/html/rfc8030) options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpushConfig {
    /// Arbitrary key/value payload. If present, it will override google.firebase.fcm.v1.Message.data.
    
    pub data: Option<HashMap<String, String>>,
    /// Options for features provided by the FCM SDK for Web.
    #[serde(rename="fcmOptions")]
    
    pub fcm_options: Option<WebpushFcmOptions>,
    /// HTTP headers defined in webpush protocol. Refer to [Webpush protocol](https://tools.ietf.org/html/rfc8030#section-5) for supported headers, e.g. "TTL": "15".
    
    pub headers: Option<HashMap<String, String>>,
    /// Web Notification options as a JSON object. Supports Notification instance properties as defined in [Web Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification). If present, "title" and "body" fields override [google.firebase.fcm.v1.Notification.title] and [google.firebase.fcm.v1.Notification.body].
    
    pub notification: Option<HashMap<String, json::Value>>,
}

impl client::Part for WebpushConfig {}


/// Options for features provided by the FCM SDK for Web.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpushFcmOptions {
    /// Label associated with the message's analytics data.
    #[serde(rename="analyticsLabel")]
    
    pub analytics_label: Option<String>,
    /// The link to open when the user clicks on the notification. For all URL values, HTTPS is required.
    
    pub link: Option<String>,
}

impl client::Part for WebpushFcmOptions {}


