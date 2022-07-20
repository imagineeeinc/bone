<p align="center">
  <img src="./bone.png" width="30%">
</p>
<h1 align="center">Bone</h1>
<h3 align="center">🦴A super light, toy Javascript runtime based on deno🦴</h3>

Bone is a bare "*bones*" **javascript runtime** based on the deno runtime.

Deno being bassed on rust and its runtime being avalible to anyone made me wonder if I can make a **"javascript runtime"** with the heavy lifting done by some one else, the someone else being the **amazing** [deno team](https://github.com/denoland/).

<h2 align="center">Usage</h2>

Make sure to have [Rust installed](https://www.rust-lang.org/tools/install). You can follow my [super simplified guide](https://github.com/imagineeeinc/repo-o-knowledge/tree/main/rust#instalation-on-windows) to setup on windows.

Then clone this repo with
```bash
git clone https://github.com/imagineeeinc/bone.git
```

run:
```bash
# for testing out
cargo run
# with the example js file
cargo run -- -i examples/hello.js
# examples avalible in the examples folder

# Making a production build
cargo build --release
```

<h2 align="center">License</h2>

This project is under [MIT License](https://github.com/imagineeeinc/bone/blob/main/LICENSE)
<!--cargo run -- -i examples/hello.js-->