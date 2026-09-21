//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! The registry client template (`schema-registry.yml`, the Java
//! `schema-registry.properties` twin) and the authentication it configures.
//!
//! The Java module passes the template verbatim to the Confluent client, so
//! any Confluent parameter works there. This client has no Confluent library
//! to pass it to, so the template is **interpreted**: the keys below are
//! honoured with the same names and values as the Confluent client; any other
//! key is logged and ignored (never silently), and TLS trust comes from the OS
//! trust store that the platform's HTTP client already uses.
//!
//! | Key | Meaning |
//! |-----|---------|
//! | `bearer.auth.credentials.source` | `OAUTHBEARER` (client credentials), `STATIC_TOKEN`, `SASL_OAUTHBEARER_INHERIT` (reuse the Kafka client template's `sasl.oauthbearer.*`) |
//! | `bearer.auth.issuer.endpoint.url`, `bearer.auth.client.id`, `bearer.auth.client.secret`, `bearer.auth.scope` | the client-credentials grant |
//! | `bearer.auth.token` | the fixed token for `STATIC_TOKEN` |
//! | `bearer.auth.cache.expiry.buffer.seconds` | refresh this long before the token expires (default 300) |
//! | `bearer.auth.logical.cluster`, `bearer.auth.identity.pool.id` | Confluent Cloud headers (`target-sr-cluster`, `Confluent-Identity-Pool-Id`) |
//! | `basic.auth.credentials.source` | `USER_INFO` (`basic.auth.user.info` = `user:password`), `URL` (credentials in the registry URL), `SASL_INHERIT` (the Kafka template's `sasl.username` / `sasl.password`) |
//!
//! The bearer token is fetched with the client-credentials grant exactly as
//! Kafka's own retriever does — the client id and secret as HTTP Basic on the
//! token request, `grant_type=client_credentials` (+ `scope`) in the form body
//! — sent as `Authorization: Bearer` on every registry call, cached, and
//! refreshed shortly before expiry.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use base64::Engine;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{AppConfigReader, AppError, ConfigReader, Platform};
use tokio::sync::Mutex;

use crate::client_config;

use super::DEFAULT_KEY_PREFIX;

/// The library's compiled-in default template — all comments, so an
/// unauthenticated registry (the local mock) works with zero configuration.
const EMBEDDED_TEMPLATE: &str = include_str!("../../resources/schema-registry.yml");
const DEFAULT_EXPIRY_BUFFER_SECONDS: u64 = 300;
const DEFAULT_TOKEN_LIFETIME_SECONDS: u64 = 3600;
const TOKEN_CALL_TIMEOUT_SECONDS: u64 = 10;

const BEARER_SOURCE: &str = "bearer.auth.credentials.source";
const BEARER_ISSUER_URL: &str = "bearer.auth.issuer.endpoint.url";
const BEARER_CLIENT_ID: &str = "bearer.auth.client.id";
const BEARER_CLIENT_SECRET: &str = "bearer.auth.client.secret";
const BEARER_SCOPE: &str = "bearer.auth.scope";
const BEARER_TOKEN: &str = "bearer.auth.token";
const BEARER_EXPIRY_BUFFER: &str = "bearer.auth.cache.expiry.buffer.seconds";
const BEARER_LOGICAL_CLUSTER: &str = "bearer.auth.logical.cluster";
const BEARER_IDENTITY_POOL: &str = "bearer.auth.identity.pool.id";
const BASIC_SOURCE: &str = "basic.auth.credentials.source";
const BASIC_USER_INFO: &str = "basic.auth.user.info";
/// A serde setting the template may carry because the serdes inherit it
/// (Java parity); read by the codec, accepted silently here.
const STRICT_JSON: &str = "json.fail.invalid.schema";

const KNOWN_KEYS: [&str; 13] = [
    BEARER_SOURCE,
    BEARER_ISSUER_URL,
    BEARER_CLIENT_ID,
    BEARER_CLIENT_SECRET,
    BEARER_SCOPE,
    BEARER_TOKEN,
    BEARER_EXPIRY_BUFFER,
    BEARER_LOGICAL_CLUSTER,
    BEARER_IDENTITY_POOL,
    BASIC_SOURCE,
    BASIC_USER_INFO,
    STRICT_JSON,
    "schema.registry.url",
];

