use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Use Google Fit to see and store your physical activity data
    ActivityRead,

    /// Add to your Google Fit physical activity data
    ActivityWrite,

    /// See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app.
    BloodGlucoseRead,

    /// Add info about your blood glucose to Google Fit. I consent to Google using my blood glucose information with this app.
    BloodGlucoseWrite,

    /// See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app.
    BloodPressureRead,

    /// Add info about your blood pressure in Google Fit. I consent to Google using my blood pressure information with this app.
    BloodPressureWrite,

    /// See info about your body measurements in Google Fit
    BodyRead,

    /// Add info about your body measurements to Google Fit
    BodyWrite,

    /// See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app.
    BodyTemperatureRead,

    /// Add to info about your body temperature in Google Fit. I consent to Google using my body temperature information with this app.
    BodyTemperatureWrite,

    /// See your heart rate data in Google Fit. I consent to Google sharing my heart rate information with this app.
    HeartRateRead,

    /// Add to your heart rate data in Google Fit. I consent to Google using my heart rate information with this app.
    HeartRateWrite,

    /// See your Google Fit speed and distance data
    LocationRead,

    /// Add to your Google Fit location data
    LocationWrite,

    /// See info about your nutrition in Google Fit
    NutritionRead,

    /// Add to info about your nutrition in Google Fit
    NutritionWrite,

    /// See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app.
    OxygenSaturationRead,

    /// Add info about your oxygen saturation in Google Fit. I consent to Google using my oxygen saturation information with this app.
    OxygenSaturationWrite,

    /// See info about your reproductive health in Google Fit. I consent to Google sharing my reproductive health information with this app.
    ReproductiveHealthRead,

    /// Add info about your reproductive health in Google Fit. I consent to Google using my reproductive health information with this app.
    ReproductiveHealthWrite,

    /// See your sleep data in Google Fit. I consent to Google sharing my sleep information with this app.
    SleepRead,

    /// Add to your sleep data in Google Fit. I consent to Google using my sleep information with this app.
    SleepWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ActivityRead => "https://www.googleapis.com/auth/fitness.activity.read",
            Scope::ActivityWrite => "https://www.googleapis.com/auth/fitness.activity.write",
            Scope::BloodGlucoseRead => "https://www.googleapis.com/auth/fitness.blood_glucose.read",
            Scope::BloodGlucoseWrite => "https://www.googleapis.com/auth/fitness.blood_glucose.write",
            Scope::BloodPressureRead => "https://www.googleapis.com/auth/fitness.blood_pressure.read",
            Scope::BloodPressureWrite => "https://www.googleapis.com/auth/fitness.blood_pressure.write",
            Scope::BodyRead => "https://www.googleapis.com/auth/fitness.body.read",
            Scope::BodyWrite => "https://www.googleapis.com/auth/fitness.body.write",
            Scope::BodyTemperatureRead => "https://www.googleapis.com/auth/fitness.body_temperature.read",
            Scope::BodyTemperatureWrite => "https://www.googleapis.com/auth/fitness.body_temperature.write",
            Scope::HeartRateRead => "https://www.googleapis.com/auth/fitness.heart_rate.read",
            Scope::HeartRateWrite => "https://www.googleapis.com/auth/fitness.heart_rate.write",
            Scope::LocationRead => "https://www.googleapis.com/auth/fitness.location.read",
            Scope::LocationWrite => "https://www.googleapis.com/auth/fitness.location.write",
            Scope::NutritionRead => "https://www.googleapis.com/auth/fitness.nutrition.read",
            Scope::NutritionWrite => "https://www.googleapis.com/auth/fitness.nutrition.write",
            Scope::OxygenSaturationRead => "https://www.googleapis.com/auth/fitness.oxygen_saturation.read",
            Scope::OxygenSaturationWrite => "https://www.googleapis.com/auth/fitness.oxygen_saturation.write",
            Scope::ReproductiveHealthRead => "https://www.googleapis.com/auth/fitness.reproductive_health.read",
            Scope::ReproductiveHealthWrite => "https://www.googleapis.com/auth/fitness.reproductive_health.write",
            Scope::SleepRead => "https://www.googleapis.com/auth/fitness.sleep.read",
            Scope::SleepWrite => "https://www.googleapis.com/auth/fitness.sleep.write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::BodyRead
    }
}

