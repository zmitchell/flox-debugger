#!/usr/bin/env bash

set -euo pipefail

fdb_tracepoint() {
	local name="$1"
	local tracepoint="${FLOX_DBG_TRACEPOINT:-}"
	if [ -z "$tracepoint" ]; then
		return
	elif [ "$tracepoint" == "all" ] || [ "$tracepoint" == "next" ] || [ "$tracepoint" == "$name" ]; then
		local output
		output=$(cargo run)
		eval "$output"
	fi
}
