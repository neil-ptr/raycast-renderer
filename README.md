# Raycasted 2.5D Horror Game

## Motivation

How Important is Sound in Horror Games Compared to Visuals?

Sound plays a crucial role in defining the horror experience, often surpassing visuals in its ability to build tension and unease. Games like [Lethal Company](https://store.steampowered.com/app/1966720/Lethal_Company/) and [Phasmophobia](https://store.steampowered.com/app/739630/Phasmophobia/) owe much of their success to their outstanding sound design. This raises the question: How important is it to get the sound design of a game right?

This project seeks to explore this by crafting a retro-inspired 2.5D horror game that combines simple retro visuals with a strrong focus on proximity chat and sound. 

## Technologies
No major frameworks used really, wanted to write the raycaster from scratch in rust. Chose Rust because I liked the `rust-wasm` api compared to c++'s `emscripten`

### Raycasting game engine
- rust
- rust-wasm
- HTML Canvas

### Backend
- Go

### Commands
* `wasm-pack build --out-dir ../static --target web`: compiles rust to wasm and puts it in the static directory of the Go app
