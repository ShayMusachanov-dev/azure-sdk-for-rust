use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::ListCollectionsResponse;
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListCollectionsBuilder<'a> {
    database_client: &'a DatabaseClient,
    user_agent: Option<azure_core::UserAgent<'a>>,
    activity_id: Option<azure_core::ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<Continuation<'a>>,
    max_item_count: MaxItemCount,
}

impl<'a> ListCollectionsBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient) -> ListCollectionsBuilder<'a> {
        ListCollectionsBuilder {
            database_client,
            max_item_count: MaxItemCount::new(-1),
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
        }
    }

    pub fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }

    fn user_agent(&self) -> Option<azure_core::UserAgent<'a>> {
        self.user_agent
    }

    fn activity_id(&self) -> Option<azure_core::ActivityId<'a>> {
        self.activity_id
    }

    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }

    fn max_item_count(&self) -> MaxItemCount {
        self.max_item_count
    }

    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(azure_core::UserAgent::new(user_agent)),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(azure_core::ActivityId::new(activity_id)),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub fn with_continuation(self, continuation: &'a str) -> Self {
        Self {
            continuation: Some(Continuation::new(continuation)),
            ..self
        }
    }

    pub fn with_max_item_count(self, max_item_count: i32) -> Self {
        Self {
            max_item_count: MaxItemCount::new(max_item_count),
            ..self
        }
    }

    pub async fn execute(&self) -> Result<ListCollectionsResponse, CosmosError> {
        trace!("ListCollectionsBuilder::execute called");
        let request = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::GET,
            ResourceType::Collections,
        );

        let request = crate::headers::add_header(self.user_agent(), request);
        let request = crate::headers::add_header(self.activity_id(), request);
        let request = crate::headers::add_header(self.consistency_level(), request);
        let request = crate::headers::add_header(self.continuation(), request);
        let request = crate::headers::add_header(Some(self.max_item_count()), request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }

    pub fn stream(&self) -> impl Stream<Item = Result<ListCollectionsResponse, CosmosError>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(&continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}

impl<'a> ListCollectionsBuilder<'a> {
    fn continuation(&self) -> Option<Continuation<'a>> {
        self.continuation
    }
}
