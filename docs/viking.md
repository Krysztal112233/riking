# OpenViking HTTP API — Complete Endpoint Reference

> Compiled from the official OpenViking API documentation markdown files.
> **Base URL**: `http://localhost:1933` (default)
> **Response Envelope**: All JSON responses follow `{"status": "ok"|"error", "result": {...}, "time": 0.123}`
> **Auth**: Most endpoints require `X-API-Key` header or `Authorization: Bearer`. `/health` and `/ready` never require auth.

---

## Table of Contents

- [1. System](#1-system)
- [2. Resources](#2-resources)
- [3. Pack (Export/Import)](#3-pack-exportimport)
- [4. File System](#4-file-system)
- [5. Content](#5-content)
- [6. Search / Retrieval](#6-search--retrieval)
- [7. Relations](#7-relations)
- [8. Sessions](#8-sessions)
- [9. Tasks](#9-tasks)
- [10. Privacy Configs](#10-privacy-configs)
- [11. Observer (Monitoring)](#11-observer-monitoring)
- [12. Debug](#12-debug)
- [13. Maintenance](#13-maintenance)
- [14. Statistics](#14-statistics)
- [15. Admin (Multi-tenant)](#15-admin-multi-tenant)
- [16. VikingBot (Optional)](#16-vikingbot-optional)
- [17. WebDAV](#17-webdav)
- [18. Metrics](#18-metrics)
- [Appendix: Error Codes](#appendix-error-codes)

---

## 1. System

### 1.1 GET `/health`

Health check. No authentication required.

**Parameters**: None

**Response**:

```json
{
  "status": "ok",
  "healthy": true,
  "version": "0.1.x",
  "auth_mode": "api_key"
}
```

**curl**:

```bash
curl http://localhost:1933/health
```

---

### 1.2 GET `/ready`

Readiness probe (for Kubernetes etc.). Checks AGFS, VectorDB, APIKeyManager, Ollama. Returns 200 when all subsystems ready, 503 otherwise. No auth required.

**Parameters**: None

**Response**:

```json
{
  "status": "ready",
  "checks": {
    "agfs": "ok",
    "vectordb": "ok",
    "api_key_manager": "ok",
    "ollama": "not_configured"
  }
}
```

**curl**:

```bash
curl http://localhost:1933/ready
```

---

### 1.3 GET `/api/v1/system/status`

Get system status including initialization state and authenticated user info. Auth required.

**Parameters**: None

**Response**:

```json
{
  "status": "ok",
  "result": {
    "initialized": true,
    "user": "alice"
  },
  "time": 0.1
}
```

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/system/status \
  -H "X-API-Key: your-key"
```

---

### 1.4 POST `/api/v1/system/wait`

Wait for all asynchronous processing (embedding, semantic generation) to complete. Blocks until done or timeout. Auth required.

**Request Body**:

| Parameter | Type  | Required | Default | Description                                  |
| --------- | ----- | -------- | ------- | -------------------------------------------- |
| timeout   | float | No       | None    | Timeout in seconds. None = wait indefinitely |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/system/wait \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"timeout": 60.0}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "Embedding": {
      "processed": 10,
      "requeue_count": 0,
      "error_count": 0,
      "errors": []
    },
    "Semantic": {
      "processed": 10,
      "requeue_count": 0,
      "error_count": 0,
      "errors": []
    }
  },
  "time": 0.1
}
```

---

## 2. Resources

### 2.1 POST `/api/v1/resources/temp_upload`

Upload a local file to server temporary storage. Returns a `temp_file_id` for subsequent use with `add_resource` or `add_skill`. Content-Type: `multipart/form-data`.

**Request Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| file | UploadFile | Yes | - | File to upload (multipart) |
| telemetry | bool | No | False | Return telemetry data |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/resources/temp_upload \
  -H "X-API-Key: your-key" \
  -F "file=@./documents/guide.md"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "temp_file_id": "upload_abc123def456.md"
  }
}
```

---

### 2.2 POST `/api/v1/resources`

Add a resource (URL or local file via `temp_file_id`). Core entry point for importing documents, code, media, etc. into the knowledge base.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| path | string | No | - | Remote URL (HTTP/HTTPS/Git). Mutually exclusive with `temp_file_id` |
| temp_file_id | string | No | - | Temp upload file ID. Mutually exclusive with `path` |
| to | string | No | - | Target Viking URI (exact location). Mutually exclusive with `parent` |
| parent | string | No | - | Parent Viking URI (resource placed under directory). Mutually exclusive with `to` |
| reason | string | No | "" | Reason for adding (for relevance improvement, experimental) |
| instruction | string | No | "" | Processing instructions for semantic extraction (experimental) |
| wait | bool | No | False | Wait for semantic processing and vectorization before returning |
| timeout | float | No | None | Timeout in seconds (only when `wait=true`) |
| strict | bool | No | False | Use strict mode |
| ignore_dirs | string | No | None | Comma-separated directory names to ignore |
| include | string | No | None | Glob patterns to include |
| exclude | string | No | None | Glob patterns to exclude |
| directly_upload_media | bool | No | True | Directly upload media files |
| preserve_structure | bool | No | None | Preserve directory structure |
| watch_interval | float | No | 0 | Scheduled update interval in minutes (>0 creates task; <=0 cancels) |
| telemetry | TelemetryRequest | No | False | Return telemetry data |

**Notes**:

- `to` and `parent` are mutually exclusive
- `path` and `temp_file_id` are mutually exclusive
- For raw HTTP calls, local files must be uploaded first via `temp_upload` to get a `temp_file_id`
- When `to` is specified and the target exists, triggers incremental update

**curl**:

```bash
# From URL
curl -X POST http://localhost:1933/api/v1/resources \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "path": "https://example.com/guide.md",
    "reason": "User guide documentation",
    "wait": true
  }'

# From local file (requires temp_upload first)
TEMP_FILE_ID=$(curl -s -X POST http://localhost:1933/api/v1/resources/temp_upload \
  -H "X-API-Key: your-key" \
  -F "file=@./documents/guide.md" | jq -r '.result.temp_file_id')

curl -X POST http://localhost:1933/api/v1/resources \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d "{\"temp_file_id\": \"$TEMP_FILE_ID\", \"to\": \"viking://resources/guide.md\"}"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "status": "success",
    "root_uri": "viking://resources/guide.md",
    "temp_uri": "viking://temp/username/.../guide.md",
    "source_path": "./documents/guide.md",
    "meta": {},
    "errors": [],
    "queue_status": {
      "pending": 5,
      "processing": 2,
      "completed": 10
    }
  }
}
```

**Response Fields**:
| Field | Type | Description |
|-------|------|-------------|
| `result.status` | string | Processing status: `"success"` or `"error"` |
| `root_uri` | string | Final URI of the resource |
| `temp_uri` | string | Temporary URI during processing |
| `source_path` | string | Original source path or URL |
| `meta` | object | Metadata from parsing |
| `errors` | array | List of processing errors |
| `warnings` | array | (Optional) Warnings when `strict=False` |
| `queue_status` | object | (Optional, when `wait=true`) Queue stats |

---

### 2.3 POST `/api/v1/skills`

Add a skill to the knowledge base. Supports structured data, SKILL.md content, MCP Tool format, or local file via `temp_file_id`.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| data | Any | No | - | Inline skill content (dict, string SKILL.md, or MCP tool). Mutually exclusive with `temp_file_id` |
| temp_file_id | string | No | - | Temp upload file ID. Mutually exclusive with `data` |
| wait | bool | No | False | Wait for skill processing to complete |
| timeout | float | No | None | Timeout when `wait=true` |
| telemetry | TelemetryRequest | No | False | Return telemetry data |

**curl**:

```bash
# Inline structured data
curl -X POST http://localhost:1933/api/v1/skills \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "data": {
      "name": "search-web",
      "description": "Search the web",
      "content": "# search-web\n\nSearch the web.\n\n## Parameters\n- **query** (string, required): Search query"
    },
    "wait": true
  }'

# Using MCP Tool format (auto-detected)
curl -X POST http://localhost:1933/api/v1/skills \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "data": {
      "name": "calculator",
      "description": "Perform calculations",
      "inputSchema": {
        "type": "object",
        "properties": { "expression": { "type": "string" } },
        "required": ["expression"]
      }
    }
  }'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "status": "success",
    "root_uri": "viking://agent/skills/my-skill",
    "uri": "viking://agent/skills/my-skill",
    "name": "my-skill",
    "auxiliary_files": 2
  }
}
```

**Response Fields**:
| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Processing status: `"success"` or `"error"` |
| `root_uri` | string | Final skill URI (same as `uri`) |
| `uri` | string | Final skill URI (same as `root_uri`) |
| `name` | string | Skill name |
| `auxiliary_files` | number | Number of auxiliary files attached |
| `queue_status` | object | (Optional, when `wait=true`) Queue stats |

---

## 3. Pack (Export/Import)

### 3.1 POST `/api/v1/pack/export`

Export a resource tree as a `.ovpack` file (ZIP format). Returns file stream. Permission: ROOT/ADMIN.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | string | Yes | - | Viking URI to export |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/pack/export \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-admin-key" \
  -d '{"uri": "viking://resources/my-project/"}' \
  --output my-project.ovpack
```

**Response**: Direct file stream (`Content-Type: application/zip`), not JSON envelope.

---

### 3.2 POST `/api/v1/pack/import`

Import a `.ovpack` file to a specified location. Requires `temp_file_id` from `temp_upload`. Permission: ROOT/ADMIN.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| temp_file_id | string | Yes | - | Temp upload file ID |
| parent | string | Yes | - | Target parent URI |
| force | bool | No | False | Overwrite existing resources |
| vectorize | bool | No | True | Trigger vectorization |

**curl**:

```bash
TEMP_FILE_ID=$(curl -s -X POST http://localhost:1933/api/v1/resources/temp_upload \
  -H "X-API-Key: your-admin-key" \
  -F "file=@./exports/my-project.ovpack" | jq -r '.result.temp_file_id')

curl -X POST http://localhost:1933/api/v1/pack/import \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-admin-key" \
  -d "{\"temp_file_id\": \"$TEMP_FILE_ID\", \"parent\": \"viking://resources/imported/\", \"force\": true}"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "uri": "viking://resources/imported/my-project/"
  }
}
```

---

## 4. File System

### 4.1 GET `/api/v1/fs/ls`

List directory contents.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI to list |
| simple | bool | No | False | Return only relative paths |
| recursive | bool | No | False | List recursively |
| output | str | No | `agent` | Output format: `agent` or `original` |
| abs_limit | int | No | 256 | Abstract length limit for `agent` output |
| show_all_hidden | bool | No | False | Include hidden files like `-a` |
| node_limit | int | No | 1000 | Max nodes to return |
| limit | int | No | None | Alias for `node_limit` |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/fs/ls?uri=viking://resources/" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    {
      "name": "docs",
      "size": 4096,
      "mode": 16877,
      "modTime": "2024-01-01T00:00:00Z",
      "isDir": true,
      "uri": "viking://resources/docs/"
    }
  ],
  "time": 0.1
}
```

**Entry Fields**:
| Field | Type | Description |
|-------|------|-------------|
| `name` | string | File/directory name |
| `size` | int | Size in bytes |
| `mode` | int | File mode |
| `modTime` | string | ISO 8601 timestamp |
| `isDir` | bool | True if directory |
| `uri` | string | Viking URI |
| `meta` | object | (Optional) Metadata |

---

### 4.2 GET `/api/v1/fs/tree`

Get directory tree structure.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI |
| output | str | No | `agent` | Output format |
| abs_limit | int | No | 256 | Abstract length limit |
| show_all_hidden | bool | No | False | Include hidden files |
| node_limit | int | No | 1000 | Max nodes |
| limit | int | No | None | Alias for `node_limit` |
| level_limit | int | No | 3 | Max directory depth |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/fs/tree?uri=viking://resources/" \
  -H "X-API-Key: your-key"
```

**Response**: Array of entries with `name`, `size`, `isDir`, `rel_path`, `uri`.

---

### 4.3 GET `/api/v1/fs/stat`

Get file/directory status information.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/fs/stat?uri=viking://resources/docs/api.md" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "name": "api.md",
    "size": 1024,
    "mode": 33188,
    "modTime": "2024-01-01T00:00:00Z",
    "isDir": false,
    "uri": "viking://resources/docs/api.md"
  },
  "time": 0.1
}
```

---

### 4.4 POST `/api/v1/fs/mkdir`

Create a directory.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI for the new directory |
| description | str | No | null | Initial directory description (written to `.abstract.md`) |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/fs/mkdir \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"uri": "viking://resources/new-project/", "description": "API docs directory"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "uri": "viking://resources/new-project/" },
  "time": 0.1
}
```

---

### 4.5 DELETE `/api/v1/fs`

Remove a file or directory (idempotent). Use `?recursive=true` for directories.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI to remove |
| recursive | bool | No | False | Remove directory recursively |

**curl**:

```bash
# Single file
curl -X DELETE "http://localhost:1933/api/v1/fs?uri=viking://resources/docs/old.md" \
  -H "X-API-Key: your-key"

# Directory recursively
curl -X DELETE "http://localhost:1933/api/v1/fs?uri=viking://resources/old-project/&recursive=true" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": { "uri": "viking://resources/docs/old.md" },
  "time": 0.1
}
```

---

### 4.6 POST `/api/v1/fs/mv`

Move or rename a file/directory.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| from_uri | str | Yes | - | Source Viking URI |
| to_uri | str | Yes | - | Destination Viking URI |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/fs/mv \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"from_uri": "viking://resources/old-name/", "to_uri": "viking://resources/new-name/"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "from": "viking://resources/old-name/",
    "to": "viking://resources/new-name/"
  },
  "time": 0.1
}
```

---

## 5. Content

### 5.1 GET `/api/v1/content/read`

Read full content (L2). Accepts file URIs only. Supports `offset` and `limit` for partial reads.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | File Viking URI |
| offset | int | No | 0 | Starting line number (0-indexed) |
| limit | int | No | -1 | Number of lines, `-1` = read to end |

**Notes**:

- Accepts `resources`, `user`, `agent`, `session` scopes only. Internal scopes (`temp`, `queue`) return `INVALID_URI`.
- Passing a directory URI returns `400 INVALID_ARGUMENT`.

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/content/read?uri=viking://resources/docs/api.md" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": "# API Documentation\n\nFull content of the file...",
  "time": 0.1
}
```

---

### 5.2 GET `/api/v1/content/abstract`

Read L0 abstract (~100 tokens summary). Applies to directories.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI (directory) |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/content/abstract?uri=viking://resources/docs/" \
  -H "X-API-Key: your-key"
```

**Response**: String summary under `result`.

---

### 5.3 GET `/api/v1/content/overview`

Read L1 overview. Applies to directories.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI (directory) |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/content/overview?uri=viking://resources/docs/" \
  -H "X-API-Key: your-key"
```

**Response**: Markdown string overview under `result`.

---

### 5.4 GET `/api/v1/content/download`

Download raw file bytes.

**Parameters**: Standard auth + Viking URI query param.

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/content/download?uri=viking://resources/docs/api.md" \
  -H "X-API-Key: your-key"
```

**Response**: Raw file bytes (not JSON).

---

### 5.5 POST `/api/v1/content/write`

Update an existing file (or create new with `mode="create"`), then automatically refresh semantics and vectors.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Existing file URI |
| content | str | Yes | - | New content |
| mode | str | No | `replace` | `replace`, `append`, or `create` |
| wait | bool | No | `false` | Wait for semantic/vector refresh |
| timeout | float | No | `null` | Timeout when `wait=true` |

**Notes**:

- `replace` and `append` require the file to exist; `create` targets new files and returns `409 Conflict` if exists.
- `create` accepts only text-writable extensions: `.md`, `.txt`, `.json`, `.yaml`, `.yml`, `.toml`, `.py`, `.js`, `.ts`.
- Parent directories are created automatically in `create` mode.
- `.abstract.md`, `.overview.md`, `.relations.json` cannot be written directly.

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/content/write \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "uri": "viking://resources/docs/api.md",
    "content": "# Updated API\n\nFresh content.",
    "mode": "replace",
    "wait": true
  }'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "uri": "viking://resources/docs/api.md",
    "root_uri": "viking://resources/docs",
    "context_type": "resource",
    "mode": "replace",
    "written_bytes": 29,
    "content_updated": true,
    "semantic_status": "complete",
    "vector_status": "complete",
    "queue_status": {
      "Semantic": { "processed": 1, "error_count": 0, "errors": [] },
      "Embedding": { "processed": 2, "error_count": 0, "errors": [] }
    }
  }
}
```

---

### 5.6 POST `/api/v1/content/reindex`

Rebuild semantic/vector index for existing content. Deprecated; use maintenance/reindex instead.

**Parameters**: Standard auth + Viking URI.

---

## 6. Search / Retrieval

### 6.1 POST `/api/v1/search/find`

Basic vector similarity search. No session context or intent analysis.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| query | str | Yes | - | Search query string |
| target_uri | str \| List[str] | No | "" | Limit search to URI prefix |
| limit | int | No | 10 | Max results |
| node_limit | int | No | None | Alias that overrides `limit` |
| score_threshold | float | No | None | Minimum relevance score |
| filter | Dict | No | None | Metadata filter |
| since | str | No | None | Lower time bound (`2h`, ISO 8601, or `YYYY-MM-DD`) |
| until | str | No | None | Upper time bound |
| time_field | str | No | "updated_at" | `"updated_at"` or `"created_at"` |
| include_provenance | bool | No | False | Include query-plan details |
| telemetry | bool/object | No | False | Attach telemetry data |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/search/find \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"query": "how to authenticate users", "limit": 10}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "memories": [],
    "resources": [
      {
        "context_type": "resource",
        "uri": "viking://resources/.../...md",
        "level": 2,
        "score": 0.128,
        "category": "",
        "match_reason": "",
        "relations": [],
        "abstract": "This document is...",
        "overview": null
      }
    ],
    "skills": [],
    "total": 1
  }
}
```

**MatchedContext Fields**:
| Field | Type | Description |
|-------|------|-------------|
| `uri` | string | Viking URI |
| `context_type` | string | `"resource"`, `"memory"`, or `"skill"` |
| `level` | int | Tier (0=L0, 1=L1, 2=L2) |
| `abstract` | string | L0 content |
| `overview` | string\|null | L1 overview (optional for non-leaf nodes) |
| `category` | string | Category |
| `score` | float | Relevance score (0–1) |
| `match_reason` | string | Why this matched |
| `relations` | array | Related contexts |

---

### 6.2 POST `/api/v1/search/search`

Context-aware search with session context and intent analysis. Supports sessions for conversational search.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| query | str | Yes | - | Search query string |
| target_uri | str \| List[str] | No | "" | Limit search to URI prefix |
| session_id | str | No | None | Session ID for context-aware search |
| limit | int | No | 10 | Max results |
| node_limit | int | No | None | Alias overrides `limit` |
| score_threshold | float | No | None | Min relevance score |
| filter | Dict | No | None | Metadata filter |
| since | str | No | None | Lower time bound |
| until | str | No | None | Upper time bound |
| time_field | str | No | "updated_at" | `"updated_at"` or `"created_at"` |
| include_provenance | bool | No | False | Include query-plan details |
| telemetry | bool/object | No | False | Attach telemetry data |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/search/search \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"query": "best practices", "session_id": "abc123", "limit": 10}'
```

