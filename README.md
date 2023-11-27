# Reproducer

In logs of imaginator we get:

```log
DispatchFailure(DispatchFailure { source: ConnectorError { kind: Other(None), source: hyper::Error(Http2, Error { kind: Reset(StreamId(1885071), REFUSED_STREAM, Remote) }), connection: Unknown } })
```

I'd like to start a http2 server with `MAX_CONCURRENT_STREAMS` set to 1 and try to see what hyper does. 


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
RUST_LOG=INFO cargo run
```

### Observed behaviour

1. Hyper oppens only one connection and reuses it:

```logs
# rust app logs:
2023-11-27T13:50:39.032471Z DEBUG http2_client: Sending request 0: Request { method: GET, uri: http://127.0.0.1:8080/0, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:39.032579Z DEBUG hyper_util::client::legacy::connect::http: connecting to 127.0.0.1:8080
2023-11-27T13:50:39.032705Z DEBUG hyper_util::client::legacy::connect::http: connected to 127.0.0.1:8080
2023-11-27T13:50:39.032760Z DEBUG h2::client: binding client connection
2023-11-27T13:50:39.032804Z DEBUG h2::client: client connection bound
2023-11-27T13:50:39.032830Z DEBUG h2::codec::framed_write: send frame=Settings { flags: (0x0), enable_push: 0, initial_window_size: 2097152, max_frame_size: 16384 }
2023-11-27T13:50:39.032915Z DEBUG hyper_util::client::legacy::pool: pooling idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:39.032954Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Settings { flags: (0x0), max_concurrent_streams: 1, initial_window_size: 65536, max_frame_size: 16777215 }
2023-11-27T13:50:39.032986Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Settings { flags: (0x1: ACK) }
2023-11-27T13:50:39.033009Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=WindowUpdate { stream_id: StreamId(0), size_increment: 2147418112 }
2023-11-27T13:50:39.033028Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=WindowUpdate { stream_id: StreamId(0), size_increment: 5177345 }
2023-11-27T13:50:39.033061Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(1), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:39.033178Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Settings { flags: (0x1: ACK) }
2023-11-27T13:50:39.033202Z DEBUG Connection{peer=Client}: h2::proto::settings: received settings ACK; applying Settings { flags: (0x0), enable_push: 0, initial_window_size: 2097152, max_frame_size: 16384 }
2023-11-27T13:50:42.036663Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(1), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:42.036771Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(1), flags: (0x1: END_STREAM) }
2023-11-27T13:50:44.033225Z  INFO http2_client: Start sending requests
2023-11-27T13:50:44.033409Z DEBUG http2_client: Sending request 0: Request { method: GET, uri: http://127.0.0.1:8080/0, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033425Z DEBUG http2_client: Sending request 1: Request { method: GET, uri: http://127.0.0.1:8080/1, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033466Z DEBUG http2_client: Sending request 2: Request { method: GET, uri: http://127.0.0.1:8080/2, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033497Z DEBUG http2_client: Sending request 3: Request { method: GET, uri: http://127.0.0.1:8080/3, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033528Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033538Z DEBUG http2_client: Sending request 4: Request { method: GET, uri: http://127.0.0.1:8080/4, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033582Z DEBUG http2_client: Sending request 6: Request { method: GET, uri: http://127.0.0.1:8080/6, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033544Z DEBUG http2_client: Sending request 5: Request { method: GET, uri: http://127.0.0.1:8080/5, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033646Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033562Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033542Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033595Z DEBUG http2_client: Sending request 7: Request { method: GET, uri: http://127.0.0.1:8080/7, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033601Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033600Z DEBUG http2_client: Sending request 8: Request { method: GET, uri: http://127.0.0.1:8080/8, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033639Z DEBUG http2_client: Sending request 9: Request { method: GET, uri: http://127.0.0.1:8080/9, version: HTTP/1.1, headers: {}, body: Empty }
2023-11-27T13:50:44.033763Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033555Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033657Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033669Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(3), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:44.033740Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:44.033790Z DEBUG hyper_util::client::legacy::pool: reuse idle connection for ("http", 127.0.0.1:8080)
2023-11-27T13:50:47.037350Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(3), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:47.037449Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(3), flags: (0x1: END_STREAM) }
2023-11-27T13:50:47.037500Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(5), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:47.037698Z  INFO http2_client: Request 0 took 3.004355455s and returned 200
2023-11-27T13:50:50.040898Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(5), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:50.040986Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(5), flags: (0x1: END_STREAM) }
2023-11-27T13:50:50.041052Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(7), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:50.041237Z  INFO http2_client: Request 6 took 6.007826991s and returned 200
2023-11-27T13:50:53.044478Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(7), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:53.044589Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(7), flags: (0x1: END_STREAM) }
2023-11-27T13:50:53.044645Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(9), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:53.044837Z  INFO http2_client: Request 3 took 9.011452756s and returned 200
2023-11-27T13:50:56.048124Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(9), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:56.048228Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(9), flags: (0x1: END_STREAM) }
2023-11-27T13:50:56.048269Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(11), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:56.048453Z  INFO http2_client: Request 1 took 12.01507805s and returned 200
2023-11-27T13:50:59.051631Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(11), flags: (0x4: END_HEADERS) }
2023-11-27T13:50:59.051730Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(11), flags: (0x1: END_STREAM) }
2023-11-27T13:50:59.051780Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(13), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:50:59.051981Z  INFO http2_client: Request 4 took 15.018586474s and returned 200
2023-11-27T13:51:02.055246Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(13), flags: (0x4: END_HEADERS) }
2023-11-27T13:51:02.055351Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(13), flags: (0x1: END_STREAM) }
2023-11-27T13:51:02.055407Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(15), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:51:02.055609Z  INFO http2_client: Request 8 took 18.022205809s and returned 200
2023-11-27T13:51:05.059052Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(15), flags: (0x4: END_HEADERS) }
2023-11-27T13:51:05.059145Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(15), flags: (0x1: END_STREAM) }
2023-11-27T13:51:05.059198Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(17), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:51:05.059384Z  INFO http2_client: Request 2 took 21.025999839s and returned 200
2023-11-27T13:51:08.062558Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(17), flags: (0x4: END_HEADERS) }
2023-11-27T13:51:08.062666Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(17), flags: (0x1: END_STREAM) }
2023-11-27T13:51:08.062739Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(19), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:51:08.062909Z  INFO http2_client: Request 5 took 24.029518801s and returned 200
2023-11-27T13:51:11.066129Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(19), flags: (0x4: END_HEADERS) }
2023-11-27T13:51:11.066227Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(19), flags: (0x1: END_STREAM) }
2023-11-27T13:51:11.066281Z DEBUG Connection{peer=Client}: h2::codec::framed_write: send frame=Headers { stream_id: StreamId(21), flags: (0x5: END_HEADERS | END_STREAM) }
2023-11-27T13:51:11.066463Z  INFO http2_client: Request 7 took 27.033060608s and returned 200
2023-11-27T13:51:14.069986Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Headers { stream_id: StreamId(21), flags: (0x4: END_HEADERS) }
2023-11-27T13:51:14.070086Z DEBUG Connection{peer=Client}: h2::codec::framed_read: received frame=Data { stream_id: StreamId(21), flags: (0x1: END_STREAM) }
2023-11-27T13:51:14.070195Z  INFO http2_client: Request 9 took 30.036793339s and returned 200
```

```logs
# nginx logs:
❯ nix run .#defaultPackage.x86_64-linux -- -c "${PWD}/nginx.conf"
127.0.0.1 - - [27/Nov/2023:14:50:42 +0100] "GET /0 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:50:47 +0100] "GET /0 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:50:50 +0100] "GET /6 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:50:53 +0100] "GET /3 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:50:56 +0100] "GET /1 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:50:59 +0100] "GET /4 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:51:02 +0100] "GET /8 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:51:05 +0100] "GET /2 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:51:08 +0100] "GET /5 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:51:11 +0100] "GET /7 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:51:14 +0100] "GET /9 HTTP/2.0" 200 8 "-" "-" "-"
```



