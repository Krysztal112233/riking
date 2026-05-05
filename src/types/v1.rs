use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub result: T,
    pub time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub account_id: String,
    pub user_id: String,
    pub agent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub pending: i64,
    pub processing: i64,
    pub completed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoriesExtracted {
    pub profile: i64,
    pub preferences: i64,
    pub entities: i64,
    pub events: i64,
    pub cases: i64,
    pub patterns: i64,
    pub tools: i64,
    pub skills: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsEntry {
    pub name: String,
    pub size: i64,
    pub mode: u32,
    #[serde(rename = "modTime")]
    pub mod_time: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedContext {
    pub context_type: String,
    pub uri: String,
    pub level: i64,
    pub score: f64,
    pub category: String,
    pub match_reason: String,
    pub relations: Vec<serde_json::Value>,
    #[serde(rename = "abstract")]
    pub abstract_content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepMatch {
    pub uri: String,
    pub line: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationEntry {
    pub uri: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverStatus {
    pub name: String,
    pub is_healthy: bool,
    pub has_errors: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub task_id: String,
    pub task_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub created_at: f64,
    pub updated_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemWaitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitQueueStatus {
    pub processed: i64,
    pub requeue_count: i64,
    pub error_count: i64,
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemWaitResponse {
    #[serde(rename = "Embedding")]
    pub embedding: WaitQueueStatus,

    #[serde(rename = "Semantic")]
    pub semantic: WaitQueueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub healthy: bool,
    pub version: String,
    pub auth_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyCheck {
    pub agfs: String,
    pub vectordb: String,
    pub api_key_manager: String,
    pub ollama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyResponse {
    pub status: String,
    pub checks: ReadyCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    pub initialized: bool,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesTempUploadResponse {
    pub temp_file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesAddRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_dirs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directly_upload_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_structure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_interval: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesAddResponse {
    pub status: String,
    pub root_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    pub meta: serde_json::Value,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_status: Option<QueueStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsAddRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsAddResponse {
    pub status: String,
    pub root_uri: String,
    pub uri: String,
    pub name: String,
    pub auxiliary_files: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_status: Option<QueueStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackExportRequest {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackImportRequest {
    pub temp_file_id: String,
    pub parent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectorize: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackImportResponse {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMkdirRequest {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMkdirResponse {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsDeleteResponse {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMvRequest {
    pub from_uri: String,
    pub to_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMvResponse {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentWriteRequest {
    pub uri: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticStatus {
    pub processed: i64,
    pub error_count: i64,
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentWriteQueueStatus {
    #[serde(rename = "Semantic")]
    pub semantic: SemanticStatus,
    #[serde(rename = "Embedding")]
    pub embedding: SemanticStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentWriteResponse {
    pub uri: String,
    pub root_uri: String,
    pub context_type: String,
    pub mode: String,
    pub written_bytes: i64,
    pub content_updated: bool,
    pub semantic_status: String,
    pub vector_status: String,
    pub queue_status: ContentWriteQueueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFindRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_provenance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFindResponse {
    pub memories: Vec<MatchedContext>,
    pub resources: Vec<MatchedContext>,
    pub skills: Vec<MatchedContext>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_provenance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSearchResponse {
    pub memories: Vec<MatchedContext>,
    pub resources: Vec<MatchedContext>,
    pub skills: Vec<MatchedContext>,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_plan: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGrepRequest {
    pub uri: String,
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGrepResponse {
    pub matches: Vec<GrepMatch>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGlobRequest {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGlobResponse {
    pub matches: Vec<String>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationsLinkRequest {
    pub from_uri: String,
    pub to_uris: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationsLinkResponse {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationsUnlinkRequest {
    pub from_uri: String,
    pub to_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationsUnlinkResponse {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionsCreateResponse {
    pub session_id: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub uri: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetailResponse {
    pub session_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: i64,
    pub total_message_count: i64,
    pub commit_count: i64,
    pub memories_extracted: MemoriesExtracted,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_at: Option<String>,
    pub llm_token_usage: TokenUsage,
    pub user: User,
    pub pending_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    #[serde(rename = "totalArchives")]
    pub total_archives: i64,
    #[serde(rename = "includedArchives")]
    pub included_archives: i64,
    #[serde(rename = "droppedArchives")]
    pub dropped_archives: i64,
    #[serde(rename = "failedArchives")]
    pub failed_archives: i64,
    #[serde(rename = "activeTokens")]
    pub active_tokens: i64,
    #[serde(rename = "archiveTokens")]
    pub archive_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContextResponse {
    pub latest_archive_overview: String,
    pub pre_archive_abstracts: Vec<serde_json::Value>,
    pub messages: Vec<serde_json::Value>,
    #[serde(rename = "estimatedTokens")]
    pub estimated_tokens: i64,
    pub stats: ContextStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionArchiveResponse {
    pub archive_id: String,
    #[serde(rename = "abstract")]
    pub abstract_content: String,
    pub overview: String,
    pub messages: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCommitResponse {
    pub session_id: String,
    pub status: String,
    pub task_id: String,
    pub archive_uri: String,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessagesAddRequest {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessagesAddResponse {
    pub session_id: String,
    pub message_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUsedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUsedResponse {
    pub session_id: String,
    pub contexts_used: i64,
    pub skills_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDeleteResponse {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDetailResponse {
    pub task_id: String,
    pub task_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksListResponse(pub Vec<TaskInfo>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigsUpsertRequest {
    pub values: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigsUpsertResponse {
    pub version: i64,
    pub category: String,
    pub target_key: String,
    pub values: HashMap<String, serde_json::Value>,
    pub change_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigMeta {
    pub category: String,
    pub target_key: String,
    pub active_version: i64,
    pub latest_version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigSnapshot {
    pub version: i64,
    pub category: String,
    pub target_key: String,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigDetailResponse {
    pub meta: PrivacyConfigMeta,
    pub current: PrivacyConfigSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfigsActivateRequest {
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountsCreateRequest {
    pub account_id: String,
    pub admin_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolate_user_scope_by_agent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolate_agent_scope_by_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountsCreateResponse {
    pub account_id: String,
    pub admin_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountInfo {
    pub account_id: String,
    pub created_at: String,
    pub user_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountDeleteResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUsersCreateRequest {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUsersCreateResponse {
    pub account_id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserInfo {
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAgentInfo {
    pub agent_id: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUserDeleteResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUserRoleUpdateRequest {
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUserRoleUpdateResponse {
    pub account_id: String,
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccountUserKeyRegenerateResponse {
    pub user_key: String,
}
