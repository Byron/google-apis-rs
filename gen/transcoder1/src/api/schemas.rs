use super::*;
/// Ad break.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdBreak {
    /// Start time in seconds for the ad break, relative to the output file timeline. The default is `0s`.
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
}

impl client::Part for AdBreak {}


/// Animation types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    /// End previous animation.
    #[serde(rename="animationEnd")]
    
    pub animation_end: Option<AnimationEnd>,
    /// Display overlay object with fade animation.
    #[serde(rename="animationFade")]
    
    pub animation_fade: Option<AnimationFade>,
    /// Display static overlay object.
    #[serde(rename="animationStatic")]
    
    pub animation_static: Option<AnimationStatic>,
}

impl client::Part for Animation {}


/// End previous overlay animation from the video. Without AnimationEnd, the overlay object will keep the state of previous animation until the end of the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnimationEnd {
    /// The time to end overlay object, in seconds. Default: 0
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
}

impl client::Part for AnimationEnd {}


/// Display overlay object with fade animation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnimationFade {
    /// The time to end the fade animation, in seconds. Default: `start_time_offset` + 1s
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time_offset: Option<client::chrono::Duration>,
    /// Required. Type of fade animation: `FADE_IN` or `FADE_OUT`.
    #[serde(rename="fadeType")]
    
    pub fade_type: Option<AnimationFadeFadeTypeEnum>,
    /// The time to start the fade animation, in seconds. Default: 0
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
    /// Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video.
    
    pub xy: Option<NormalizedCoordinate>,
}

impl client::Part for AnimationFade {}


/// Display static overlay object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnimationStatic {
    /// The time to start displaying the overlay object, in seconds. Default: 0
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
    /// Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video.
    
    pub xy: Option<NormalizedCoordinate>,
}

impl client::Part for AnimationStatic {}


/// Audio preprocessing configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Audio {
    /// Enable boosting high frequency components. The default is `false`. **Note:** This field is not supported.
    #[serde(rename="highBoost")]
    
    pub high_boost: Option<bool>,
    /// Enable boosting low frequency components. The default is `false`. **Note:** This field is not supported.
    #[serde(rename="lowBoost")]
    
    pub low_boost: Option<bool>,
    /// Specify audio loudness normalization in loudness units relative to full scale (LUFS). Enter a value between -24 and 0 (the default), where: * -24 is the Advanced Television Systems Committee (ATSC A/85) standard * -23 is the EU R128 broadcast standard * -19 is the prior standard for online mono audio * -18 is the ReplayGain standard * -16 is the prior standard for stereo audio * -14 is the new online audio standard recommended by Spotify, as well as Amazon Echo * 0 disables normalization
    
    pub lufs: Option<f64>,
}

impl client::Part for Audio {}


/// The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudioMapping {
    /// Required. The `EditAtom.key` that references the atom with audio inputs in the `Job.edit_list`.
    #[serde(rename="atomKey")]
    
    pub atom_key: Option<String>,
    /// Audio volume control in dB. Negative values decrease volume, positive values increase. The default is 0.
    #[serde(rename="gainDb")]
    
    pub gain_db: Option<f64>,
    /// Required. The zero-based index of the channel in the input audio stream.
    #[serde(rename="inputChannel")]
    
    pub input_channel: Option<i32>,
    /// Required. The `Input.key` that identifies the input file.
    #[serde(rename="inputKey")]
    
    pub input_key: Option<String>,
    /// Required. The zero-based index of the track in the input file.
    #[serde(rename="inputTrack")]
    
    pub input_track: Option<i32>,
    /// Required. The zero-based index of the channel in the output audio stream.
    #[serde(rename="outputChannel")]
    
    pub output_channel: Option<i32>,
}

impl client::Part for AudioMapping {}


