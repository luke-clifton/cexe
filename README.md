# cexe interpreter

This is a `#!` interpreter for when you can't use a real one.

You can use `cexe` to run arbitrary `/bin/sh` scripts to launch your program.
This let's you perform additional set up, or call out to compilers, or do
anything else you could possibly want to do to actually intepret your file.

```python
#! /usr/bin/env cexe
# export A="An Environment Variable"
# exec nix run -f '<nixpkgs>' pkgs.python3 -c python3 "$0" "$@"

import sys;
import os;

print("Welcome to Python!")
print(os.getenv("A"))
print(sys.argv)
```

You simply place a comment block (with a consistent prefix for each line) with
the shell script to run to launch your program. That script is given the file
to be interpreted as the `$0` argument, and the rest of the arguments after
that, just like a regular shell script. The script goes until the first
non-prefixed line.

```rust
#! /usr/bin/env cexe
// tmp="$(mktemp -d)"
// rustc -o "$tmp/$(basename $0)" 1>&2 "$0"
// "$tmp/$(basename $0)" "$@"
// r="$?"
// rm -Rf $tmp
// exit "$r"

pub fn main() {
    println!("Hello, rust");
    for arg in std::env::args() {
        println!("{}", arg);
    }
}
```
