/*客户端proto会自动生成*/
use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;
use log::{info, error};

pub mod records {
    tonic::include_proto!("records");
}

/*一般客户端都是有一块限定的数据库or访问权限的处理*/

#[tokio::main]
//cargo run --bin client
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志记录
    env_logger::init();

     // 建立与服务端的连接
     info!("Connecting to server...");
    let mut client  = RecorderClient::connect("http://[::1]:50050").await?;
    
    // 创建请求
    info!("Creating request...");
    let request = Request::new(
        RecordRequest {
            user_name: "Jeffy".to_string(),
            user_age: 25,
        }
    );
    
    // 发送请求并获取响应
    info!("Sending request...");
    let response= client.send_message(request).await?;

    // 服务端接收请求并且处理请求数据，构造响应返回给客户端，客户端再检查响应中的successful状态，成功了就取出message 
    if response.get_ref().successful {
        info!("Metadata response from server is: {:#?}", response.get_ref().message);
        println!("Metadata response from server is: {:#?}", response.get_ref().message);
    } else {
        error!("Request failed.");
    }

    Ok(())
}