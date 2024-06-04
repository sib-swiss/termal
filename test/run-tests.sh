#!/usr/bin/env bash

set -u

# Maps a function (first argument) to the values of an associative array (whose
# name is the second argument), and sets corresponding (k,f(v)) pairs in another
# associative array (whose name is the 3rd argument).

aamap() {
    local f=$1
    local -n src=$2
    local -n tgt=$3
    for k in "${!src[@]}"; do
        tgt["$k"]="$($f "${src["$k"]}")"
    done
}

# Same idea, but for a regular array.

map() {
    local f=$1
    local -n src=$2
    local -n tgt=$3
    for i in "${!src[@]}"; do
        tgt[$i]="$("$f" "$e")"
    done
}

fold() {
    local f=$1
    local acc=$2
    local -n list=$3
    for e in "${list[@]}"; do
        acc="$($f "$acc" "$e")"
    done
    printf "%s" "$acc"
}

add(){ echo $(($1 + $2)); }

sum() {
    local integers=$1 # NAME of the array! (not ref...)
    printf "%d" "$(fold add 0 $integers)"
}

# Converts Unix exit status to label ("success" or "failure")
#
status_to_label() {
    local -ri status=$1
    if ((0 == status)); then 
        echo success
    else
        echo failure
    fi
}

# Converts Unix exit status to boolean (0 / !=0 -> 1 / 0 resp)
#
status_to_boolean() {
    local -ri status=$1
    if ((0 == status)); then 
        echo 1
    else
        echo 0
    fi
}

declare -A test_names
declare -Ai test_status

for test_script in test-*.exp; do
    printf "Launching %s...\n" "$test_script"
    ./$test_script >/dev/null 2>&1 &
    test_names[$!]=$test_script
done

echo
printf "Waiting for completion...\n"

for PID in "${!test_names[@]}"; do
    wait -n "$PID"
    test_status["$PID"]=$?
done

declare -A test_result_labels
aamap status_to_label test_status test_result_labels
#declare -p test_result_labels

echo
for PID in "${!test_names[@]}"; do
    printf "%s -> %s\n" "${test_names["$PID"]}" \
        "${test_result_labels["$PID"]}"
done

declare -A test_result_bools
aamap status_to_boolean test_status test_result_bools
#declare -p test_result_bools

declare -i nb_successes="$(sum test_result_bools)"
declare -i nb_failures=$((${#test_names[@]} - nb_successes))
printf "%d success(es), %d failure(s)\n" "$nb_successes" \
    "$nb_failures"
