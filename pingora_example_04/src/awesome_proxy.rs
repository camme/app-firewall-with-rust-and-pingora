use std::collections::HashSet;

use pingora::{http::ResponseHeader, prelude::HttpPeer};
use pingora_proxy::{ProxyHttp, Session};
use async_trait::async_trait;
use pingora_core::Result;

pub struct AwesomeProxy { 
    banned_paths: HashSet<String>,
}

impl AwesomeProxy {

    pub fn new() -> Self {

        let list = "./lists/wordpress.fuzz.txt";
        let banned_paths: HashSet<String> = std::fs::read_to_string(list)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();

        Self { banned_paths }

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

        let req_header = session.req_header();
        let path = req_header.uri.path().strip_prefix("/").unwrap();

        println!("Requested path: {}", format!("/{}", path));

        if self.banned_paths.contains(path) {
            println!("Stop!!");
            session.respond_error_with_body(404, "no no no".into()).await?;
            return Ok(true);
        }

        Ok(false)
    }

}