/// Audio stream resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AudioStream {
    /// Required. Audio bitrate in bits per second. Must be between 1 and 10,000,000.
    #[serde(rename="bitrateBps")]
    
    pub bitrate_bps: Option<i32>,
    /// Number of audio channels. Must be between 1 and 6. The default is 2.
    #[serde(rename="channelCount")]
    
    pub channel_count: Option<i32>,
    /// A list of channel names specifying layout of the audio channels. This only affects the metadata embedded in the container headers, if supported by the specified format. The default is `["fl", "fr"]`. Supported channel names: - `fl` - Front left channel - `fr` - Front right channel - `sl` - Side left channel - `sr` - Side right channel - `fc` - Front center channel - `lfe` - Low frequency
    #[serde(rename="channelLayout")]
    
    pub channel_layout: Option<Vec<String>>,
    /// The codec for this audio stream. The default is `aac`. Supported audio codecs: - `aac` - `aac-he` - `aac-he-v2` - `mp3` - `ac3` - `eac3`
    
    pub codec: Option<String>,
    /// The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`.
    
    pub mapping: Option<Vec<AudioMapping>>,
    /// The audio sample rate in Hertz. The default is 48000 Hertz.
    #[serde(rename="sampleRateHertz")]
    
    pub sample_rate_hertz: Option<i32>,
}

impl client::Part for AudioStream {}


/// Bob Weaver Deinterlacing Filter Configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BwdifConfig {
    /// Deinterlace all frames rather than just the frames identified as interlaced. The default is `false`.
    #[serde(rename="deinterlaceAllFrames")]
    
    pub deinterlace_all_frames: Option<bool>,
    /// Specifies the deinterlacing mode to adopt. The default is `send_frame`. Supported values: - `send_frame`: Output one frame for each frame - `send_field`: Output one frame for each field
    
    pub mode: Option<String>,
    /// The picture field parity assumed for the input interlaced video. The default is `auto`. Supported values: - `tff`: Assume the top field is first - `bff`: Assume the bottom field is first - `auto`: Enable automatic detection of field parity
    
    pub parity: Option<String>,
}

impl client::Part for BwdifConfig {}


/// Color preprocessing configuration. **Note:** This configuration is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// Control brightness of the video. Enter a value between -1 and 1, where -1 is minimum brightness and 1 is maximum brightness. 0 is no change. The default is 0.
    
    pub brightness: Option<f64>,
    /// Control black and white contrast of the video. Enter a value between -1 and 1, where -1 is minimum contrast and 1 is maximum contrast. 0 is no change. The default is 0.
    
    pub contrast: Option<f64>,
    /// Control color saturation of the video. Enter a value between -1 and 1, where -1 is fully desaturated and 1 is maximum saturation. 0 is no change. The default is 0.
    
    pub saturation: Option<f64>,
}

impl client::Part for Color {}


/// Video cropping configuration for the input video. The cropped input video is scaled to match the output resolution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Crop {
    /// The number of pixels to crop from the bottom. The default is 0.
    #[serde(rename="bottomPixels")]
    
    pub bottom_pixels: Option<i32>,
    /// The number of pixels to crop from the left. The default is 0.
    #[serde(rename="leftPixels")]
    
    pub left_pixels: Option<i32>,
    /// The number of pixels to crop from the right. The default is 0.
    #[serde(rename="rightPixels")]
    
    pub right_pixels: Option<i32>,
    /// The number of pixels to crop from the top. The default is 0.
    #[serde(rename="topPixels")]
    
    pub top_pixels: Option<i32>,
}

impl client::Part for Crop {}


/// Deblock preprocessing configuration. **Note:** This configuration is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deblock {
    /// Enable deblocker. The default is `false`.
    
    pub enabled: Option<bool>,
    /// Set strength of the deblocker. Enter a value between 0 and 1. The higher the value, the stronger the block removal. 0 is no deblocking. The default is 0.
    
    pub strength: Option<f64>,
}

impl client::Part for Deblock {}


/// Deinterlace configuration for input video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deinterlace {
    /// Specifies the Bob Weaver Deinterlacing Filter Configuration.
    
    pub bwdif: Option<BwdifConfig>,
    /// Specifies the Yet Another Deinterlacing Filter Configuration.
    
    pub yadif: Option<YadifConfig>,
}

impl client::Part for Deinterlace {}


