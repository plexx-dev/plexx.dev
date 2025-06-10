#!/bin/bash
 
if [ -z "$(which inotifywait)" ]; then
    echo "inotifywait not installed."
    echo "In most distros, it is available in the inotify-tools package."
    exit 1
fi

if [ -z "$(which wasm-pack)" ]; then
    echo "wasm-pack not installed."
    echo "In most distros, it is available in the wasm-pack package."
    exit 1
fi

if [ ! -d "pkg" ]; then
  echo "pkg does not exist."
  wasm-pack build --target web
fi


counter=0;
 
function execute() {
    counter=$((counter+1))
    echo "Detected change n. $counter" 
    eval "$@"
}
 
inotifywait --recursive --monitor --format "%e %w%f" \
--event modify,move,create,delete ./src \
| while read changed; do
    echo $changed
    execute "clear; wasm-pack build --target web"
done