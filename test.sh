#!/usr/bin/zsh

cargo build --bin insploray_cli --release
./target/release/insploray_cli ../InsploRayMemorialScenes/Bunny.obj -o ./outputs/stanfordbunny.exr  -W 720 -H 720 -s 1024
./target/release/insploray_cli ../InsploRayMemorialScenes/Armadillo.obj -o ./outputs/stanfordarmadillo.exr  -W 720 -H 720 -s 1024
./target/release/insploray_cli ../InsploRayMemorialScenes/Stanford_dragon.obj -o ./outputs/stanforddragon.exr  -W 720 -H 720 -s 1024