// the Kafka client template keys reused by the two *_INHERIT sources
const SASL_OAUTH_TOKEN_URL: &str = "sasl.oauthbearer.token.endpoint.url";
const SASL_OAUTH_CLIENT_ID: &str = "sasl.oauthbearer.client.id";
const SASL_OAUTH_CLIENT_SECRET: &str = "sasl.oauthbearer.client.secret";
const SASL_OAUTH_SCOPE: &str = "sasl.oauthbearer.scope";
const SASL_USERNAME: &str = "sasl.username";
const SASL_PASSWORD: &str = "sasl.password";

/// Load the registry client template for a key prefix: the configured
/// location(s) when the application set `<prefix>.properties` (exactly
/// those, failing loudly when none exists), else the default classpath chain
/// with the compiled-in template as the final fallback for the default
/// prefix. Blank values (an unresolved `${VAR:}`) are dropped, so a
/// commented-out block that resolves to nothing is the same as absent.
pub fn load_template(
    config: &AppConfigReader,
    key_prefix: &str,
) -> Result<BTreeMap<String, String>, AppError> {
    let location_key = format!("{key_prefix}.properties");
    let file_stem = key_prefix.replace('.', "-");
    let default_locations =
        format!("classpath:/{file_stem}.yml, classpath:/{file_stem}.properties");
    let configured = config.get_property_or(&location_key, "");
    let reader = if configured.trim().is_empty() {
        match client_config::load_first(&default_locations) {
            Some(reader) => Some(reader),
            None if key_prefix == DEFAULT_KEY_PREFIX => Some(
                ConfigReader::from_yaml_text(EMBEDDED_TEMPLATE).map_err(|e| {
                    AppError::new(
                        500,
                        format!("Bundled schema registry template invalid - {e}"),
                    )
                })?,
            ),
            None => None,
        }
    } else {
        Some(client_config::load_first(&configured).ok_or_else(|| {
            AppError::new(
                500,
                format!("No schema registry client config found at any of: {configured}"),
            )
        })?)
    };
    let mut template = BTreeMap::new();
    if let Some(reader) = reader {
        for (key, value) in reader.get_composite_key_values() {
            let text = value.to_display_string().trim().to_string();
            if !text.is_empty() {
                template.insert(key.clone(), text);
            }
        }
    }
    Ok(template)
}

/// A base URL split the way the platform's HTTP client takes it: the
/// `scheme://host[:port]` target and a path prefix (no trailing slash).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpTarget {
    pub host: String,
    pub path: String,
}

impl HttpTarget {
    /// Parse `scheme://[user:password@]host[:port][/prefix]`; the user info,
    /// when present, is returned separately (and never logged).
    pub fn parse(url: &str) -> Result<(HttpTarget, Option<(String, String)>), AppError> {
        let url = url.trim();
        let Some((scheme, rest)) = url.split_once("://") else {
            return Err(AppError::new(
                500,
                format!("'{url}' is not an http(s) URL (expected scheme://host[:port][/path])"),
            ));
        };
        if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
            return Err(AppError::new(
                500,
                format!("'{url}' is not an http(s) URL (scheme '{scheme}')"),
            ));
        }
        let (authority, path) = match rest.find('/') {
            Some(slash) => (&rest[..slash], rest[slash..].trim_end_matches('/')),
            None => (rest, ""),
        };
        let (userinfo, host) = match authority.rsplit_once('@') {
            Some((userinfo, host)) => (Some(userinfo), host),
            None => (None, authority),
        };
        if host.is_empty() {
            return Err(AppError::new(500, format!("'{url}' has no host")));
        }
        let credentials = userinfo.map(|info| match info.split_once(':') {
            Some((user, password)) => (percent_decode(user), percent_decode(password)),
            None => (percent_decode(info), String::new()),
        });
        Ok((
            HttpTarget {
                host: format!("{}://{host}", scheme.to_ascii_lowercase()),
                path: path.to_string(),
            },
            credentials,
        ))
    }

    /// The request path for an endpoint under this target's prefix.
    pub fn join(&self, endpoint: &str) -> String {
        format!("{}{endpoint}", self.path)
    }

    /// The target rendered as a URL (for messages; never carries credentials).
    pub fn display(&self) -> String {
        format!("{}{}", self.host, self.path)
    }
}

