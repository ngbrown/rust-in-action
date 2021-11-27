## ch8-mget

Requires Linux.

For Ubuntu 20.04 LTS:

Install curl (from apt, not snap):

```bash
sudo apt install curl
```

Install rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install a linker:

```bash
sudo apt install build-essential
```

### Create a virtual network device

Create a `tap-rust` device:

```bash
sudo ip tuntap add mode tap name tap-rust user $USER
```

Verify:

```bash
ip tuntap list
```

Activate tap-rust:

```bash
sudo ip link set tap-rust up
```

Assign an IP address (different subnet than a currently used one, also coded into program):

```bash
sudo ip addr add 192.168.42.100/24 dev tap-rust
```

Enable internet packets to reach the IP address:

```bash
sudo iptables -t nat -A POSTROUTING -s 192.168.42.100/24 -j MASQUERADE
```

Instruct the kernel to enable IPv4 packet forwarding:

```bash
sudo sysctl net.ipv4.ip_forward=1
```

Run command:

```bash
./ch8-mget http://info.cern.ch tap-rust
```

Cleanup and remove device:

```bash
sudo ip tuntap del mode tap name tap-rust
```



### For editing:

Install libssl:

```bash
sudo apt install libssl-dev
```

Install cargo-edit for the `cargo add` command:

```bash
cargo install cargo-edit
```
