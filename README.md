# Reproducer

In logs of imaginator we get:

```log
DispatchFailure(DispatchFailure { source: ConnectorError { kind: Other(None), source: hyper::Error(Http2, Error { kind: Reset(StreamId(1885071), REFUSED_STREAM, Remote) }), connection: Unknown } })
```

My idea is that it is a result of some race condition in hyper. I'd like to start a http2 server with `MAX_CONCURRENT_STREAMS` set to 1 and try to see what hyper does. 


### Repro


1. Start local nginx in one shell:


```bash
nix run .#defaultPackage.x86_64-linux -- -c "${PWD}/nginx.conf"
```

2. Test it in another shell:

```bash
curl --http2-prior-knowledge http://127.0.0.1:8080
```

3. Run hyper application

```bash
cd http2_client
RUST_LOG=DEBUG cargo run
```

