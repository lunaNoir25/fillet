# Fillet

All in one file editing tool!

# Usage

Reading:
```bash
fillet read <file> # Read the contents of a file.
fillet read -l <line_number> <file> # Read a specific line.
```

Writing:
```bash
fillet write -o <file> <contents> # Overwrite entire file.
fillet write -a <file> <contents> # Append to the end of a file.
fillet write -l <line> <file> <contents> # Overwrite a specific line.
```

# Installation

You will need rust and cargo installed on your system.
You can install it by:
 1. Clone the repository. `git clone https://github.com/lunaNoir25/fillet.git && cd fillet`
 2. Build it using cargo. `cargo build --release`
 3. Copy the built file `./target/release/fillet(.exe)` to your system path to easily use it anywhere, anytime!

# Desclaimer

This was built for a school project and I have no intentions on maintaining this myself.
Feel free to fork it and and release your own version.
Fillet is licensed under the GNU GPLv3.
