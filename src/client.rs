use std::path::Path;

use crate::endpoints;
use crate::error::Error;
use crate::types::v1::*;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct VikingClient {
    base_url: String,
    key: String,
    client: Client,
    account: Option<String>,
    user: Option<String>,
}

impl VikingClient {
    pub fn new(base_url: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            key: key.into(),
            client: Client::new(),
            account: None,
            user: None,
        }
    }

    pub fn with_tenant(
        base_url: impl Into<String>,
        key: impl Into<String>,
        account: impl Into<String>,
        user: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            key: key.into(),
            client: Client::new(),
            account: Some(account.into()),
            user: Some(user.into()),
        }
    }
}

impl VikingClient {
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn auth(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req = req.bearer_auth(&self.key);
        if let Some(account) = &self.account {
            req = req.header("X-OpenViking-Account", account);
        }
        if let Some(user) = &self.user {
            req = req.header("X-OpenViking-User", user);
        }
        req
    }

    async fn _extract<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T, Error> {
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        let env: ApiResponse<T> = resp.json().await?;
        if env.status == "error" {
            return Err(Error::Api {
                status,
                message: "API returned error status".into(),
            });
        }
        Ok(env.result)
    }

    async fn _get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.client.get(&self.url(path));
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, Error> {
        let req = self.client.get(&self.url(path)).query(query);
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _get_no_env<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.client.get(&self.url(path));
        let resp = self.auth(req).send().await?;
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        Ok(resp.json().await?)
    }

    async fn _get_raw<Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<Vec<u8>, Error> {
        let mut req = self.client.get(&self.url(path));
        if let Some(q) = query {
            req = req.query(q);
        }
        let resp = self.auth(req).send().await?;
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        Ok(resp.bytes().await?.to_vec())
    }

    async fn _post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let req = self.client.post(&self.url(path)).json(body);
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, Error> {
        let req = self.client.post(&self.url(path)).multipart(form);
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _post_no_body<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.client.post(&self.url(path));
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.client.delete(&self.url(path));
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _delete_with_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, Error> {
        let req = self.client.delete(&self.url(path)).query(query);
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }

    async fn _put<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let req = self.client.put(&self.url(path)).json(body);
        let resp = self.auth(req).send().await?;
        self._extract(resp).await
    }
}

fn q(pairs: &[(&str, Option<String>)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| (k.to_string(), v.clone())))
        .collect()
}

impl VikingClient {
    pub async fn health(&self) -> Result<HealthResponse, Error> {
        self._get_no_env(endpoints::v1::HEALTH).await
    }

    pub async fn ready(&self) -> Result<ReadyResponse, Error> {
        self._get_no_env(endpoints::v1::READY).await
    }

    pub async fn system_status(&self) -> Result<SystemStatusResponse, Error> {
        self._get(endpoints::v1::SYSTEM_STATUS).await
    }

    pub async fn system_wait(&self, timeout: Option<f64>) -> Result<SystemWaitResponse, Error> {
        let req = SystemWaitRequest { timeout };
        self._post(endpoints::v1::SYSTEM_WAIT, &req).await
    }
}

impl VikingClient {
    pub async fn temp_upload(
        &self,
        file_path: impl AsRef<Path>,
    ) -> Result<ResourcesTempUploadResponse, Error> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload")
            .to_string();
        let file_bytes = std::fs::read(path)?;
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| Error::Api {
                status: 0,
                message: e.to_string(),
            })?;
        let form = reqwest::multipart::Form::new().part("file", part);
        self._post_multipart(endpoints::v1::RESOURCES_TEMP_UPLOAD, form)
            .await
    }

    pub async fn add_resource(
        &self,
        req: &ResourcesAddRequest,
    ) -> Result<ResourcesAddResponse, Error> {
        self._post(endpoints::v1::RESOURCES, req).await
    }

    pub async fn add_skill(&self, req: &SkillsAddRequest) -> Result<SkillsAddResponse, Error> {
        self._post(endpoints::v1::SKILLS, req).await
    }
}

impl VikingClient {
    pub async fn pack_export(&self, uri: &str) -> Result<Vec<u8>, Error> {
        let req = PackExportRequest {
            uri: uri.to_string(),
        };
        let req_builder = self
            .client
            .post(&self.url(endpoints::v1::PACK_EXPORT))
            .json(&req);
        let resp = self.auth(req_builder).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        Ok(resp.bytes().await?.to_vec())
    }

