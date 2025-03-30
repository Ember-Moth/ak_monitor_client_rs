#!/bin/bash
echo "  _____   _    _   _____  _______   _____   _____   _______  _    _  ______   ____   ______   _____  _______ "
echo " |  __ \ | |  | | / ____||__   __| |_   _| / ____| |__   __|| |  | ||  ____| |  _ \ |  ____| / ____||__   __| "
echo " | |__) || |  | || (___     | |      | |  | (___      | |   | |__| || |__    | |_) || |__   | (___     | | "
echo " |  _  / | |  | | \___ \    | |      | |   \___ \     | |   |  __  ||  __|   |  _ < |  __|   \___ \    | | "
echo " | | \ \ | |__| | ____) |   | |     _| |_  ____) |    | |   | |  | || |____  | |_) || |____  ____) |   | | "
echo " |_|  \_\ \____/ |_____/    |_|    |_____||_____/     |_|   |_|  |_||______| |____/ |______||_____/    |_| "

if [ "$EUID" -ne 0 ]; then
	echo "请使用 Root 用户运行"
	exit 1
fi

if [ "$1" = "--uninstall" ]; then
	/usr/bin/ak_monitor_client_rs --uninstall
	exit 0
fi

if command -v wget >/dev/null; then
	echo "已安装 wget"
else
	echo "未安装 wget，正在安装..."
	if command -v apt-get >/dev/null; then
		apt-get update && apt-get install -y wget
	elif command -v yum >/dev/null; then
		yum update -y && yum install -y wget
	elif command -v apk >/dev/null; then
		apk update && apk add wget
	elif command -v dnf >/dev/null; then
		dnf update -y && dnf install -y wget
	elif command -v zypper >/dev/null; then
		zypper refresh && zypper install wget
	elif command -v pkg >/dev/null; then
		pkg install wget
	elif command -v emerge >/dev/null; then
		emerge wget
	elif command -v pacman >/dev/null; then
		pacman -Syu wget
	elif command -v xbps-install >/dev/null; then
		xbps-install -Sy wget
	else
		echo "不支持的发行版或包管理器"
		exit 1
	fi
fi

if pgrep -x "systemd" >/dev/null; then
	PID1="systemd"
elif [ -f /run/openrc/softlevel ]; then
	PID1="openrc"
else
	echo "非 Systemd / OpenRC 系统, 暂不支持, 退出"
	exit 1
fi

ARCH=$(uname -m)

case "$ARCH-$PID1" in
x86_64-systemd)
	CLIENT_FILE="ak_monitor_client_rs-linux_x86_64_gnu"
	;;
x86_64-openrc)
	CLIENT_FILE="ak_monitor_client_rs-linux_x86_64_musl"
	;;
aarch64-systemd)
	CLIENT_FILE="ak_monitor_client_rs-linux_aarch64_gnu"
	;;
aarch64-openrc)
	CLIENT_FILE="ak_monitor_client_rs-linux_aarch64_musl"
	;;
*)
	echo "本脚本暂不支持 $ARCH 架构或 $PID1 守护进程"
	exit 1
	;;
esac

