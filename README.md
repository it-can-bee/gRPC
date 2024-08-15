# Rust gRPC
欢迎交流
我的主页：[博客园](https://www.cnblogs.com/live-passion)
## 项目简介
*调用方提供参数去调用 被调用方的调用机器上的程序这样的一种调用服务方式*
**gRPC是类似于Json、且为协议缓冲区大小不超过几兆字节的结构化数据提供序列化格式的一种机制**

RPC：给你一个RPC的ip+端口（这也是为什么编写客户端的时候需要先建立链接，就是在模仿rpc调用


proto定义的service就是rpc给你提供的方法，客户端对应的调用其方法就可以去拿到对应的响应数据）


然后用户可以利用它与服务器建立链接，然后再发送请求，调用对应的远程服务调用方法，然后服务器给客户端进行响应


## 运行
```
cargo run --bin server
cargo run --bin client
```

## 运行结果

```
服务端server

Recorder listening on [::1]:50050
request: Request {
    metadata: MetadataMap {
        headers: {
            "te": "trailers",
            "content-type": "application/grpc",
            "user-agent": "tonic/0.7.2",
        },
    },
    message: RecordRequest {
        user_name: "Jeffy",
        user_age: 25,
    },
    extensions: Extensions,
}

```
```
客户端client


Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.94s
     Running `target/debug/client`
Metadata response from server is: "User Jeffy is 25 old"

```
