# Rust jpg_2_pdf

Base on [Quantaly/jpeg-to-pdf](https://github.com/Quantaly/jpeg-to-pdf).

## Bash batch script example

```bash
#!/bin/zsh

exec=/home/user/rust/jpg_2_pdf/target/debug/image_2_pdf

source_directory="/home/user/photo/portrait"

target=()

while IFS= read -r dir; do
dir="${dir%/}"
target+=("$dir")
done < <(ls -d "$source_directory"/*/)

for i in "${target[@]}"
do
echo "$exec "$i" "$i.pdf""
"$exec" "$i" "$i.pdf"
done
```
