#[macro_use]
extern crate log;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use futures::sync::oneshot;
use futures::future::Future;

use rustcache::groupcache_grpc::GroupCache;
use rustcache::groupcache::{GetRequest, GetResponse};

#[derive(Clone)]
struct RustCacheService;

impl GroupCache for RustCacheService {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut resp = GetResponse::new();
        resp.set_key(req.get_key().to_string() + " - " + &req.get_group().to_string());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = rustcache::groupcache_grpc::create_group_cache(RustCacheService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 8080)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
let _ = server.shutdown().wait();
}

