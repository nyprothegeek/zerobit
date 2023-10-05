use crate::{Thread, ThreadError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use versa_common::traits::Config;
use versa_model::{Model, Output};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// Simple Thread is a bare bones chain that does nothing insteresting by itself.
///
/// Without middlewares, it is just a wrapper around a model.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleThread<M>
where
    M: Model,
{
    #[serde(flatten)]
    pub config: SimpleThreadConfig<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleThreadConfig<M>
where
    M: Model,
{
    // middlewares: Vec<Box<dyn DynMiddleware>>,
    pub model: M,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl<M> SimpleThread<M>
where
    M: Model,
{
    /// Creates a new simple chain using the given configuration.
    pub fn with_config(config: SimpleThreadConfig<M>) -> Self {
        Self { config }
    }

    /// Sets the model for the chain.
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
impl<M> Thread<M> for SimpleThread<M>
where
    M: Model,
{
    // fn get_middlewares<T, I>(&self) -> I
    // where
    //      I: IntoIterator<Item = Box<dyn DynMiddleware>> {
    //     self.config.middlewares.iter()
    // }

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

impl<M> Config for SimpleThreadConfig<M> where M: Model + Clone + Serialize + DeserializeOwned {}