    pub async fn pack_import(&self, req: &PackImportRequest) -> Result<PackImportResponse, Error> {
        self._post(endpoints::v1::PACK_IMPORT, req).await
    }
}

impl VikingClient {
    pub async fn fs_ls(
        &self,
        uri: &str,
        simple: Option<bool>,
        recursive: Option<bool>,
        output: Option<&str>,
        abs_limit: Option<i64>,
        show_all_hidden: Option<bool>,
        node_limit: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<FsEntry>, Error> {
        let query = q(&[
            ("uri", Some(uri.to_string())),
            ("simple", simple.map(|v| v.to_string())),
            ("recursive", recursive.map(|v| v.to_string())),
            ("output", output.map(|v| v.to_string())),
            ("abs_limit", abs_limit.map(|v| v.to_string())),
            ("show_all_hidden", show_all_hidden.map(|v| v.to_string())),
            ("node_limit", node_limit.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ]);
        self._get_with_query(endpoints::v1::FS_LS, &query).await
    }

    pub async fn fs_tree(
        &self,
        uri: &str,
        output: Option<&str>,
        abs_limit: Option<i64>,
        show_all_hidden: Option<bool>,
        node_limit: Option<i64>,
        limit: Option<i64>,
        level_limit: Option<i64>,
    ) -> Result<Vec<FsEntry>, Error> {
        let query = q(&[
            ("uri", Some(uri.to_string())),
            ("output", output.map(|v| v.to_string())),
            ("abs_limit", abs_limit.map(|v| v.to_string())),
            ("show_all_hidden", show_all_hidden.map(|v| v.to_string())),
            ("node_limit", node_limit.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
            ("level_limit", level_limit.map(|v| v.to_string())),
        ]);
        self._get_with_query(endpoints::v1::FS_TREE, &query).await
    }

    pub async fn fs_stat(&self, uri: &str) -> Result<FsEntry, Error> {
        let query = q(&[("uri", Some(uri.to_string()))]);
        self._get_with_query(endpoints::v1::FS_STAT, &query).await
    }

    pub async fn fs_mkdir(
        &self,
        uri: &str,
        description: Option<&str>,
    ) -> Result<FsMkdirResponse, Error> {
        let req = FsMkdirRequest {
            uri: uri.to_string(),
            description: description.map(|s| s.to_string()),
        };
        self._post(endpoints::v1::FS_MKDIR, &req).await
    }

    pub async fn fs_delete(
        &self,
        uri: &str,
        recursive: Option<bool>,
    ) -> Result<FsDeleteResponse, Error> {
        let query = q(&[
            ("uri", Some(uri.to_string())),
            ("recursive", recursive.map(|v| v.to_string())),
        ]);
        self._delete_with_query(endpoints::v1::FS_DELETE, &query)
            .await
    }

    pub async fn fs_mv(&self, from_uri: &str, to_uri: &str) -> Result<FsMvResponse, Error> {
        let req = FsMvRequest {
            from_uri: from_uri.to_string(),
            to_uri: to_uri.to_string(),
        };
        self._post(endpoints::v1::FS_MV, &req).await
    }
}

impl VikingClient {
    pub async fn content_read(
        &self,
        uri: &str,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<String, Error> {
        let query = q(&[
            ("uri", Some(uri.to_string())),
            ("offset", offset.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ]);
        self._get_with_query(endpoints::v1::CONTENT_READ, &query)
            .await
    }

    pub async fn content_abstract(&self, uri: &str) -> Result<String, Error> {
        let query = q(&[("uri", Some(uri.to_string()))]);
        self._get_with_query(endpoints::v1::CONTENT_ABSTRACT, &query)
            .await
    }

    pub async fn content_overview(&self, uri: &str) -> Result<String, Error> {
        let query = q(&[("uri", Some(uri.to_string()))]);
        self._get_with_query(endpoints::v1::CONTENT_OVERVIEW, &query)
            .await
    }

    pub async fn content_download(&self, uri: &str) -> Result<Vec<u8>, Error> {
        let query = q(&[("uri", Some(uri.to_string()))]);
        self._get_raw(endpoints::v1::CONTENT_DOWNLOAD, Some(&query))
            .await
    }

    pub async fn content_write(
        &self,
        req: &ContentWriteRequest,
    ) -> Result<ContentWriteResponse, Error> {
        self._post(endpoints::v1::CONTENT_WRITE, req).await
    }

    pub async fn content_reindex(&self, uri: &str) -> Result<serde_json::Value, Error> {
        let body = serde_json::json!({"uri": uri});
        self._post(endpoints::v1::CONTENT_REINDEX, &body).await
    }
}

impl VikingClient {
    pub async fn search_find(&self, req: &SearchFindRequest) -> Result<SearchFindResponse, Error> {
        self._post(endpoints::v1::SEARCH_FIND, req).await
    }

    pub async fn search(&self, req: &SearchSearchRequest) -> Result<SearchSearchResponse, Error> {
        self._post(endpoints::v1::SEARCH_SEARCH, req).await
    }

    pub async fn search_grep(&self, req: &SearchGrepRequest) -> Result<SearchGrepResponse, Error> {
        self._post(endpoints::v1::SEARCH_GREP, req).await
    }

    pub async fn search_glob(&self, req: &SearchGlobRequest) -> Result<SearchGlobResponse, Error> {
        self._post(endpoints::v1::SEARCH_GLOB, req).await
    }
}

impl VikingClient {
    pub async fn relations(&self, uri: &str) -> Result<Vec<RelationEntry>, Error> {
        let query = q(&[("uri", Some(uri.to_string()))]);
        self._get_with_query(endpoints::v1::RELATIONS, &query).await
    }

    pub async fn relations_link(
        &self,
        req: &RelationsLinkRequest,
    ) -> Result<RelationsLinkResponse, Error> {
        self._post(endpoints::v1::RELATIONS_LINK, req).await
    }

    pub async fn relations_unlink(
        &self,
        req: &RelationsUnlinkRequest,
    ) -> Result<RelationsUnlinkResponse, Error> {
        let req_builder = self
            .client
            .delete(&self.url(endpoints::v1::RELATIONS_LINK))
            .json(req);
        let resp = self.auth(req_builder).send().await?;
        self._extract(resp).await
    }
}

impl VikingClient {
    pub async fn create_session(
        &self,
        session_id: Option<&str>,
    ) -> Result<SessionsCreateResponse, Error> {
        let req = SessionsCreateRequest {
            session_id: session_id.map(|s| s.to_string()),
        };
        self._post(endpoints::v1::SESSIONS, &req).await
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionInfo>, Error> {
        self._get(endpoints::v1::SESSIONS).await
    }

    pub async fn get_session(&self, session_id: &str) -> Result<SessionDetailResponse, Error> {
        let path = endpoints::v1::SESSION_BY_ID.replace("{session_id}", session_id);
        self._get(&path).await
    }

    pub async fn get_session_context(
        &self,
        session_id: &str,
        token_budget: Option<i64>,
    ) -> Result<SessionContextResponse, Error> {
        let path = endpoints::v1::SESSION_CONTEXT.replace("{session_id}", session_id);
        if let Some(budget) = token_budget {
            let query = q(&[("token_budget", Some(budget.to_string()))]);
            self._get_with_query(&path, &query).await
        } else {
            self._get(&path).await
        }
    }

    pub async fn get_archive(
        &self,
        session_id: &str,
        archive_id: &str,
    ) -> Result<SessionArchiveResponse, Error> {
        let path = endpoints::v1::SESSION_ARCHIVE
            .replace("{session_id}", session_id)
            .replace("{archive_id}", archive_id);
        self._get(&path).await
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<SessionDeleteResponse, Error> {
        let path = endpoints::v1::SESSION_BY_ID.replace("{session_id}", session_id);
        self._delete(&path).await
    }

    pub async fn commit_session(&self, session_id: &str) -> Result<SessionCommitResponse, Error> {
        let path = endpoints::v1::SESSION_COMMIT.replace("{session_id}", session_id);
        self._post_no_body(&path).await
    }

    pub async fn extract_session(&self, session_id: &str) -> Result<serde_json::Value, Error> {
        let path = endpoints::v1::SESSION_EXTRACT.replace("{session_id}", session_id);
        self._post_no_body(&path).await
    }

    pub async fn add_message(
        &self,
        session_id: &str,
        req: &SessionMessagesAddRequest,
    ) -> Result<SessionMessagesAddResponse, Error> {
        let path = endpoints::v1::SESSION_MESSAGES.replace("{session_id}", session_id);
        self._post(&path, req).await
    }

    pub async fn record_used(
        &self,
        session_id: &str,
        req: &SessionUsedRequest,
    ) -> Result<SessionUsedResponse, Error> {
        let path = endpoints::v1::SESSION_USED.replace("{session_id}", session_id);
        self._post(&path, req).await
    }
}

impl VikingClient {
    pub async fn get_task(&self, task_id: &str) -> Result<TaskDetailResponse, Error> {
        let path = endpoints::v1::TASK_BY_ID.replace("{task_id}", task_id);
        self._get(&path).await
    }

    pub async fn list_tasks(
        &self,
        task_type: Option<&str>,
        status: Option<&str>,
        resource_id: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<TaskInfo>, Error> {
        let query = q(&[
            ("task_type", task_type.map(|v| v.to_string())),
            ("status", status.map(|v| v.to_string())),
            ("resource_id", resource_id.map(|v| v.to_string())),
            ("limit", limit.map(|v| v.to_string())),
        ]);
        self._get_with_query(endpoints::v1::TASKS, &query).await
    }
}

impl VikingClient {
    pub async fn privacy_list_categories(&self) -> Result<Vec<String>, Error> {
        self._get(endpoints::v1::PRIVACY_CONFIGS).await
    }

    pub async fn privacy_list_keys(&self, category: &str) -> Result<Vec<String>, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_CATEGORY.replace("{category}", category);
        self._get(&path).await
    }

    pub async fn privacy_get(
        &self,
        category: &str,
        target_key: &str,
    ) -> Result<PrivacyConfigDetailResponse, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_TARGET
            .replace("{category}", category)
            .replace("{target_key}", target_key);
        self._get(&path).await
    }

    pub async fn privacy_upsert(
        &self,
        category: &str,
        target_key: &str,
        req: &PrivacyConfigsUpsertRequest,
    ) -> Result<PrivacyConfigsUpsertResponse, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_TARGET
            .replace("{category}", category)
            .replace("{target_key}", target_key);
        self._post(&path, req).await
    }

    pub async fn privacy_list_versions(
        &self,
        category: &str,
        target_key: &str,
    ) -> Result<Vec<i64>, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_VERSIONS
            .replace("{category}", category)
            .replace("{target_key}", target_key);
        self._get(&path).await
    }

    pub async fn privacy_get_version(
        &self,
        category: &str,
        target_key: &str,
        version: i64,
    ) -> Result<PrivacyConfigSnapshot, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_VERSION
            .replace("{category}", category)
            .replace("{target_key}", target_key)
            .replace("{version}", &version.to_string());
        self._get(&path).await
    }

    pub async fn privacy_activate(
        &self,
        category: &str,
        target_key: &str,
        version: i64,
    ) -> Result<PrivacyConfigsUpsertResponse, Error> {
        let path = endpoints::v1::PRIVACY_CONFIGS_ACTIVATE
            .replace("{category}", category)
            .replace("{target_key}", target_key);
        let req = PrivacyConfigsActivateRequest { version };
        self._post(&path, &req).await
    }
}

impl VikingClient {
    pub async fn observer_queue(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_QUEUE).await
    }

    pub async fn observer_vikingdb(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_VIKINGDB).await
    }

    pub async fn observer_models(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_MODELS).await
    }

