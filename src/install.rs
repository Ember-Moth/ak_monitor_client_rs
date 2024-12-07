use crate::args::Args;
use log::{error, info, warn};
use std::fs::File;
use std::io::{Read, Write};
use std::process::{exit, Command};
use std::{env, fs, io};

#[derive(Debug)]
enum PID1 {
    Systemd,
    OpenRC
}
pub fn check_pid1() -> PID1 {
    let pid1_path = "/proc/1/comm";
    let mut pid1_file = match File::open(pid1_path) {
        Ok(file) => file,
        Err(e) => {
            error!("无法获取守护进程信息: {}", e);
            exit(1);
        },
    };
    let mut string_of_pid1 = String::new();
    match pid1_file.read_to_string(&mut string_of_pid1) {
        Ok(_) => {}
        Err(e) => {
            error!("无法读取 /proc/1/comm: {}", e);
            exit(1);
        },
    };

    let pid1 = if string_of_pid1 == "systemd" {
        PID1::Systemd
    } else if string_of_pid1 == "openrc" || string_of_pid1 == "init" {
        PID1::OpenRC
    } else { 
        error!("无法识别守护进程, 退出！");
        exit(1);
    };

    println!("检测到守护进程: {:?}", pid1);

    pid1
}

pub fn install_to_systemd(args: Args) {
    // 检查操作系统是否为 Linux
    if env::consts::OS != "linux" {
        error!("Install 功能仅适用于 Linux 系统");
        exit(1);
    }

    if env::var("USER") == Ok("root".to_string()) {
        info!("正在使用 root 用户");
    } else {
        error!("非 root 用户, 请使用 root 用户运行 Install 功能");
        exit(1);
    }

    // 检查是否已存在相同名称的服务文件
    match fs::metadata("/etc/systemd/system/akile_monitor_client.service") {
        Ok(_) => {
            error!("已存在相同名称的服务文件, 请先使用 `--uninstall` 参数卸载后再安装");
            exit(1);
        },
        Err(_) => {}
    }

    // 复制可执行文件到 /usr/bin
    match env::current_exe() {
        Ok(path_to_bin) => {
            if path_to_bin.to_str().unwrap() == "/usr/bin/ak_monitor_client_rs" {
                info!("无需复制可执行文件");
            } else {
                match fs::copy(path_to_bin, "/usr/bin/ak_monitor_client_rs") {
                    Ok(_) => {
                        info!("成功将可执行文件复制到 /usr/bin/ak_monitor_client_rs");
                    }
                    Err(e) => {
                        error!(
                            "无法将可执行文件复制到 /usr/bin/ak_monitor_client_rs: {}",
                            e
                        );
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            error!("无法获取可执行文件路径: {}", e);
            exit(1);
        }
    }

    let template = r#"[Unit]
Description=Akile Monitor Client Service
After=network.target

[Install]
WantedBy=multi-user.target

[Service]
Type=simple
ExecStart=COMMAND
Restart=always
"#;
    let debug_ = if args.debug { "--debug" } else { "" };

    let tls = if args.tls { "--tls" } else { "" };

    let command = format!(
        "/usr/bin/ak_monitor_client_rs {} {} -n \"{}\" -s \"{}\" -a \"{}\" -i {} -f {} --monitor-path \"{}\"",
        debug_,
        tls,
        args.name,
        args.server,
        args.auth_secret,
        args.interval,
        args.fake_times,
        args.monitor_path
    );

    let service_string = template.replace("COMMAND", &command);
    info!("最终服务文件: ");
    println!("{}", service_string);

    // 删除旧的服务文件
    match fs::remove_file("/etc/systemd/system/akile_monitor_client.service") {
        Ok(_) => {
            info!("成功删除 /etc/systemd/system/akile_monitor_client.service");
        }
        Err(e) => {
            error!(
                "无法删除 /etc/systemd/system/akile_monitor_client.service: {}",
                e
            );
        }
    }

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

    if env::var("USER") == Ok("root".to_string()) {
        info!("正在使用 root 用户");
    } else {
        error!("非 root 用户, 请使用 root 用户运行 Uninstall 功能");
        exit(1);
    }

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
    match fs::remove_file("/usr/local/bin/akile_monitor_client_rs") {
        Ok(_) => {
            info!("成功删除 /usr/local/bin/akile_monitor_client_rs");
        }
        Err(e) => {
            warn!("无法删除 /usr/local/bin/akile_monitor_client_rs: {}", e);
        }
    }
    info!("成功卸载 Akile Monitor Client Service");
    exit(1);
}