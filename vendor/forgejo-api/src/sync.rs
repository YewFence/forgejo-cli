//! Blocking (non-async) API

use super::{
    structs, ApiError, ApiResponse, Auth, Endpoint, ForgejoError, FromResponse, OAuthError,
    RawRequest, RequestBody, TypedRequest,
};
use reqwest::blocking::Client;
use soft_assert::soft_assert;
use url::Url;

/// A blocking client for Forgejo's web API.
///
/// ## Differences from `async` version
///
/// This has largely the same interface as the [async version](crate::Forgejo),
/// but isn't entirely identical.
///
/// ### `.send()` instead of `.await`
///
/// Calling an endpoint requires calling `.call()` instead of using `.await`.
///
/// ```no_run
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// # let api = forgejo_api::Forgejo::new(forgejo_api::Auth::None, url::Url::parse("")?)?;
/// // Instead of this
/// let following = api.user_current_list_following().await;
/// # let api = forgejo_api::sync::Forgejo::new(forgejo_api::Auth::None, url::Url::parse("")?)?;
/// // Do this!
/// let following = api.user_current_list_following().send();
/// # Ok(())
/// # }
/// ```
pub struct Forgejo {
    url: Url,
    client: Client,
}

impl Forgejo {
    /// Creates a new client connected to the API of the specified Forgejo instance.
    ///
    /// The default user agent is "forgejo-api-rs". Use
    /// [`Forgejo::with_user_agent`] to set a custom one.
    pub fn new(auth: Auth, url: Url) -> Result<Self, ForgejoError> {
        Self::with_user_agent(auth, url, "forgejo-api-rs")
    }

    /// Just like [`Forgejo::new`], but includes a custom user agent to be sent
    /// with each request.
    pub fn with_user_agent(auth: Auth, url: Url, user_agent: &str) -> Result<Self, ForgejoError> {
        soft_assert!(
            matches!(url.scheme(), "http" | "https"),
            Err(ForgejoError::HttpRequired)
        );

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(auth.to_headers()?)
            .build()?;
        Ok(Self { url, client })
    }

    pub fn download_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        attach: i64,
    ) -> Result<bytes::Bytes, ForgejoError> {
        let release = self
            .repo_get_release_attachment(owner, repo, release, attach)
            .send()?;
        let mut url = self.url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .extend(["attachments", &release.uuid.unwrap().to_string()]);
        let request = self.client.get(url).build()?;
        Ok(self.client.execute(request)?.bytes()?)
    }

    /// Requests a new OAuth2 access token
    ///
    /// More info at [Forgejo's docs](https://forgejo.org/docs/latest/user/oauth2-provider).
    pub fn oauth_get_access_token(
        &self,
        body: structs::OAuthTokenRequest<'_>,
    ) -> Result<structs::OAuthToken, ForgejoError> {
        let url = self.url.join("login/oauth/access_token").unwrap();
        let request = self.client.post(url).json(&body).build()?;
        let response = self.client.execute(request)?;
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json()?),
            status if status.is_client_error() => {
                let err = response.json::<OAuthError>()?;
                Err(ApiError::from(err).into())
            }
            _ => Err(ForgejoError::UnexpectedStatusCode(response.status())),
        }
    }

    pub fn send_request(&self, request: &RawRequest) -> Result<ApiResponse, ForgejoError> {
        let mut url = self
            .url
            .join(&request.path)
            .expect("url fail. bug in forgejo-api");
        let mut query_pairs = url.query_pairs_mut();
        if let Some(query) = &request.query {
            query_pairs.extend_pairs(query.iter());
        }
        if let Some(page) = request.page {
            query_pairs.append_pair("page", &format!("{page}"));
        }
        if let Some(limit) = request.limit {
            query_pairs.append_pair("limit", &format!("{limit}"));
        }
        drop(query_pairs);
        let mut reqwest_request = self.client.request(request.method.clone(), url);
        reqwest_request = match &request.body {
            RequestBody::Json(bytes) => reqwest_request
                .body(bytes.clone())
                .header(reqwest::header::CONTENT_TYPE, "application/json"),
            RequestBody::Form(list) => {
                let mut form = reqwest::blocking::multipart::Form::new();
                for (k, v) in list {
                    form = form.part(
                        *k,
                        reqwest::blocking::multipart::Part::bytes(v.clone()).file_name("file"),
                    );
                }
                reqwest_request.multipart(form)
            }
            RequestBody::None => reqwest_request,
        };
        let mut reqwest_response = reqwest_request.send()?;
        let response = ApiResponse {
            status_code: reqwest_response.status(),
            headers: std::mem::take(reqwest_response.headers_mut()),
            body: reqwest_response.bytes()?,
        };
        Ok(response)
    }

    pub fn hit_endpoint<E: Endpoint, R: FromResponse>(
        &self,
        endpoint: E,
    ) -> Result<R, ForgejoError> {
        let (response, has_body) = E::handle_error(self.send_request(&endpoint.make_request())?)?;
        Ok(R::from_response(response, has_body)?)
    }
}

pub struct Request<'a, E, R> {
    pub(crate) inner: TypedRequest<E, R>,
    pub(crate) client: &'a Forgejo,
}

impl<'a, E: Endpoint, R: FromResponse> Request<'a, E, R> {
    pub fn send(self) -> Result<R, ForgejoError> {
        self.inner.send_sync(self.client)
    }

    pub fn response_type<T: FromResponse>(self) -> Request<'a, E, T> {
        Request {
            inner: TypedRequest {
                inner: self.inner.inner,
                __endpoint: std::marker::PhantomData,
                __response: std::marker::PhantomData,
            },
            client: self.client,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.inner.inner.page = Some(page);
        self
    }

    pub fn page_size(mut self, limit: u32) -> Self {
        self.inner.inner.limit = Some(limit);
        self
    }
}
