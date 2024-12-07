mod args;
mod build_message;
mod get_info;
mod model;

use futures_util::{SinkExt, StreamExt};
use log::{debug, info};
use tokio_tungstenite::{connect_async, connect_async_tls_with_config, tungstenite::protocol::Message};

use crate::args::Args;
use crate::build_message::{build_host, build_host_state, build_post_gziped_json};
use sysinfo::System;

#[tokio::main]
async fn main() {
    let args = Args::init_args();

    if args.debug {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        info!("服务正在启动")
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
        info!("服务正在启动")
    }

    let url_str = format!("ws://{}/monitor", args.server);
    info!("自动构建 WebSocket URL 为: {}", url_str);

    loop {
        let (ws_stream, _) = if !args.tls {
            match connect_async(url_str.clone()).await {
            Ok(s) => s,
            Err(e) => {
                log::error!("WebSocket 连接失败，5 秒后重试: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
            }
        } else {
            match connect_async_tls_with_config(url_str.clone(), None, false, None).await {
                Ok(s) => s,
                Err(e) => {
                    log::error!("WebSocket 连接失败，5 秒后重试: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            }
        };

        let (mut write, mut read) = ws_stream.split();

        match write
            .send(Message::Text(args.auth_secret.to_string()))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                log::error!("WebSocket 发送验证信息失败，5 秒后重试: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        }

        if let Some(Ok(Message::Text(message))) = read.next().await {
            if message == "auth success" {
                info!("WebSocket 连接验证成功");
            }
        } else {
            log::error!("WebSocket 连接验证失败，也许是 Auth Secret 错误，5 秒后重试");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        }

        let mut sys = System::new();
        let host = build_host(args.name.to_string(), args.fake_times).await;
        loop {
            sys.refresh_cpu_all();
            let host_state = build_host_state(&sys, args.fake_times).await;
            let compressed_bytes = build_post_gziped_json(host.clone(), host_state).await;
            match write.send(Message::Binary(compressed_bytes.clone())).await {
                Ok(_) => {
                    debug!("WebSocket 数据上传成功");
                }
                Err(e) => {
                    log::error!("WebSocket 发送本机数据失败，5 秒后重试: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    break;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(args.interval)).await
        }
    }
}
