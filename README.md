# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the eleventh entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## WASM

Today is the eleventh day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, we are using [the same code as yesterday](https://github.com/awwsmm/daily-bevy/tree/ui/button), but we will compile to WASM and run our button code in a browser!

#### Discussion

Yesterday, we created a simple UI button and changed the style of the button in response to user events (hovering over or pressing the button).

Today, we're going to make the minimum change to that example to compile it to WASM and run it in a browser.

The following example is based on [these instructions in the Bevy README](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/README.md#wasm).

We only need to make one code change: add an HTML file in the root directory (at the same level as `Cargo.toml`) with the following contents

```html
<html lang="en">
<head>
    <title>Compiling Bevy to WASM</title>
</head>
<script type="module">
    import init from './target/wasm_example.js'
    init()
</script>
</html>
```

I've called this `example.html`, but it could be anything.

The important thing to note here is the `<script>` tag, which imports from a `target/wasm_example.js` file, which we're about to create. This `.js` file could also be called anything, it doesn't need to match the name of the `.html` file in any way.

You might be thinking to yourself: "I thought we were compiling to WASM, not transpiling to JavaScript?"

We are, but we need some ["shim" code](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts#porting_from_cc) as JavaScript can do some things that WASM still can't (yet) like manipulating the DOM. Our Rust code is compiled to WASM, but there is a bit of JavaScript glue around that WASM.  We'll get to that in a bit.

---

First, we build our Rust code for a `wasm` target

```shell
cargo build --release --target wasm32-unknown-unknown
```

You may need to [install the `wasm32-unknown-unknown` _target triple_](https://doc.rust-lang.org/cargo/commands/cargo-install.html#compilation-options) if it isn't already installed on your machine.

[The `--release` flag](https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options) compiles your Rust code [with extra optimizations](https://doc.rust-lang.org/cargo/reference/profiles.html#release) such that it runs faster, but this takes longer to compile. `cargo build` by default compiles your Rust code [in `dev` mode](https://doc.rust-lang.org/cargo/reference/profiles.html#dev).  

After the above command succeeds, you will have a `target/wasm32-unknown-unknown/release` directory containing your compiled code.

---

Next, we need to create the JavaScript bindings for the WASM file.

If you haven't already, [install `wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) with

```shell
cargo install wasm-bindgen-cli
```

Then, use `wasm-bindgen` to `gen`erate the `bind`ings

```shell
wasm-bindgen --out-name wasm_example --out-dir target --target web target/wasm32-unknown-unknown/release/daily_bevy.wasm
```

The `--out-name` specifies the name of the generated JavaScript file, without the `.js` extension. This file will be placed in the `--out-dir` directory (`target` above). So this command will create a file at `target/wasm_example.js`.

The `--target web` flag and argument specify that we want to [generate a JavaScript file that we can serve directly](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html), rather than import into other JavaScript code, like Deno or Node modules, or bundle with a tool like webpack.  

Finally, in my case, the name of the WASM file itself is `daily_bevy.wasm` (in the `target/wasm32-unknown-unknown/release` directory), because that is the name of this `package`, as defined in the `Cargo.toml` file. We are generating bindings for this `.wasm` file.

Running the above command will result in a few files in the `--out-dir`, namely

- `wasm_example.d.ts`
- `wasm_example.js`
- `wasm_example_bg.wasm`
- `wasm_example_bg.wasm.d.ts`

The `.d.ts` files contain [type declarations used by TypeScript](https://microsoft.github.io/TypeScript-New-Handbook/chapters/type-declarations/#dts-files).

The `_bg` files are ["implementation details"](https://docs.rs/crate/wasm-bindgen/0.2.10/source/DESIGN.md) of `wasm-bindgen`.

The `.js` file is the one we really care about -- that's the one in our `.html` file!

---

Now, to run our Rust-WASM app, you can simply open the `example.html` file in a browser.

Ta da! We've got a web app written in Rust.

If you're interested in learning a bit more about how Rust interfaces with WASM, and you want to do more of the above process "by hand", check out the [MDN "Compiling from Rust to WebAssembly" guide here](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm).

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
