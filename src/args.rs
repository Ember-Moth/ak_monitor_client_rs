use crate::get_info::get_hostname;
use clap::Parser;

/// Akile Monitor Rust Client
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 主机名，将展示在面板上，默认为本机 Hostname
    #[arg(short, long, default_value_t = get_hostname())]
    pub name: String,

    /// 主端地址，需要 ip:port (Demo: 192.168.111.1:3000)
    #[arg(short, long)]
    pub server: String,

    /// 在主端设置的 Auth Secret
    #[arg(short, long)]
    pub auth_secret: String,

    /// 采集间隔，单位为毫秒 (不建议低于 1000ms 与高于 5000ms)
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,

    /// 虚假倍率 (随手改一改，全世界算力都在你手上)
    #[arg(short, long, default_value_t = 1)]
    pub fake_times: u64,

    /// Debug 日志输出
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// 开启 TLS 支持 
    #[arg(long, default_value_t = false)]
    pub tls: bool,
}
impl Args {
    pub fn init_args() -> Args {
        let args: Args = Args::parse();
        args
    }
}
