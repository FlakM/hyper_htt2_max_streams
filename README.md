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
[2m2023-11-27T13:45:52.837323Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 0: Request { method: GET, uri: http://127.0.0.1:8080/0, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:52.837393Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::connect::http[0m[2m:[0m connecting to 127.0.0.1:8080
[2m2023-11-27T13:45:52.837482Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::connect::http[0m[2m:[0m connected to 127.0.0.1:8080
[2m2023-11-27T13:45:52.837504Z[0m [34mDEBUG[0m [2mh2::client[0m[2m:[0m binding client connection
[2m2023-11-27T13:45:52.837519Z[0m [34mDEBUG[0m [2mh2::client[0m[2m:[0m client connection bound
[2m2023-11-27T13:45:52.837534Z[0m [34mDEBUG[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mSettings { flags: (0x0), enable_push: 0, initial_window_size: 2097152, max_frame_size: 16384 }
[2m2023-11-27T13:45:52.837606Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m pooling idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:52.837707Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mSettings { flags: (0x0), max_concurrent_streams: 1, initial_window_size: 65536, max_frame_size: 16777215 }
[2m2023-11-27T13:45:52.837737Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mSettings { flags: (0x1: ACK) }
[2m2023-11-27T13:45:52.837752Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mWindowUpdate { stream_id: StreamId(0), size_increment: 2147418112 }
[2m2023-11-27T13:45:52.837765Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mWindowUpdate { stream_id: StreamId(0), size_increment: 5177345 }
[2m2023-11-27T13:45:52.837783Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(1), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:45:52.837911Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mSettings { flags: (0x1: ACK) }
[2m2023-11-27T13:45:52.837929Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::proto::settings[0m[2m:[0m received settings ACK; applying Settings { flags: (0x0), enable_push: 0, initial_window_size: 2097152, max_frame_size: 16384 }
[2m2023-11-27T13:45:55.841290Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(1), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:45:55.841352Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(1), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:45:57.838735Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Start sending requests
[2m2023-11-27T13:45:57.838845Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 0: Request { method: GET, uri: http://127.0.0.1:8080/0, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838881Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 1: Request { method: GET, uri: http://127.0.0.1:8080/1, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838894Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 2: Request { method: GET, uri: http://127.0.0.1:8080/2, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838902Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838914Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838930Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 4: Request { method: GET, uri: http://127.0.0.1:8080/4, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838923Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 3: Request { method: GET, uri: http://127.0.0.1:8080/3, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838935Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838939Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838946Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 6: Request { method: GET, uri: http://127.0.0.1:8080/6, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838954Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 7: Request { method: GET, uri: http://127.0.0.1:8080/7, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838962Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838965Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838970Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838972Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 8: Request { method: GET, uri: http://127.0.0.1:8080/8, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838964Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 5: Request { method: GET, uri: http://127.0.0.1:8080/5, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838979Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.838979Z[0m [34mDEBUG[0m [2mhttp2_client[0m[2m:[0m Sending request 9: Request { method: GET, uri: http://127.0.0.1:8080/9, version: HTTP/1.1, headers: {}, body: Empty }
[2m2023-11-27T13:45:57.838989Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.839009Z[0m [34mDEBUG[0m [2mhyper_util::client::legacy::pool[0m[2m:[0m reuse idle connection for ("http", 127.0.0.1:8080)
[2m2023-11-27T13:45:57.839007Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(3), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:00.842593Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(3), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:00.842665Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(3), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:00.842724Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(5), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:00.842893Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 0 took 3.004088076s and returned 200
[2m2023-11-27T13:46:03.846205Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(5), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:03.846266Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(5), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:03.846310Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(7), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:03.846474Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 1 took 6.007622548s and returned 200
[2m2023-11-27T13:46:06.849784Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(7), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:06.849846Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(7), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:06.849891Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(9), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:06.850044Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 4 took 9.011181236s and returned 200
[2m2023-11-27T13:46:09.853271Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(9), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:09.853320Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(9), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:09.853358Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(11), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:09.853489Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 2 took 12.014644283s and returned 200
[2m2023-11-27T13:46:12.856842Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(11), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:12.856906Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(11), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:12.856953Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(13), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:12.857097Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 6 took 15.018228717s and returned 200
[2m2023-11-27T13:46:15.860443Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(13), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:15.860505Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(13), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:15.860555Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(15), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:15.860726Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 3 took 18.02186555s and returned 200
[2m2023-11-27T13:46:18.864078Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(15), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:18.864142Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(15), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:18.864189Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(17), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:18.864358Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 7 took 21.025484497s and returned 200
[2m2023-11-27T13:46:21.867233Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(17), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:21.867296Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(17), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:21.867343Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(19), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:21.867501Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 8 took 24.028621267s and returned 200
[2m2023-11-27T13:46:24.871053Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(19), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:24.871116Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(19), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:24.871163Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_write[0m[2m:[0m send [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(21), flags: (0x5: END_HEADERS | END_STREAM) }
[2m2023-11-27T13:46:24.871315Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 9 took 27.03243268s and returned 200
[2m2023-11-27T13:46:27.874639Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mHeaders { stream_id: StreamId(21), flags: (0x4: END_HEADERS) }
[2m2023-11-27T13:46:27.874709Z[0m [34mDEBUG[0m [1mConnection[0m[1m{[0m[3mpeer[0m[2m=[0mClient[1m}[0m[2m:[0m [2mh2::codec::framed_read[0m[2m:[0m received [3mframe[0m[2m=[0mData { stream_id: StreamId(21), flags: (0x1: END_STREAM) }
[2m2023-11-27T13:46:27.874834Z[0m [32m INFO[0m [2mhttp2_client[0m[2m:[0m Request 5 took 30.035976017s and returned 200
```

```logs
# nginx logs:
❯ nix run .#defaultPackage.x86_64-linux -- -c "${PWD}/nginx.conf"
warning: Git tree '/home/flakm/programming/modivo/reset_repro' is dirty
127.0.0.1 - - [27/Nov/2023:14:45:55 +0100] "GET /0 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:00 +0100] "GET /0 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:03 +0100] "GET /1 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:06 +0100] "GET /4 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:09 +0100] "GET /2 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:12 +0100] "GET /6 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:15 +0100] "GET /3 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:18 +0100] "GET /7 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:21 +0100] "GET /8 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:24 +0100] "GET /9 HTTP/2.0" 200 8 "-" "-" "-"
127.0.0.1 - - [27/Nov/2023:14:46:27 +0100] "GET /5 HTTP/2.0" 200 8 "-" "-" "-"
```



