$script = <<-SCRIPT
    export RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    DEBIAN_FRONTEND=noninteractive

    sudo apt-get update \
    && sudo apt-get -y install \
    pkg-config \
    curl \
    wget \
    lsb-release \
    libelf-dev \
    build-essential \
    linux-headers-generic \
    software-properties-common \
    gnupg2 \
    && wget https://apt.llvm.org/llvm.sh \
    && sudo -E chmod +x ./llvm.sh \
    && sudo -E ./llvm.sh 14 \
    && rm -f llvm.sh \
    && sudo apt-get -y install "linux-image-$(ls /lib/modules)" \
    && sudo apt-get clean -y

    sudo ln -s /usr/bin/llvm-config-14 /usr/bin/llvm-config
    sudo llvm-config --version | grep -q '^14'

    curl https://sh.rustup.rs -sSf > rustup.sh \
    && sudo -E sh rustup.sh -y \
          --default-toolchain stable \
          --profile default \
          --no-modify-path \
    && rm -f rustup.sh \
    && rustup --version \
    && cargo -vV \
    && rustc -vV

    if [ x$TARGETPLATFORM = x"linux/arm64" ]; then \
        sudo rustc extract-btf-aarch64.rs \
        && sudo ./extract-btf-aarch64 /boot/vmlinuz-* /boot/vmlinux-btf; \
    else \
        wget https://raw.githubusercontent.com/torvalds/linux/master/scripts/extract-vmlinux \
        && su - root <<! sh extract-vmlinux /boot/vmlinuz-* > /boot/vmlinux ! \
        # && sh extract-vmlinux /boot/vmlinuz-* > /boot/vmlinux \
        && rm -f extract-vmlinux;
    fi
    rm -f extract-btf-aarch64.rs

SCRIPT

Vagrant.configure("2") do |config|
    config.vm.box = "debian/bullseye64"
    config.vm.synced_folder "./", "/build"

    config.vm.provider "virtualbox" do |vb|
        # Customize the amount of memory on the VM:
        vb.memory = "4096"
    end

    config.vm.provision "shell", inline: $script
end