/// The OAuth 2.0 client-credentials grant.
#[derive(Clone, Debug)]
pub struct OAuthClient {
    pub token_endpoint: HttpTarget,
    pub client_id: String,
    client_secret: String,
    pub scope: Option<String>,
    pub expiry_buffer: Duration,
}

impl OAuthClient {
    /// `Authorization: Basic` for the token request (client id + secret).
    fn basic_header(&self) -> String {
        basic_header(&self.client_id, &self.client_secret)
    }

    /// The form body of the token request.
    fn form_body(&self) -> String {
        match &self.scope {
            Some(scope) if !scope.is_empty() => {
                format!("grant_type=client_credentials&scope={}", form_encode(scope))
            }
            _ => "grant_type=client_credentials".to_string(),
        }
    }
}

/// How registry requests authenticate.
#[derive(Clone, Debug)]
pub enum RegistryAuth {
    /// No `Authorization` header (the local mock, an unauthenticated registry).
    None,
    /// `Authorization: Basic <base64 user:password>`.
    Basic {
        header: String,
        source: &'static str,
    },
    /// `Authorization: Bearer <fixed token>`.
    StaticBearer(String),
    /// `Authorization: Bearer <token>` from the client-credentials grant,
    /// cached and refreshed before expiry.
    OAuth {
        client: OAuthClient,
        source: &'static str,
    },
}

impl RegistryAuth {
    /// The credentials source name for the startup log.
    pub fn label(&self) -> &'static str {
        match self {
            RegistryAuth::None => "none",
            RegistryAuth::Basic { source, .. } => source,
            RegistryAuth::StaticBearer(_) => "STATIC_TOKEN",
            RegistryAuth::OAuth { source, .. } => source,
        }
    }

    /// The `Authorization` header value for the next registry call, fetching
    /// or refreshing the bearer token when the grant is in use.
    pub(crate) async fn header(&self, tokens: &TokenCache) -> Result<Option<String>, AppError> {
        match self {
            RegistryAuth::None => Ok(None),
            RegistryAuth::Basic { header, .. } => Ok(Some(header.clone())),
            RegistryAuth::StaticBearer(token) => Ok(Some(format!("Bearer {token}"))),
            RegistryAuth::OAuth { client, .. } => Ok(Some(format!(
                "Bearer {}",
                tokens.bearer_token(client).await?
            ))),
        }
    }
}

/// Everything the client needs to reach one registry.
#[derive(Clone, Debug)]
pub struct RegistrySettings {
    pub registry: HttpTarget,
    pub auth: RegistryAuth,
    /// Installation headers sent on every registry call.
    pub extra_headers: Vec<(String, String)>,
}

