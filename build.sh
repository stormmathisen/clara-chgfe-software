cross build --target aarch64-unknown-linux-musl --release
echo "Copying binary to buildroot"
cp target/aarch64-unknown-linux-musl/release/cgfe4 ~/buildroot/buildroot/rpi_configs/overlay/opt/bin
echo "done"