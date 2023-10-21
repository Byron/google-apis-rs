use super::*;
/// Metadata describing a family of fonts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list webfonts](WebfontListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Webfont {
    /// The category of the font.
    
    pub category: Option<String>,
    /// The name of the font.
    
    pub family: Option<String>,
    /// The font files (with all supported scripts) for each one of the available variants, as a key : value map.
    
    pub files: Option<HashMap<String, String>>,
    /// This kind represents a webfont object in the webfonts service.
    
    pub kind: Option<String>,
    /// The date (format "yyyy-MM-dd") the font was modified for the last time.
    #[serde(rename="lastModified")]
    
    pub last_modified: Option<String>,
    /// The scripts supported by the font.
    
    pub subsets: Option<Vec<String>>,
    /// The available variants for the font.
    
    pub variants: Option<Vec<String>>,
    /// The font version.
    
    pub version: Option<String>,
}

impl client::Resource for Webfont {}


/// Response containing the list of fonts currently served by the Google Fonts API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list webfonts](WebfontListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebfontList {
    /// The list of fonts currently served by the Google Fonts API.
    
    pub items: Option<Vec<Webfont>>,
    /// This kind represents a list of webfont objects in the webfonts service.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for WebfontList {}


