use anyhow::Result;
use versa_common::{utils, Env};
use versa_model::{
    openai::{OpenAICompletionModel, OpenAIModel},
    Model,
};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let model = OpenAIModel::default()
        .temperature(0.)
        .max_tokens(5)
        .top_p(0.2);

    let output: String = model.prompt("Hello there!").await?;

    println!("chat model output = {output:#?}");

    let model = OpenAICompletionModel::default()
        .temperature(0.)
        .max_tokens(5)
        .top_p(0.2);

    let output: String = model.prompt("Hello there!").await?;

    println!("completion model output = {output:#?}");

    Ok(())
}
