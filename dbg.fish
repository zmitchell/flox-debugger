# Fish has a built-in stack trace function that produces output like this:
# in function 'otherfunc'
#         called on line 8 of file ./run.fish
# in function 'myfunction'
#         called on line 19 of file ./run.fish
#
# Because of the way we've written our `stack_trace` function,
# its first entry is 'in command substitution', which we want to drop.
# Next is the `stack_trace` function itself, which we also want to drop.
function stack_trace
    set st (status stack-trace)
    set output (string join ';' $st[5..])
    echo $output
end

function fdb_tracepoint
    set name $argv[1]
    if not set -q FLOX_DBG_TRACEPOINT
        return
    else if test $FLOX_DBG_TRACEPOINT = all; or test $FLOX_DBG_TRACEPOINT = next; or test $FLOX_DBG_TRACEPOINT = $name
        set call_stack (stack_trace)
        set output (target/debug/flox-debugger --shell fish --tracepoint $name --call-stack "$call_stack")
        eval "$output"
    end
end
