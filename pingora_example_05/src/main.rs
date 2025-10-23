mod awesome_proxy;
use awesome_proxy::AwesomeProxy;
use pingora::server::{configuration::ServerConf, Server};

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

    let mut proxy = pingora_proxy::http_proxy_service( &server.configuration, AwesomeProxy::new() );

    proxy.add_tcp("0.0.0.0:3000");

    server.add_service(proxy);

    println!("Listening on port 3000");

    server.run_forever();

}
