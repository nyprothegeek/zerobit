use anyhow::Result;
use versa_prompt::prompt;

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    let prompt = prompt!(
        system: "Classify the text into neutral, negative or positive.",
        user: "I think the vacation is okay.",
        assistant: "neutral",
        user: "I was not happy with the service."
    );

    println!("Prompt = {prompt:?}");

    let prompt = prompt!(
        system: [
            "Classify the text into: ",
            "neutral, negative or positive."
        ],
        user: "I think the vacation is okay.",
        assistant: "neutral",
        user: "I was not happy with the service."
    );

    println!("Prompt = {prompt:?}");

    Ok(())
}
