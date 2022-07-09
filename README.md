# 💐 azf

a simple rust-based tool for fetching system information

you need a patched [nerd font] and the [material design icons font]

![](gallery/screenshot.png)

[nerd font]: https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts
[material design icons font]: https://github.com/Mangeshrex/rxfetch/raw/main/ttf-material-design-icons/materialdesignicons-webfont%20(1).woff

# 🔨 compiling

you can compile normally with `cargo build --release` using the stable toolchain, but if you want a even smaller binary size, you can use the nightly toolchain and run `cargo +nightly build -Z build-std=std,panic_abort --target $(rustc -vV | sed -n 's|host: ||p') --release`;

