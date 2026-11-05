# Build upload API

API for receiving app build artifacts from GitHub CI/CD. Metadata is stored in
PostgreSQL (managed by `sqlx` migrations in `migrations/`), the artifacts
themselves are stored in S3 (works with any S3-compatible storage: AWS S3,
Cloudflare R2, MinIO, Backblaze B2, ...).

## Endpoints

### `POST /api/builds`

Authenticated multipart upload of one build artifact. Multipart fields:

| Field       | Required | Constraints                                             |
| ----------- | -------- | ------------------------------------------------------- |
| `file`      | yes      | the artifact; filename becomes part of the storage key  |
| `platform`  | yes      | 1-32 chars, ASCII letters/digits/`-`/`_` (e.g. `windows`, `macos`, `linux`) |
| `version`   | yes      | 1-64 chars, letters/digits and `. _ - + ~` (e.g. `0.2.1`) |
| `channel`   | no       | default `stable`; same charset as `platform`            |
| `commit_sha`| no       | up to 64 chars                                          |
| `run_id`    | no       | up to 64 chars                                          |

Auth: `Authorization: Bearer <UPLOAD_TOKEN>` (constant-time comparison).

Responses:

- `200` — JSON object with the stored row: `id`, `platform`, `version`,
  `channel`, `filename`, `size_bytes`, `sha256`, `s3_key`, `content_type`,
  `commit_sha`, `ci_run_id`, `created_at` (RFC3339 UTC).
- `400` invalid fields/body · `401` bad token · `413` over `MAX_UPLOAD_MB` ·
  `502` S3 upload failed · `500` database failure.

The server streams the file to a temp file (computing SHA-256 while streaming),
so memory usage stays flat regardless of artifact size. The object is stored at
`builds/{channel}/{platform}/{version}/{filename}`.

### `GET /api/builds/latest/{platform}?channel=stable`

Public lookup of the most recent build for a platform and channel. Returns the
same JSON fields as above plus `download_url` (a presigned S3 GET URL valid for
`PRESIGN_EXPIRES_SECS`) and `url_expires_in_secs`. `404` when there is no build.

## Configuration

| Env var                 | Required | Default | Purpose                                    |
| ----------------------- | -------- | ------- | ------------------------------------------ |
| `DATABASE_URL`          | yes      | —       | Postgres connection string                 |
| `UPLOAD_TOKEN`          | yes      | —       | Bearer token expected from CI              |
| `S3_BUCKET`             | yes      | —       | Bucket for build artifacts                 |
| `AWS_REGION`            | no       | SDK default | AWS region                             |
| `AWS_ACCESS_KEY_ID`     | no       | —       | AWS credentials (standard SDK variables)   |
| `AWS_SECRET_ACCESS_KEY` | no       | —       | AWS credentials (standard SDK variables)   |
| `AWS_ENDPOINT_URL`      | no       | —       | Custom endpoint for MinIO / R2 / other S3-compatible storage |
| `MAX_UPLOAD_MB`         | no       | `500`   | Upload size limit in MB                    |
| `PRESIGN_EXPIRES_SECS`  | no       | `3600`  | Lifetime of presigned download URLs        |

Migrations run automatically at server startup. The API is optional: without
its env vars the site serves normally, one ERROR line notes that the API is
disabled, and the endpoints answer `503`. To enable it, export the variables
in the same shell that runs `dx serve` (or your process manager).

`docker-compose.dev.yml` starts local Postgres and RustFS (S3-compatible)
containers for development: `docker compose -f docker-compose.dev.yml up -d`,
then export

```sh
export DATABASE_URL=postgres://companion:companion@localhost:5433/companion_builds
export UPLOAD_TOKEN=dev-token
export S3_BUCKET=companion-builds
export AWS_ACCESS_KEY_ID=companion
export AWS_SECRET_ACCESS_KEY=companion-secret
export AWS_ENDPOINT_URL=http://localhost:8333
export AWS_REGION=us-east-1
```

The bucket is created automatically when it is missing, and path-style
addressing is enabled automatically whenever `AWS_ENDPOINT_URL` is set.

## Usage from GitHub Actions

In the **app repository's** workflow (the repo that produces the builds):

```yaml
      - name: Upload build to landing page
        env:
          UPLOAD_TOKEN: ${{ secrets.UPLOAD_TOKEN }}
        run: |
          curl --fail-with-body -X POST "https://<your-site>/api/builds" \
            -H "Authorization: Bearer $UPLOAD_TOKEN" \
            -F "platform=windows" \
            -F "version=${GITHUB_REF_NAME}" \
            -F "commit_sha=${GITHUB_SHA}" \
            -F "run_id=${GITHUB_RUN_ID}" \
            -F "file=@dist/Companion-Setup.exe"
```

Store the token in the app repository's Actions secrets as `UPLOAD_TOKEN`.
Repeat the step per artifact with a different `platform`.

Quick local check with curl:

```sh
curl --fail-with-body -X POST "http://127.0.0.1:8080/api/builds" \
  -H "Authorization: Bearer $UPLOAD_TOKEN" \
  -F "platform=windows" -F "version=0.2.1" \
  -F "file=@./companion-setup.exe"

curl "http://127.0.0.1:8080/api/builds/latest/windows"
```
