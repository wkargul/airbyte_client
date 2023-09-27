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
pub struct StreamStatusUpdateRequestBody {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "attemptNumber")]
    pub attempt_number: i32,
    #[serde(rename = "connectionId")]
    pub connection_id: uuid::Uuid,
    #[serde(rename = "jobId")]
    pub job_id: i64,
    #[serde(rename = "incompleteRunCause", skip_serializing_if = "Option::is_none")]
    pub incomplete_run_cause: Option<crate::models::StreamStatusIncompleteRunCause>,
    #[serde(rename = "jobType")]
    pub job_type: crate::models::StreamStatusJobType,
    #[serde(rename = "runState")]
    pub run_state: crate::models::StreamStatusRunState,
    #[serde(rename = "streamName")]
    pub stream_name: String,
    #[serde(rename = "streamNamespace", skip_serializing_if = "Option::is_none")]
    pub stream_namespace: Option<String>,
    #[serde(rename = "transitionedAt")]
    pub transitioned_at: i64,
    #[serde(rename = "workspaceId")]
    pub workspace_id: uuid::Uuid,
}

impl StreamStatusUpdateRequestBody {
    pub fn new(
        id: uuid::Uuid,
        attempt_number: i32,
        connection_id: uuid::Uuid,
        job_id: i64,
        job_type: crate::models::StreamStatusJobType,
        run_state: crate::models::StreamStatusRunState,
        stream_name: String,
        transitioned_at: i64,
        workspace_id: uuid::Uuid,
    ) -> StreamStatusUpdateRequestBody {
        StreamStatusUpdateRequestBody {
            id,
            attempt_number,
            connection_id,
            job_id,
            incomplete_run_cause: None,
            job_type,
            run_state,
            stream_name,
            stream_namespace: None,
            transitioned_at,
            workspace_id,
        }
    }
}
