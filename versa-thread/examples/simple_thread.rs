use anyhow::Result;
use versa_common::{utils, Env};
use versa_model::openai::OpenAIModel;
use versa_prompt::{prompt, FinalizablePrompt};
use versa_thread::{basic_thread::BasicThread, Thread};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let thread = BasicThread::default().model(OpenAIModel::default());

    println!("Thread: {thread:#?}");

    let prompt = prompt!("Hello there!");

    println!("Prompt: {prompt:?}");

    let final_prompt = prompt.finalize()?;

    let output: String = thread.prompt(final_prompt).await?;

    println!("Output: {output:#?}");

    Ok(())
}
