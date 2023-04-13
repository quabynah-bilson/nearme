/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.41.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConferenceRecordingEnumStatus {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "absent")]
    Absent,

}

impl ToString for ConferenceRecordingEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::InProgress => String::from("in-progress"),
            Self::Paused => String::from("paused"),
            Self::Stopped => String::from("stopped"),
            Self::Processing => String::from("processing"),
            Self::Completed => String::from("completed"),
            Self::Absent => String::from("absent"),
        }
    }
}

impl Default for ConferenceRecordingEnumStatus {
    fn default() -> ConferenceRecordingEnumStatus {
        Self::InProgress
    }
}




