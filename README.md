# sertest
> 🏋️ Serial Port Stress Testing Tool

## Command Line Options
- `-p`, `--port` Serial port path, default: `/dev/ttyUSB0`
- `-e`, `--encoding` Data encoding type (utf8 or hex), default: `utf8`
- `-d`, `--data` Data to send, default: `PING`
- `-i`, `--interval` Send interval (milliseconds), default: `100`
- `-c`, `--count` Number of times to repeat (infinite loop if not specified)
- `-h`, `--help` Show help information

## Examples
### Send hex data every 100ms
```bash
sertest -p /dev/ttyUSB0 -i 100 -e hex -d FF00AA55
```

### Send text data
```bash
sertest -p /dev/ttyUSB0 -d "{\"id\":1,\"name\":\"test\"}"
```

## Development
### Debug
```bash
cargo run -- -p /dev/ttyUSB0 -i 100 -e hex -d FF00AA55
```

### Build
```bash
cargo build --release
```

## License
MIT &copy; 2025-present [Yelo](https://github.com/imyelo)