if [ $# != 0 ]; then
	echo "快速安装模式"
	# 初始化变量为空值
	server=""
	auth_token=""
	name=""
	tls=""

	# 检查并分配参数到对应的变量中
	assign_params() {
		local index=1
		for param in "$@"; do
			# 如果参数为空字符串，则停止处理剩余参数
			if [ -z "$param" ]; then
				break
			fi

			case $index in
			1) server=$param ;;
			2) auth_token=$param ;;
			3) name=$param ;;
			4) tls=$param ;;
			*) break ;;
			esac
			((index++))
		done
	}

	# 分配参数
	assign_params "$@"

	if [ $# -lt 2 ]; then
		echo "快速安装时至少提供两个参数"
		exit 1
	fi

	wget -O /usr/bin/ak_monitor_client_rs "https://ghfast.top/https://github.com/GenshinMinecraft/ak_monitor_client_rs/releases/download/latest/$CLIENT_FILE"
	chmod 777 /usr/bin/ak_monitor_client_rs
	# 检查 tls 是否为 0 或 1 并且 name 是否为空或非空
	if [[ "$tls" == "0" || -z "$tls" && -n "$name" ]]; then
		# 当 tls 是 0 且 name 不为空时执行的命令
		/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -n "$name" --install
	elif [[ "$tls" == "1" && -n "$name" ]]; then
		# 当 tls 是 1 且 name 不为空时执行的命令
		/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -n "$name" --tls true --install
	elif [[ "$tls" == "0" || -z "$tls" && -z "$name" ]]; then
		# 当 tls 是 0 且 name 为空时执行的命令
		/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" --install
	elif [[ "$tls" == "1" && -z "$name" ]]; then
		# 当 tls 是 1 且 name 为空时执行的命令
		/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" --tls true --install
	else
		echo "TLS 参数必须是 0 / 1 "
		exit 1
	fi
	exit 0
fi

wget -O /usr/bin/ak_monitor_client_rs "https://ghfast.top/https://github.com/GenshinMinecraft/ak_monitor_client_rs/releases/download/latest/$CLIENT_FILE-upxed"
chmod 777 /usr/bin/ak_monitor_client_rs

get_input() {
	local prompt=$1
	local default_value=$2
	local var_name=$3

	read -p "$prompt" input
	if [[ -z "$input" ]]; then
		# 如果用户没有输入，默认值为空则提示重新输入
		if [[ -z "$default_value" ]]; then
			echo "此选项是必填的，请输入有效值。"
			get_input "$prompt" "$default_value" "$var_name"
		else
			declare -g $var_name="$default_value"
		fi
	else
		declare -g $var_name="$input"
	fi
}

# 必填项
echo "下面是参数填写环节，除了 Server 与 Auth Token 外，均可直接回车默认值"
get_input "请输入 Server 地址 (只需要 IP[DOMAIN]:PORT): " "" server
get_input "请输入 Auth Token: " "" auth_token

# 可选项，带默认值
get_input "请输入 name (默认自动获取): " "default_name" name
get_input "是否开启 TLS (0 或 1, 默认: 0): " "0" tls
get_input "请输入虚假倍率 (默认: 1，即为不开启): " "1" fake_times
get_input "请输入采集间隔 (毫秒, 默认: 1000ms): " "1000" collection_interval
get_input "请输入 monitor 路径 (默认: 'monitor'): " "monitor" monitor_path

echo "参数填写完毕，正在安装..."

echo "$server"
echo "$auth_token"
echo "$name"
echo "$tls"
echo "$fake_times"
echo "$collection_interval"
echo "$monitor_path"

# 检查 tls 是否为 0 或 1 并且 name 是否为 "default_name" 或其他值
if [[ "$tls" == "0" && "$name" != "default_name" ]]; then
	# 当 tls 是 0 且 name 不是 "default_name" 时执行的命令
	/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -f "$fake_times" -i "$collection_interval" --monitor-path "$monitor_path" -n "$name" --install
elif [[ "$tls" == "1" && "$name" != "default_name" ]]; then
	# 当 tls 是 1 且 name 不是 "default_name" 时执行的命令
	/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -f "$fake_times" -i "$collection_interval" --monitor-path "$monitor_path" -n "$name" --tls true --install
elif [[ "$tls" == "0" && "$name" == "default_name" ]]; then
	# 当 tls 是 0 且 name 是 "default_name" 时执行的命令
	/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -f "$fake_times" -i "$collection_interval" --monitor-path "$monitor_path" --install
elif [[ "$tls" == "1" && "$name" == "default_name" ]]; then
	# 当 tls 是 1 且 name 是 "default_name" 时执行的命令
	/usr/bin/ak_monitor_client_rs -s "$server" -a "$auth_token" -f "$fake_times" -i "$collection_interval" --monitor-path "$monitor_path" --tls true --install
	# 在这里放置你需要执行的具体命令
else
	# 如果 tls 不是 0 或 1，则给出提示信息
	echo "TLS 参数必须是 0 / 1 "
	exit 1
fi
