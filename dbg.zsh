function get_stack_trace() {
    local trace_parts=()
    local i
    
    # Start from index 1 (immediate caller) through all callers
    for i in {1..${#funcfiletrace[@]}}; do
        local file_line="${funcfiletrace[$i]}"
        local calling_func="${funcstack[$((i + 1))]:-<script>}"  # Get the caller's name
        trace_parts+=("${file_line} (${calling_func})")
    done
    
    # Join with " -> " separator
    local IFS=$'\n'
    echo "${trace_parts[*]}"
}

fdb_tracepoint() {
	local name="$1"
	local tracepoint="${FLOX_DBG_TRACEPOINT:-}"
	if [ -z "$tracepoint" ]; then
		return
	elif [ "$tracepoint" == "all" ] || [ "$tracepoint" == "next" ] || [ "$tracepoint" == "$name" ]; then
		local output
		output=$(target/debug/flox-debugger --shell zsh)
		eval "$output"
	fi
}
