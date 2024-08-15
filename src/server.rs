use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/*引入records package，不然上面的records import都不能生效*/
pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService{}

/*在trait中定义异步函数，绕过了Rust的编译器限制，因为异步得到的Future返回值没办法确定大小*/
#[tonic::async_trait]
/** service实现
 *  func：实现了在proto文件中定义的Recorder服务接口
 *  service：实现了在proto文件中定义的send_message方法
 *  param：接收到来自客户端的消息，然后返回一个包含了消息的RecordResponse
**/
impl Recorder for RecorderService {
    async fn send_message(
        &self,
        /*gRPC 请求对象*/
        request: Request<RecordRequest>
    ) -> Result<Response<RecordResponse>, Status> {
        println!("request: {:#?}", request);
        //into_inner获取请求消息的实体 也就是RecordRequest
        let req = request.into_inner();
        let response = RecordResponse {
            successful: true,
            message: format!("User {} is {} old", req.user_name, req.user_age).into()
        };
        /*将要响应的response包装进去Ok变体*/
        Ok(Response::new(response))
    }
}

/*启动server*/
/*服务端都是有一块限定的数据库的处理*/
#[tokio::main]
//获取请求 拿到参数 然后返回字符串给客户端
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    let recorder = RecorderService::default();
    println!("Recorder listening on {}", addr);

    //创建一个gRPC服务器
    Server::builder()
        //向服务器添加我们定义的服务
        .add_service(RecorderServer::new(recorder))
        //指定服务器监听的地址和端口 并启动服务器
        .serve(addr)
        .await?;

    Ok(())
}