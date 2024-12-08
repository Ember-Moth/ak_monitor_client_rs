# ak-monitor-client-rs

该项目是一个 `高性能的`、`低占用的`、`配置项更多的`、`可自由调整作弊倍率的` 第三方 [Akile Monitor](https://github.com/akile-network/akile_monitor) 客户端

该项目并非官方项目, 出现的任何问题本人概不负责

## 一键脚本

- 交互模式
  ```bash
  wget -O setup-client-rs.sh "https://ghp.ci/https://raw.githubusercontent.com/GenshinMinecraft/ak_monitor_client_rs/refs/heads/main/setup-client-rs.sh" && chmod +x setup-client-rs.sh && sudo bash ./setup-client-rs.sh
  ```
- 直接传入参数
  ```bash
  wget -O setup-client-rs.sh "https://ghp.ci/https://raw.githubusercontent.com/GenshinMinecraft/ak_monitor_client_rs/refs/heads/main/setup-client-rs.sh" && chmod +x setup-client-rs.sh
  # 四个参数依次为 主端地址 AuthSecret 主机名 是否开启 TLS
  # 其中，前两者是必须的，后两者可选，TLS 用 0 / 1 表示
  bash ./setup-client-rs.sh "192.168.111.3090" "GenshinMinecraft"
  bash ./setup-client-rs.sh "192.168.111.3090" "GenshinMinecraft" "GenArch" 1
  ```

## 下载

请前往本项目的 [Release](https://github.com/GenshinMinecraft/ak_monitor_client_rs/releases/tag/latest) 处下载

每次 Push 将会自动构建并推送到该 Tag, 所以直接右键获取的文件链接是永久性的

挑选对应架构的压缩文件下载解压上传至服务器即可使用

## 使用

不论何时何地, 你都可以使用 `--help` 参数以查阅帮助信息

```
Akile Monitor Client By Rust

Usage: 

Options:
  -n, --name <NAME>                  主机名, 将展示在面板上, 默认为本机 Hostname
  -s, --server <SERVER>              主端地址, (Demo: 192.168.111.1:3000)
  -a, --auth-secret <AUTH_SECRET>    在主端设置的 Auth Secret
  -i, --interval <INTERVAL>          采集间隔, 单位为 ms
  -f, --fake-times <FAKE_TIMES>      虚假倍率
      --debug <DEBUG>                Debug 日志输出 [possible values: true, false]
      --tls <TLS>                    开启 TLS 支持 [possible values: true, false]
      --monitor-path <MONITOR_PATH>  Monitor 路径
      --install                      Install 模式
      --uninstall                    Uninstall 模式
  -h, --help                         Print help
  -V, --version                      Print version
```

### Examples

下列例子均以 `GenshinMinecraft` 为 Auth Secret 连接至 `192.168.111.1:3090` 为例

- 连接, 并自动获取主机名:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft
```

- 连接, 并设置主机名为 `GenArch`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -n GenArch
```

- 连接, 并设置设置虚假倍率为 `2`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -f 2
```

- 连接, 并设置上报间隔时间为 `2400ms`: 
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -i 2400
```

- 连接, 并设置上报间隔时间为 `2400ms`, 设置设置虚假倍率为 `2`, 设置主机名为 `GenArch`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -n GenArch -f 2 -i 2400
```

- 安装并连接, 并设置上报间隔时间为 `2400ms`, 设置设置虚假倍率为 `2`, 设置主机名为 `GenArch`:
```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft -n GenArch -f 2 -i 2400 --install
```
- 卸载:
```bash
./ak_monitor_client_rs --uninstall
```
## 与原版比较

既然是重写，那就必须有比原版好的地方

测试环境均为 `Redmi Book Pro 15 锐龙版 + Arch Linux`

### 空间占用

![f9a352a41fb743794b644cda162caa85.png](https://ice.frostsky.com/2024/12/08/f9a352a41fb743794b644cda162caa85.png)

上为原版，下为重写的 Rust 版本

可见 Binary 的占用两者相差约 **39** 倍 ~~(其实我也不太知道原版作为一个监控端是怎么编译出来 24M 的)~~

### 内存占用

- 原版
  ![alt text](https://blog.c1oudf1are.eu.org/p/akile-monitor-client-rs/image-1.png)

- 重写的 Rust 版本
  ![2d8937c15aad5b6be95d53cd67a437b2.png](https://ice.frostsky.com/2024/12/08/2d8937c15aad5b6be95d53cd67a437b2.png)

可见，原版占用约为 `18MiB`，重写的 Rust 版本占用约为 `3.5MiB`

两者相差约 5 倍，虽然这点内存对于一个正常的小鸡来说无伤大雅，但能少一点就少一点

PS: Arm64 架构内存更少，约 `1.76MiB`，

### 便于配置

原版的配置十分麻烦 (即使有一键脚本)，需要手动配置 `client.json` 来指定连接的主端

而使用重写的 Rust 版本，则只需要在命令行上设置即可，Demo:

```bash
./ak_monitor_client_rs -s 192.168.111.1:3090 -a GenshinMinecraft
```

### 功能更多

- 美观输出:
  原版仅有最普通的控制台输出，而 Rust 版本则使用了丰富的 `log` 库来优化输出 ~~(虽然也没多少人看)~~
- 虚假倍率:
  你是否想让你的小鸡拥有顶天立地的算力？虚假倍率来助你:

  - 总物理内存
  - 总 Swap 内存
  - 已用物理内存
  - 已用 Swap 内存
  - 网络进出总量
  - 网络进出速度
  - Load 1 / 5 / 15

  以上的这些都可以随心所欲地自定义倍率，拳打太湖之光，脚踢前沿
- 自定义间隔时间: 这个功能我觉得是没啥用的，但是还是加上了。也就是自定义数据上报的间隔
- 自动获取主机名: 懒得填写主机名？这功能能帮你自动获取主机的 Hostname
- 自动重连: 原版只要连不上主端，就会直接退出，Rust 版即使断连也会在五秒之后自动尝试重连


## 保活

**PS: 该功能仅仅在 `Linux` 下且使用 `SystemD` / `OpenRC` 的发行版下可用**

需要保活？ 来让 Install 功能帮你！

你只需要在启动命令的基础上加上 `--install` 参数, 就会进入 Install 模式并**自动根据参数生成服务文件**

![360335e73aa8c82806089336754039bb.png](https://ice.frostsky.com/2024/12/07/360335e73aa8c82806089336754039bb.png)

这样便完成了安装保活

## 卸载

保活完了想卸载？ 来让 Uninstall 功能帮你！

你只需要直接在可执行文件后加上 `--uninstall` 参数, 就会进入 Uninstall 模式并**自动删除服务文件**

```bash
./ak_monitor_client_rs --uninstall
```

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
