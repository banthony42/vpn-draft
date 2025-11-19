# VPN Principle

This little project aim to practice and understand the basics of a VPN.

For that i use rust to create TUN interfaces and UDP client, "server".

<p align="left">
  <img src="assets/demo.gif" alt="Demo" />
</p>

As a client, `vpn-draft` will :

- create a TUN interface and attach to it, then it will assign the given local IPv4 address for this TUN interface.
- Try to connect using UDP to the given `--remote` IPv4 address targeting `4242` as default port.
- Finally it will transmit any receive packets from the TUN to the remote UDP connection.

```sh
$ ./target/debug/vpn-draft client -h
Run VPN client instance

Usage: vpn-draft client --tun-addr <TUN_ADDR> --remote <REMOTE>

Options:
      --tun-addr <TUN_ADDR>  VPN client local tunnel IPv4 address
      --remote <REMOTE>      VPN remote server IPv4 address. (default remote port 4242 will be use automatically)
  -h, --help                 Print help
```

As a "server", `vpn-draft` will:

- Do almost the same as the client mode.
- The only difference is that receives packets from TUN interfaces are send to the client address.

```sh
$ ./target/debug/vpn-draft server -h
Run VPN server instance

Usage: vpn-draft server --tun-addr <TUN_ADDR>

Options:
      --tun-addr <TUN_ADDR>  VPN server local tunnel IPv4 address
  -h, --help                 Print help
```

```sh
vagrant up
```

## Prerequisite

- Install vagrant and rust toolchain
- Ensure you have an ssh key (`~/.ssh/id_rsa.pub` will be import to the vagrant VM)

## Setup

1 - Build the vagrant VM to host the "server"

```sh
cd vpn-draft
vagrant up
```

When the VM is ready, `vpn-draft` is run in "server" mode in the VM, accepting UDP connection on `0.0.0.0:4242`.
Therefore we can use the VM `eth1` ip `192.168.56.110`. (See Vagrantfile)

2 - You can ensure both TUN interface and UDP "server" are up and running:

```sh
# Open another terminal
$ cd vpn-draft
$ vagrant ssh
# [...]
vagrant@VPNServer:~$ sudo netstat -tulnp | grep vpn-draft
udp        0      0 0.0.0.0:4242            0.0.0.0:*                           5500/vpn-draft

vagrant@VPNServer:~$ ip a | grep vpn-draft
4: vpn-draft: <POINTOPOINT,MULTICAST,NOARP,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UNKNOWN group default qlen 500
    inet 192.168.42.1/24 scope global vpn-draft

# We can see that tun rust crate correctly add new routes for our interfaces
vagrant@VPNServer:~$ ip route | grep 192.168.42
192.168.42.0/24 dev vpn-draft proto kernel scope link src 192.168.42.1
```

3 - On your host you can now run `vpn-draft` as client to connect:

```sh
cd vpn-draft
cargo build
sudo ./target/debug/vpn-draft client --tun-addr 192.168.42.2 --remote 192.168.56.110:4242
```

4 - You can run same commands from step 2 on your host to see the TUN interfaces, etc ...

5 - At this point you should be able to ping `192.168.42.1`.
Then packets are transmit to the process attach to the TUN interface (`vpn-draft`) and transmit to the server through UDP.
Finally the `vpn-draft` running on the "server" receive the packets
and send them to the "server" TUN interfaces.

Same process for the ping reply, in the opposite direction.

```sh
# Host - Terminal 1

$ ping 192.168.42.1
PING 192.168.42.1 (192.168.42.1) 56(84) bytes of data.
64 bytes from 192.168.42.1: icmp_seq=2 ttl=64 time=2.56 ms
64 bytes from 192.168.42.1: icmp_seq=3 ttl=64 time=2.99 ms
64 bytes from 192.168.42.1: icmp_seq=4 ttl=64 time=2.91 ms
# ...
```

```sh
# Host - Terminal 2

sudo ./target/debug/vpn-draft client --tun-addr 192.168.42.2 --remote 192.168.56.110:4242
vpn-draft started ...
Begin ...
client: tun.recv, transmitting to remote ...
client: sock.recv, transmitting to local tun interface ...
client: tun.recv, transmitting to remote ...
client: tun.recv, transmitting to remote ...
client: sock.recv, transmitting to local tun interface ...
client: tun.recv, transmitting to remote ...
client: sock.recv, transmitting to local tun interface ...
client: tun.recv, transmitting to remote ...
client: sock.recv, transmitting to local tun interface ...
```

```sh
# "VPNServer" - Terminal 3

    VPNServer:    Compiling vpn-draft v0.1.0 (/home/vagrant/vpn-draft)
    VPNServer:     Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.08s
==> VPNServer: Running provisioner: shell...
    VPNServer: Running: inline script
    VPNServer: vpn-draft started ...
    VPNServer: Begin ...
    VPNServer: server: sock.recv, transmitting to local tun interface ...
    VPNServer: server: tun.recv, transmitting to remote ...
    VPNServer: server: sock.recv, transmitting to local tun interface ...
    VPNServer: server: sock.recv, transmitting to local tun interface ...
    VPNServer: server: tun.recv, transmitting to remote ...
    VPNServer: server: sock.recv, transmitting to local tun interface ...
    VPNServer: server: tun.recv, transmitting to remote ...
    VPNServer: server: sock.recv, transmitting to local tun interface ...
    VPNServer: server: tun.recv, transmitting to remote ...
```