**Response**: Same `FindResult` structure as `find()`, plus optional `query_plan` with intent analysis details.

---

### 6.3 POST `/api/v1/search/grep`

Regex pattern matching in the file system.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI to search in |
| pattern | str | Yes | - | Regex pattern |
| case_insensitive | bool | No | False | Ignore case |
| exclude_uri | str | No | None | URI prefix to exclude |
| node_limit | int | No | None | Max nodes to search |
| level_limit | int | No | 5 | Max directory depth |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/search/grep \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"uri": "viking://resources/", "pattern": "authentication", "case_insensitive": true}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "matches": [
      {
        "uri": "viking://resources/docs/auth.md",
        "line": 15,
        "content": "User authentication is handled by..."
      }
    ],
    "count": 1
  },
  "time": 0.1
}
```

---

### 6.4 POST `/api/v1/search/glob`

File pattern matching (glob). Similar to Unix shell glob.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| pattern | str | Yes | - | Glob pattern (e.g., `**/*.md`) |
| uri | str | No | "viking://" | Starting URI |
| node_limit | int | No | None | Max matches to return |

**Pattern Syntax**: `*` (non-separator chars), `**` (recursive), `?` (single char), `[]` (char range).

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/search/glob \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"pattern": "**/*.md", "uri": "viking://resources/"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "matches": [
      "viking://resources/docs/api.md",
      "viking://resources/docs/guide.md"
    ],
    "count": 2
  },
  "time": 0.1
}
```

---

## 7. Relations

### 7.1 GET `/api/v1/relations`

Get relations for a resource. (Experimental.)

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| uri | str | Yes | - | Viking URI |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/relations?uri=viking://resources/docs/auth/" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    {
      "uri": "viking://resources/docs/security/",
      "reason": "Security best practices"
    }
  ],
  "time": 0.1
}
```

