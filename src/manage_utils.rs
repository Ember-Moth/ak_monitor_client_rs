// For Install / Uninstall
use crate::args::Args;
use log::{error, info, warn};
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{env, fs};

pub fn check_system() {
    // 检查操作系统是否为 Linux
    if env::consts::OS != "linux" {
        error!("Install 功能仅适用于 Linux 系统");
        exit(1);
    }
}
pub fn check_root() {
    if env::var("USER") == Ok("root".to_string()) {
        info!("正在使用 root 用户");
    } else {
        error!("非 root 用户, 请使用 root 用户运行 Install 功能");
        exit(1);
    }
}

#[derive(Debug)]
pub enum PID1 {
    Systemd,
    OpenRC,
}
pub fn check_pid1() -> PID1 {
    let pid1_path = "/proc/1/comm";
    let mut pid1_file = match File::open(pid1_path) {
        Ok(file) => file,
        Err(e) => {
            error!("无法获取守护进程信息: {}", e);
            exit(1);
        }
    };
    let mut string_of_pid1 = String::new();
    match pid1_file.read_to_string(&mut string_of_pid1) {
        Ok(_) => {}
        Err(e) => {
            error!("无法读取 /proc/1/comm: {}", e);
            exit(1);
        }
    };

    let pid1 = if string_of_pid1 == "systemd\n" {
        PID1::Systemd
    } else if string_of_pid1 == "openrc\n" || string_of_pid1 == "init\n" {
        PID1::OpenRC
    } else {
        error!("无法识别守护进程, 退出！");
        exit(1);
    };

    println!("检测到守护进程: {:?}", pid1);

    pid1
}

pub fn check_installed(service_path: &str) {
    // 检查是否已存在相同名称的服务文件
    match fs::metadata(service_path) {
        Ok(_) => {
            error!("已存在相同名称的服务文件, 请先使用 `--uninstall` 参数卸载后再安装");
            exit(1);
        }
        Err(_) => {}
    }
}

pub fn generate_client_args(args: Args) -> String {
    format!(
        "--debug {} --tls {} -n \"{}\" -s \"{}\" -a \"{}\" -i {} -f {} --monitor-path \"{}\"",
        args.debug,
        args.tls,
        args.name,
        args.server,
        args.auth_secret,
        args.interval,
        args.fake_times,
        args.monitor_path
    )
}

pub fn copy_binary() {
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
}

pub fn delete_binary() {
    match fs::remove_file("/usr/bin/ak_monitor_client_rs") {
        Ok(_) => {
            info!("成功删除 /usr/bin/ak_monitor_client_rs");
        }
        Err(e) => {
            warn!("无法删除 /usr/bin/ak_monitor_client_rs: {}", e);
        }
    }
}
