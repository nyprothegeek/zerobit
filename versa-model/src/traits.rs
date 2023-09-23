use crate::ModelError;
use async_trait::async_trait;
use versa_common::traits::Config;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for language models.
#[async_trait(?Send)]
pub trait Model: Sized {
    /// The configuration type for the model.
    type Config: Config;
    type Input;

    /// Generates output from the given input.
    async fn prompt<O>(&self, input: impl Into<Self::Input>) -> Result<O, ModelError>
    where
        O: Output<Self>;

    /// Generates output from the given input and configuration.
    async fn prompt_with_config<O>(
        &self,
        input: impl Into<Self::Input>,
        config: Self::Config,
    ) -> Result<O, ModelError>
    where
        O: Output<Self>;

    /// Gets the configuration for the model.
    fn get_config(&self) -> &Self::Config;
}

/// A trait for language model outputs.
#[async_trait(?Send)]
pub trait Output<M>: Sized
where
    M: Model,
{
    /// Creates a new output from sending the input to the model.
    async fn from_call(input: impl Into<M::Input>, model: &M) -> Result<Self, ModelError> {
        Self::from_call_with_config(input, model, model.get_config().clone()).await
    }

    /// Creates a new output from sending the input to the model with the given configuration.
    async fn from_call_with_config(
        input: impl Into<M::Input>,
        model: &M,
        config: M::Config,
    ) -> Result<Self, ModelError>;
}