---

### 7.2 POST `/api/v1/relations/link`

Create relations (links) between resources. (Experimental.)

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| from_uri | str | Yes | - | Source URI |
| to_uris | str \| List[str] | Yes | - | Target URI(s) |
| reason | str | No | "" | Reason for the link |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/relations/link \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"from_uri": "viking://resources/docs/auth/", "to_uris": "viking://resources/docs/security/", "reason": "Security best practices"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "from": "viking://resources/docs/auth/",
    "to": "viking://resources/docs/security/"
  },
  "time": 0.1
}
```

---

### 7.3 DELETE `/api/v1/relations/link`

Remove a relation. (Experimental.)

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| from_uri | str | Yes | - | Source URI |
| to_uri | str | Yes | - | Target URI |

**curl**:

```bash
curl -X DELETE http://localhost:1933/api/v1/relations/link \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"from_uri": "viking://resources/docs/auth/", "to_uri": "viking://resources/docs/security/"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "from": "viking://resources/docs/auth/",
    "to": "viking://resources/docs/security/"
  },
  "time": 0.1
}
```

---

## 8. Sessions

### 8.1 POST `/api/v1/sessions`

Create a new session.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| session_id | str | No | None | Session ID (auto-generated if None) |

**curl**:

```bash
# Auto-generated ID
curl -X POST http://localhost:1933/api/v1/sessions \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key"

