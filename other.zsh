function get_stack_trace() {
    local trace_parts=()
    local i
    
    # Start from index 1 (caller) and go up the stack
    for i in {1..${#funcfiletrace[@]}}; do
        local file_line="${funcfiletrace[$i]}"
        local func_name="${funcstack[$i]}"
        
        # Handle top-level script (no function name)
        if [[ -z "$func_name" ]]; then
            func_name="<script>"
        fi
        
        trace_parts+=("${file_line} (${func_name})")
    done
    
    # Join with " -> " separator
    local IFS=" -> "
    echo "${trace_parts[*]}"
}

function get_full_stack_trace() {
    local trace_parts=()
    local i
    
    # Start from index 1 (immediate caller) through all callers
    for i in {1..${#funcfiletrace[@]}}; do
        local file_line="${funcfiletrace[$i]}"
        local calling_func="${funcstack[$((i + 1))]:-<script>}"  # Get the caller's name
        
        # Skip if no file info
        [[ -z "$file_line" ]] && continue
        
        # Handle different contexts
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

