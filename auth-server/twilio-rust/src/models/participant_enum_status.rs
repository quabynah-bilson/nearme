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
pub enum ParticipantEnumStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for ParticipantEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Queued => String::from("queued"),
            Self::Connecting => String::from("connecting"),
            Self::Ringing => String::from("ringing"),
            Self::Connected => String::from("connected"),
            Self::Complete => String::from("complete"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for ParticipantEnumStatus {
    fn default() -> ParticipantEnumStatus {
        Self::Queued
    }
}




