use super::{ChatStreamMessage, OpenAIChatModel, OpenAICompletionModel, OpenAIError};
use futures::{stream::Skip, Stream, StreamExt};
use pin_project_lite::pin_project;
use reqwest_eventsource::{Event, EventSource};
use serde::Deserialize;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

//-------------------------------------------------------------------------------------------------
// Aliases
//-------------------------------------------------------------------------------------------------

pub type ChatModelStream = OutputStream<OpenAIChatModel>;
pub type CompletionModelStream = OutputStream<OpenAICompletionModel>;

pub type CompletionModelStreamResponse = ModelStreamResponse<CompletionStreamChoice>;
pub type ChatModelStreamResponse = ModelStreamResponse<ChatStreamChoice>;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

pin_project! {
    pub struct OutputStream<M> {
        model: PhantomData<M>,
        #[pin]
        event_src: Skip<EventSource>,
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelStreamResponse<T> {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct ChatStreamChoice {
    pub index: u64,
    pub delta: ChatStreamMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionStreamChoice {
    pub index: u64,
    pub text: String,
    pub logprobs: Option<u8>,
    pub finish_reason: Option<String>,
}

//-------------------------------------------------------------------------------------------------

// Methods
//-------------------------------------------------------------------------------------------------

impl<M> OutputStream<M> {
    pub fn new(event_src: EventSource) -> Self {
        Self {
            model: PhantomData,
            // Skip the first event, which is always the "open" event
            event_src: event_src.skip(1),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl Stream for ChatModelStream {
    type Item = Result<String, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.event_src.poll_next(cx) {
            Poll::Ready(Some(Ok(Event::Message(event)))) => {
                #[cfg(feature = "log")]
                log::debug!("eventsource message: {event:#?}");

                if event.data == "[DONE]" {
                    return Poll::Ready(None);
                }

                let response: ChatModelStreamResponse =
                    serde_json::from_str(&event.data).map_err(OpenAIError::SerdeJson)?;

                Poll::Ready(Some(Ok(response.choices[0]
                    .delta
                    .content
                    .clone()
                    .unwrap_or_default())))
            }
            Poll::Ready(Some(Ok(Event::Open))) => unreachable!(),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Stream for CompletionModelStream {
    type Item = Result<String, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.event_src.poll_next(cx) {
            Poll::Ready(Some(Ok(Event::Message(event)))) => {
                #[cfg(feature = "log")]
                log::debug!("eventsource message: {event:#?}");

                if event.data == "[DONE]" {
                    return Poll::Ready(None);
                }

                let response: CompletionModelStreamResponse =
                    serde_json::from_str(&event.data).map_err(OpenAIError::SerdeJson)?;

                Poll::Ready(Some(Ok(response.choices[0].text.clone())))
            }
            Poll::Ready(Some(Ok(Event::Open))) => unreachable!(),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
