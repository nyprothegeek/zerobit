use super::{ChatConfig, ChatMessages, CompletionConfig, OpenAIConfig};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum_macros::Display;

//-------------------------------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------------------------------

pub trait ModelKind: Clone + Serialize + DeserializeOwned {
    type Config: OpenAIConfig;
    type Input;
}

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum ChatModel {
    #[strum(serialize = "gpt-3.5-turbo-0613")]
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GPT3_5Turbo0613,

    #[strum(serialize = "gpt-3.5-turbo-0301")]
    #[serde(rename = "gpt-3.5-turbo-0301")]
    GPT3_5Turbo0301,

    #[strum(serialize = "gpt-3.5-turbo")]
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3_5Turbo,

    #[strum(serialize = "gpt-3.5-turbo-16k-0613")]
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    GPT3_5Turbo16k0613,

    #[strum(serialize = "gpt-3.5-turbo-16k")]
    #[serde(rename = "gpt-3.5-turbo-16k")]
    GPT3_5Turbo16k,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum CompletionModel {
    #[strum(serialize = "babbage")]
    #[serde(rename = "babbage")]
    Babbage,

    #[strum(serialize = "davinci")]
    #[serde(rename = "davinci")]
    Davinci,

    #[strum(serialize = "babbage-code-search-code")]
    #[serde(rename = "babbage-code-search-code")]
    BabbageCodeSearchCode,

    #[strum(serialize = "text-similarity-babbage-001")]
    #[serde(rename = "text-similarity-babbage-001")]
    TextSimilarityBabbage001,

    #[strum(serialize = "text-davinci-001")]
    #[serde(rename = "text-davinci-001")]
    TextDaVinci001,

    #[strum(serialize = "ada")]
    #[serde(rename = "ada")]
    Ada,

    #[strum(serialize = "babbage-code-search-text")]
    #[serde(rename = "babbage-code-search-text")]
    BabbageCodeSearchText,

    #[strum(serialize = "babbage-similarity")]
    #[serde(rename = "babbage-similarity")]
    BabbageSimilarity,

    #[strum(serialize = "code-search-babbage-text-001")]
    #[serde(rename = "code-search-babbage-text-001")]
    CodeSearchBabbageText001,

    #[strum(serialize = "text-curie-001")]
    #[serde(rename = "text-curie-001")]
    TextCurie001,

    #[strum(serialize = "code-search-babbage-code-001")]
    #[serde(rename = "code-search-babbage-code-001")]
    CodeSearchBabbageCode001,

    #[strum(serialize = "text-ada-001")]
    #[serde(rename = "text-ada-001")]
    TextAda001,

    #[strum(serialize = "text-similarity-ada-001")]
    #[serde(rename = "text-similarity-ada-001")]
    TextSimilarityAda001,

    #[strum(serialize = "curie-instruct-beta")]
    #[serde(rename = "curie-instruct-beta")]
    CurieInstructBeta, // TODO(nyprothegeek): InstructModel?

    #[strum(serialize = "ada-code-search-code")]
    #[serde(rename = "ada-code-search-code")]
    AdaCodeSearchCode,

    #[strum(serialize = "ada-similarity")]
    #[serde(rename = "ada-similarity")]
    AdaSimilarity,

    #[strum(serialize = "code-search-ada-text-001")]
    #[serde(rename = "code-search-ada-text-001")]
    CodeSearchAdaText001,

    #[strum(serialize = "text-search-ada-query-001")]
    #[serde(rename = "text-search-ada-query-001")]
    TextSearchAdaQuery001,

    #[strum(serialize = "davinci-search-document")]
    #[serde(rename = "davinci-search-document")]
    DaVinciSearchDocument,

    #[strum(serialize = "ada-code-search-text")]
    #[serde(rename = "ada-code-search-text")]
    AdaCodeSearchText,

    #[strum(serialize = "text-search-ada-doc-001")]
    #[serde(rename = "text-search-ada-doc-001")]
    TextSearchAdaDoc001,

    #[strum(serialize = "davinci-instruct-beta")]
    #[serde(rename = "davinci-instruct-beta")]
    DaVinciInstructBeta,

    #[strum(serialize = "text-similarity-curie-001")]
    #[serde(rename = "text-similarity-curie-001")]
    TextSimilarityCurie001,

    #[strum(serialize = "code-search-ada-code-001")]
    #[serde(rename = "code-search-ada-code-001")]
    CodeSearchAdaCode001,

    #[strum(serialize = "ada-search-query")]
    #[serde(rename = "ada-search-query")]
    AdaSearchQuery,

    #[strum(serialize = "text-search-davinci-query-001")]
    #[serde(rename = "text-search-davinci-query-001")]
    TextSearchDaVinciQuery001,

    #[strum(serialize = "curie-search-query")]
    #[serde(rename = "curie-search-query")]
    CurieSearchQuery,

    #[strum(serialize = "davinci-search-query")]
    #[serde(rename = "davinci-search-query")]
    DaVinciSearchQuery,

    #[strum(serialize = "babbage-search-document")]
    #[serde(rename = "babbage-search-document")]
    BabbageSearchDocument,

    #[strum(serialize = "ada-search-document")]
    #[serde(rename = "ada-search-document")]
    AdaSearchDocument,

    #[strum(serialize = "text-search-curie-query-001")]
    #[serde(rename = "text-search-curie-query-001")]
    TextSearchCurieQuery001,

    #[strum(serialize = "text-search-babbage-doc-001")]
    #[serde(rename = "text-search-babbage-doc-001")]
    TextSearchBabbageDoc001,

    #[strum(serialize = "curie-search-document")]
    #[serde(rename = "curie-search-document")]
    CurieSearchDocument,

    #[strum(serialize = "text-search-curie-doc-001")]
    #[serde(rename = "text-search-curie-doc-001")]
    TextSearchCurieDoc001,

    #[strum(serialize = "babbage-search-query")]
    #[serde(rename = "babbage-search-query")]
    BabbageSearchQuery,

    #[strum(serialize = "text-babbage-001")]
    #[serde(rename = "text-babbage-001")]
    TextBabbage001,

    #[strum(serialize = "text-search-davinci-doc-001")]
    #[serde(rename = "text-search-davinci-doc-001")]
    TextSearchDaVinciDoc001,

    #[strum(serialize = "text-search-babbage-query-001")]
    #[serde(rename = "text-search-babbage-query-001")]
    TextSearchBabbageQuery001,

    #[strum(serialize = "curie-similarity")]
    #[serde(rename = "curie-similarity")]
    CurieSimilarity,

    #[strum(serialize = "curie")]
    #[serde(rename = "curie")]
    Curie,

    #[strum(serialize = "text-embedding-ada-002")]
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,

    #[strum(serialize = "text-similarity-davinci-001")]
    #[serde(rename = "text-similarity-davinci-001")]
    TextSimilarityDaVinci001,

    #[strum(serialize = "text-davinci-002")]
    #[serde(rename = "text-davinci-002")]
    TextDaVinci002,

    #[strum(serialize = "text-davinci-003")]
    #[serde(rename = "text-davinci-003")]
    TextDaVinci003,

    #[strum(serialize = "davinci-similarity")]
    #[serde(rename = "davinci-similarity")]
    DaVinciSimilarity,
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl ModelKind for ChatModel {
    type Config = ChatConfig;
    type Input = ChatMessages;
}

impl ModelKind for CompletionModel {
    type Config = CompletionConfig;
    type Input = String;
}
