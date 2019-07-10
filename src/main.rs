use grpcio::{RpcContext, UnarySink};

// use rustcache::{GroupCache, GetRequest, GetResponse};

use rustcache::groupcache_grpc::GroupCache;
use rustcache::groupcache::{GetRequest, GetResponse};

struct RustCacheService;

impl GroupCache for RustCacheService {
    fn get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {

    }
}

fn main() {
}

