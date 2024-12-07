# ak-monitor-client-rs

该项目是一个 `高性能的`、`低占用的`、`配置项更多的`、`可自由调整作弊倍率的` 第三方 [Akile Monitor](https://github.com/akile-network/akile_monitor) 客户端

该项目并非官方项目，出现的任何问题本人概不负责

## 下载

请前往本项目的 [Action](https://github.com/GenshinMinecraft/ak_monitor_client_rs/actions) 处下载，请注意 Github Action 下载需 Github 账号

挑选对应架构的压缩文件下载解压上传至服务器即可使用

## 使用

不论何时何地，你都可以使用 `--help` 参数以查阅帮助信息

```
Akile Monitor Rust Client

Usage: 

Options:
  -n, --name <NAME>                主机名，将展示在面板上，默认为本机 Hostname [default: GenArch]
  -s, --server <SERVER>            主端地址，需要 ip:port (Demo: 192.168.111.1:3000)
  -a, --auth-secret <AUTH_SECRET>  在主端设置的 Auth Secret
  -i, --interval <INTERVAL>        采集间隔，单位为毫秒 (不建议低于 1000ms 与高于 5000ms) [default: 1000]
  -f, --fake-times <FAKE_TIMES>    虚假倍率 (随手改一改，全世界算力都在你手上) [default: 1]
      --debug                      Debug 日志输出
      --tls                        开启 TLS 支持
  -h, --help                       Print help
  -V, --version                    Print version
```

- `--name`： (非必须，建议设置) 主机名，将展示在面板上，默认为本机 Hostname
- `--server`： (必须) 主端地址，需要 ip:port (Demo: 192.168.111.1:3000)
- `--auth-secret`： (必须) 在主端设置的 Auth Secret
- `--interval`： (非必须，不建议设置) 采集间隔，单位为毫秒 (不建议低于 1000ms 与高于 5000ms)
- `--fake-times`： (非必须，不建议设置) 虚假倍率 (随手改一改，全世界算力都在你手上)
- `--debug`： (非必须) Debug 日志输出
- `--tls`： (非必须) 开启 TLS 支持
- `--help`： 查看帮助

下列例子均以 `GenshinMinecraft` 为 Auth Secret 连接至 `192.168.111.1:3090` 为例

- 连接，并自动获取主机名:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft
```

- 连接，并设置主机名为 `GenArch`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -n GenArch
```

- 连接，并设置设置虚假倍率为 `2`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -f 2
```

- 连接，并设置上报间隔时间为 `2400ms`: 
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -i 2400
```

- 连接，并设置上报间隔时间为 `2400ms`，设置设置虚假倍率为 `2`，设置主机名为 `GenArch`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -n GenArch -f 2 -i 2400
```

## 与原版相比之优势

- Binary 可执行文件大小:
  
  ![2e1ed8d14b7924297aa65cb62013453c.png](https://ice.frostsky.com/2024/12/05/2e1ed8d14b7924297aa65cb62013453c.png)
  
  上为原版，下为 Rust 版本，两者相差约 15 倍 (均为 Linux amd64)

- 性能表现
  原版:
  ![7fec014e900e612a8a90b1efe4c6cd84.png](https://ice.frostsky.com/2024/12/05/7fec014e900e612a8a90b1efe4c6cd84.png)

  Rust 版本:
  ![6b0a65cbc6659ac1d4eff212ce29e2a6.png](https://ice.frostsky.com/2024/12/05/6b0a65cbc6659ac1d4eff212ce29e2a6.png)
  
  可见，原版占用内存约为 `18M`，而 Rust 版本占用内存约为 `4M`，相差约 4.5 倍  (Arch Linux Amd64 下测试)

- 便于配置
  官方版本需要手动修改 `client.json` 文件，不便于配置，Rust 版本直接通过命令行读取参数，更加便捷
- 更多功能
  在原版的基础上，增加了 `虚假倍率`、`自定义间隔时间` 等功能

## 保活

目前，大部分 Linux 发行版均已经使用 SystemD 作为 Pid 1，所以本文只使用 SystemD

用你喜欢的编辑器打开 `/etc/systemd/system/akile_monitor_client.service`

填入: 
```
Description=Cloudflare Speedtest Slave
After=network.target

[Install]
WantedBy=multi-user.target

[Service]
Type=simple
ExecStart=/path/to/ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft 
Restart=always
```

随后重载并开启本服务即可:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now akile_monitor_client
```

这样便完成了安装保活

## 鸣谢
- [Akile Monitor](https://github.com/akile-network/akile_monitor)
- [GenshinMinecraft 的 nezha-agent-rs 项目](https://github.com/GenshinMinecraft/nezha-agent-rs)

## 协议

本项目根据 `WTFPL` 协议开源

```license
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                   Version 2, December 2004

Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.

           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
  TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

 0. You just DO WHAT THE FUCK YOU WANT TO.
```