impl RegistrySettings {
    /// Interpret the template for a registry URL (see the module docs).
    pub fn from_template(
        template: &BTreeMap<String, String>,
        registry_url: &str,
    ) -> Result<RegistrySettings, AppError> {
        let (registry, url_credentials) = HttpTarget::parse(registry_url)?;
        for key in template.keys() {
            if !KNOWN_KEYS.contains(&key.as_str()) {
                if key.contains("ssl.") {
                    log::warn!(
                        "Schema registry template key '{key}' is ignored - this client trusts the \
                         OS trust store (add a private CA there); the key has no analog here"
                    );
                } else {
                    log::warn!(
                        "Schema registry template key '{key}' has no analog in this client and is \
                         ignored"
                    );
                }
            }
        }
        let get = |key: &str| {
            template
                .get(key)
                .map(String::as_str)
                .filter(|v| !v.is_empty())
        };
        let bearer_source = get(BEARER_SOURCE).map(str::to_ascii_uppercase);
        let basic_source = get(BASIC_SOURCE).map(str::to_ascii_uppercase);
        let auth = match (bearer_source.as_deref(), basic_source.as_deref()) {
            (Some(bearer), basic) => {
                if basic.is_some() {
                    log::warn!(
                        "{BEARER_SOURCE} and {BASIC_SOURCE} are both set - bearer authentication \
                         is used for the Schema Registry"
                    );
                }
                match bearer {
                    "OAUTHBEARER" => RegistryAuth::OAuth {
                        client: oauth_client(
                            get(BEARER_ISSUER_URL),
                            get(BEARER_CLIENT_ID),
                            get(BEARER_CLIENT_SECRET),
                            get(BEARER_SCOPE),
                            get(BEARER_EXPIRY_BUFFER),
                            "OAUTHBEARER",
                            &[BEARER_ISSUER_URL, BEARER_CLIENT_ID, BEARER_CLIENT_SECRET],
                        )?,
                        source: "OAUTHBEARER",
                    },
                    "STATIC_TOKEN" => RegistryAuth::StaticBearer(
                        get(BEARER_TOKEN)
                            .ok_or_else(|| {
                                AppError::new(
                                    500,
                                    format!("{BEARER_SOURCE}=STATIC_TOKEN requires {BEARER_TOKEN}"),
                                )
                            })?
                            .to_string(),
                    ),
                    "SASL_OAUTHBEARER_INHERIT" => {
                        let kafka = client_config::kafka_template_values(&[
                            SASL_OAUTH_TOKEN_URL,
                            SASL_OAUTH_CLIENT_ID,
                            SASL_OAUTH_CLIENT_SECRET,
                            SASL_OAUTH_SCOPE,
                        ])?;
                        let kafka_get = |key: &str| kafka.get(key).map(String::as_str);
                        RegistryAuth::OAuth {
                            client: oauth_client(
                                kafka_get(SASL_OAUTH_TOKEN_URL),
                                kafka_get(SASL_OAUTH_CLIENT_ID),
                                kafka_get(SASL_OAUTH_CLIENT_SECRET),
                                kafka_get(SASL_OAUTH_SCOPE).or(get(BEARER_SCOPE)),
                                get(BEARER_EXPIRY_BUFFER),
                                "SASL_OAUTHBEARER_INHERIT",
                                &[
                                    SASL_OAUTH_TOKEN_URL,
                                    SASL_OAUTH_CLIENT_ID,
                                    SASL_OAUTH_CLIENT_SECRET,
                                ],
                            )?,
                            source: "SASL_OAUTHBEARER_INHERIT",
                        }
                    }
                    other => {
                        return Err(AppError::new(
                            500,
                            format!(
                                "{BEARER_SOURCE}={other} is not supported by this client (use \
                                 OAUTHBEARER, STATIC_TOKEN or SASL_OAUTHBEARER_INHERIT)"
                            ),
                        ))
                    }
                }
            }
            (None, Some("USER_INFO")) => {
                let info = get(BASIC_USER_INFO).ok_or_else(|| {
                    AppError::new(
                        500,
                        format!(
                            "{BASIC_SOURCE}=USER_INFO requires {BASIC_USER_INFO} (user:password)"
                        ),
                    )
                })?;
                let (user, password) = info.split_once(':').ok_or_else(|| {
                    AppError::new(500, format!("{BASIC_USER_INFO} must be 'user:password'"))
                })?;
                RegistryAuth::Basic {
                    header: basic_header(user, password),
                    source: "USER_INFO",
                }
            }
            (None, Some("URL")) => match &url_credentials {
                Some((user, password)) => RegistryAuth::Basic {
                    header: basic_header(user, password),
                    source: "URL",
                },
                None => {
                    return Err(AppError::new(
                        500,
                        format!("{BASIC_SOURCE}=URL but the registry URL carries no user:password"),
                    ))
                }
            },
            (None, Some("SASL_INHERIT")) => {
                let kafka = client_config::kafka_template_values(&[SASL_USERNAME, SASL_PASSWORD])?;
                match (kafka.get(SASL_USERNAME), kafka.get(SASL_PASSWORD)) {
                    (Some(user), Some(password)) => RegistryAuth::Basic {
                        header: basic_header(user, password),
                        source: "SASL_INHERIT",
                    },
                    _ => {
                        return Err(AppError::new(
                            500,
                            format!(
                                "{BASIC_SOURCE}=SASL_INHERIT requires {SASL_USERNAME} and \
                                 {SASL_PASSWORD} in the Kafka client template"
                            ),
                        ))
                    }
                }
            }
            (None, Some(other)) => {
                return Err(AppError::new(
                    500,
                    format!(
                        "{BASIC_SOURCE}={other} is not supported by this client (use USER_INFO, \
                         URL or SASL_INHERIT)"
                    ),
                ))
            }
            (None, None) => match &url_credentials {
                // Confluent's default basic source is URL: credentials in the
                // registry URL authenticate without a template entry
                Some((user, password)) => RegistryAuth::Basic {
                    header: basic_header(user, password),
                    source: "URL",
                },
                None => RegistryAuth::None,
            },
        };
        let mut extra_headers = Vec::new();
        if let Some(cluster) = get(BEARER_LOGICAL_CLUSTER) {
            extra_headers.push(("target-sr-cluster".to_string(), cluster.to_string()));
        }
        if let Some(pool) = get(BEARER_IDENTITY_POOL) {
            extra_headers.push(("Confluent-Identity-Pool-Id".to_string(), pool.to_string()));
        }
        Ok(RegistrySettings {
            registry,
            auth,
            extra_headers,
        })
    }

