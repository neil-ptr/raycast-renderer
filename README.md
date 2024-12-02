# Raycasted 2.5D Horror Game

[PLAY HERE](#) Earbuds + Fullscreen is recommended

## Motivation

How Important is Sound in Horror Games Compared to Visuals?

Sound plays a crucial role in defining the horror experience, often surpassing visuals in its ability to build tension and unease. Games like [Lethal Company](https://store.steampowered.com/app/1966720/Lethal_Company/) and [Phasmophobia](https://store.steampowered.com/app/739630/Phasmophobia/) owe much of their success to their outstanding sound design. This raises the question: How important is it to nail the sound design of a game?

This project seeks to explore this by crafting a retro-inspired 2.5D horror game that combines simple retro visuals with a strrong focus on proximity chat and sound. 

## Technologies
No major frameworks or libraries used really, wanted to write the raycaster from scratch. Chose to use WASM instead of writing JS since I want to I really want to focus on simulating sound propagation and don't want to worry about performance too much. Chose Rust because I liked the `rust-wasm` api compared to c++'s `emscripten`. Also canvas has a pretty simple API that should suffice for what I want to achieve, hence I'm not using WebGL.

### Raycasting game engine
- rust
- rust-wasm
- HTML Canvas

### Backend
- Go

### Commands
* `wasm-pack build --out-dir ../static --target web`: compiles rust to wasm and puts it in the static directory of the Go app
* `cargo watch -s "wasm-pack build --out-dir ../static --target web && echo 'Recompiled!'"`: hot reloading
