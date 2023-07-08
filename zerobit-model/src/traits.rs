use async_trait::async_trait;
use std::fmt::Debug;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

/// A trait for language models.
#[async_trait(?Send)]
pub trait Model: Sized {
    /// The type of error the model can produce.
    type Error;

    /// Generates output from the given input.
    async fn prompt<O>(&self, prompt: impl Into<String>) -> Result<O, Self::Error>
    where
        O: Output<Self> + Debug;

    // TODO(nyprothegeek): Uncomment
    // async fn prompt_with_config<O>(
    //     &self,
    //     prompt: String,
    //     config: Self::Config,
    // ) -> Result<O, Self::Error>
    //     O: Output;
}

#[async_trait(?Send)]
pub trait Output<M>: Sized {
    async fn from_call(prompt: impl Into<String>, model: &M) -> anyhow::Result<Self>;
}