    /// The credentials source name for the startup log.
    pub fn auth_label(&self) -> &'static str {
        self.auth.label()
    }
}

#[allow(clippy::too_many_arguments)]
fn oauth_client(
    issuer_url: Option<&str>,
    client_id: Option<&str>,
    client_secret: Option<&str>,
    scope: Option<&str>,
    expiry_buffer: Option<&str>,
    source: &str,
    required: &[&str],
) -> Result<OAuthClient, AppError> {
    let (Some(issuer_url), Some(client_id), Some(client_secret)) =
        (issuer_url, client_id, client_secret)
    else {
        return Err(AppError::new(
            500,
            format!("{BEARER_SOURCE}={source} requires {}", required.join(", ")),
        ));
    };
    let (token_endpoint, _) = HttpTarget::parse(issuer_url)?;
    let buffer_seconds = match expiry_buffer {
        None => DEFAULT_EXPIRY_BUFFER_SECONDS,
        Some(text) => text.trim().parse::<u64>().map_err(|_| {
            AppError::new(
                500,
                format!("{BEARER_EXPIRY_BUFFER} must be a number of seconds, got '{text}'"),
            )
        })?,
    };
    Ok(OAuthClient {
        token_endpoint,
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        scope: scope.map(str::to_string),
        expiry_buffer: Duration::from_secs(buffer_seconds),
    })
}

fn basic_header(user: &str, password: &str) -> String {
    format!(
        "Basic {}",
        base64::engine::general_purpose::STANDARD.encode(format!("{user}:{password}"))
    )
}

fn form_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(value) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(value);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

/// A bearer token with its refresh deadline.
struct CachedToken {
    token: String,
    refresh_at: Instant,
}

/// The per-client bearer-token cache: one fetch at a time (concurrent callers
/// wait for it and reuse the result), refreshed `expiry_buffer` before the
/// token expires.
#[derive(Default)]
pub struct TokenCache {
    slot: Mutex<Option<CachedToken>>,
}

impl TokenCache {
    /// The cached token, or a fresh one from the token endpoint.
    pub(crate) async fn bearer_token(&self, client: &OAuthClient) -> Result<String, AppError> {
        let mut slot = self.slot.lock().await;
        if let Some(cached) = slot.as_ref() {
            if Instant::now() < cached.refresh_at {
                return Ok(cached.token.clone());
            }
        }
        let (token, lifetime) = fetch_token(client).await?;
        let refresh_at = Instant::now() + lifetime.saturating_sub(client.expiry_buffer);
        *slot = Some(CachedToken {
            token: token.clone(),
            refresh_at,
        });
        Ok(token)
    }
}

