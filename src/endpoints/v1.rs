use cfmt::formatcp;

pub const BASE: &str = "/api/v1";

pub const SYSTEM_STATUS: &str = formatcp!("{BASE}/system/status");
pub const SYSTEM_WAIT: &str = formatcp!("{BASE}/system/wait");

pub const RESOURCES_TEMP_UPLOAD: &str = formatcp!("{BASE}/resources/temp_upload");
pub const RESOURCES: &str = formatcp!("{BASE}/resources");
pub const SKILLS: &str = formatcp!("{BASE}/skills");

pub const PACK_EXPORT: &str = formatcp!("{BASE}/pack/export");
pub const PACK_IMPORT: &str = formatcp!("{BASE}/pack/import");

pub const FS_LS: &str = formatcp!("{BASE}/fs/ls");
pub const FS_TREE: &str = formatcp!("{BASE}/fs/tree");
pub const FS_STAT: &str = formatcp!("{BASE}/fs/stat");
pub const FS_MKDIR: &str = formatcp!("{BASE}/fs/mkdir");
pub const FS_DELETE: &str = formatcp!("{BASE}/fs");
pub const FS_MV: &str = formatcp!("{BASE}/fs/mv");

pub const CONTENT_READ: &str = formatcp!("{BASE}/content/read");
pub const CONTENT_ABSTRACT: &str = formatcp!("{BASE}/content/abstract");
pub const CONTENT_OVERVIEW: &str = formatcp!("{BASE}/content/overview");
pub const CONTENT_DOWNLOAD: &str = formatcp!("{BASE}/content/download");
pub const CONTENT_WRITE: &str = formatcp!("{BASE}/content/write");
pub const CONTENT_REINDEX: &str = formatcp!("{BASE}/content/reindex");

pub const SEARCH_FIND: &str = formatcp!("{BASE}/search/find");
pub const SEARCH_SEARCH: &str = formatcp!("{BASE}/search/search");
pub const SEARCH_GREP: &str = formatcp!("{BASE}/search/grep");
pub const SEARCH_GLOB: &str = formatcp!("{BASE}/search/glob");

pub const RELATIONS: &str = formatcp!("{BASE}/relations");
pub const RELATIONS_LINK: &str = formatcp!("{BASE}/relations/link");

pub const SESSIONS: &str = formatcp!("{BASE}/sessions");
pub const SESSION_BY_ID: &str = formatcp!("{BASE}/sessions/{{session_id}}");
pub const SESSION_CONTEXT: &str = formatcp!("{BASE}/sessions/{{session_id}}/context");
pub const SESSION_ARCHIVE: &str =
    formatcp!("{BASE}/sessions/{{session_id}}/archives/{{archive_id}}");
pub const SESSION_COMMIT: &str = formatcp!("{BASE}/sessions/{{session_id}}/commit");
pub const SESSION_EXTRACT: &str = formatcp!("{BASE}/sessions/{{session_id}}/extract");
pub const SESSION_MESSAGES: &str = formatcp!("{BASE}/sessions/{{session_id}}/messages");
pub const SESSION_USED: &str = formatcp!("{BASE}/sessions/{{session_id}}/used");

pub const TASKS: &str = formatcp!("{BASE}/tasks");
pub const TASK_BY_ID: &str = formatcp!("{BASE}/tasks/{{task_id}}");

pub const PRIVACY_CONFIGS: &str = formatcp!("{BASE}/privacy-configs");
pub const PRIVACY_CONFIGS_CATEGORY: &str = formatcp!("{BASE}/privacy-configs/{{category}}");
pub const PRIVACY_CONFIGS_TARGET: &str =
    formatcp!("{BASE}/privacy-configs/{{category}}/{{target_key}}");
pub const PRIVACY_CONFIGS_VERSIONS: &str =
    formatcp!("{BASE}/privacy-configs/{{category}}/{{target_key}}/versions");
pub const PRIVACY_CONFIGS_VERSION: &str =
    formatcp!("{BASE}/privacy-configs/{{category}}/{{target_key}}/versions/{{version}}");
pub const PRIVACY_CONFIGS_ACTIVATE: &str =
    formatcp!("{BASE}/privacy-configs/{{category}}/{{target_key}}/activate");

pub const OBSERVER_QUEUE: &str = formatcp!("{BASE}/observer/queue");
pub const OBSERVER_VIKINGDB: &str = formatcp!("{BASE}/observer/vikingdb");
pub const OBSERVER_MODELS: &str = formatcp!("{BASE}/observer/models");
pub const OBSERVER_LOCK: &str = formatcp!("{BASE}/observer/lock");
pub const OBSERVER_RETRIEVAL: &str = formatcp!("{BASE}/observer/retrieval");
pub const OBSERVER_SYSTEM: &str = formatcp!("{BASE}/observer/system");

pub const DEBUG_HEALTH: &str = formatcp!("{BASE}/debug/health");
pub const DEBUG_VECTOR_SCROLL: &str = formatcp!("{BASE}/debug/vector/scroll");
pub const DEBUG_VECTOR_COUNT: &str = formatcp!("{BASE}/debug/vector/count");

pub const MAINTENANCE_REINDEX: &str = formatcp!("{BASE}/maintenance/reindex");

pub const STATS_MEMORIES: &str = formatcp!("{BASE}/stats/memories");
pub const STATS_SESSION: &str = formatcp!("{BASE}/stats/sessions/{{session_id}}");

pub const ADMIN_ACCOUNTS: &str = formatcp!("{BASE}/admin/accounts");
pub const ADMIN_ACCOUNT_BY_ID: &str = formatcp!("{BASE}/admin/accounts/{{account_id}}");
pub const ADMIN_ACCOUNT_USERS: &str = formatcp!("{BASE}/admin/accounts/{{account_id}}/users");
pub const ADMIN_ACCOUNT_AGENTS: &str = formatcp!("{BASE}/admin/accounts/{{account_id}}/agents");
pub const ADMIN_ACCOUNT_USER: &str =
    formatcp!("{BASE}/admin/accounts/{{account_id}}/users/{{user_id}}");
pub const ADMIN_ACCOUNT_USER_ROLE: &str =
    formatcp!("{BASE}/admin/accounts/{{account_id}}/users/{{user_id}}/role");
pub const ADMIN_ACCOUNT_USER_KEY: &str =
    formatcp!("{BASE}/admin/accounts/{{account_id}}/users/{{user_id}}/key");

pub const HEALTH: &str = "/health";
pub const READY: &str = "/ready";

pub const CHAT: &str = "/chat";
pub const CHAT_STREAM: &str = "/chat/stream";

pub const WEBDAV_RESOURCES: &str = "/webdav/resources";
pub const WEBDAV_RESOURCES_PATH: &str = "/webdav/resources/{path}";

pub const METRICS: &str = "/metrics";