    pub async fn observer_lock(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_LOCK).await
    }

    pub async fn observer_retrieval(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_RETRIEVAL).await
    }

    pub async fn observer_system(&self) -> Result<ObserverStatus, Error> {
        self._get(endpoints::v1::OBSERVER_SYSTEM).await
    }
}

impl VikingClient {
    pub async fn debug_health(&self) -> Result<serde_json::Value, Error> {
        self._get(endpoints::v1::DEBUG_HEALTH).await
    }

    pub async fn debug_vector_scroll(&self) -> Result<serde_json::Value, Error> {
        self._get(endpoints::v1::DEBUG_VECTOR_SCROLL).await
    }

    pub async fn debug_vector_count(&self) -> Result<serde_json::Value, Error> {
        self._get(endpoints::v1::DEBUG_VECTOR_COUNT).await
    }
}

impl VikingClient {
    pub async fn maintenance_reindex(&self, uri: &str) -> Result<serde_json::Value, Error> {
        let body = serde_json::json!({"uri": uri});
        self._post(endpoints::v1::MAINTENANCE_REINDEX, &body).await
    }
}

impl VikingClient {
    pub async fn stats_memories(&self) -> Result<serde_json::Value, Error> {
        self._get(endpoints::v1::STATS_MEMORIES).await
    }