/// The client-credentials token request: `access_token` and its lifetime
/// (`expires_in`, else the JWT `exp` claim, else one hour).
async fn fetch_token(client: &OAuthClient) -> Result<(String, Duration), AppError> {
    let platform = Platform::get_instance();
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&client.token_endpoint.host)
        .set_url(&client.token_endpoint.join(""))
        .set_header("authorization", &client.basic_header())
        .set_header("content-type", "application/x-www-form-urlencoded")
        .set_header("accept", "application/json")
        .set_body(rmpv::Value::from(client.form_body()))
        .set_timeout_seconds(TOKEN_CALL_TIMEOUT_SECONDS);
    let (status, body) = super::registry::call(&platform, request).await?;
    if !(200..300).contains(&status) {
        let detail = body
            .get("error")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| body.to_string());
        return Err(AppError::new(
            500,
            format!(
                "OAuth token request to {} failed with status {status} - {detail}",
                client.token_endpoint.display()
            ),
        ));
    }
    let token = body
        .get("access_token")
        .and_then(serde_json::Value::as_str)
        .filter(|t| !t.is_empty())
        .ok_or_else(|| {
            AppError::new(
                500,
                format!(
                    "OAuth token response from {} carries no access_token",
                    client.token_endpoint.display()
                ),
            )
        })?;
    let lifetime = body
        .get("expires_in")
        .and_then(serde_json::Value::as_u64)
        .or_else(|| jwt_lifetime_seconds(token))
        .unwrap_or(DEFAULT_TOKEN_LIFETIME_SECONDS);
    Ok((token.to_string(), Duration::from_secs(lifetime)))
}

