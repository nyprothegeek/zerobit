use anyhow::Result;
use versa_chain::{simple_chain::SimpleChain, Chain};
use versa_common::{utils, Env};
use versa_model::openai::OpenAIModel;
use versa_prompt::{prompt, FinalizablePrompt};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let chain = SimpleChain::default().model(OpenAIModel::default());

    println!("Chain: {chain:#?}");

    let prompt = prompt!("Hello there!");

    println!("Prompt: {prompt:?}");

    let output: String = chain.prompt(prompt.finalize()?).await?;

    println!("Output: {output:#?}");

    Ok(())
}
