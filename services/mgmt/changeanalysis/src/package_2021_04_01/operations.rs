#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn changes(&self) -> changes::Client {
        changes::Client(self.clone())
    }
    pub fn operations(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn resource_changes(&self) -> resource_changes::Client {
        resource_changes::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the supported operations by the Microsoft.ChangeAnalysis resource provider along with their descriptions."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::ResourceProviderOperationList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/providers/Microsoft.ChangeAnalysis/operations", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url
                                    .join(&token.into_raw())
                                    .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                if let Some(skip_token) = &this.skip_token {
                                    url.query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::ResourceProviderOperationList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod resource_changes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the changes of a resource within the specified time range. Customer data will be masked if the user doesn't have access."]
        pub fn list(&self, resource_id: impl Into<String>, start_time: impl Into<String>, end_time: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_id: resource_id.into(),
                start_time: start_time.into(),
                end_time: end_time.into(),
                skip_token: None,
            }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::ChangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_id: String,
            pub(crate) start_time: String,
            pub(crate) end_time: String,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/providers/Microsoft.ChangeAnalysis/resourceChanges",
                            this.client.endpoint(),
                            &this.resource_id
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url
                                    .join(&token.into_raw())
                                    .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::POST);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::POST);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                let start_time = &this.start_time;
                                url.query_pairs_mut().append_pair("$startTime", start_time);
                                let end_time = &this.end_time;
                                url.query_pairs_mut().append_pair("$endTime", end_time);
                                if let Some(skip_token) = &this.skip_token {
                                    url.query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::ChangeList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod changes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the changes of a resource group within the specified time range. Customer data will always be masked."]
        pub fn list_changes_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            start_time: impl Into<String>,
            end_time: impl Into<String>,
        ) -> list_changes_by_resource_group::Builder {
            list_changes_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                start_time: start_time.into(),
                end_time: end_time.into(),
                skip_token: None,
            }
        }
        #[doc = "List the changes of a subscription within the specified time range. Customer data will always be masked."]
        pub fn list_changes_by_subscription(
            &self,
            subscription_id: impl Into<String>,
            start_time: impl Into<String>,
            end_time: impl Into<String>,
        ) -> list_changes_by_subscription::Builder {
            list_changes_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                start_time: start_time.into(),
                end_time: end_time.into(),
                skip_token: None,
            }
        }
    }
    pub mod list_changes_by_resource_group {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::ChangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) start_time: String,
            pub(crate) end_time: String,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ChangeAnalysis/changes",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url
                                    .join(&token.into_raw())
                                    .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                let start_time = &this.start_time;
                                url.query_pairs_mut().append_pair("$startTime", start_time);
                                let end_time = &this.end_time;
                                url.query_pairs_mut().append_pair("$endTime", end_time);
                                if let Some(skip_token) = &this.skip_token {
                                    url.query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::ChangeList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_changes_by_subscription {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::ChangeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) start_time: String,
            pub(crate) end_time: String,
            pub(crate) skip_token: Option<String>,
        }
        impl Builder {
            pub fn skip_token(mut self, skip_token: impl Into<String>) -> Self {
                self.skip_token = Some(skip_token.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.ChangeAnalysis/changes",
                            this.client.endpoint(),
                            &this.subscription_id
                        );
                        let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url
                                    .join(&token.into_raw())
                                    .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2021-04-01");
                                let start_time = &this.start_time;
                                url.query_pairs_mut().append_pair("$startTime", start_time);
                                let end_time = &this.end_time;
                                url.query_pairs_mut().append_pair("$endTime", end_time);
                                if let Some(skip_token) = &this.skip_token {
                                    url.query_pairs_mut().append_pair("$skipToken", skip_token);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder
                                    .body(req_body)
                                    .context(azure_core::error::ErrorKind::Other, "build request")?;
                                this.client
                                    .send(req)
                                    .await
                                    .context(azure_core::error::ErrorKind::Io, "execute request")?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::ChangeList = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code.as_u16(),
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
