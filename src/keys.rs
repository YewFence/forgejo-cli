use eyre::eyre;
use forgejo_api::{Auth, Forgejo};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::ErrorKind,
    path::PathBuf,
    sync::OnceLock,
};
use tokio::io::AsyncWriteExt;
use url::Url;

static CONFIG_DIR: OnceLock<Option<PathBuf>> = OnceLock::new();

pub fn set_config_dir(path: Option<PathBuf>) -> eyre::Result<()> {
    CONFIG_DIR
        .set(path)
        .map_err(|_| eyre!("config directory was already initialized"))
}

/// Return the data directory for storing keys.json.
///
/// Respects `--config`, `FORGEJO_CONFIG`, and `FJ_DATA_DIR` if set, otherwise
/// falls back to the platform-specific data directory.
fn data_dir() -> eyre::Result<PathBuf> {
    if let Some(dir) = CONFIG_DIR.get().and_then(|path| path.as_ref()) {
        return Ok(dir.clone());
    }
    if let Ok(dir) = std::env::var("FORGEJO_CONFIG") {
        return Ok(PathBuf::from(dir));
    }
    if let Ok(dir) = std::env::var("FJ_DATA_DIR") {
        return Ok(PathBuf::from(dir));
    }
    Ok(directories::ProjectDirs::from("", "Cyborus", "forgejo-cli")
        .ok_or_else(|| eyre!("Could not find data directory"))?
        .data_dir()
        .to_path_buf())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct KeyInfo {
    pub hosts: BTreeMap<String, LoginInfo>,
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
    #[serde(default)]
    pub default_ssh: BTreeSet<String>,
}

impl KeyInfo {
    pub async fn load() -> eyre::Result<Self> {
        let path = data_dir()?.join("keys.json");
        let json = tokio::fs::read(path).await;
        let this = match json {
            Ok(x) => serde_json::from_slice::<Self>(&x)?,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                crate::verbose_log!("keys file not found, starting with empty keys");
                Self::default()
            }
            Err(e) => return Err(e.into()),
        };
        Ok(this)
    }

    pub async fn save(&self) -> eyre::Result<()> {
        let json = serde_json::to_vec_pretty(self)?;
        let path = data_dir()?;

        tokio::fs::create_dir_all(&path).await?;

        let mut options = std::fs::OpenOptions::new();
        options.create(true).write(true).truncate(true);

        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }

        let mut file = tokio::fs::OpenOptions::from(options)
            .open(path.join("keys.json"))
            .await?;
        file.write_all(&json).await?;

        Ok(())
    }

    pub fn get_login(&mut self, url: &Url) -> Option<&mut LoginInfo> {
        let host = crate::host_name(url);
        let login_info = self.hosts.get_mut(host)?;
        Some(login_info)
    }

    fn has_auth(&self, url: &Url) -> bool {
        crate::auth_token().is_some() || self.hosts.contains_key(crate::host_name(url))
    }

    pub async fn get_authenticated_api(&mut self, url: &Url) -> eyre::Result<Forgejo> {
        if !self.has_auth(url) {
            eyre::bail!("not logged in");
        }
        self.get_api(url).await
    }

    pub async fn get_api(&mut self, url: &Url) -> eyre::Result<Forgejo> {
        if let Some(token) = crate::auth_token() {
            crate::verbose_log!(
                "Using token from --token or environment for {}",
                crate::host_name(url)
            );
            return Forgejo::with_user_agent(Auth::Token(token), url.clone(), crate::USER_AGENT)
                .map_err(Into::into);
        }

        match self.get_login(url) {
            Some(login) => {
                crate::verbose_log!("Using saved login for {}", crate::host_name(url));
                let was_refreshed = login.refresh(url).await?;
                let api = login.api_for(url).await?;
                if was_refreshed {
                    crate::verbose_log!("Refreshed OAuth token for {}", crate::host_name(url));
                    self.save().await?;
                }
                Ok(api)
            }
            None => {
                crate::verbose_log!(
                    "No saved login for {}, using unauthenticated access",
                    crate::host_name(url)
                );
                Forgejo::with_user_agent(Auth::None, url.clone(), crate::USER_AGENT)
                    .map_err(Into::into)
            }
        }
    }

    pub fn deref_alias(&self, url: url::Url) -> url::Url {
        match self.aliases.get(crate::host_name(&url)) {
            Some(replacement) => {
                let s = format!(
                    "{}{}{}",
                    &url[..url::Position::BeforeHost],
                    replacement,
                    &url[url::Position::AfterPort..]
                );
                url::Url::parse(&s).unwrap()
            }
            None => url,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(tag = "type")]
pub enum LoginInfo {
    Application {
        token: String,
    },
    OAuth {
        token: String,
        refresh_token: String,
        expires_at: time::OffsetDateTime,
    },
}

impl LoginInfo {
    async fn refresh(&mut self, url: &Url) -> eyre::Result<bool> {
        if let LoginInfo::OAuth {
            token,
            refresh_token,
            expires_at,
            ..
        } = self
            && time::OffsetDateTime::now_utc() >= *expires_at
        {
            let api = Forgejo::with_user_agent(Auth::None, url.clone(), crate::USER_AGENT)?;
            let client_id = crate::auth::get_client_info_for(url)
                .await?
                .ok_or_else(|| eyre::eyre!("Can't refresh token: no client info for {url}."))?;
            let response = api
                .oauth_get_access_token(forgejo_api::structs::OAuthTokenRequest::Refresh {
                    refresh_token,
                    client_id: &client_id,
                    client_secret: "",
                })
                .await?;
            *token = response.access_token;
            *refresh_token = response.refresh_token;
            // A minute less, in case any weirdness happens at the exact moment it
            // expires. Better to refresh slightly too soon than slightly too late.
            let expires_in =
                std::time::Duration::from_secs(response.expires_in.saturating_sub(60) as u64);
            *expires_at = time::OffsetDateTime::now_utc() + expires_in;
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn api_for(&self, url: &Url) -> eyre::Result<Forgejo> {
        match self {
            LoginInfo::Application { token, .. } => {
                let api =
                    Forgejo::with_user_agent(Auth::Token(token), url.clone(), crate::USER_AGENT)?;
                Ok(api)
            }
            LoginInfo::OAuth { token, .. } => {
                let api =
                    Forgejo::with_user_agent(Auth::Token(token), url.clone(), crate::USER_AGENT)?;
                Ok(api)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key_info(aliases: Vec<(&str, &str)>) -> KeyInfo {
        KeyInfo {
            hosts: BTreeMap::new(),
            aliases: aliases
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            default_ssh: BTreeSet::new(),
        }
    }

    #[test]
    fn deref_alias_rewrites_matching_host() {
        // In practice, aliases map ssh_host_name -> http_host_name using
        // crate::host_name() which returns "host" for base URLs like
        // "https://host/". The replacement value is also a host_name string.
        let keys = make_key_info(vec![("myalias.local", "codeberg.org")]);
        let url = Url::parse("https://myalias.local/").unwrap();
        let result = keys.deref_alias(url);
        assert_eq!(result.host_str().unwrap(), "codeberg.org");
        assert_eq!(result.scheme(), "https");
    }

    #[test]
    fn deref_alias_no_match_returns_original() {
        let keys = make_key_info(vec![("other.local", "codeberg.org")]);
        let url = Url::parse("https://git.example.com/").unwrap();
        let result = keys.deref_alias(url.clone());
        assert_eq!(result, url);
    }

    #[test]
    fn login_info_ignores_legacy_name_field() {
        // keys.json files written before the username removal (a5831f5)
        // still contain a "name" field; loading them must keep working.
        let json = r#"{"type":"Application","name":"alice","token":"token123"}"#;
        let login: LoginInfo = serde_json::from_str(json).unwrap();
        assert!(matches!(login, LoginInfo::Application { .. }));
    }

    #[tokio::test]
    async fn refresh_is_noop_for_application_logins() {
        let mut login = LoginInfo::Application {
            token: "token123".into(),
        };
        let url = Url::parse("https://git.example.com/").unwrap();
        let was_refreshed = login.refresh(&url).await.unwrap();
        assert!(!was_refreshed);
    }
}
