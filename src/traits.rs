use futures_util::Stream as FutStream;
use std::future::Future as StdFuture;
use std::pin::Pin;

crate::dispatch_to_submodules!();

/// A future that is `Send` on native, but `!Send` on web
pub trait Future: Send + StdFuture {}
impl<T: Send + StdFuture> Future for T {}

/// A stream that is `Send` on native, but `!Send` on web
pub trait Stream: Send + FutStream {}
impl<T: Send + FutStream> Stream for T {}

/// An owned dynamically typed `Future` for use in cases where you can’t statically type your result or need to add some indirection.
///
/// Automatically `Send` on non-wasm32, and not `Send` on wasm32.
pub type BoxFuture<'a, T> = Pin<Box<dyn 'a + Future<Output = T>>>;

/// An owned dynamically typed `Stream` for use in cases where you can’t statically type your result or need to add some indirection.
///
/// Automatically `Send` on non-wasm32, and not `Send` on wasm32.
pub type BoxStream<'a, T> = Pin<Box<dyn 'a + Stream<Item = T>>>;
