mod args;
mod build_message;
mod get_info;
mod install;
mod model;

use futures_util::future::err;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use std::process::exit;
use tokio_tungstenite::{
    connect_async, connect_async_tls_with_config, tungstenite::protocol::Message,
};

use crate::args::*;
use crate::build_message::{build_host, build_host_state, build_post_gziped_json};
use sysinfo::System;
use crate::install::{check_pid1, PID1};

#[tokio::main]
async fn main() {
    let args = Args_cli::init_args();

    if args.install && args.uninstall {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        error!("朋友你要上天啊，先安装再卸载 还是 先卸载再安装？");
        exit(1);
    } else if args.install {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        info!("检测到 --install 参数, 正在进入安装模式");
        match check_pid1() {
            PID1::Systemd => {
                info!("检测到 SystemD");
                install::install_to_systemd(args.clone().to_args());
            }
            PID1::OpenRC => {
                info!("检测到 OpenRC");
                install::install_to_openrc(args.clone().to_args());
            }
        }

    } else if args.uninstall {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        info!("检测到 --uninstall 参数, 正在进入卸载模式");
        match check_pid1() {
            PID1::Systemd => {
                info!("检测到 SystemD");
                install::uninstall_from_systemd();
            }
            PID1::OpenRC => {
                info!("检测到 OpenRC");
                install::uninstall_from_openrc();
            }
        }
    }

    let args = args.to_args();
    if args.debug {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        info!("服务正在启动")
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
        info!("服务正在启动")
    }

    loop {
        let (ws_stream, _) = if !args.tls {
            let url_str = format!("ws://{}/{}", args.server, args.monitor_path);
            info!("自动构建 WebSocket URL 为: {}", url_str);
            match connect_async(url_str.clone()).await {
                Ok(s) => s,
                Err(e) => {
                    log::error!("WebSocket 连接失败, 5 秒后重试: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            }
        } else {
            let url_str = format!("wss://{}/{}", args.server, args.monitor_path);
            info!("自动构建 WebSocket URL 为: {}", url_str);
            match connect_async_tls_with_config(url_str.clone(), None, false, None).await {
                Ok(s) => s,
                Err(e) => {
                    log::error!("WebSocket 连接失败, 5 秒后重试: {}", e);
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
                log::error!("WebSocket 发送验证信息失败, 5 秒后重试: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        }

        if let Some(Ok(Message::Text(message))) = read.next().await {
            if message == "auth success" {
                info!("WebSocket 连接验证成功");
            }
        } else {
            log::error!("WebSocket 连接验证失败, 也许是 Auth Secret 错误, 5 秒后重试");
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
                    log::error!("WebSocket 发送本机数据失败, 5 秒后重试: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    break;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(args.interval)).await
        }
    }
}
