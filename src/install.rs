use crate::args::Args;
use crate::manage_utils;
use log::{error, info, warn};
use std::fs::File;
use std::io::Write;
use std::process::{exit, Command};
use std::{fs, io};

pub fn install_to_systemd(args: Args) {
    manage_utils::check_installed("/etc/systemd/system/akile_monitor_client.service");
    manage_utils::copy_binary();

    let command_args = manage_utils::generate_client_args(args);
    let service_string = SERVICE_TEMPLATE_SYSTEMD.replace("COMMAND_ARGS", &command_args);
    info!("最终服务文件: ");
    println!("{}", service_string);

    // 创建并写入新的服务文件
    let mut service_file = match File::create("/etc/systemd/system/akile_monitor_client.service") {
        Ok(tmp) => tmp,
        Err(e) => {
            error!(
                "无法新建 /etc/systemd/system/akile_monitor_client.service: {}",
                e
            );
            exit(1);
        }
    };

    match service_file.write_all(service_string.as_bytes()) {
        Ok(_) => {
            info!("成功写入 Systemd 配置文件")
        }
        Err(e) => {
            error!("无法写入 Systemd 配置文件: {}", e);
            exit(1);
        }
    }

    // 重新加载 Systemd 配置
    match Command::new("systemctl").arg("daemon-reload").output() {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功运行 systemctl daemon-reload");
            } else {
                error!("无法运行 systemctl daemon-reload")
            }
        }
        Err(e) => {
            error!("无法运行 systemctl daemon-reload: {}", e);
            exit(1);
        }
    }

    // 启动服务
    match Command::new("systemctl")
        .arg("start")
        .arg("akile_monitor_client.service")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功启动 Akile Monitor Client Service");
            } else {
                error!("无法启动 Akile Monitor Client Service")
            }
        }
        Err(e) => {
            error!("无法启动 Akile Monitor Client Service: {}", e);
            exit(1);
        }
    }
    // 询问用户是否开启开机自启动
    loop {
        info!("是否打开开机自启? (Y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if input == "Y" {
            match Command::new("systemctl")
                .arg("enable")
                .arg("akile_monitor_client.service")
                .output()
            {
                Ok(tmp) => {
                    if tmp.status.success() {
                        info!("成功打开开机自启");
                    } else {
                        error!("无法打开开机自启")
                    }
                }
                Err(e) => {
                    error!("无法打开开机自启: {}", e);
                    exit(1);
                }
            }
            info!("成功完成安装！你可以通过 `systemctl status akile_monitor_client.service` 以获取运行状态");
            exit(0);
        } else if input == "N" {
            info!("不打开, 退出程序");
            info!("成功完成安装！你可以通过 `systemctl status akile_monitor_client.service` 以获取运行状态");
            exit(1);
        } else {
            warn!("输入错误, 请重新输入 Y 或 N。");
        }
    }
}

pub fn uninstall_from_systemd() {
    info!("开始卸载 Akile Monitor Client Service");

    match Command::new("systemctl")
        .arg("stop")
        .arg("akile_monitor_client.service")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功停止 Akile Monitor Client Service");
            } else {
                warn!("无法停止 Akile Monitor Client Service")
            }
        }
        Err(e) => {
            warn!("无法停止 Akile Monitor Client Service: {}", e);
            exit(1);
        }
    }

    match Command::new("systemctl")
        .arg("disable")
        .arg("akile_monitor_client.service")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功关闭开机自启");
            } else {
                warn!("无法关闭开机自启")
            }
        }
        Err(e) => {
            warn!("无法关闭开机自启: {}", e);
            exit(1);
        }
    }

    match fs::remove_file("/etc/systemd/system/akile_monitor_client.service") {
        Ok(_) => {
            info!("成功删除 /etc/systemd/system/akile_monitor_client.service");
        }
        Err(e) => {
            warn!(
                "无法删除 /etc/systemd/system/akile_monitor_client.service: {}",
                e
            );
        }
    }
    match Command::new("systemctl").arg("daemon-reload").output() {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功运行 systemctl daemon-reload");
            } else {
                warn!("无法运行 systemctl daemon-reload")
            }
        }
        Err(e) => {
            warn!("无法运行 systemctl daemon-reload: {}", e);
        }
    }
    manage_utils::delete_binary();
    info!("成功卸载 Akile Monitor Client Service");
    exit(1);
}

