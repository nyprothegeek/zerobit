use crate::{Thread, ThreadError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
// use std::sync::Arc;
use versa_common::traits::Config;
// use versa_middleware::Middleware;
use versa_model::{Model, Output};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// BasicThread is a bare bones thread that does nothing insteresting by itself.
///
/// Without middlewares, it is just a wrapper around a model.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BasicThread<M>
where
    M: Model,
{
    #[serde(flatten)]
    pub config: BasicThreadConfig<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BasicThreadConfig<M>
where
    M: Model,
{
    // TODO(appcypher): Figure Serialize issue. Figure Output issue.
    // pub input_middlewares: Vec<Arc<dyn Middleware<Meta = (), Value = M::Input>>>,
    // pub output_middlewares: Vec<Arc<dyn Middleware<Meta = (), Value = Output<M>>>>,
    pub model: M,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl<M> BasicThread<M>
where
    M: Model,
{
    /// Creates a new simple thread using the given configuration.
    pub fn with_config(config: BasicThreadConfig<M>) -> Self {
        Self { config }
    }

    /// Sets the model for the thread.
    pub fn model(mut self, model: M) -> Self {
        self.config.model = model;
        self
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

// TODO(nyprothegeek): Implement middleware calls.
#[async_trait(?Send)]
impl<M> Thread<M> for BasicThread<M>
where
    M: Model,
{
    async fn prompt<O>(&self, prompt: impl Into<M::Input>) -> Result<O, ThreadError>
    where
        O: Output<M>,
    {
        self.prompt_with_config(prompt, self.config.model.get_config().clone())
            .await
    }

    async fn prompt_with_config<O>(
        &self,
        prompt: impl Into<M::Input>,
        config: M::Config,
    ) -> Result<O, ThreadError>
    where
        O: Output<M>,
    {
        Ok(O::from_call_with_config(prompt, &self.config.model, config).await?)
    }
}

impl<M> Config for BasicThreadConfig<M> where M: Model + Clone + Serialize + DeserializeOwned {}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------
