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
pub enum CallRecordingEnumSource {
    #[serde(rename = "DialVerb")]
    DialVerb,
    #[serde(rename = "Conference")]
    Conference,
    #[serde(rename = "OutboundAPI")]
    OutboundApi,
    #[serde(rename = "Trunking")]
    Trunking,
    #[serde(rename = "RecordVerb")]
    RecordVerb,
    #[serde(rename = "StartCallRecordingAPI")]
    StartCallRecordingApi,
    #[serde(rename = "StartConferenceRecordingAPI")]
    StartConferenceRecordingApi,

}

impl ToString for CallRecordingEnumSource {
    fn to_string(&self) -> String {
        match self {
            Self::DialVerb => String::from("DialVerb"),
            Self::Conference => String::from("Conference"),
            Self::OutboundApi => String::from("OutboundAPI"),
            Self::Trunking => String::from("Trunking"),
            Self::RecordVerb => String::from("RecordVerb"),
            Self::StartCallRecordingApi => String::from("StartCallRecordingAPI"),
            Self::StartConferenceRecordingApi => String::from("StartConferenceRecordingAPI"),
        }
    }
}

impl Default for CallRecordingEnumSource {
    fn default() -> CallRecordingEnumSource {
        Self::DialVerb
    }
}




