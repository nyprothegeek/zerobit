use crate::ChainError;
use async_trait::async_trait;
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
