# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/xenial64"
  config.vm.provision "shell", inline: <<-SHELL
    sudo apt-get update
    sudo apt-get install -y clang make binutils cmake
    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- -y 2>/dev/null
  SHELL
end
