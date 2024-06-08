#!/usr/bin/env bash

# Don't set -e, as we need to report scripts that fail.
#
set -u

source ~/bin/bash-warn.sh || {
	printf "Could not source functional-bash.sh - aborting." >&2
	exit 1
}

source ~/bin/functional-bash.sh || die "Could not source ~/bin/functional-bash.sh"


# Converts Unix exit status to label ("success" or "failure")
#
status_to_label() {
    local -ri status=$1
     # We treat 126 and 127 (which indicate permission error and file not found,
     # respectively) as indications of error rather than failure.
    case $status in
        0 ) echo success ;;
        126 | 127 ) echo error ;;
        * ) echo failure ;;
    esac
}

colour_label() {
    local -r label=$1
    case $label in 
        success ) printf "%s" "$(green "$label")" ;;
        failure ) printf "%s" "$(red "$label")" ;;
        error   ) printf "%s" "$(yellow "$label")" ;;
        *       ) die "unknown status label '$label'"
    esac
}

################################################################
# Main

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

echo
for PID in "${!test_names[@]}"; do
    printf "%s -> %s\n" "${test_names["$PID"]}" \
        "$(colour_label "${test_result_labels["$PID"]}")"
done

declare -A test_result_counts
count test_result_labels test_result_counts

[[ -v test_result_counts[success] ]] && nb_successes="${test_result_counts[success]}" || nb_successes=0
[[ -v test_result_counts[failure] ]] && nb_failures="${test_result_counts[failure]}"  || nb_failures=0
[[ -v test_result_counts[error] ]]   && nb_errors="${test_result_counts[error]}"      || nb_errors=0

printf "\n%s:\t%d\n%s:\t%d\n%s:\t\t%d\n" \
    "$(green Successes)" "$nb_successes" \
    "$(red Failures)" "$nb_failures" \
    "$(yellow Errors)" "$nb_errors" \

printf "\n"
if ((${#test_names[@]} == $nb_successes)) ; then
    printf "All %d tests %s!\n" "$nb_successes" "$(green OK)"
else
    printf "There were %s or %s.\n" "$(yellow errors)" "$(red failures)"
fi
