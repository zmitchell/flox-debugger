# funcfiletrace
# - Contains the <relpath>:<lineno> of the callsites in the call stack.
#
# funcstack
# - Contains the function names of the functions in the call stack.
# - Index 0 is the current function.

function stack_trace() {
	local output=""

	local n_frames=${#funcstack[@]}

	# Index 1 is this function, which we don't want to report
	local first_frame=2
	
	for i in $(seq $first_frame $n_frames); do
		local file_and_line="${funcfiletrace[$i]}"
		local relpath="$(echo "$file_and_line" | cut -d ':' -f 1)"
		local call_line="$(echo "$file_and_line" | cut -d ':' -f 2)"

		local abspath="$(realpath "$relpath")"

		local fname="${funcstack[$i]}"

		output="$(printf "%s%s:%s:%s" "$output" "$abspath" "$call_line" "$fname")"
		output="$output"$'\n'
	done

	echo "$output"
}

fdb_tracepoint() {
	local name="$1"
	local tracepoint="${FLOX_DBG_TRACEPOINT:-}"
	if [ -z "$tracepoint" ]; then
		return
	elif [ "$tracepoint" == "all" ] || [ "$tracepoint" == "next" ] || [ "$tracepoint" == "$name" ]; then
		local call_stack="$(stack_trace)"
		local output=$(target/debug/flox-debugger --shell zsh --tracepoint "$name" --call-stack "$call_stack")
		eval "$output"
	fi
}
