#!/bin/bash
set -eo pipefail

show-help(){
    items=()
    while IFS='' read -r line; do items+=("$line"); done < \
        <(compgen -A "function" | grep "run-" | sed "s/run-//")
    printf -v items "\t%s\n" "${items[@]}"

    usage="USAGE: $(basename "$0") CMD [ARGUMENTS]
  CMD:\n$items"
    printf "$usage"
}

name=$1
case "$name" in
    "" | "-h" | "--help" | "help")
        show-help
        ;;
    *)
        shift
        if compgen -A "function" | grep "run-$name" >/dev/null ; then
            run-"${name}" "$@"
        else
            echo "ERROR: run-$name not found."
            exit 123
        fi
        ;;
esac
