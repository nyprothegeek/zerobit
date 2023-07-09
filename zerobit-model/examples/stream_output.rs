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
    let mut stream = model
        .prompt::<ChatModelStream>("Hello there!")
        .await?
        .enumerate();

    while let Some((index, output)) = stream.next().await {
        println!("output {index} = {:#?}", output?);
    }

    Ok(())
}
