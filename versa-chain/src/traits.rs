use std::pin::Pin;

use crate::ChainError;
use async_trait::async_trait;
use futures::Stream;
use versa_model::{Model, Output};

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
pub trait Chain<M>
where
    M: Model,
{
    // fn get_middlewares<T, I>(&self) -> I
    // where
    //      I: IntoIterator<Item = Box<dyn DynMiddleware>>>;

    /// Prompts the model with the given input.
    async fn prompt<O>(&self, prompt: impl Into<M::Input>) -> Result<O, ChainError>
    where
        O: Output<M>;

    /// Prompts the model with the given input and configuration.
    async fn prompt_with_config<O>(
        &self,
        prompt: impl Into<M::Input>,
        config: M::Config,
    ) -> Result<O, ChainError>
    where
        O: Output<M>;

    // TODO(nyprothegeek): Implement a custom ChainStream<T> type to hide the Box<dyn Stream<Item = Result<T, ChainError>>> type.
    /// Prompts the model with the given inputs.
    async fn prompt_many<'a, O, I>(
        &'a self,
        prompts: impl IntoIterator<Item = I>,
    ) -> Pin<Box<dyn Stream<Item = Result<O, ChainError>> + 'a>>
    where
        O: Output<M> + 'a,
        I: Into<M::Input> + 'a;

    // TODO(nyprothegeek): Implement a custom ChainStream<T> type to hide the Box<dyn Stream<Item = Result<T, ChainError>>> type.
    /// Prompts the model with the given inputs and configuration.
    async fn prompt_many_with_config<'a, O, I>(
        &'a self,
        prompts: impl IntoIterator<Item = I>,
        config: M::Config,
    ) -> Pin<Box<dyn Stream<Item = Result<O, ChainError>> + 'a>>
    where
        O: Output<M> + 'a,
        I: Into<M::Input> + 'a;
}

pub trait DynChain {}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_dyn_chain() {
        // TODO(nyprothegeek): Check that the dyn_chain trait object can be created and necessary type converted.
        todo!()
    }
}