/// Denoise preprocessing configuration. **Note:** This configuration is not supported.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Denoise {
    /// Set strength of the denoise. Enter a value between 0 and 1. The higher the value, the smoother the image. 0 is no denoising. The default is 0.
    
    pub strength: Option<f64>,
    /// Set the denoiser mode. The default is `standard`. Supported denoiser modes: - `standard` - `grain`
    
    pub tune: Option<String>,
}

impl client::Part for Denoise {}


/// Edit atom.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditAtom {
    /// End time in seconds for the atom, relative to the input file timeline. When `end_time_offset` is not specified, the `inputs` are used until the end of the atom.
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time_offset: Option<client::chrono::Duration>,
    /// List of `Input.key`s identifying files that should be used in this atom. The listed `inputs` must have the same timeline.
    
    pub inputs: Option<Vec<String>>,
    /// A unique key for this atom. Must be specified when using advanced mapping.
    
    pub key: Option<String>,
    /// Start time in seconds for the atom, relative to the input file timeline. The default is `0s`.
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
}

impl client::Part for EditAtom {}


/// Encoding of an input file such as an audio, video, or text track. Elementary streams must be packaged before mapping and sharing between different output formats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ElementaryStream {
    /// Encoding of an audio stream.
    #[serde(rename="audioStream")]
    
    pub audio_stream: Option<AudioStream>,
    /// A unique key for this elementary stream.
    
    pub key: Option<String>,
    /// Encoding of a text stream. For example, closed captions or subtitles.
    #[serde(rename="textStream")]
    
    pub text_stream: Option<TextStream>,
    /// Encoding of a video stream.
    #[serde(rename="videoStream")]
    
    pub video_stream: Option<VideoStream>,
}

impl client::Part for ElementaryStream {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job templates delete projects](ProjectLocationJobTemplateDeleteCall) (response)
/// * [locations jobs delete projects](ProjectLocationJobDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// H264 codec settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct H264CodecSettings {
    /// Specifies whether an open Group of Pictures (GOP) structure should be allowed or not. The default is `false`.
    #[serde(rename="allowOpenGop")]
    
    pub allow_open_gop: Option<bool>,
    /// Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A higher value equals a lower bitrate but smoother image. The default is 0.
    #[serde(rename="aqStrength")]
    
    pub aq_strength: Option<f64>,
    /// The number of consecutive B-frames. Must be greater than or equal to zero. Must be less than `VideoStream.gop_frame_count` if set. The default is 0.
    #[serde(rename="bFrameCount")]
    
    pub b_frame_count: Option<i32>,
    /// Allow B-pyramid for reference frame selection. This may not be supported on all decoders. The default is `false`.
    #[serde(rename="bPyramid")]
    
    pub b_pyramid: Option<bool>,
    /// Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 800,000,000.
    #[serde(rename="bitrateBps")]
    
    pub bitrate_bps: Option<i32>,
    /// Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21.
    #[serde(rename="crfLevel")]
    
    pub crf_level: Option<i32>,
    /// Use two-pass encoding strategy to achieve better video quality. `VideoStream.rate_control_mode` must be `vbr`. The default is `false`.
    #[serde(rename="enableTwoPass")]
    
    pub enable_two_pass: Option<bool>,
    /// The entropy coder to use. The default is `cabac`. Supported entropy coders: - `cavlc` - `cabac`
    #[serde(rename="entropyCoder")]
    
    pub entropy_coder: Option<String>,
    /// Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<f64>,
    /// Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`.
    #[serde(rename="gopDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub gop_duration: Option<client::chrono::Duration>,
    /// Select the GOP size based on the specified frame count. Must be greater than zero.
    #[serde(rename="gopFrameCount")]
    
    pub gop_frame_count: Option<i32>,
    /// The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the height, in pixels, per the horizontal ASR. The API calculates the width per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format
    #[serde(rename="pixelFormat")]
    
    pub pixel_format: Option<String>,
    /// Enforces the specified codec preset. The default is `veryfast`. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Preset). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message.
    
    pub preset: Option<String>,
    /// Enforces the specified codec profile. The following profiles are supported: * `baseline` * `main` * `high` (default) The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Tune). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message.
    
    pub profile: Option<String>,
    /// Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate - `crf` - constant rate factor
    #[serde(rename="rateControlMode")]
    
    pub rate_control_mode: Option<String>,
    /// Enforces the specified codec tune. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Tune). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message.
    
    pub tune: Option<String>,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to 90% of `VideoStream.vbv_size_bits`.
    #[serde(rename="vbvFullnessBits")]
    
    pub vbv_fullness_bits: Option<i32>,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to `VideoStream.bitrate_bps`.
    #[serde(rename="vbvSizeBits")]
    
    pub vbv_size_bits: Option<i32>,
    /// The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the width, in pixels, per the horizontal ASR. The API calculates the height per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for H264CodecSettings {}


