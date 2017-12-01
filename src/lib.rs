#![no_std]
#![deny(warnings)]
//! `Sink` implementations for testing that simulate different types of errors.
//!
//! ## Sinks:
//! - `StartSendErrSink`: returns an error on `start_send`
//! - `PollCompleteErrSink`: returns an error on `poll_complete`
//! - `CloseErrSink`: returns an error on `close`
extern crate futures;

pub use start_send_error_sink::StartSendErrSink;
pub use poll_complete_error_sink::PollCompleteErrSink;
pub use close_error_sink::CloseErrSink;

pub mod start_send_error_sink {
    use futures::{Async, Poll, Sink, StartSend};
    use core::marker::PhantomData;

    #[derive(Debug, PartialEq)]
    pub struct Error<I>(pub I);

    /// This sink returns an error on `start_send`, panics on `poll_complete`, and returns ready on
    /// `close`.
    #[derive(Debug, Default)]
    pub struct StartSendErrSink<I> {
        phantom_i: PhantomData<I>,
    }

    impl<I> Sink for StartSendErrSink<I> {
        type SinkItem = I;
        type SinkError = Error<I>;

        fn start_send(
            &mut self,
            item: Self::SinkItem,
        ) -> StartSend<Self::SinkItem, Self::SinkError> {
            Err(Error(item))
        }

        fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
            panic!("poll_complete is allowed to panic if start_send failed")
        }

        fn close(&mut self) -> Poll<(), Self::SinkError> {
            Ok(Async::Ready(()))
        }
    }
}

pub mod poll_complete_error_sink {
    use futures::{Async, AsyncSink, Poll, Sink, StartSend};
    use core::marker::PhantomData;

    #[derive(Debug, PartialEq)]
    pub struct Error();

    /// This sink returns ready on `start_send`, error on `poll_complete`, and ready on `close`.
    #[derive(Debug, Default)]
    pub struct PollCompleteErrSink<I> {
        phantom_i: PhantomData<I>,
    }

    impl<I> Sink for PollCompleteErrSink<I> {
        type SinkItem = I;
        type SinkError = Error;

        fn start_send(
            &mut self,
            _item: Self::SinkItem,
        ) -> StartSend<Self::SinkItem, Self::SinkError> {
            Ok(AsyncSink::Ready)
        }

        fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
            Err(Error())
        }

        fn close(&mut self) -> Poll<(), Self::SinkError> {
            Ok(Async::Ready(()))
        }
    }
}

pub mod close_error_sink {
    use futures::{Async, AsyncSink, Poll, Sink, StartSend};
    use core::marker::PhantomData;

    #[derive(Debug, PartialEq)]
    pub struct Error();

    /// This sink returns ready on `start_send`, ready on `poll_complete`, and `error` on close.
    #[derive(Debug, Default)]
    pub struct CloseErrSink<I> {
        phantom_i: PhantomData<I>,
    }

    impl<I> Sink for CloseErrSink<I> {
        type SinkItem = I;
        type SinkError = Error;

        fn start_send(
            &mut self,
            _item: Self::SinkItem,
        ) -> StartSend<Self::SinkItem, Self::SinkError> {
            Ok(AsyncSink::Ready)
        }

        fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
            Ok(Async::Ready(()))
        }

        fn close(&mut self) -> Poll<(), Self::SinkError> {
            Err(Error())
        }
    }
}