pub fn install_to_openrc(args: Args) {
    manage_utils::check_installed("/etc/init.d/akile_monitor_client_rs");
    manage_utils::copy_binary();

    let command_args = manage_utils::generate_client_args(args);
    let service_string = SERVICE_TEMPLATE_OPENRC.replace("COMMAND_ARGS", &command_args);
    info!("最终服务文件: ");
    println!("{}", service_string);

    // 创建并写入新的服务文件
    let mut service_file = match File::create("/etc/init.d/akile_monitor_client_rs") {
        Ok(tmp) => tmp,
        Err(e) => {
            error!("无法新建 /etc/init.d/akile_monitor_client_rs: {}", e);
            exit(1);
        }
    };

    match service_file.write_all(service_string.as_bytes()) {
        Ok(_) => {
            info!("成功写入 OpenRC 配置文件")
        }
        Err(e) => {
            error!("无法写入 OpenRC 配置文件: {}", e);
            exit(1);
        }
    }

    match Command::new("chmod")
        .arg("+x")
        .arg("/etc/init.d/akile_monitor_client_rs")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功设置 /etc/init.d/akile_monitor_client_rs 为可执行文件");
            } else {
                warn!("无法设置 /etc/init.d/akile_monitor_client_rs 为可执行文件")
            }
        }
        Err(e) => {
            warn!(
                "无法设置 /etc/init.d/akile_monitor_client_rs 为可执行文件: {}",
                e
            );
        }
    }

    match Command::new("rc-update").arg("-u").output() {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功重载 OpenRC 服务");
            } else {
                warn!("无法重载 OpenRC 服务")
            }
        }
        Err(e) => {
            warn!("无法重载 OpenRC 服务: {}", e);
        }
    }

    // 询问用户是否开启开机自启动
    loop {
        info!("是否打开开机自启? (Y/N)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if input == "Y" {
            match Command::new("rc-update")
                .arg("add")
                .arg("akile_monitor_client_rs")
                .arg("default")
                .output()
            {
                Ok(tmp) => {
                    if tmp.status.success() {
                        info!("成功打开开机自启");
                    } else {
                        error!("无法打开开机自启")
                    }
                }
                Err(e) => {
                    error!("无法打开开机自启: {}", e);
                    exit(1);
                }
            }
            break;
        } else if input == "N" {
            info!("不打开开机自启");
            break;
        } else {
            warn!("输入错误, 请重新输入 Y 或 N。");
        }
    }

    warn!("请手动执行 `rc-service akile_monitor_client_rs start`!")
}

pub fn uninstall_from_openrc() {
    info!("开始卸载 Akile Monitor Client Service");

    match Command::new("rc-service")
        .arg("akile_monitor_client_rs")
        .arg("stop")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功停止 Akile Monitor Client Service");
            } else {
                warn!("无法停止 Akile Monitor Client Service")
            }
        }
        Err(e) => {
            warn!("无法停止 Akile Monitor Client Service: {}", e);
            exit(1);
        }
    }

    match Command::new("rc-update")
        .arg("del")
        .arg("akile_monitor_client_rs")
        .arg("default")
        .output()
    {
        Ok(tmp) => {
            if tmp.status.success() {
                info!("成功关闭开机自启");
            } else {
                warn!("无法关闭开机自启")
            }
        }
        Err(e) => {
            warn!("无法关闭开机自启: {}", e);
            exit(1);
        }
    }

    match fs::remove_file("/etc/init.d/akile_monitor_client_rs") {
        Ok(_) => {
            info!("成功删除 /etc/init.d/akile_monitor_client_rs");
        }
        Err(e) => {
            warn!("无法删除 /etc/init.d/akile_monitor_client_rs: {}", e);
        }
    }

    manage_utils::delete_binary();

    info!("成功卸载 Akile Monitor Client Service");
    exit(1);
}

const SERVICE_TEMPLATE_OPENRC: &str = r#"#!/sbin/openrc-run

command=/usr/bin/ak_monitor_client_rs
command_args="COMMAND_ARGS"
pidfile=/run/akile_monitor_client_rs.pid
description="Akile Monitor Client Service"

depend() {
    need net
    after firewall
}

start() {
    ebegin "Starting Akile Monitor Client Service"
    start-stop-daemon --start --background --make-pidfile --pidfile $pidfile --exec $command -- $command_args
    eend $?
}

stop() {
    ebegin "Stopping Akile Monitor Client Service"
    start-stop-daemon --stop --pidfile $pidfile
    eend $?
}

restart() {
    stop
    start
}
"#;

const SERVICE_TEMPLATE_SYSTEMD: &str = r#"[Unit]
Description=Akile Monitor Client Service
After=network.target

[Install]
WantedBy=multi-user.target

[Service]
Type=simple
ExecStart=/usr/bin/ak_monitor_client_rs COMMAND_ARGS
Restart=always
"#;
