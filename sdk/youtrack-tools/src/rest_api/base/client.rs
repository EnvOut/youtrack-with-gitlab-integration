use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::{Client, Uri, Response, Body, Method};
use hyper::client::{HttpConnector, ResponseFuture};
use std::ops::Deref;
use std::fmt;
use std::time::Instant;
use crate::rest_api::client::Config;
use hyper::http::HeaderValue;
use serde::Serialize;

pub type HyperClient = Client<HttpConnector, hyper::Body>;

pub struct HttpClient {
    inner: Arc<HyperClient>,
    created_at: Instant,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Self {
        // let client: Client<_, hyper::Body> = Client::builder()
        //     .pool_idle_timeout(Duration::from_secs(30))
        //     .http2_only(true)
        //     .build_http();

        let default_client = Default::default();
        let inner = Arc::new(default_client);
        HttpClient { inner, created_at: Instant::now(), config }
    }

    pub fn refresh_client(&mut self) -> &Self {
        *Arc::make_mut(&mut self.inner) = Default::default();
        self.created_at = Instant::now();
        self
    }

    /// Async method for getting data from the server
    /// Uses GET method and support auth using Bearer and Auth token
    pub async fn fetch_data(&self, path: String) -> hyper::Result<Response<Body>> {
        let uri = self.to_uri(path);
        log::info!("Fetching data from uri: {}...", &uri[0..100]);
        log::trace!("Fetching data from uri: {}", &uri);

        let request = hyper::Request::builder()
            .uri(uri)
            .header(hyper::header::AUTHORIZATION, self.get_bearer())
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();

        self.inner.request(request).await
    }

    pub async fn post_data<T>(&self, path: String, data: T) -> hyper::Result<Response<Body>> where T: Clone+Serialize {
        let uri = self.to_uri(path);
        log::info!("POST Request: {}...", &uri[0..100]);

        let body = serde_json::to_string(&data).unwrap();
        log::trace!("POST Request: {} \nbody: {}", &uri, &body);
        let request = hyper::Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(hyper::header::AUTHORIZATION, self.get_bearer())
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap();

        self.inner.request(request).await
    }

    fn get_bearer(&self)->String {
        format!("Bearer {}", self.config.token)
    }
    
    fn to_uri(&self, path: String) -> String {
        let mut uri = self.config.host.clone();
        uri.push_str(path.as_str());
        uri
    }
}

impl Deref for HttpClient {
    type Target = HyperClient;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpClient")
            .field("hyper", &self.inner)
            .field("created at", &self.created_at)
            .finish()
    }
}
