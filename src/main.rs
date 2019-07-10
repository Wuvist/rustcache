#[macro_use]
extern crate log;

use grpcio::{RpcContext, UnarySink};

// use rustcache::{GroupCache, GetRequest, GetResponse};
use futures::future::Future;

use rustcache::groupcache_grpc::GroupCache;
use rustcache::groupcache::{GetRequest, GetResponse};

struct RustCacheService;

impl GroupCache for RustCacheService {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut resp = GetResponse::new();
        resp.set_key(req.get_key().to_string());
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
}

