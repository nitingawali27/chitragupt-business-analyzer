# Flow 04 — Document Ingestion Pipeline

## What this covers

When a BA uploads a document at any checkpoint, it passes through a processing pipeline before its content becomes searchable. PII scrubbing always happens before embedding — never after. Every chunk is tagged with the tenant and project IDs to enforce strict data isolation.

**Key rule (INV-SEC-02):** PII is scrubbed before any embedding call. The order is fixed — scrub then embed, never the reverse.

## Important Components

| Component | Role |
|---|---|
| Object Storage | Raw files stored at `s3://{tenant_id}/{project_id}/` |
| PII Scrubber | Redacts personal identifiers before any embedding |
| Chunker | Splits documents into 512-token overlapping windows |
| Embedding Model | Converts text to dense 1536-dimension vectors |
| BM25 Sparse Generator | Produces keyword-match vectors alongside dense embeddings |
| pgvector | Stores both dense and sparse vectors with metadata filters |
| `UPLOAD_COMPLETE` event | Fires after successful indexing; triggers AC re-evaluation |

## Supported File Types

`pdf` · `docx` · `txt` · `md` · `xlsx` · `image` · `audio` · `confluence_url` · `jira_epic` · `notion_page` · `web_url`

## Input → Process → Output

- **Input:** Raw uploaded file at any checkpoint (A, B, C, or D)
- **Process:** Store → record metadata → scrub PII → chunk → embed → index → notify
- **Output:** Searchable chunks in pgvector; upload acceptance criteria re-evaluated by GapAnalyzer

## Diagram

```mermaid
flowchart TD
    %% PII scrub always runs before embedding — this order is non-negotiable

    UPLOAD([Document Uploaded\nat Checkpoint A B C or D]) --> STORE[Store Raw File\nObject Storage: s3 tenant and project path]

    STORE --> META[Record Document Metadata\nPostgreSQL · status = processing]

    META --> PII[PII Scrub\nRedact names · emails · IDs before embedding]

    PII --> CHUNK[Split into Chunks\n512-token windows with overlap]

    CHUNK --> EMBED[Generate Dense Embeddings\nEmbedding model · 1536 dimensions]

    EMBED --> SPARSE[Generate Sparse Vectors\nBM25 for keyword search]

    SPARSE --> INDEX[Index to pgvector\nWith tenant · project · trust tier metadata]

    INDEX --> STATUS[Update Document Status\nstatus = indexed]

    STATUS --> EVENT([Fire UPLOAD_COMPLETE Event])

    EVENT --> REEVAL[GapAnalyzer Re-evaluates\nAll upload AC for current state]
```

## Chunk Metadata Stored per Vector

Every chunk stored in pgvector carries this metadata envelope for filtered retrieval:

```
chunk_id       · document_id   · project_id    · tenant_id
source_type    · trust_tier    · page_number   · section_title
valid_from     · valid_until   · is_active     · confidence_modifier
```

**Mandatory filters applied to every retrieval query:**
- `tenant_id == {current_tenant_id}`
- `project_id == {current_project_id}`
- `is_active == true`
- `valid_until IS NULL OR valid_until > NOW()`

---

> Chitragupt · Flow 04 of 11 · May 2026
