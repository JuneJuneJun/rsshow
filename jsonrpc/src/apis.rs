use jsonrpc_core::futures::future::{self, Future, FutureResult};
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait Methods {
    #[rpc(name = "add")]
    fn add(&self) -> Result<u64>;
    #[rpc(name = "del", alias("jj"))]
    fn delete();
}

pub struct MyMethods;

impl Methods for MyMethods {
    fn add(&self) -> Result<u64> {
        Ok(23)
    }

    fn delete() {
    }
}