function get_full_stack_trace() {
    local trace_parts=()
    local i
    
    # Start from index 1 (immediate caller) through all callers
    for i in $(seq 0 $((${#BASH_SOURCE[@]} - 1))); do
        local file_line="${BASH_SOURCE[$i]}:${BASH_LINENO[$((i - 1))]}"
        local calling_func="${FUNCNAME[$((i + 1))]:-<script>}"
        
        # Handle out-of-bounds and empty function names
        if [[ -z "$calling_func" ]]; then
            calling_func="<script>"
        fi
        
        trace_parts+=("${file_line} (${calling_func})")
    done
    
    # Join with " -> " separator
    local IFS=" -> "
    echo "${trace_parts[*]}"
}

otherfunc() {
  echo "other function"
  get_full_stack_trace
}
