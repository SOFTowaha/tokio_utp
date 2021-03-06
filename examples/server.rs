extern crate env_logger;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_utp;
#[macro_use]
extern crate unwrap;
extern crate void;

use futures::{future, Future, Stream};
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use tokio_utp::*;
use void::Void;

use std::net::SocketAddr;

pub fn main() {
    unwrap!(::env_logger::init());

    // Start a simple echo server

    let addr: SocketAddr = unwrap!("127.0.0.1:4561".parse());
    let mut core = unwrap!(Core::new());
    let handle = core.handle();
    let _: Result<(), Void> = core.run(future::lazy(|| {
        let (_, listener) = unwrap!(UtpSocket::bind(&addr, &handle));
        listener
            .incoming()
            .for_each(|stream| {
                let (reader, writer) = stream.split();
                tokio_io::io::copy(reader, writer).map(|_| ())
            })
            .then(|res| {
                unwrap!(res);
                Ok(())
            })
    }));
}
