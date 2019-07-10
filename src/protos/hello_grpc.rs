// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GROUP_CACHE_GET: ::grpcio::Method<super::hello::GetRequest, super::hello::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/groupcachepb.GroupCache/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GroupCacheClient {
    client: ::grpcio::Client,
}

impl GroupCacheClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GroupCacheClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::hello::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::hello::GetResponse> {
        self.client.unary_call(&METHOD_GROUP_CACHE_GET, req, opt)
    }

    pub fn get(&self, req: &super::hello::GetRequest) -> ::grpcio::Result<super::hello::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::hello::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hello::GetResponse>> {
        self.client.unary_call_async(&METHOD_GROUP_CACHE_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::hello::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hello::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GroupCache {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::hello::GetRequest, sink: ::grpcio::UnarySink<super::hello::GetResponse>);
}

pub fn create_group_cache<S: GroupCache + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_CACHE_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    builder.build()
}