# Custom ID
curl -X POST http://localhost:1933/api/v1/sessions \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"session_id": "my-custom-session-id"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "session_id": "a1b2c3d4",
    "user": {
      "account_id": "default",
      "user_id": "alice",
      "agent_id": "default"
    }
  },
  "time": 0.1
}
```

---

### 8.2 GET `/api/v1/sessions`

List all sessions for the current user.

**Parameters**: None.

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/sessions \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    {
      "session_id": "a1b2c3d4",
      "uri": "viking://session/alice/a1b2c3d4",
      "is_dir": true
    }
  ],
  "time": 0.1
}
```

---

### 8.3 GET `/api/v1/sessions/{session_id}`

Get session details including metadata, message stats, commit history, token usage.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| auto_create | bool | No | False | Auto-create session if not found |

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/sessions/a1b2c3d4 \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "session_id": "a1b2c3d4",
    "created_at": "2026-03-23T10:00:00+08:00",
    "updated_at": "2026-03-23T11:30:00+08:00",
    "message_count": 5,
    "total_message_count": 20,
    "commit_count": 3,
    "memories_extracted": {
      "profile": 1,
      "preferences": 2,
      "entities": 3,
      "events": 1,
      "cases": 2,
      "patterns": 1,
      "tools": 0,
      "skills": 0,
      "total": 10
    },
    "last_commit_at": "2026-03-23T11:00:00+08:00",
    "llm_token_usage": {
      "prompt_tokens": 5200,
      "completion_tokens": 1800,
      "total_tokens": 7000
    },
    "user": {
      "account_id": "default",
      "user_id": "alice",
      "agent_id": "default"
    },
    "pending_tokens": 450
  }
}
```

---

### 8.4 GET `/api/v1/sessions/{session_id}/context`

Get assembled session context used for LLM context building. Returns latest archive overview and current live messages.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| token_budget | int | No | 128000 | Token budget for archive payload |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/sessions/a1b2c3d4/context?token_budget=128000" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "latest_archive_overview": "# Session Summary\n\n**Overview**: User discussed deployment and auth setup.",
    "pre_archive_abstracts": [],
    "messages": [
      {
        "id": "msg_pending_1",
        "role": "user",
        "parts": [{ "type": "text", "text": "Pending user message" }],
        "created_at": "..."
      }
    ],
    "estimatedTokens": 160,
    "stats": {
      "totalArchives": 2,
      "includedArchives": 1,
      "droppedArchives": 0,
      "failedArchives": 0,
      "activeTokens": 98,
      "archiveTokens": 62
    }
  }
}
```

