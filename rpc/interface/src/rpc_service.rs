//! RPC [`tower::Service`] trait.

//---------------------------------------------------------------------------------------------------- Use
use std::future::Future;

use tower::Service;

//---------------------------------------------------------------------------------------------------- RpcService
/// An RPC [`tower::Service`].
///
/// This trait solely exists to encapsulate the traits needed
/// to handle RPC requests and respond with responses - **it is
/// not meant to be used directly.**
///
/// The `Request` and `Response` are generic and
/// are used in the [`tower::Service`] bounds.
///
/// The error type is always [`anyhow::Error`].
///
/// There is a blanket implementation that implements this
/// trait on types that implement `tower::Service` correctly.
///
/// See [`RpcHandler`](crate::RpcHandler) for more information.
pub trait RpcService<Request, Response>:
    Clone
    + Send
    + Sync
    + 'static
    + Service<
        Request,
        Response = Response,
        Error = anyhow::Error,
        Future: Future<Output = Result<Response, anyhow::Error>> + Send + 'static,
    >
{
}

impl<Request, Response, T> RpcService<Request, Response> for T where
    Self: Clone
        + Send
        + Sync
        + 'static
        + Service<
            Request,
            Response = Response,
            Error = anyhow::Error,
            Future: Future<Output = Result<Response, anyhow::Error>> + Send + 'static,
        >
{
}