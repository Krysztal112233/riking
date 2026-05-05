use cfmt::{concatcp, formatcp};

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
pub const SESSION_BY_ID: &str = concatcp!(BASE, "/sessions/{}");
pub const SESSION_CONTEXT: &str = concatcp!(BASE, "/sessions/{}/context");
pub const SESSION_ARCHIVE: &str = concatcp!(BASE, "/sessions/{}/archives/{}");
pub const SESSION_COMMIT: &str = concatcp!(BASE, "/sessions/{}/commit");
pub const SESSION_EXTRACT: &str = concatcp!(BASE, "/sessions/{}/extract");
pub const SESSION_MESSAGES: &str = concatcp!(BASE, "/sessions/{}/messages");
pub const SESSION_USED: &str = concatcp!(BASE, "/sessions/{}/used");

pub const TASKS: &str = formatcp!("{BASE}/tasks");
pub const TASK_BY_ID: &str = concatcp!(BASE, "/tasks/{}");

pub const PRIVACY_CONFIGS: &str = formatcp!("{BASE}/privacy-configs");
pub const PRIVACY_CONFIGS_CATEGORY: &str = concatcp!(BASE, "/privacy-configs/{}");
pub const PRIVACY_CONFIGS_TARGET: &str = concatcp!(BASE, "/privacy-configs/{}/{}");
pub const PRIVACY_CONFIGS_VERSIONS: &str = concatcp!(BASE, "/privacy-configs/{}/{}/versions");
pub const PRIVACY_CONFIGS_VERSION: &str = concatcp!(BASE, "/privacy-configs/{}/{}/versions/{}");
pub const PRIVACY_CONFIGS_ACTIVATE: &str = concatcp!(BASE, "/privacy-configs/{}/{}/activate");

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
pub const STATS_SESSION: &str = concatcp!(BASE, "/stats/sessions/{}");

pub const ADMIN_ACCOUNTS: &str = formatcp!("{BASE}/admin/accounts");
pub const ADMIN_ACCOUNT_BY_ID: &str = concatcp!(BASE, "/admin/accounts/{}");
pub const ADMIN_ACCOUNT_USERS: &str = concatcp!(BASE, "/admin/accounts/{}/users");
pub const ADMIN_ACCOUNT_AGENTS: &str = concatcp!(BASE, "/admin/accounts/{}/agents");
pub const ADMIN_ACCOUNT_USER: &str = concatcp!(BASE, "/admin/accounts/{}/users/{}");
pub const ADMIN_ACCOUNT_USER_ROLE: &str = concatcp!(BASE, "/admin/accounts/{}/users/{}/role");
pub const ADMIN_ACCOUNT_USER_KEY: &str = concatcp!(BASE, "/admin/accounts/{}/users/{}/key");

pub const HEALTH: &str = "/health";
pub const READY: &str = "/ready";

pub const CHAT: &str = "/chat";
pub const CHAT_STREAM: &str = "/chat/stream";

pub const WEBDAV_RESOURCES: &str = "/webdav/resources";
pub const WEBDAV_RESOURCES_PATH: &str = "/webdav/resources/{}";

pub const METRICS: &str = "/metrics";

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_path {
        ($t:expr, $($a:expr),+ $(,)?) => {{
            let mut _s = $t.to_owned();
            $(
                _s = _s.replacen("{}", &$a.to_string(), 1);
            )+
            _s
        }};
    }

    #[test]
    fn task_by_id() {
        let path = make_path!(TASK_BY_ID, "uuid-abc");
        assert_eq!(path, "/api/v1/tasks/uuid-abc");
    }

    #[test]
    fn session_by_id() {
        let path = make_path!(SESSION_BY_ID, "sess-123");
        assert_eq!(path, "/api/v1/sessions/sess-123");
    }

    #[test]
    fn session_archive_two_args() {
        let path = make_path!(SESSION_ARCHIVE, "sess-123", "archive_002");
        assert_eq!(path, "/api/v1/sessions/sess-123/archives/archive_002");
    }

    #[test]
    fn privacy_configs_version_three_args() {
        let path = make_path!(PRIVACY_CONFIGS_VERSION, "skill", "my-key", "3");
        assert_eq!(path, "/api/v1/privacy-configs/skill/my-key/versions/3");
    }

    #[test]
    fn admin_account_user_key() {
        let path = make_path!(ADMIN_ACCOUNT_USER_KEY, "acme", "bob");
        assert_eq!(path, "/api/v1/admin/accounts/acme/users/bob/key");
    }

    #[test]
    fn templates_contain_no_named_placeholders() {
        assert!(!SESSION_BY_ID.contains("{session_id}"));
        assert!(!TASK_BY_ID.contains("{task_id}"));
        assert!(!ADMIN_ACCOUNT_USER_KEY.contains("{account_id}"));
        assert!(!ADMIN_ACCOUNT_USER_KEY.contains("{user_id}"));
    }

    #[test]
    fn webdav_resources_path() {
        let path = make_path!(WEBDAV_RESOURCES_PATH, "docs/api.md");
        assert_eq!(path, "/webdav/resources/docs/api.md");
    }
}
