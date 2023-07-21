use anyhow::Result;
use versa_prompt::{map, prompt, ResolvablePrompt};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    let mut prompt = prompt!("What is a good name for a company that makes {{product}}?");
    prompt.format(map!("product" => "children toys"))?;
    println!("Prompt: {prompt:?}");

    let mut prompt = prompt!([
        "What is a good name for ",
        "a company that makes {{product}}?"
    ]);
    prompt.format(map!("product" => "children toys"))?;
    println!("Prompt: {prompt:?}");

    let resolved_prompt = prompt.resolve()?;
    println!("Resolved Prompt: {resolved_prompt:?}");

    Ok(())
}