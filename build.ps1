cross build --target aarch64-unknown-linux-gnu
cargo build
scp .\target\aarch64-unknown-linux-gnu\debug\cgfe4 pi@192.168.83.84:/home/pi/Desktop
Write-Output "Done"