---

### 8.5 GET `/api/v1/sessions/{session_id}/archives/{archive_id}`

Get the full contents of one completed archive.

**Path Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| session_id | str | Yes | Session ID |
| archive_id | str | Yes | Archive ID (e.g. `archive_002`) |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/sessions/a1b2c3d4/archives/archive_002" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "archive_id": "archive_002",
    "abstract": "User discussed deployment and authentication setup.",
    "overview": "# Session Summary...",
    "messages": [
      {
        "id": "msg_archive_1",
        "role": "user",
        "parts": [
          { "type": "text", "text": "How should I deploy this service?" }
        ],
        "created_at": "..."
      }
    ]
  }
}
```

Returns `404 NOT_FOUND` if archive does not exist.

---

### 8.6 DELETE `/api/v1/sessions/{session_id}`

Delete a session and all its data (messages, archives, memories). Irreversible.

**curl**:

```bash
curl -X DELETE http://localhost:1933/api/v1/sessions/a1b2c3d4 \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": { "session_id": "a1b2c3d4" },
  "time": 0.1
}
```

---

### 8.7 POST `/api/v1/sessions/{session_id}/commit`

Commit a session. Phase 1 (archiving) completes synchronously; Phase 2 (summary + memory extraction) runs asynchronously. Returns a `task_id` for polling.

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/commit \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "session_id": "a1b2c3d4",
    "status": "accepted",
    "task_id": "uuid-xxx",
    "archive_uri": "viking://session/alice/a1b2c3d4/history/archive_001",
    "archived": true
  }
}
```

---

### 8.8 POST `/api/v1/sessions/{session_id}/extract`

HTTP API only. Trigger memory extraction immediately without creating a new commit.

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/extract \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key"
```

**Response**: JSON list of extracted memory write results.

---

### 8.9 POST `/api/v1/sessions/{session_id}/messages`

Add a message to a session. Supports simple text mode and Parts mode.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| role | str | Yes | - | `"user"` or `"assistant"` |
| parts | List[Part] | Conditional | - | Parts list (mutually exclusive with `content`) |
| content | str | Conditional | - | Simple text (mutually exclusive with `parts`) |
| created_at | str | No | None | ISO 8601 timestamp |
| role_id | str | No | None | Explicit participant ID |

**Note**: If both `content` and `parts` are provided, `parts` takes precedence.

**Part Types**: `TextPart` (text), `ContextPart` (uri + context_type + abstract), `ToolPart` (tool_id, tool_name, skill_uri, tool_input, tool_output, tool_status).

**curl**:

```bash
# Simple mode
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/messages \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"role": "user", "content": "How do I authenticate users?"}'

# Parts mode
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/messages \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "role": "assistant",
    "parts": [
      {"type": "text", "text": "Based on the authentication guide..."},
      {"type": "context", "uri": "viking://resources/docs/auth/", "context_type": "resource", "abstract": "Auth guide"}
    ]
  }'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "session_id": "a1b2c3d4", "message_count": 2 },
  "time": 0.1
}
```

---

### 8.10 POST `/api/v1/sessions/{session_id}/used`

Record actually used contexts and skills in a session. Used to optimize future retrieval ranking on commit.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| contexts | List[str] | No | None | List of context URIs that were used |
| skill | Dict | No | None | Skill usage record: `{uri, input, output, success}` |

**curl**:

```bash
# Record contexts
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/used \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"contexts": ["viking://resources/docs/auth/"]}'

