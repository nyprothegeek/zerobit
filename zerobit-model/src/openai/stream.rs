use futures::Stream;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

pub struct OutputStream<T> {
    phantom: PhantomData<T>, // ...
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl<T> Stream for OutputStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
