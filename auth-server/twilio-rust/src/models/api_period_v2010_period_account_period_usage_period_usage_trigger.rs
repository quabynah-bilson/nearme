/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.41.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that the trigger monitors.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to create the resource.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The HTTP method we use to call `callback_url`. Can be: `GET` or `POST`.
    #[serde(rename = "callback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub callback_method: Option<Option<CallbackMethod>>,
    /// The URL we call using the `callback_method` when the trigger fires.
    #[serde(rename = "callback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Option<String>>,
    /// The current value of the field the trigger is watching.
    #[serde(rename = "current_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the trigger was last fired specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_fired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_fired: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The string that you assigned to describe the trigger.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none")]
    pub recurring: Option<crate::models::UsageTriggerEnumRecurring>,
    /// The unique string that that we created to identify the UsageTrigger resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    #[serde(rename = "trigger_by", skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<crate::models::UsageTriggerEnumTriggerField>,
    /// The value at which the trigger will fire.  Must be a positive, numeric value.
    #[serde(rename = "trigger_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trigger_value: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    #[serde(rename = "usage_category", skip_serializing_if = "Option::is_none")]
    pub usage_category: Option<crate::models::UsageTriggerEnumUsageCategory>,
    /// The URI of the [UsageRecord](https://www.twilio.com/docs/usage/api/usage-record) resource this trigger watches, relative to `https://api.twilio.com`.
    #[serde(rename = "usage_record_uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub usage_record_uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger {
        ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger {
            account_sid: None,
            api_version: None,
            callback_method: None,
            callback_url: None,
            current_value: None,
            date_created: None,
            date_fired: None,
            date_updated: None,
            friendly_name: None,
            recurring: None,
            sid: None,
            trigger_by: None,
            trigger_value: None,
            uri: None,
            usage_category: None,
            usage_record_uri: None,
        }
    }
}

/// The HTTP method we use to call `callback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for CallbackMethod {
    fn default() -> CallbackMethod {
        Self::Head
    }
}

