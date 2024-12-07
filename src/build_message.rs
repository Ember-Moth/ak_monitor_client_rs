use crate::get_info::*;
use crate::model::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;

pub async fn build_host(name: String, fake_times: u64) -> Host {
    let (platform, kernel_version) = get_platform_info().await;
    let (cpus, arch, virt) = get_cpu_info().await;
    let boot_time = get_boot_time();
    let mem_info = get_mem_info(System::new()).await;

    Host {
        Name: name,
        Platform: platform,
        PlatformVersion: kernel_version,
        CPU: cpus,
        MemTotal: mem_info.0 * fake_times,
        SwapTotal: mem_info.3 * fake_times,
        Arch: arch,
        Virtualization: virt,
        BootTime: boot_time,
    }
}

pub async fn build_host_state(sys: &System, fake_times: u64) -> HostState {
    let cpus = sys.cpus();
    let cpu_usage = get_cpu_usage(cpus).await;

    let mem_info = get_mem_info(System::new()).await;

    let network_info = get_network_info().await.unwrap_or((0, 0, 0, 0));

    let load = get_uptime_info().await.unwrap_or((0, 0.0, 0.0, 0.0));

    HostState {
        CPU: cpu_usage,
        MemUsed: mem_info.2 * fake_times,
        SwapUsed: mem_info.5 * fake_times,
        NetInTransfer: network_info.0 * fake_times,
        NetOutTransfer: network_info.1 * fake_times,
        NetInSpeed: network_info.2 * fake_times,
        NetOutSpeed: network_info.3 * fake_times,
        Uptime: load.0,
        Load1: format!("{:.2}", load.1 * fake_times as f64)
            .parse()
            .unwrap(),
        Load5: format!("{:.2}", load.2 * fake_times as f64)
            .parse()
            .unwrap(),
        Load15: format!("{:.2}", load.3 * fake_times as f64)
            .parse()
            .unwrap(),
    }
}

pub async fn build_post_gziped_json(host: Host, host_state: HostState) -> Vec<u8> {
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let push_json = PushJson {
        Host: host,
        State: host_state,
        TimeStamp: time_stamp as i64,
    };
    let text = serde_json::to_string(&push_json).unwrap();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(text.as_ref()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    compressed_bytes
}
