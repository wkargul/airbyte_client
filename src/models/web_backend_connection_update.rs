/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  The Configuration API is an internal Airbyte API that is designed for communications between different Airbyte components. * Its main purpose is to enable the Airbyte Engineering team to configure the internal state of [Airbyte Cloud](https://airbyte.com/airbyte-cloud) * It is also sometimes used by OSS users to configure their own Self-Hosted Airbyte deployment (internal state, etc)  WARNING * Airbyte does NOT have active commitments to support this API long-term. * OSS users can utilize the Configuration API, but at their own risk. * This API is utilized internally by the Airbyte Engineering team and may be modified in the future if the need arises. * Modifications by the Airbyte Engineering team could create breaking changes and OSS users would need to update their code to catch up to any backwards incompatible changes in the API.  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/api/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/api/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Authentication (OSS): * When authenticating to the Configuration API, you must use Basic Authentication by setting the Authentication Header to Basic and base64 encoding the username and password (which are `airbyte` and `password` by default - so base64 encoding `airbyte:password` results in `YWlyYnl0ZTpwYXNzd29yZA==`). So the full header reads `'Authorization': \"Basic YWlyYnl0ZTpwYXNzd29yZA==\"`
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// WebBackendConnectionUpdate : Used to apply a patch-style update to a connection, which means that null properties remain unchanged

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebBackendConnectionUpdate {
    /// Name that will be set to the connection
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "connectionId")]
    pub connection_id: uuid::Uuid,
    #[serde(
        rename = "namespaceDefinition",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_definition: Option<crate::models::NamespaceDefinitionType>,
    /// Used when namespaceDefinition is 'customformat'. If blank then behaves like namespaceDefinition = 'destination'. If \"${SOURCE_NAMESPACE}\" then behaves like namespaceDefinition = 'source'.
    #[serde(rename = "namespaceFormat", skip_serializing_if = "Option::is_none")]
    pub namespace_format: Option<String>,
    /// Prefix that will be prepended to the name of each stream when it is written to the destination.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "syncCatalog", skip_serializing_if = "Option::is_none")]
    pub sync_catalog: Option<Box<crate::models::AirbyteCatalog>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::ConnectionSchedule>>,
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<crate::models::ConnectionScheduleType>,
    #[serde(rename = "scheduleData", skip_serializing_if = "Option::is_none")]
    pub schedule_data: Option<Box<crate::models::ConnectionScheduleData>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ConnectionStatus>,
    #[serde(
        rename = "resourceRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_requirements: Option<Box<crate::models::ResourceRequirements>>,
    #[serde(rename = "skipReset", skip_serializing_if = "Option::is_none")]
    pub skip_reset: Option<bool>,
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<crate::models::WebBackendOperationCreateOrUpdate>>,
    #[serde(rename = "sourceCatalogId", skip_serializing_if = "Option::is_none")]
    pub source_catalog_id: Option<uuid::Uuid>,
    #[serde(rename = "geography", skip_serializing_if = "Option::is_none")]
    pub geography: Option<crate::models::Geography>,
    #[serde(
        rename = "notifySchemaChanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_schema_changes: Option<bool>,
    #[serde(
        rename = "notifySchemaChangesByEmail",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify_schema_changes_by_email: Option<bool>,
    #[serde(
        rename = "nonBreakingChangesPreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_breaking_changes_preference: Option<crate::models::NonBreakingChangesPreference>,
}

impl WebBackendConnectionUpdate {
    /// Used to apply a patch-style update to a connection, which means that null properties remain unchanged
    pub fn new(connection_id: uuid::Uuid) -> WebBackendConnectionUpdate {
        WebBackendConnectionUpdate {
            name: None,
            connection_id,
            namespace_definition: None,
            namespace_format: None,
            prefix: None,
            sync_catalog: None,
            schedule: None,
            schedule_type: None,
            schedule_data: None,
            status: None,
            resource_requirements: None,
            skip_reset: None,
            operations: None,
            source_catalog_id: None,
            geography: None,
            notify_schema_changes: None,
            notify_schema_changes_by_email: None,
            non_breaking_changes_preference: None,
        }
    }
}
