#!/usr/bin/fish

set Usage "Usage: new_day.fish [newfile]"

if test (count $argv) -lt 1; or test $argv[1] = "--help"
    echo $Usage
end

set newfile $argv[1]
cp -r template/ $newfile

sed $newfile/Cargo.toml -i -e "s/template/$newfile/g"
sed $newfile/src/main.rs -i -e "s/template/$newfile/g"
