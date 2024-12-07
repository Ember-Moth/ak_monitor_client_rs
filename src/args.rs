use std::process::exit;
use crate::get_info::get_hostname;
use clap::Parser;

/// Akile Monitor Rust Client
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args_cli {
    /// 主机名, 将展示在面板上, 默认为本机 Hostname
    #[arg(short, long)]
    pub name: Option<String>,

    /// 主端地址, (Demo: 192.168.111.1:3000)
    #[arg(short, long)]
    pub server: Option<String>,

    /// 在主端设置的 Auth Secret
    #[arg(short, long)]
    pub auth_secret: Option<String>,

    /// 采集间隔, 单位为 ms
    #[arg(short, long)]
    pub interval: Option<u64>,

    /// 虚假倍率
    #[arg(short, long)]
    pub fake_times: Option<u64>,

    /// Debug 日志输出
    #[arg(long)]
    pub debug: Option<bool>,

    /// 开启 TLS 支持
    #[arg(long)]
    pub tls: Option<bool>,

    /// Monitor 路径
    #[arg(long)]
    pub monitor_path: Option<String>,

    /// Install 模式
    #[arg(long, default_value_t = false)]
    pub install: bool,

    /// Uninstall 模式
    #[arg(long, default_value_t = false)]
    pub uninstall: bool,
}
impl Args_cli {
    pub fn init_args() -> Args_cli {
        let args: Args_cli = Args_cli::parse();
        args
    }

    pub fn to_args(self) -> Args {
        let name = self.name.unwrap_or(get_hostname());
        let server = match self.server {
            Some(s) => s,
            None => {
                eprintln!("请输入 --server 参数");
                exit(1);
            },
        };
        let auth_secret = match self.auth_secret {
            Some(s) => s,
            None => {
                eprintln!("请输入 --auth_secret 参数");
                exit(1);
            },
        };
        let interval = self.interval.unwrap_or(1000);
        let fake_times = self.fake_times.unwrap_or(1);
        let debug = self.debug.unwrap_or(false);
        let tls = self.tls.unwrap_or(false);
        let monitor_path = self.monitor_path.unwrap_or("monitor".to_string());
        Args {
            name,
            server,
            auth_secret,
            interval,
            fake_times,
            debug,
            tls,
            monitor_path,
            install: self.install,
            uninstall: self.uninstall,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Args {
    pub name: String,
    pub server: String,
    pub auth_secret: String,
    pub interval: u64,
    pub fake_times: u64,
    pub debug: bool,
    pub tls: bool,
    pub monitor_path: String,
    pub install: bool,
    pub uninstall: bool,
}