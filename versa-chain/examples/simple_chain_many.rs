use anyhow::Result;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use versa_chain::{simple_chain::SimpleChain, Chain, ChainError};
use versa_common::{utils, Env};
use versa_model::openai::OpenAIModel;

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let chain = SimpleChain::default().model(OpenAIModel::default());

    let mut stream: Pin<Box<dyn Stream<Item = Result<String, ChainError>>>> = chain
        .prompt_many([
            "Reply with just one good name I can give my dog?",
            "Reply with just one good name I can give my cat?",
            "Reply with just one good name I can give my fish?",
        ])
        .await;

    while let Some(output) = stream.next().await {
        println!("Output: {output:#?}");
    }

    Ok(())
}
