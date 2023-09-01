# self_dev
cd ./self_dev/
cd ./wr_syscalls && cargo build -r && cd -
cd ./syscaller && cargo build -r && cd -
cd ./executor && cargo build -r && cd -
cd ./speed_executor && cargo build -r && cd -
cd ./enc_executor && cargo build -r && cd -
cd ../

# oss

cd ./oss/net/ &&
mkdir release
gcc l2.c -o ./release/l2 &&
g++ udp.c -o ./release/udp &&
cd - &&

podman build ./ -t my-harbor.net/ebpf-bypass/ebpf-bypass