    pub async fn stats_session(&self, session_id: &str) -> Result<serde_json::Value, Error> {
        let path = endpoints::v1::STATS_SESSION.replace("{session_id}", session_id);
        self._get(&path).await
    }
}

impl VikingClient {
    pub async fn admin_create_account(
        &self,
        req: &AdminAccountsCreateRequest,
    ) -> Result<AdminAccountsCreateResponse, Error> {
        self._post(endpoints::v1::ADMIN_ACCOUNTS, req).await
    }

    pub async fn admin_list_accounts(&self) -> Result<Vec<AdminAccountInfo>, Error> {
        self._get(endpoints::v1::ADMIN_ACCOUNTS).await
    }

    pub async fn admin_delete_account(
        &self,
        account_id: &str,
    ) -> Result<AdminAccountDeleteResponse, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_BY_ID.replace("{account_id}", account_id);
        self._delete(&path).await
    }

    pub async fn admin_create_user(
        &self,
        account_id: &str,
        req: &AdminAccountUsersCreateRequest,
    ) -> Result<AdminAccountUsersCreateResponse, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_USERS.replace("{account_id}", account_id);
        self._post(&path, req).await
    }

    pub async fn admin_list_users(
        &self,
        account_id: &str,
        limit: Option<i64>,
        name: Option<&str>,
        role: Option<&str>,
    ) -> Result<Vec<AdminUserInfo>, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_USERS.replace("{account_id}", account_id);
        let query = q(&[
            ("limit", limit.map(|v| v.to_string())),
            ("name", name.map(|v| v.to_string())),
            ("role", role.map(|v| v.to_string())),
        ]);
        self._get_with_query(&path, &query).await
    }

    pub async fn admin_list_agents(&self, account_id: &str) -> Result<Vec<AdminAgentInfo>, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_AGENTS.replace("{account_id}", account_id);
        self._get(&path).await
    }

    pub async fn admin_delete_user(
        &self,
        account_id: &str,
        user_id: &str,
    ) -> Result<AdminAccountUserDeleteResponse, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_USER
            .replace("{account_id}", account_id)
            .replace("{user_id}", user_id);
        self._delete(&path).await
    }

    pub async fn admin_change_role(
        &self,
        account_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<AdminAccountUserRoleUpdateResponse, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_USER_ROLE
            .replace("{account_id}", account_id)
            .replace("{user_id}", user_id);
        let req = AdminAccountUserRoleUpdateRequest {
            role: role.to_string(),
        };
        self._put(&path, &req).await
    }

    pub async fn admin_regen_key(
        &self,
        account_id: &str,
        user_id: &str,
    ) -> Result<AdminAccountUserKeyRegenerateResponse, Error> {
        let path = endpoints::v1::ADMIN_ACCOUNT_USER_KEY
            .replace("{account_id}", account_id)
            .replace("{user_id}", user_id);
        self._post_no_body(&path).await
    }
}

impl VikingClient {
    pub async fn chat(&self, message: &str) -> Result<serde_json::Value, Error> {
        let body = serde_json::json!({"message": message});
        self._post(endpoints::v1::CHAT, &body).await
    }

    pub async fn chat_stream(&self, message: &str) -> Result<reqwest::Response, Error> {
        let req = self
            .client
            .post(&self.url(endpoints::v1::CHAT_STREAM))
            .json(&serde_json::json!({"message": message}));
        let resp = self.auth(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        Ok(resp)
    }
}

impl VikingClient {
    pub async fn metrics(&self) -> Result<String, Error> {
        let req = self.client.get(&self.url(endpoints::v1::METRICS));
        let resp = req.send().await?;
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: body,
            });
        }
        Ok(resp.text().await?)
    }
}