/// The seconds until a JWT's `exp` claim, when the token is a JWT.
fn jwt_lifetime_seconds(token: &str) -> Option<u64> {
    let payload = token.split('.').nth(1)?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .ok()?;
    let claims: serde_json::Value = serde_json::from_slice(&decoded).ok()?;
    let exp = claims.get("exp")?.as_u64()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(exp.saturating_sub(now))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn http_target_splits_host_prefix_and_user_info() {
        let (target, credentials) =
            HttpTarget::parse("https://sr.example.com:8081/registry/").unwrap();
        assert_eq!("https://sr.example.com:8081", target.host);
        assert_eq!("/registry", target.path);
        assert_eq!("/registry/schemas/ids/1", target.join("/schemas/ids/1"));
        assert!(credentials.is_none());
        let (plain, credentials) = HttpTarget::parse("http://user:p%40ss@127.0.0.1:8081").unwrap();
        assert_eq!("http://127.0.0.1:8081", plain.host);
        assert_eq!("", plain.path);
        assert_eq!(Some(("user".to_string(), "p@ss".to_string())), credentials);
        assert!(HttpTarget::parse("ftp://x").is_err());
        assert!(HttpTarget::parse("no-scheme").is_err());
    }

    #[test]
    fn empty_template_means_no_authentication() {
        let settings = RegistrySettings::from_template(&template(&[]), "http://127.0.0.1:8081")
            .expect("settings");
        assert!(matches!(settings.auth, RegistryAuth::None));
        assert_eq!("none", settings.auth_label());
        assert!(settings.extra_headers.is_empty());
    }

    #[test]
    fn oauth_client_credentials_are_read_with_defaults() {
        let settings = RegistrySettings::from_template(
            &template(&[
                (BEARER_SOURCE, "oauthbearer"),
                (
                    BEARER_ISSUER_URL,
                    "https://login.example.com/tenant/oauth2/v2.0/token",
                ),
                (BEARER_CLIENT_ID, "client-1"),
                (BEARER_CLIENT_SECRET, "secret-1"),
                (BEARER_SCOPE, "sr.read"),
                (BEARER_LOGICAL_CLUSTER, "lsrc-1"),
                (BEARER_IDENTITY_POOL, "pool-1"),
            ]),
            "https://sr.example.com",
        )
        .expect("settings");
        let RegistryAuth::OAuth { client, source } = &settings.auth else {
            panic!("expected OAuth")
        };
        assert_eq!("OAUTHBEARER", *source);
        assert_eq!("https://login.example.com", client.token_endpoint.host);
        assert_eq!("/tenant/oauth2/v2.0/token", client.token_endpoint.path);
        assert_eq!("client-1", client.client_id);
        assert_eq!(Some("sr.read"), client.scope.as_deref());
        assert_eq!(Duration::from_secs(300), client.expiry_buffer);
        assert_eq!(
            "grant_type=client_credentials&scope=sr.read",
            client.form_body()
        );
        assert_eq!(
            format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode("client-1:secret-1")
            ),
            client.basic_header()
        );
        assert_eq!(
            vec![
                ("target-sr-cluster".to_string(), "lsrc-1".to_string()),
                (
                    "Confluent-Identity-Pool-Id".to_string(),
                    "pool-1".to_string()
                )
            ],
            settings.extra_headers
        );
    }

    #[test]
    fn oauth_requires_its_three_parameters() {
        let error = RegistrySettings::from_template(
            &template(&[(BEARER_SOURCE, "OAUTHBEARER"), (BEARER_CLIENT_ID, "c")]),
            "http://sr",
        )
        .expect_err("rejected");
        assert!(error.message().contains(
            "bearer.auth.credentials.source=OAUTHBEARER requires bearer.auth.issuer.endpoint.url, \
             bearer.auth.client.id, bearer.auth.client.secret"
        ));
    }

    #[test]
    fn static_token_and_basic_sources() {
        let fixed = RegistrySettings::from_template(
            &template(&[(BEARER_SOURCE, "STATIC_TOKEN"), (BEARER_TOKEN, "t-123")]),
            "http://sr",
        )
        .expect("settings");
        assert!(matches!(fixed.auth, RegistryAuth::StaticBearer(ref t) if t == "t-123"));
        let user_info = RegistrySettings::from_template(
            &template(&[
                (BASIC_SOURCE, "USER_INFO"),
                (BASIC_USER_INFO, "alice:s3cret"),
            ]),
            "http://sr",
        )
        .expect("settings");
        let expected = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode("alice:s3cret")
        );
        assert!(
            matches!(user_info.auth, RegistryAuth::Basic { ref header, source: "USER_INFO" } if *header == expected)
        );
        // Confluent's default basic source is URL - credentials in the URL alone authenticate
        let in_url = RegistrySettings::from_template(&template(&[]), "http://alice:s3cret@sr:8081")
            .expect("settings");
        assert!(
            matches!(in_url.auth, RegistryAuth::Basic { ref header, source: "URL" } if *header == expected)
        );
        assert!(
            RegistrySettings::from_template(&template(&[(BASIC_SOURCE, "URL")]), "http://sr")
                .expect_err("rejected")
                .message()
                .contains("carries no user:password")
        );
        assert!(RegistrySettings::from_template(
            &template(&[(BEARER_SOURCE, "CUSTOM")]),
            "http://sr"
        )
        .expect_err("rejected")
        .message()
        .contains("is not supported by this client"));
    }

    #[test]
    fn jwt_lifetime_reads_the_exp_claim() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(format!("{{\"exp\":{}}}", now + 600));
        let token = format!("eyJhbGciOiJIUzI1NiJ9.{payload}.sig");
        let lifetime = jwt_lifetime_seconds(&token).expect("exp");
        assert!((598..=600).contains(&lifetime), "{lifetime}");
        assert_eq!(None, jwt_lifetime_seconds("opaque-token"));
    }

    #[test]
    fn form_and_percent_encoding() {
        assert_eq!("a+b%26c", form_encode("a b&c"));
        assert_eq!("p@ss word", percent_decode("p%40ss%20word"));
        assert_eq!("100%", percent_decode("100%"));
    }
}
