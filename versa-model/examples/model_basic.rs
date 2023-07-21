use anyhow::Result;
use versa_model::{
    openai::{OpenAICompletionModel, OpenAIModel},
    Model,
};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let model = OpenAIModel::default();
    let output: String = model.prompt("Hello there!").await?;

    println!("chat model output = {output:#?}");

    let model = OpenAICompletionModel::default();
    let output: String = model.prompt("Hello there!").await?;

    println!("completion model output = {output:#?}");

    Ok(())
}
