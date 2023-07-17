use anyhow::Result;
use futures_util::StreamExt;
use zerobit_model::{
    openai::{ChatModelStream, OpenAIModel},
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
    let mut stream: ChatModelStream = model.prompt("Hello there!").await?;

    while let Some(output) = stream.next().await {
        println!("word = {:#?}", output?);
    }

    Ok(())
}
