use crate::ThreadError;
use async_trait::async_trait;
use versa_model::{Model, Output};

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
pub trait Thread<M>
where
    M: Model,
{
    // fn get_middlewares<T, I>(&self) -> I
    // where
    //     I: Iterator<Item = Box<dyn DynMiddleware>>;

    /// Prompts the model with the given input.
    async fn prompt<O>(&self, prompt: impl Into<M::Input>) -> Result<O, ThreadError>
    where
        O: Output<M>;

    /// Prompts the model with the given input and configuration.
    async fn prompt_with_config<O>(
        &self,
        prompt: impl Into<M::Input>,
        config: M::Config,
    ) -> Result<O, ThreadError>
    where
        O: Output<M>;
}

// TODO(appcypher): Implement common patterns.
#[async_trait(?Send)]
pub trait ThreadExt<M>: Thread<M>
where
    M: Model,
{
    // fn prompt_many...
    // fn prompt_many_with_config...
}

pub trait DynThread {}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_dyn_chain() {
        // TODO(nyprothegeek): Check that the dyn_chain trait object can be created and necessary type converted.
    }
}
