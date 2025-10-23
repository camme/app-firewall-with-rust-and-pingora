use std::{net::IpAddr, time::Duration};

use once_cell::sync::Lazy;
use pingora::{http::ResponseHeader, prelude::HttpPeer};
use pingora_limits::rate::Rate;
use pingora_proxy::{ProxyHttp, Session};
use async_trait::async_trait;
use pingora_core::Result;

pub struct AwesomeProxy { }

// Rate limiter
static RATE_LIMITER: Lazy<Rate> = Lazy::new(|| Rate::new(Duration::from_secs(1)));
 
// max request per second per client
static MAX_REQ_PER_SEC: isize = 1;


impl AwesomeProxy {
    pub fn new() -> Self {
        Self { }
    }
}

pub struct Context { }

#[async_trait]
impl ProxyHttp for AwesomeProxy {

    type CTX = Context;

    fn new_ctx(&self) -> Context{ 
        Context { }
    }

    async fn upstream_peer( &self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        let peer = Box::new( HttpPeer::new( "127.0.0.1:3500".to_string(), false, "".to_string() ) );
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool>
where
        Self::CTX: Send + Sync,
    {

        let client_addr = session.client_addr();
        let ip: Option<IpAddr> = client_addr.map(|ip_addr| ip_addr.as_inet().unwrap().ip());

        let curr_window_requests = RATE_LIMITER.observe(&ip, 1);
        println!("curr_window_requests: {}", curr_window_requests);
        if curr_window_requests > MAX_REQ_PER_SEC {
            session.respond_error_with_body(404, "<html><style>body{background-color: red;}</style><body>no no no</body>".into()).await?;
            return Ok(true);
        }

        Ok(false)
    }

}