/// H265 codec settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct H265CodecSettings {
    /// Specifies whether an open Group of Pictures (GOP) structure should be allowed or not. The default is `false`.
    #[serde(rename="allowOpenGop")]
    
    pub allow_open_gop: Option<bool>,
    /// Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A higher value equals a lower bitrate but smoother image. The default is 0.
    #[serde(rename="aqStrength")]
    
    pub aq_strength: Option<f64>,
    /// The number of consecutive B-frames. Must be greater than or equal to zero. Must be less than `VideoStream.gop_frame_count` if set. The default is 0.
    #[serde(rename="bFrameCount")]
    
    pub b_frame_count: Option<i32>,
    /// Allow B-pyramid for reference frame selection. This may not be supported on all decoders. The default is `false`.
    #[serde(rename="bPyramid")]
    
    pub b_pyramid: Option<bool>,
    /// Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 800,000,000.
    #[serde(rename="bitrateBps")]
    
    pub bitrate_bps: Option<i32>,
    /// Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21.
    #[serde(rename="crfLevel")]
    
    pub crf_level: Option<i32>,
    /// Use two-pass encoding strategy to achieve better video quality. `VideoStream.rate_control_mode` must be `vbr`. The default is `false`.
    #[serde(rename="enableTwoPass")]
    
    pub enable_two_pass: Option<bool>,
    /// Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<f64>,
    /// Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`.
    #[serde(rename="gopDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub gop_duration: Option<client::chrono::Duration>,
    /// Select the GOP size based on the specified frame count. Must be greater than zero.
    #[serde(rename="gopFrameCount")]
    
    pub gop_frame_count: Option<i32>,
    /// The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the height, in pixels, per the horizontal ASR. The API calculates the width per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format
    #[serde(rename="pixelFormat")]
    
    pub pixel_format: Option<String>,
    /// Enforces the specified codec preset. The default is `veryfast`. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.265). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message.
    
    pub preset: Option<String>,
    /// Enforces the specified codec profile. The following profiles are supported: * 8-bit profiles * `main` (default) * `main-intra` * `mainstillpicture` * 10-bit profiles * `main10` (default) * `main10-intra` * `main422-10` * `main422-10-intra` * `main444-10` * `main444-10-intra` * 12-bit profiles * `main12` (default) * `main12-intra` * `main422-12` * `main422-12-intra` * `main444-12` * `main444-12-intra` The available options are [FFmpeg-compatible](https://x265.readthedocs.io/). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message.
    
    pub profile: Option<String>,
    /// Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate - `crf` - constant rate factor
    #[serde(rename="rateControlMode")]
    
    pub rate_control_mode: Option<String>,
    /// Enforces the specified codec tune. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.265). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message.
    
    pub tune: Option<String>,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to 90% of `VideoStream.vbv_size_bits`.
    #[serde(rename="vbvFullnessBits")]
    
    pub vbv_fullness_bits: Option<i32>,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to `VideoStream.bitrate_bps`.
    #[serde(rename="vbvSizeBits")]
    
    pub vbv_size_bits: Option<i32>,
    /// The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the width, in pixels, per the horizontal ASR. The API calculates the height per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for H265CodecSettings {}


