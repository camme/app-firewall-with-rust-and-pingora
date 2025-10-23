use std::{collections::HashSet, net::IpAddr};

use pingora::{http::ResponseHeader, prelude::HttpPeer};
use pingora_proxy::{ProxyHttp, Session};
use async_trait::async_trait;
use pingora_core::Result;
use reqwest::blocking::get;

pub struct AwesomeProxy { 
    banned_ips: HashSet<String>,
}

pub const BAD_IP_LIST: &str = "https://raw.githubusercontent.com/mitchellkrogza/nginx-ultimate-bad-bot-blocker/refs/heads/master/_generator_lists/bad-ip-addresses.list";

impl AwesomeProxy {

    pub fn new() -> Self {

        let list = get(BAD_IP_LIST)
            .unwrap()
            .text()
            .unwrap();

        let banned_ips: HashSet<String> = list
            .lines()
            .map(|line| line.to_string())
            .collect();

        Self { banned_ips }

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
        let ip_string = match ip {
            Some(ip) => ip.to_string(),
            None => "unknown".to_string(),
        };

        if self.banned_ips.contains(&ip_string) {
            let header = ResponseHeader::build(401, None).unwrap();
            session.set_keepalive(None);
            session.write_response_header(Box::new(header), false) .await?;
            session.write_response_body(Some("No no no!".to_string().into()), true).await?;
            return Ok(true);
        }

        Ok(false)
    }

}

