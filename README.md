# Hash Finder

## Overview

Hash Finder is a simple Rust application designed to find and display hashes that end with a specific number of trailing zeros. The application uses the SHA-256 algorithm for hash computation and leverages multithreading to find hashes efficiently.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Features

- Efficient hash computation using SHA-256.
- Multithreading for faster hash generation and verification.
- Command-line arguments for customization.

## Requirements

- Rust programming language
- Cargo package manager

## Installation

1. Clone this repository:

    \```bash
    git clone https://github.com/trepachkoDm/pyshop.git
    cd hash_finder
    \```

2. Navigate to the directory:

    \```bash
    cd hash_finder
    \```

3. Build the project:

    \```bash
    cargo build --release
    \```

    The compiled binary will be in the `./target/release/` directory.

## Usage

Run the program with the `-N` and `-F` flags to specify the number of zeros at the end of the hash and the number of such hashes to find, respectively.

\```bash
./hash_finder -N 3 -F 5
\```

This will find 5 hashes that end with three zeros.

## Testing

To run the unit tests:

\```bash
cargo test
\```

## Documentation

### `find_hashes(n: usize, f: usize) -> Vec<(usize, String)>`

- **n**: The number of zeros at the end of the hash.
- **f**: The number of such hashes to find.

Returns a vector of tuples, where the first element is the number hashed and the second element is the hash.

### `compute_hash(num: usize) -> String`

- **num**: The number to hash.

Returns the SHA-256 hash of the number as a hexadecimal string.

## Contributing

If you'd like to contribute, please fork the repository and make changes as you'd like. Pull requests are warmly welcome.

## License

MIT License. See the LICENSE file for details.
