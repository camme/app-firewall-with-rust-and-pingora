use async_trait::async_trait;
use pingora::{prelude::HttpPeer, server::{configuration::ServerConf, Server}};
use pingora_proxy::{ProxyHttp, Session};
use pingora_core::Result;

struct Context { }
struct Guard { }

#[async_trait]
impl ProxyHttp for Guard {

    type CTX = Context;

    fn new_ctx(&self) -> Context{ 
        Context { }
    }

    async fn upstream_peer( &self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        let peer = Box::new( HttpPeer::new( "127.0.0.1:3500".to_string(), false, "one.one.one.one".to_string() ) );
        println!("Proxy!!!!");
        Ok(peer)
    }

}

pub fn main() {

    let conf = ServerConf {
        threads: num_cpus::get(),
        work_stealing: true,
        listener_tasks_per_fd: 4,
        upstream_keepalive_pool_size: 8192,
        ..Default::default()
    };
    let mut server = Server::new_with_opt_and_conf(None, conf);
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service( &server.configuration, Guard { } );

    proxy.add_tcp("0.0.0.0:3000");

    server.add_service(proxy);
    server.run_forever();

}
