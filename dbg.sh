
# FUNCNAME
# - Contains the shell functions in the call stack
# - Index 0 is the currently executing function
# - The bottom of the call stack is called "main" e.g. the top level of a script
#
# BASH_SOURCE
# - Contains the files in which functions in the call stack were called.
# - BASH_SOURCE[i] contains the file in which FUNCNAME[i] is defined
# - BASH_SOURCE[i+1] contains the file in which FUNCNAME[i] was called
#
# BASH_LINENO
# - Contains the line numbers in source files where the items in FUNCNAME were called
# - BASH_LINENO[i] contains the line number in the caller script where FUNCNAME[i] was called.

# Output format:
# <calling file>:<line of callsite>(<function called>)

function stack_trace() {
	local output
	output=""

	local n_frames
	n_frames=${#FUNCNAME[@]}
	# Index 0 is this function, which we don't want to report
	local first_frame=1

	# The frames are 0 indexed, so n-1 is the last recorded frame.
	# That frame is kind of a dummy frame that just says
	# "a script called by the shell" so we strip that out
	local last_frame
	last_frame=$((n_frames - 2))

	for i in $(seq $first_frame $last_frame); do
		local relpath
		relpath="${BASH_SOURCE[$i]}"
		local abspath
		abspath="$(realpath "$relpath")"

		local fname
		fname="${FUNCNAME[$i]}"
		if [ "$fname" == "main" ]; then
			fname="<script>"
		fi

		local call_line
		call_line="${BASH_LINENO[$i]}"

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
		local call_stack
		call_stack="$(stack_trace)"
		local output
		output=$(target/debug/flox-debugger --shell bash --tracepoint "$name" --call-stack "$call_stack")
		eval "$output"
	fi
}