/// Overlaid jpeg image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Target image opacity. Valid values are from `1.0` (solid, default) to `0.0` (transparent), exclusive. Set this to a value greater than `0.0`.
    
    pub alpha: Option<f64>,
    /// Normalized image resolution, based on output video resolution. Valid values: `0.0`–`1.0`. To respect the original image aspect ratio, set either `x` or `y` to `0.0`. To use the original image resolution, set both `x` and `y` to `0.0`.
    
    pub resolution: Option<NormalizedCoordinate>,
    /// Required. URI of the JPEG image in Cloud Storage. For example, `gs://bucket/inputs/image.jpeg`. JPEG is the only supported image type.
    
    pub uri: Option<String>,
}

impl client::Part for Image {}


/// Input asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    /// A unique key for this input. Must be specified when using advanced mapping and edit lists.
    
    pub key: Option<String>,
    /// Preprocessing configurations.
    #[serde(rename="preprocessingConfig")]
    
    pub preprocessing_config: Option<PreprocessingConfig>,
    /// URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). If empty, the value is populated from `Job.input_uri`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats).
    
    pub uri: Option<String>,
}

impl client::Part for Input {}


/// Transcoding job resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations jobs create projects](ProjectLocationJobCreateCall) (request|response)
/// * [locations jobs get projects](ProjectLocationJobGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// The configuration for this job.
    
    pub config: Option<JobConfig>,
    /// Output only. The time the job was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time the transcoding finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. An error object that describes the reason for the failure. This property is always present when `state` is `FAILED`.
    
    pub error: Option<Status>,
    /// Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats).
    #[serde(rename="inputUri")]
    
    pub input_uri: Option<String>,
    /// The labels associated with this job. You can use these to organize and group your jobs.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource name of the job. Format: `projects/{project_number}/locations/{location}/jobs/{job}`
    
    pub name: Option<String>,
    /// Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats).
    #[serde(rename="outputUri")]
    
    pub output_uri: Option<String>,
    /// Output only. The time the transcoding started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the job.
    
    pub state: Option<JobStateEnum>,
    /// Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`. Preset Transcoder templates: - `preset/{preset_id}` - User defined JobTemplate: `{job_template_id}`
    #[serde(rename="templateId")]
    
    pub template_id: Option<String>,
    /// Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30.
    #[serde(rename="ttlAfterCompletionDays")]
    
    pub ttl_after_completion_days: Option<i32>,
}

impl client::RequestValue for Job {}
impl client::ResponseResult for Job {}


/// Job configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfig {
    /// List of ad breaks. Specifies where to insert ad break tags in the output manifests.
    #[serde(rename="adBreaks")]
    
    pub ad_breaks: Option<Vec<AdBreak>>,
    /// List of `Edit atom`s. Defines the ultimate timeline of the resulting file or manifest.
    #[serde(rename="editList")]
    
    pub edit_list: Option<Vec<EditAtom>>,
    /// List of elementary streams.
    #[serde(rename="elementaryStreams")]
    
    pub elementary_streams: Option<Vec<ElementaryStream>>,
    /// List of input assets stored in Cloud Storage.
    
    pub inputs: Option<Vec<Input>>,
    /// List of output manifests.
    
    pub manifests: Option<Vec<Manifest>>,
    /// List of multiplexing settings for output streams.
    #[serde(rename="muxStreams")]
    
    pub mux_streams: Option<Vec<MuxStream>>,
    /// Output configuration.
    
    pub output: Option<Output>,
    /// List of overlays on the output video, in descending Z-order.
    
    pub overlays: Option<Vec<Overlay>>,
    /// Destination on Pub/Sub.
    #[serde(rename="pubsubDestination")]
    
    pub pubsub_destination: Option<PubsubDestination>,
    /// List of output sprite sheets. Spritesheets require at least one VideoStream in the Jobconfig.
    #[serde(rename="spriteSheets")]
    
    pub sprite_sheets: Option<Vec<SpriteSheet>>,
}

impl client::Part for JobConfig {}


