#!/usr/bin/env bash

#
# Purpose: Container entrypoint for devcontainer
# Version: 1.2
# Create Date: 10 24th 2022
# Author: Vincent Lauria <vincent@lauria.ch>
# Descritpion:
#   Add property postAttachCommand in your devcontainer.json to use this function, e.g. :
#   ```json
#   {
#     ...
#     "postAttachCommand": "/container-entrypoint -c start_ssh_agent"
#     ...
#   }
#   ```
#

persist_bash_history() {
	if [[ -d "${HISTFILE_DIRECTORY}" ]]; then
		if [[ ! -z "${USERNAME}" ]]; then
			local USER="${USERNAME}"
		fi

		sudo chown -R ${USER}:${USER} "${HISTFILE_DIRECTORY}" &&
			touch ${HISTFILE_DIRECTORY}/.bash_history
	fi
}

start_docker_proxy() {
	local SOCAT_LOG_FILE=/var/log/docker-socat-proxy.log
	local SOCAT_PID_FILE=/run/docker-socat-proxy.pid

	if [[ -f ${SOCAT_PID_FILE} && "$(pgrep -c -F ${SOCAT_PID_FILE})" = "1" ]]; then
		echo "Docker socket proxy already running"
		return
	fi
	
	if [[ -S "/var/run/docker.sock" ]]; then
		sudo rm -f /var/run/docker.sock
	fi

	setsid -f bash <<-EOL
		echo "\$\$" | sudo tee ${SOCAT_PID_FILE} >/dev/null
		sudo socat \
			UNIX-LISTEN:/var/run/docker.sock,fork,mode=660,user=${USERNAME} \
			UNIX-CONNECT:/var/run/docker-host.sock 2>&1 | sudo tee -a ${SOCAT_LOG_FILE} >/dev/null
	EOL
}

start_ssh_agent() {
	if [[ ! -z "$SSH_AUTH_SOCK" ]]; then
		if [[ "$(pgrep -i -c ssh-agent)" = "0" ]]; then
			# Launch a new instance of the agent
			ssh-agent -s &>${HOME}/.ssh/ssh-agent
		fi
		eval $(cat ${HOME}/.ssh/ssh-agent|grep -v echo)
	fi
}

# Default workspace path
if [[ -z "${WORKSPACE}" ]]; then
	export WORKSPACE=/workspace
fi

# Default bash history directory path
if [[ -z "${HISTFILE_DIRECTORY}" ]]; then
	export HISTFILE_DIRECTORY=/commandhistory
fi

# Include scripts in /container-entrypoint.d directory
if [[ -d /container-entrypoint.d ]]; then
	INCLUDES=($(find /container-entrypoint.d -type f -name "*.sh" | xargs))
	for INCLUDE in "${INCLUDES[@]}"; do
		. ${INCLUDE}
	done
fi

# Default command option
OPT_COMMAND="persist_bash_history,start_docker_proxy,start_ssh_agent"

eval set -- $(getopt -o c: --long command: -n 'entrypoint.sh' -- "$@")

while true; do
	case "$1" in
	-c | --command)
		OPT_COMMAND="$2"
		shift 2
		;;
	--)
		shift
		break
		;;
	esac
done

IFS=',' read -ra COMMAND <<<"${OPT_COMMAND}"
for METHOD in "${COMMAND[@]}"; do
	if declare -F "${METHOD}" >/dev/null; then
		"${METHOD}"
	fi
done

exec "$@"
