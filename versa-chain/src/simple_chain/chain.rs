use crate::{Chain, ChainError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use versa_common::traits::Config;
use versa_model::{Model, Output};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// Simple Chain is a bare bones chain that does nothing insteresting by itself.
///
/// Without middlewares, it is just a wrapper around a model.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleChain<M>
where
    M: Model,
{
    #[serde(flatten)]
    pub config: SimpleChainConfig<M>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleChainConfig<M>
where
    M: Model,
{
    // middlewares: Vec<Box<dyn DynMiddleware>>,
    pub model: M,
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl<M> SimpleChain<M>
where
    M: Model,
{
    /// Creates a new simple chain using the given configuration.
    pub fn with_config(config: SimpleChainConfig<M>) -> Self {
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
impl<M> Chain<M> for SimpleChain<M>
where
    M: Model,
{
    // fn get_middlewares<T, I>(&self) -> I
    // where
    //      I: IntoIterator<Item = Box<dyn DynMiddleware>> {
    //     self.config.middlewares.iter()
    // }

    async fn prompt<O>(&self, prompt: impl Into<M::Input>) -> Result<O, ChainError>
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
    ) -> Result<O, ChainError>
    where
        O: Output<M>,
    {
        Ok(O::from_call_with_config(prompt, &self.config.model, config).await?)
    }

    async fn prompt_many<O, I>(
        &self,
        prompt: impl IntoIterator<Item = I>,
    ) -> Result<Vec<O>, ChainError>
    where
        O: Output<M>,
        I: Into<M::Input>,
    {
        self.prompt_many_with_config(prompt, self.config.model.get_config().clone())
            .await
    }

    async fn prompt_many_with_config<O, I>(
        &self,
        prompt: impl IntoIterator<Item = I>,
        config: M::Config,
    ) -> Result<Vec<O>, ChainError>
    where
        O: Output<M>,
        I: Into<M::Input>,
    {
        let mut outputs = Vec::new();

        for input in prompt {
            outputs
                .push(O::from_call_with_config(input, &self.config.model, config.clone()).await?);
        }

        Ok(outputs)
    }
}

impl<M> Config for SimpleChainConfig<M> where M: Model + Clone + Serialize + DeserializeOwned {}