# Record skill
curl -X POST http://localhost:1933/api/v1/sessions/a1b2c3d4/used \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"skill": {"uri": "viking://agent/skills/search-web/", "input": {"query": "OAuth"}, "output": "Results...", "success": true}}'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "session_id": "a1b2c3d4", "contexts_used": 1, "skills_used": 0 },
  "time": 0.1
}
```

---

## 9. Tasks

### 9.1 GET `/api/v1/tasks/{task_id}`

Get background task status (e.g., commit summary generation and memory extraction progress).

**Task Statuses**: `pending`, `running`, `completed`, `failed`

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/tasks/uuid-xxx \
  -H "X-API-Key: your-key"
```

**Response (in progress)**:

```json
{
  "status": "ok",
  "result": {
    "task_id": "uuid-xxx",
    "task_type": "session_commit",
    "status": "running"
  }
}
```

**Response (completed)**:

```json
{
  "status": "ok",
  "result": {
    "task_id": "uuid-xxx",
    "task_type": "session_commit",
    "status": "completed",
    "result": {
      "session_id": "a1b2c3d4",
      "archive_uri": "viking://session/alice/a1b2c3d4/history/archive_001",
      "memories_extracted": {
        "profile": 1,
        "preferences": 2,
        "entities": 1,
        "cases": 1
      },
      "active_count_updated": 2,
      "token_usage": {
        "llm": {
          "prompt_tokens": 5200,
          "completion_tokens": 1800,
          "total_tokens": 7000
        }
      }
    }
  }
}
```

---

### 9.2 GET `/api/v1/tasks`

List background tasks visible to the current caller, with filtering support.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| task_type | str | No | None | Filter by type (e.g. `session_commit`) |
| status | str | No | None | Filter by status: `pending`, `running`, `completed`, `failed` |
| resource_id | str | No | None | Filter by resource ID (e.g. session ID) |
| limit | int | No | 50 | Max records to return |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/tasks?task_type=session_commit&status=running&limit=20" \
  -H "X-API-Key: your-key"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    {
      "task_id": "uuid-xxx",
      "task_type": "session_commit",
      "status": "running",
      "resource_id": "a1b2c3d4",
      "created_at": 1770000000.0,
      "updated_at": 1770000005.0,
      "result": null,
      "error": null
    }
  ]
}
```

---

## 10. Privacy Configs

Manage sensitive values by `category + target_key`. Each update creates a version snapshot.

### 10.1 GET `/api/v1/privacy-configs`

List categories that have privacy configs for the current user.

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/privacy-configs \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice"
```

**Response**:

```json
{ "status": "ok", "result": ["skill"], "time": 0.01 }
```

---

### 10.2 GET `/api/v1/privacy-configs/{category}`

List target keys under a category.

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/privacy-configs/skill \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice"
```

**Response**:

```json
{
  "status": "ok",
  "result": ["byted-viking-search-knowledgebase"],
  "time": 0.01
}
```

---

### 10.3 GET `/api/v1/privacy-configs/{category}/{target_key}`

Get active config (`meta + current`).

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/privacy-configs/skill/byted-viking-search-knowledgebase" \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "meta": {
      "category": "skill",
      "target_key": "byted-viking-search-knowledgebase",
      "active_version": 3,
      "latest_version": 5
    },
    "current": {
      "version": 3,
      "category": "skill",
      "target_key": "byted-viking-search-knowledgebase",
      "values": { "api_key": "***", "base_url": "https://example.com" }
    }
  },
  "time": 0.01
}
```

Returns `NOT_FOUND` if target does not exist.

---

### 10.4 POST `/api/v1/privacy-configs/{category}/{target_key}`

Upsert and activate a new version. If `values` is identical to current, no new version is created.

**Request Body**:
| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| values | object | Yes | - | Privacy key-value pairs |
| change_reason | string | No | "" | Reason for change |
| labels | object | No | null | Labels stored in meta |

**curl**:

```bash
curl -X POST "http://localhost:1933/api/v1/privacy-configs/skill/byted-viking-search-knowledgebase" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice" \
  -d '{"values": {"api_key": "***", "base_url": "https://example.com"}, "change_reason": "rotate key"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "version": 4, "category": "skill", "target_key": "byted-viking-search-knowledgebase", "values": {...}, "change_reason": "rotate key" },
  "time": 0.02
}
```

---

### 10.5 GET `/api/v1/privacy-configs/{category}/{target_key}/versions`

List all version numbers for a target.

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/privacy-configs/skill/byted-viking-search-knowledgebase/versions" \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice"
```

**Response**:

```json
{ "status": "ok", "result": [1, 2, 3, 4], "time": 0.01 }
```

Returns `NOT_FOUND` if target does not exist.

---

### 10.6 GET `/api/v1/privacy-configs/{category}/{target_key}/versions/{version}`

Get a specific version snapshot.

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/privacy-configs/skill/byted-viking-search-knowledgebase/versions/2" \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice"
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "version": 2,
    "category": "skill",
    "target_key": "...",
    "values": { "api_key": "***", "base_url": "https://example.com" }
  },
  "time": 0.01
}
```

Returns `NOT_FOUND` if target/version does not exist.

---

### 10.7 POST `/api/v1/privacy-configs/{category}/{target_key}/activate`

Switch active version.

**Request Body**:
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| version | int | Yes | Version number to activate |

**curl**:

```bash
curl -X POST "http://localhost:1933/api/v1/privacy-configs/skill/byted-viking-search-knowledgebase/activate" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -H "X-OpenViking-Account: default" \
  -H "X-OpenViking-User: alice" \
  -d '{"version": 2}'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "version": 2, "category": "skill", "target_key": "...", "values": {...} },
  "time": 0.01
}
```

Returns `NOT_FOUND` if target/version does not exist.

---

## 11. Observer (Monitoring)

All observer endpoints return the same structure: `{"name": "...", "is_healthy": bool, "has_errors": bool, "status": "..."}`.