/// Transcoding job template resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job templates create projects](ProjectLocationJobTemplateCreateCall) (request|response)
/// * [locations job templates get projects](ProjectLocationJobTemplateGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobTemplate {
    /// The configuration for this template.
    
    pub config: Option<JobConfig>,
    /// The labels associated with this job template. You can use these to organize and group your job templates.
    
    pub labels: Option<HashMap<String, String>>,
    /// The resource name of the job template. Format: `projects/{project_number}/locations/{location}/jobTemplates/{job_template}`
    
    pub name: Option<String>,
}

impl client::RequestValue for JobTemplate {}
impl client::ResponseResult for JobTemplate {}


/// Response message for `TranscoderService.ListJobTemplates`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations job templates list projects](ProjectLocationJobTemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobTemplatesResponse {
    /// List of job templates in the specified region.
    #[serde(rename="jobTemplates")]
    
    pub job_templates: Option<Vec<JobTemplate>>,
    /// The pagination token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of regions that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListJobTemplatesResponse {}


/// Response message for `TranscoderService.ListJobs`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations jobs list projects](ProjectLocationJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// List of jobs in the specified region.
    
    pub jobs: Option<Vec<Job>>,
    /// The pagination token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of regions that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListJobsResponse {}


/// Manifest configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// The name of the generated file. The default is `manifest` with the extension suffix corresponding to the `Manifest.type`.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// Required. List of user given `MuxStream.key`s that should appear in this manifest. When `Manifest.type` is `HLS`, a media manifest with name `MuxStream.key` and `.m3u8` extension is generated for each element of the `Manifest.mux_streams`.
    #[serde(rename="muxStreams")]
    
    pub mux_streams: Option<Vec<String>>,
    /// Required. Type of the manifest, can be `HLS` or `DASH`.
    #[serde(rename="type")]
    
    pub type_: Option<ManifestTypeEnum>,
}

impl client::Part for Manifest {}


/// Multiplexing settings for output stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MuxStream {
    /// The container format. The default is `mp4` Supported container formats: - `ts` - `fmp4`- the corresponding file extension is `.m4s` - `mp4` - `vtt` See also: [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)
    
    pub container: Option<String>,
    /// List of `ElementaryStream.key`s multiplexed in this stream.
    #[serde(rename="elementaryStreams")]
    
    pub elementary_streams: Option<Vec<String>>,
    /// The name of the generated file. The default is `MuxStream.key` with the extension suffix corresponding to the `MuxStream.container`. Individual segments also have an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `mux_stream0000000123.ts`.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// A unique key for this multiplexed stream. HLS media manifests will be named `MuxStream.key` with the `.m3u8` extension suffix.
    
    pub key: Option<String>,
    /// Segment settings for `ts`, `fmp4` and `vtt`.
    #[serde(rename="segmentSettings")]
    
    pub segment_settings: Option<SegmentSettings>,
}

impl client::Part for MuxStream {}


/// 2D normalized coordinates. Default: `{0.0, 0.0}`
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NormalizedCoordinate {
    /// Normalized x coordinate.
    
    pub x: Option<f64>,
    /// Normalized y coordinate.
    
    pub y: Option<f64>,
}

impl client::Part for NormalizedCoordinate {}


/// Location of output file(s) in a Cloud Storage bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    /// URI for the output file(s). For example, `gs://my-bucket/outputs/`. If empty, the value is populated from `Job.output_uri`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats).
    
    pub uri: Option<String>,
}

impl client::Part for Output {}


/// Overlay configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Overlay {
    /// List of Animations. The list should be chronological, without any time overlap.
    
    pub animations: Option<Vec<Animation>>,
    /// Image overlay.
    
    pub image: Option<Image>,
}

impl client::Part for Overlay {}


/// Pad filter configuration for the input video. The padded input video is scaled after padding with black to match the output resolution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pad {
    /// The number of pixels to add to the bottom. The default is 0.
    #[serde(rename="bottomPixels")]
    
    pub bottom_pixels: Option<i32>,
    /// The number of pixels to add to the left. The default is 0.
    #[serde(rename="leftPixels")]
    
    pub left_pixels: Option<i32>,
    /// The number of pixels to add to the right. The default is 0.
    #[serde(rename="rightPixels")]
    
    pub right_pixels: Option<i32>,
    /// The number of pixels to add to the top. The default is 0.
    #[serde(rename="topPixels")]
    
    pub top_pixels: Option<i32>,
}

