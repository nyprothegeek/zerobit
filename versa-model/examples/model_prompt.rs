use anyhow::Result;
use versa_common::{utils, Env};
use versa_model::{
    openai::{OpenAICompletionModel, OpenAIModel},
    Model,
};
use versa_prompt::{map, prompt, FinalizablePrompt};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let prompt = prompt!(
        system: "Classify the text into neutral, negative or positive.",
        user: "I think the vacation is okay.",
        assistant: "neutral",
        user: "I was not happy with the service."
    );

    let model = OpenAIModel::default();
    let output: String = model.prompt(prompt.finalize()?).await?;

    println!("chat model output = {output:#?}");

    let prompt = prompt!("What is a good name for a company that makes {{product}}?");

    let model = OpenAICompletionModel::default();
    let output: String = model
        .prompt(prompt.resolve(map!("product" => "children toys"))?)
        .await?;

    println!("completion model output = {output:#?}");

    Ok(())
}