### 11.1 GET `/api/v1/observer/queue`

Get queue system status (embedding and semantic processing queues). Shows pending, in-progress, completed, and error counts.

```bash
curl http://localhost:1933/api/v1/observer/queue -H "X-API-Key: your-key"
```

### 11.2 GET `/api/v1/observer/vikingdb`

Get VikingDB status (collections, indexes, vector counts).

```bash
curl http://localhost:1933/api/v1/observer/vikingdb -H "X-API-Key: your-key"
```

### 11.3 GET `/api/v1/observer/models`

Get aggregated model subsystem status (VLM, embedding, rerank).

```bash
curl http://localhost:1933/api/v1/observer/models -H "X-API-Key: your-key"
```

### 11.4 GET `/api/v1/observer/lock`

Get distributed lock system status.

```bash
curl http://localhost:1933/api/v1/observer/lock -H "X-API-Key: your-key"
```

### 11.5 GET `/api/v1/observer/retrieval`

Get retrieval quality metrics.

```bash
curl http://localhost:1933/api/v1/observer/retrieval -H "X-API-Key: your-key"
```

### 11.6 GET `/api/v1/observer/system`

Get overall system status aggregating all components (queue, vikingdb, models, lock, retrieval).

```bash
curl http://localhost:1933/api/v1/observer/system -H "X-API-Key: your-key"
```

---

## 12. Debug

### 12.1 GET `/api/v1/debug/health`

Quick health check.

### 12.2 GET `/api/v1/debug/vector/scroll`

Paginated vector record inspection.

### 12.3 GET `/api/v1/debug/vector/count`

Count vector records.

---

## 13. Maintenance

### 13.1 POST `/api/v1/maintenance/reindex`

Reindex content (optional abstract regeneration). Permission: ROOT/ADMIN.

---

## 14. Statistics

### 14.1 GET `/api/v1/stats/memories`

Get memory health statistics. Supports category filtering.

### 14.2 GET `/api/v1/stats/sessions/{session_id}`

Get session extraction statistics.

---

## 15. Admin (Multi-tenant)

All admin endpoints require ROOT or ADMIN permissions. See roles table below.

### Roles and Permissions

| Operation               | ROOT | ADMIN           | USER |
| ----------------------- | ---- | --------------- | ---- |
| Create/delete workspace | Y    | N               | N    |
| List workspaces         | Y    | N               | N    |
| Register/remove users   | Y    | Y (own account) | N    |
| List agents             | Y    | Y (own account) | N    |
| Regenerate user key     | Y    | Y (own account) | N    |
| Change user role        | Y    | N               | N    |

---

### 15.1 POST `/api/v1/admin/accounts`

Create a new workspace with first admin user. Permission: ROOT.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| account_id | str | Yes | - | Workspace ID |
| admin_user_id | str | Yes | - | First admin user ID |
| isolate_user_scope_by_agent | bool | No | false | Further isolate user scope by agent |
| isolate_agent_scope_by_user | bool | No | false | Further isolate agent scope by user |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/admin/accounts \
  -H "Content-Type: application/json" \
  -H "X-API-Key: <root-key>" \
  -d '{"account_id": "acme", "admin_user_id": "alice"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "account_id": "acme",
    "admin_user_id": "alice",
    "user_key": "7f3a9c1e..."
  },
  "time": 0.1
}
```

In `trusted` mode, `user_key` is omitted.

---

### 15.2 GET `/api/v1/admin/accounts`

List all workspaces. Permission: ROOT.

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/admin/accounts \
  -H "X-API-Key: <root-key>"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    {
      "account_id": "default",
      "created_at": "2026-02-12T10:00:00Z",
      "user_count": 1
    }
  ],
  "time": 0.1
}
```

---

### 15.3 DELETE `/api/v1/admin/accounts/{account_id}`

Delete a workspace and all associated data (cascade cleanup). Permission: ROOT. Irreversible.

**curl**:

```bash
curl -X DELETE http://localhost:1933/api/v1/admin/accounts/acme \
  -H "X-API-Key: <root-key>"
```

**Response**:

```json
{ "status": "ok", "result": { "deleted": true }, "time": 0.1 }
```

---

### 15.4 POST `/api/v1/admin/accounts/{account_id}/users`

Register a new user in a workspace. Permission: ROOT or ADMIN (own account).

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| user_id | str | Yes | - | User ID |
| role | str | No | "user" | Role: `"admin"` or `"user"` |

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/admin/accounts/acme/users \
  -H "Content-Type: application/json" \
  -H "X-API-Key: <root-or-admin-key>" \
  -d '{"user_id": "bob", "role": "user"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": {
    "account_id": "acme",
    "user_id": "bob",
    "user_key": "d91f5b2a..."
  },
  "time": 0.1
}
```

---

### 15.5 GET `/api/v1/admin/accounts/{account_id}/users`

List users in a workspace. Permission: ROOT or ADMIN.

**Query Parameters**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| limit | int | No | 100 | Max users to return |
| name | str | No | null | Filter by user ID (prefix match) |
| role | str | No | null | Filter by role |

**curl**:

```bash
curl -X GET "http://localhost:1933/api/v1/admin/accounts/acme/users?role=admin&limit=50" \
  -H "X-API-Key: <root-or-admin-key>"
```

**Response**:

```json
{
  "status": "ok",
  "result": [
    { "user_id": "alice", "role": "admin" },
    { "user_id": "bob", "role": "user" }
  ],
  "time": 0.1
}
```

---

### 15.6 GET `/api/v1/admin/accounts/{account_id}/agents`

List agent namespaces under a workspace. Permission: ROOT or ADMIN.

**curl**:

```bash
curl -X GET http://localhost:1933/api/v1/admin/accounts/acme/agents \
  -H "X-API-Key: <root-or-admin-key>"
