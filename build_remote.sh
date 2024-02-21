cross build --target aarch64-unknown-linux-musl --release
echo "Copying binary to remote target"
scp target/aarch64-unknown-linux-musl/release/cgfe4 root@192.168.93.13:/opt/bin
echo "done"