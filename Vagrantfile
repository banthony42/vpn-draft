vpn_server_provision = <<-SHELL
    echo -e "\n#MY_KEY" >> /home/vagrant/.ssh/authorized_keys
    cat /home/vagrant/.ssh/my_id_rsa.pub >> /home/vagrant/.ssh/authorized_keys

    sudo apt-get update   
    DEBIAN_FRONTEND=noninteractive sudo apt-get install -y curl build-essential net-tools tcpdump
    SHELL

install_rust_toolchain = <<-SHELL
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
    cd /home/vagrant/vpn-draft/
    cargo build
    SHELL

   
run_vpn_draft = <<-SHELL
    sudo /home/vagrant/vpn-draft/target/debug/vpn-draft server --tun-addr 192.168.42.1
    SHELL

Vagrant.configure("2") do |config|

    config.vm.box = "debian/bookworm64"

    config.ssh.insert_key = true

    config.vm.define "VPNServer" do |server|
        server.vm.hostname = "VPNServer"
        server.vm.network "private_network", ip: "192.168.56.110"

        server.vm.provider "virtualbox" do |vb|
            vb.customize ["modifyvm", :id, "--name", "VPNServer"]
            vb.customize ["modifyvm", :id, "--cpus", "2"]
            vb.customize ["modifyvm", :id, "--memory", "2048"]
        end
        server.vm.provision "shell", inline: install_rust_toolchain, privileged: false
        server.vm.provision "shell", inline: run_vpn_draft, privileged: true
    end
    config.vm.provision "file", source: "~/.ssh/id_rsa.pub", destination: "~/.ssh/my_id_rsa.pub"
    config.vm.provision "file", source: "./src", destination: "~/vpn-draft/src"
    config.vm.provision "file", source: "./Cargo.toml", destination: "~/vpn-draft/Cargo.toml"
    config.vm.provision "shell", inline: vpn_server_provision, privileged: true
end