impl client::Part for Pad {}


/// Preprocessing configurations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    /// Audio preprocessing configuration.
    
    pub audio: Option<Audio>,
    /// Color preprocessing configuration.
    
    pub color: Option<Color>,
    /// Specify the video cropping configuration.
    
    pub crop: Option<Crop>,
    /// Deblock preprocessing configuration.
    
    pub deblock: Option<Deblock>,
    /// Specify the video deinterlace configuration.
    
    pub deinterlace: Option<Deinterlace>,
    /// Denoise preprocessing configuration.
    
    pub denoise: Option<Denoise>,
    /// Specify the video pad filter configuration.
    
    pub pad: Option<Pad>,
}

impl client::Part for PreprocessingConfig {}


/// A Pub/Sub destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish job completion notification to. For example: `projects/{project}/topics/{topic}`.
    
    pub topic: Option<String>,
}

impl client::Part for PubsubDestination {}


/// Segment settings for `ts`, `fmp4` and `vtt`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SegmentSettings {
    /// Required. Create an individual segment file. The default is `false`.
    #[serde(rename="individualSegments")]
    
    pub individual_segments: Option<bool>,
    /// Duration of the segments in seconds. The default is `6.0s`. Note that `segmentDuration` must be greater than or equal to [`gopDuration`](#videostream), and `segmentDuration` must be divisible by [`gopDuration`](#videostream).
    #[serde(rename="segmentDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub segment_duration: Option<client::chrono::Duration>,
}

impl client::Part for SegmentSettings {}


/// Sprite sheet configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpriteSheet {
    /// The maximum number of sprites per row in a sprite sheet. The default is 0, which indicates no maximum limit.
    #[serde(rename="columnCount")]
    
    pub column_count: Option<i32>,
    /// End time in seconds, relative to the output file timeline. When `end_time_offset` is not specified, the sprites are generated until the end of the output file.
    #[serde(rename="endTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub end_time_offset: Option<client::chrono::Duration>,
    /// Required. File name prefix for the generated sprite sheets. Each sprite sheet has an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `sprite_sheet0000000123.jpeg`.
    #[serde(rename="filePrefix")]
    
    pub file_prefix: Option<String>,
    /// Format type. The default is `jpeg`. Supported formats: - `jpeg`
    
    pub format: Option<String>,
    /// Starting from `0s`, create sprites at regular intervals. Specify the interval value in seconds.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub interval: Option<client::chrono::Duration>,
    /// The quality of the generated sprite sheet. Enter a value between 1 and 100, where 1 is the lowest quality and 100 is the highest quality. The default is 100. A high quality value corresponds to a low image data compression ratio.
    
    pub quality: Option<i32>,
    /// The maximum number of rows per sprite sheet. When the sprite sheet is full, a new sprite sheet is created. The default is 0, which indicates no maximum limit.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// Required. The height of sprite in pixels. Must be an even integer. To preserve the source aspect ratio, set the SpriteSheet.sprite_height_pixels field or the SpriteSheet.sprite_width_pixels field, but not both (the API will automatically calculate the missing field). For portrait videos that contain horizontal ASR and rotation metadata, provide the height, in pixels, per the horizontal ASR. The API calculates the width per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="spriteHeightPixels")]
    
    pub sprite_height_pixels: Option<i32>,
    /// Required. The width of sprite in pixels. Must be an even integer. To preserve the source aspect ratio, set the SpriteSheet.sprite_width_pixels field or the SpriteSheet.sprite_height_pixels field, but not both (the API will automatically calculate the missing field). For portrait videos that contain horizontal ASR and rotation metadata, provide the width, in pixels, per the horizontal ASR. The API calculates the height per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="spriteWidthPixels")]
    
    pub sprite_width_pixels: Option<i32>,
    /// Start time in seconds, relative to the output file timeline. Determines the first sprite to pick. The default is `0s`.
    #[serde(rename="startTimeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub start_time_offset: Option<client::chrono::Duration>,
    /// Total number of sprites. Create the specified number of sprites distributed evenly across the timeline of the output media. The default is 100.
    #[serde(rename="totalCount")]
    
    pub total_count: Option<i32>,
}

