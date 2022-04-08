
## VIACEP CACHE with RUST
---

So I finished the Learn Rust book recently and I realized that I never messed with threads and memory so why not try to solve a problem that I faced in my full-time job with it.
The whole idea is to cache responses from the VIACEP which contains some zipcode information in memory.

---
### Benchmarking with wrk.

```bash
Running 30s test @ http://127.0.0.1:8080/api/check/87035270
  8 threads and 256 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.94ms  707.82us   7.19ms   72.16%
    Req/Sec     6.51k     1.02k   32.71k    90.22%
  1555696 requests in 30.10s, 347.17MB read
Requests/sec:  51685.22
Transfer/sec:     11.53MB
```

This project was important to clarify why we use Redis or any solution as cache storage and how in memory can be difficult to implement when working with multithread software. For example, a NodeJS application due to how it works in production may need to run more than one instance to handle all requests, and using a Redis this cache can be shared by the instances and be a hot cache for newly spawned instances.  




