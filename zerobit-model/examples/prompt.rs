use anyhow::Result;
use zerobit_model::{openai::OpenAIModel, Model};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let model = OpenAIModel::default();
    let string: String = model.prompt("Hello there!").await?;

    println!("string = {string:#?}");

    Ok(())
}