impl client::Part for SpriteSheet {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextMapping {
    /// Required. The `EditAtom.key` that references atom with text inputs in the `Job.edit_list`.
    #[serde(rename="atomKey")]
    
    pub atom_key: Option<String>,
    /// Required. The `Input.key` that identifies the input file.
    #[serde(rename="inputKey")]
    
    pub input_key: Option<String>,
    /// Required. The zero-based index of the track in the input file.
    #[serde(rename="inputTrack")]
    
    pub input_track: Option<i32>,
}

impl client::Part for TextMapping {}


/// Encoding of a text stream. For example, closed captions or subtitles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextStream {
    /// The codec for this text stream. The default is `webvtt`. Supported text codecs: - `srt` - `ttml` - `cea608` - `cea708` - `webvtt`
    
    pub codec: Option<String>,
    /// The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`.
    
    pub mapping: Option<Vec<TextMapping>>,
}

impl client::Part for TextStream {}


/// Video stream resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoStream {
    /// H264 codec settings.
    
    pub h264: Option<H264CodecSettings>,
    /// H265 codec settings.
    
    pub h265: Option<H265CodecSettings>,
    /// VP9 codec settings.
    
    pub vp9: Option<Vp9CodecSettings>,
}

impl client::Part for VideoStream {}


/// VP9 codec settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vp9CodecSettings {
    /// Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 480,000,000.
    #[serde(rename="bitrateBps")]
    
    pub bitrate_bps: Option<i32>,
    /// Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21. **Note:** This field is not supported.
    #[serde(rename="crfLevel")]
    
    pub crf_level: Option<i32>,
    /// Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<f64>,
    /// Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`.
    #[serde(rename="gopDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub gop_duration: Option<client::chrono::Duration>,
    /// Select the GOP size based on the specified frame count. Must be greater than zero.
    #[serde(rename="gopFrameCount")]
    
    pub gop_frame_count: Option<i32>,
    /// The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the height, in pixels, per the horizontal ASR. The API calculates the width per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format
    #[serde(rename="pixelFormat")]
    
    pub pixel_format: Option<String>,
    /// Enforces the specified codec profile. The following profiles are supported: * `profile0` (default) * `profile1` * `profile2` * `profile3` The available options are [WebM-compatible](https://www.webmproject.org/vp9/profiles/). Note that certain values for this field may cause the transcoder to override other fields you set in the `Vp9CodecSettings` message.
    
    pub profile: Option<String>,
    /// Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate
    #[serde(rename="rateControlMode")]
    
    pub rate_control_mode: Option<String>,
    /// The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used. For portrait videos that contain horizontal ASR and rotation metadata, provide the width, in pixels, per the horizontal ASR. The API calculates the height per the horizontal ASR. The API detects any rotation metadata and swaps the requested height and width for the output.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for Vp9CodecSettings {}


/// Yet Another Deinterlacing Filter Configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YadifConfig {
    /// Deinterlace all frames rather than just the frames identified as interlaced. The default is `false`.
    #[serde(rename="deinterlaceAllFrames")]
    
    pub deinterlace_all_frames: Option<bool>,
    /// Disable spacial interlacing. The default is `false`.
    #[serde(rename="disableSpatialInterlacing")]
    
    pub disable_spatial_interlacing: Option<bool>,
    /// Specifies the deinterlacing mode to adopt. The default is `send_frame`. Supported values: - `send_frame`: Output one frame for each frame - `send_field`: Output one frame for each field
    
    pub mode: Option<String>,
    /// The picture field parity assumed for the input interlaced video. The default is `auto`. Supported values: - `tff`: Assume the top field is first - `bff`: Assume the bottom field is first - `auto`: Enable automatic detection of field parity
    
    pub parity: Option<String>,
}

impl client::Part for YadifConfig {}