```

**Response**:

```json
{
  "status": "ok",
  "result": [{ "agent_id": "default", "uri": "viking://agent/default" }],
  "time": 0.1
}
```

---

### 15.7 DELETE `/api/v1/admin/accounts/{account_id}/users/{user_id}`

Remove a user from a workspace. API key deleted immediately. Permission: ROOT or ADMIN.

**curl**:

```bash
curl -X DELETE http://localhost:1933/api/v1/admin/accounts/acme/users/bob \
  -H "X-API-Key: <root-or-admin-key>"
```

**Response**:

```json
{ "status": "ok", "result": { "deleted": true }, "time": 0.1 }
```

---

### 15.8 PUT `/api/v1/admin/accounts/{account_id}/users/{user_id}/role`

Change a user's role. Permission: ROOT only.

**Request Body**:
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| role | str | Yes | - | New role: `"admin"`, `"user"`, or `"root"` |

**curl**:

```bash
curl -X PUT http://localhost:1933/api/v1/admin/accounts/acme/users/bob/role \
  -H "Content-Type: application/json" \
  -H "X-API-Key: <root-key>" \
  -d '{"role": "admin"}'
```

**Response**:

```json
{
  "status": "ok",
  "result": { "account_id": "acme", "user_id": "bob", "role": "admin" },
  "time": 0.1
}
```

---

### 15.9 POST `/api/v1/admin/accounts/{account_id}/users/{user_id}/key`

Regenerate a user's API key. Old key immediately invalidated. Permission: ROOT or ADMIN.

**curl**:

```bash
curl -X POST http://localhost:1933/api/v1/admin/accounts/acme/users/bob/key \
  -H "Content-Type: application/json" \
  -H "X-API-Key: <root-or-admin-key>"
```

**Response**:

```json
{ "status": "ok", "result": { "user_key": "e82d4e0f..." }, "time": 0.1 }
```

---

## 16. VikingBot (Optional)

Requires server started with `--with-bot` option.

### 16.1 GET `/health`

Health check (shared with system /health).

### 16.2 POST `/chat`

Send message to bot.

### 16.3 POST `/chat/stream`

Bot streaming response.

---

## 17. WebDAV

Exposes a minimal WebDAV adapter for resource files at `/webdav/resources`.

### 17.1 OPTIONS `/webdav/resources` or `/webdav/resources/{path}`

WebDAV options query.

### 17.2 PROPFIND `/webdav/resources` or `/webdav/resources/{path}`

WebDAV property query.

### 17.3 GET / HEAD `/webdav/resources/{path}`

Read file via WebDAV.

**Notes**:

- Phase 1 supports: `OPTIONS`, `PROPFIND`, `GET`, `HEAD`, `PUT`, `DELETE`, `MKCOL`, `MOVE`
- Resources only. Memories, skills, sessions are not exposed.
- Text-first writes. `PUT` accepts UTF-8 text only.
- Derived files (`.abstract.md`, `.overview.md`, `.relations.json`, `.path.ovlock`) are hidden.

---

## 18. Metrics

### 18.1 GET `/metrics`

Export Prometheus metrics for the current process. Returns `text/plain; version=0.0.4; charset=utf-8` (Prometheus exposition text), not standard JSON.

**Auth**: Effectively public (not wired to auth dependencies in current implementation).

**curl**:

```bash
curl http://localhost:1933/metrics
```

**Response** (Prometheus text format):

```text
# HELP openviking_http_requests_total Total number of HTTP requests
# TYPE openviking_http_requests_total counter
openviking_http_requests_total{method="GET",route="/api/v1/system/status",status="200"} 12
```

Returns `404` with body `"Prometheus metrics are disabled."` if metrics are disabled.

---

## Appendix: Error Codes

| Code                  | HTTP Status | Description                                                 |
| --------------------- | ----------- | ----------------------------------------------------------- |
| `OK`                  | 200         | Success                                                     |
| `INVALID_ARGUMENT`    | 400         | Invalid parameter                                           |
| `INVALID_URI`         | 400         | Invalid Viking URI format                                   |
| `NOT_FOUND`           | 404         | Resource not found                                          |
| `ALREADY_EXISTS`      | 409         | Resource already exists                                     |
| `UNAUTHENTICATED`     | 401         | Missing or invalid API key                                  |
| `PERMISSION_DENIED`   | 403         | Insufficient permissions                                    |
| `RESOURCE_EXHAUSTED`  | 429         | Rate limit exceeded                                         |
| `FAILED_PRECONDITION` | 412         | Precondition failed                                         |
| `CONFLICT`            | 409         | Operation conflicts with in-progress task or existing state |
| `DEADLINE_EXCEEDED`   | 504         | Operation timed out                                         |
| `UNAVAILABLE`         | 503         | Service unavailable                                         |
| `PROCESSING_ERROR`    | 500         | Resource or semantic processing failed                      |
| `INTERNAL`            | 500         | Internal server error                                       |
| `UNIMPLEMENTED`       | 501         | Feature not implemented                                     |
| `EMBEDDING_FAILED`    | 500         | Embedding generation failed                                 |
| `VLM_FAILED`          | 500         | VLM call failed                                             |
| `SESSION_EXPIRED`     | 410         | Session no longer exists                                    |
| `NOT_INITIALIZED`     | -           | Service not initialized (need `initialize()` first)         |

---

## Auth Methods

1. **Authorization Bearer**: `Authorization: Bearer ***` (recommended)
2. **X-API-Key header**: `X-API-Key: your-key`
3. If the server has no API Key configured, authentication is skipped.
4. `/health` and `/ready` never require authentication.
5. For multi-tenant: `X-OpenViking-Account` and `X-OpenViking-User` headers scope requests.

---

_Compiled from OpenViking API documentation. See https://github.com/volcengine/OpenViking for full details._
