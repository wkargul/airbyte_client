#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SourceCheckConnectionRequestBody {
    #[serde(rename = "sourceDefinitionId")]
    pub source_definition_id: uuid::Uuid,
    /// The values required to configure the source. The schema for this must match the schema return by source_definition_specifications/get for the source.
    #[serde(
    rename = "connectionConfiguration",
    deserialize_with = "Option::deserialize"
    )]
    pub connection_configuration: Option<serde_json::Value>,
    #[serde(rename = "workspaceId")]
    pub workspace_id: uuid::Uuid,

}

impl SourceCheckConnectionRequestBody {
    pub fn new(
        source_definition_id: uuid::Uuid,
        connection_configuration: Option<serde_json::Value>,
        workspace_id: uuid::Uuid,
    ) -> SourceCheckConnectionRequestBody {
        SourceCheckConnectionRequestBody {
            source_definition_id,
            connection_configuration,
            workspace_id,
        }
    }
}
