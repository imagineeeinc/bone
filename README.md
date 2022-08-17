<p align="center">
  <img src="https://raw.githubusercontent.com/imagineeeinc/bone/main/bone.png" width="30%">
</p>
<h1 align="center">Bone (core edition)</h1>
<h3 align="center">🦴A super light, toy Javascript runtime🦴</h3>

Bone (core edition) is a *super* bare "*bones*" **javascript runtime** based on the deno core module.

Deno being bassed on rust and its core being avalible to anyone made me wonder if I can make a **"javascript runtime"** with the heavy lifting done by some one else, the someone else being the **amazing** [deno team](https://github.com/denoland/).

<h3 align="center">Core edition</h3>

The core edition is an older version of bone bassed on the `deno_core` module which means no js browser functions like console log or fetch, its just basic runtime. For a full feateured runtime check the [main branch](https://github.com/imagineeeinc/bone) 

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
