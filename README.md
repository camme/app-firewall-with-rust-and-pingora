# Examples from the presentaion "Stop the Noise: Build Your Own Smart App Firewall with Rust and Pingora"

These are the examples shown in my session about building a guard, or protector, that you can put infront of any api/service/website.

1. pingora_example_01 - This example is just an completly "transparent" proxy, ie it just reversse proxies all traffic.
2. pingora_example_02 - This example shows how to block a single IP number
3. pingora_example_03 - This example downloads the bad bot list of ips from another repo and creates a hashset to complare to the incomping ip of the request
4. pingora_example_04 - This example uses a seclist for wordpress to stop all traffic to those paths
5. pingora_example_05 - This example shows how to use the pingora rate limit to, well, have a rate limiter.
