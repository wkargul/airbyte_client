/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  The Configuration API is an internal Airbyte API that is designed for communications between different Airbyte components. * Its main purpose is to enable the Airbyte Engineering team to configure the internal state of [Airbyte Cloud](https://airbyte.com/airbyte-cloud) * It is also sometimes used by OSS users to configure their own Self-Hosted Airbyte deployment (internal state, etc)  WARNING * Airbyte does NOT have active commitments to support this API long-term. * OSS users can utilize the Configuration API, but at their own risk. * This API is utilized internally by the Airbyte Engineering team and may be modified in the future if the need arises. * Modifications by the Airbyte Engineering team could create breaking changes and OSS users would need to update their code to catch up to any backwards incompatible changes in the API.  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/api/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/api/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttemptRead {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "status")]
    pub status: crate::models::AttemptStatus,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "endedAt", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<i64>,
    #[serde(rename = "bytesSynced", skip_serializing_if = "Option::is_none")]
    pub bytes_synced: Option<i64>,
    #[serde(rename = "recordsSynced", skip_serializing_if = "Option::is_none")]
    pub records_synced: Option<i64>,
    #[serde(rename = "totalStats", skip_serializing_if = "Option::is_none")]
    pub total_stats: Option<Box<crate::models::AttemptStats>>,
    #[serde(rename = "streamStats", skip_serializing_if = "Option::is_none")]
    pub stream_stats: Option<Vec<crate::models::AttemptStreamStats>>,
    #[serde(rename = "failureSummary", skip_serializing_if = "Option::is_none")]
    pub failure_summary: Option<Box<crate::models::AttemptFailureSummary>>,
}

impl AttemptRead {
    pub fn new(
        id: i64,
        status: crate::models::AttemptStatus,
        created_at: i64,
        updated_at: i64,
    ) -> AttemptRead {
        AttemptRead {
            id,
            status,
            created_at,
            updated_at,
            ended_at: None,
            bytes_synced: None,
            records_synced: None,
            total_stats: None,
            stream_stats: None,
            failure_summary: None,
        }
    }
}
