use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersResponse {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    pub value: Vec<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub business_phones: Vec<Value>,
    pub display_name: String,
    pub given_name: Option<String>,
    pub job_title: Value,
    pub mail: Value,
    pub mobile_phone: Value,
    pub office_location: Value,
    pub preferred_language: Value,
    pub surname: Option<String>,
    pub user_principal_name: String,
    pub id: String,
}
