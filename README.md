# file-local-ip-replacer
Replace IPs in a file with the local network IP of the host machine

## Usage:
`file-local-ip-replacer [OPTIONS] <PATH>`

#### Args:
- `<PATH>`: Path to the file the will have it's IPs replaced

#### Options:

- `-h, --help`: Print help information
- `--replace-localhost`: Also replace 'localhost' with local IP
- `-V, --version`: Print version information

<br/>

## Compiling:

With cargo, you can build using `cargo build --release`.
<br/>
The program will be located in `target/release/file-local-ip-replacer`

<br/>

## Running Before launch (JetBrains)

1. Click `Edit Configurations...`
2. Add Before launch -> External Tools
3. Create Tool
- Name: `File local ip replacer`
- Program: path to `file-local-ip-replacer`
- Arguments: `--replace-localhost <PATH TO FILE TO REPLACE IPS>`
4. `Ok` -> `Apply` -> `Ok`
