set -e

test ! -d examples && mkdir examples

MAX_PATH_LENGTH=0

for path in bf/*.bf; do
  len=${#path}
  test "$len" -gt "$MAX_PATH_LENGTH" && MAX_PATH_LENGTH="$len"
done

cargo build
for path in bf/*.bf; do
  file="$(basename "$path")"
  output="examples/${file%.*}"
  lhs="$path"
  printf "%$((MAX_PATH_LENGTH - ${#path}))s"
  echo "$lhs -> $output"
  ./target/debug/rlbf --input "$path" --output "$output" --full
done
