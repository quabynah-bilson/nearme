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
pub enum CallEnumEvent {
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "completed")]
    Completed,

}

impl ToString for CallEnumEvent {
    fn to_string(&self) -> String {
        match self {
            Self::Initiated => String::from("initiated"),
            Self::Ringing => String::from("ringing"),
            Self::Answered => String::from("answered"),
            Self::Completed => String::from("completed"),
        }
    }
}

impl Default for CallEnumEvent {
    fn default() -> CallEnumEvent {
        Self::Initiated
    }
}




