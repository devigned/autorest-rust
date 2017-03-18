#![deny(warnings)]
use std::fmt;
use std::io::{self, Write};
use std::sync::Arc;

use hyper;
use hyper::client::Client;
use hyper::client::HttpConnector;
// use hyper_openssl::OpensslClient;
use tokio_core;
use tokio_core::reactor::Handle;

use futures::Future;
use futures::stream::Stream;
use ::futures_cpupool::{CpuPool, CpuFuture};

pub struct ResourceGroups {
    client: Client<HttpConnector>,
    core: tokio_core::reactor::Core
}

#[derive(Clone)]
pub struct Dns {
    pool: CpuPool,
}

impl Dns {
    pub fn new(threads: usize) -> Dns {
        Dns {
            pool: CpuPool::new(threads)
        }
    }

    pub fn resolve(&self, host: String, port: u16) -> Query {
        Query(self.pool.spawn_fn(move || work(host, port)))
    }
}

fn work(hostname: String, port: u16) -> Answer {
    debug!("resolve {:?}:{:?}", hostname, port);
    (&*hostname, port).to_socket_addrs().map(|i| IpAddrs { iter: i })
}

pub struct Query(CpuFuture<IpAddrs, io::Error>);

impl Future for Query {
    type Item = IpAddrs;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

/// A connector for the `https` scheme.
#[derive(Clone)]
pub struct HttpsConnector {
    dns: Dns,
    handle: Handle
}

impl HttpsConnector {
    /// Create a new connector using the provided SSL implementation.
    pub fn new(threads: usize, handle: &Handle) -> HttpsConnector {
        HttpsConnector {
            dns: Dns::new(threads),
            handle: handle.clone()
        }
    }
}

impl fmt::Debug for HttpsConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HttpsConnector")
            .finish()
    }
}

impl ResourceGroups {
    pub fn new() -> ResourceGroups{
        let core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        ResourceGroups {
            client: Client::<HttpConnector>::new(&handle),
            core: core
        }
    }

    pub fn get(&mut self, name: &str, location: &str) {
        println!("Name: {}", name);
        println!("Location: {}", location);
        let url = hyper::Url::parse("http://google.com").unwrap();
        let work = self.client.get(url).and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("\n\nDone.");
        });
        self.core.run(work).unwrap();
    }
}

