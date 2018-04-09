extern crate chrono;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate json_api;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;

mod events;
mod json_uuid;
mod models;
pub mod status;

use failure::{Error, SyncFailure};
use futures::prelude::*;
use hyper::{Client, Method};
use hyper::client::{Connect, HttpConnector, Request};
use hyper::header::{qitem, Accept, Authorization, Bearer, UserAgent};
use hyper::mime::{self, Mime};
use hyper_tls::HttpsConnector;
use json_api::doc::Object;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_core::reactor::Handle;

pub use events::{pc, xbox};
pub use models::Match;
use status::StatusRequest;

const DEFAULT_HOST: &str = "https://api.playbattlegrounds.com";

pub type GenericFuture<T> = Box<futures::Future<Item = T, Error = Error>>;

header! {
    #[doc(hidden)]
    (XRequestId, "X-Request-Id") => [uuid::Uuid]
}

header! {
    #[doc(hidden)]
    (XRatelimitLimit, "X-Ratelimit-Limit") => [u16]
}

header! {
    #[doc(hidden)]
    (XRatelimitRemaining, "X-Ratelimit-Remaining") => [u32]
}

header! {
    #[doc(hidden)]
    (XRatelimitReset, "X-Ratelimit-Reset") => [u32]
}

#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    Bearer(String),
}

#[derive(Clone, Copy)]
pub enum MediaType {
    Json,
    JsonApi,
}

impl Default for MediaType {
    fn default() -> MediaType {
        MediaType::JsonApi
    }
}

impl From<MediaType> for Mime {
    fn from(media: MediaType) -> Mime {
        match media {
            MediaType::Json => mime::APPLICATION_JSON,
            MediaType::JsonApi => "application/vnd.api+json".parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pubg<C>
where
    C: Clone + Connect,
{
    host: String,
    agent: String,
    client: Client<C>,
    credentials: Option<Credentials>,
}

impl Pubg<HttpsConnector<HttpConnector>> {
    pub fn new<A>(agent: A, credentials: Option<Credentials>, handle: &Handle) -> Self
    where
        A: Into<String>,
    {
        Self::host(DEFAULT_HOST, agent, credentials, handle)
    }

    pub fn host<H, A>(host: H, agent: A, credentials: Option<Credentials>, handle: &Handle) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let client = Client::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Self::custom(host, agent, client, credentials)
    }
}

impl<C> Pubg<C>
where
    C: Clone + Connect,
{
    pub fn custom<H, A>(
        host: H,
        agent: A,
        client: Client<C>,
        credentials: Option<Credentials>,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client,
            credentials,
        }
    }

    fn request<Out>(
        &self,
        method: Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: MediaType,
    ) -> GenericFuture<Out>
    where
        Out: DeserializeOwned + 'static,
    {
        let url = uri.parse().into_future();
        let instance = self.clone();
        let body2 = body.clone();
        let method2 = method.clone();
        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::new(method2, url);
            {
                let headers = req.headers_mut();
                headers.set(UserAgent::new(instance.agent.clone()));
                headers.set(Accept(vec![qitem(From::from(media_type))]));
                if let Some(Credentials::Bearer(token)) = instance.credentials {
                    headers.set(Authorization(Bearer { token }))
                }
            }

            if let Some(body) = body2 {
                req.set_body(body)
            }
            instance.client.request(req).map_err(Error::from)
        });
        Box::new(response.and_then(move |response| {
            for value in response.headers().get::<XRequestId>() {
                debug!("X-Request-Id: {}", value.0)
            }
            for value in response.headers().get::<XRatelimitLimit>() {
                debug!("X-Ratelimit-Limit: {}", value.0)
            }
            let remaining = response
                .headers()
                .get::<XRatelimitRemaining>()
                .map(|val| val.0);
            let reset = response.headers().get::<XRatelimitReset>().map(|val| val.0);
            for value in remaining {
                debug!("X-Ratelimit-Remaining: {}", value)
            }
            for value in reset {
                debug!("X-Ratelimit-Reset: {}", value)
            }
            let status = response.status();
            Box::new(response.body().concat2().map_err(Error::from).and_then(
                move |response_body| {
                    if status.is_success() {
                        debug!(
                            "response payload {}",
                            String::from_utf8_lossy(&response_body)
                        );
                        json_api::from_slice::<Object, Out>(&response_body)
                            .map_err(SyncFailure::new)
                            .map_err(Error::from)
                    } else {
                        serde_json::from_str::<Out>("").map_err(Error::from)
                    }
                },
            ))
        }))
    }

    fn request_entity<D>(
        &self,
        method: Method,
        uri: &str,
        body: Option<Vec<u8>>,
        media_type: MediaType,
    ) -> GenericFuture<D>
    where
        D: DeserializeOwned + 'static,
    {
        Box::new(self.request(method, uri, body, media_type))
    }

    fn get<D>(&self, uri: &str) -> GenericFuture<D>
    where
        D: DeserializeOwned + 'static,
    {
        let url = &(self.host.to_owned() + uri);
        self.get_media(url, MediaType::JsonApi)
    }

    fn get_media<D>(&self, url: &str, media: MediaType) -> GenericFuture<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request_entity(Method::Get, url, None, media)
    }

    pub fn status(&self) -> StatusRequest<C> {
        StatusRequest::new(self.clone())
    }
}

/// Filters empty links from a "links" section
pub fn filter_empty_links(val: Value) -> Value {
    match val {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|&(_, ref v)| match *v {
                    Value::String(ref s) => !s.is_empty(),
                    _ => true,
                })
                .collect(),
        ),
        _ => val,
    }
}

/// Crawls a tree looking for a "links" section
pub fn find_links(val: Value) -> Value {
    match val {
        Value::Array(arr) => Value::Array(arr.into_iter().map(find_links).collect()),
        Value::Object(obj) => Value::Object(
            obj.into_iter()
                .map(|(k, v)| {
                    if k == "links" {
                        (k, filter_empty_links(v))
                    } else {
                        (k, find_links(v))
                    }
                })
                .collect(),
        ),
        _ => val,
    }
}
