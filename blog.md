The last months I have been focusing on using the Rust programming language in relation to game development.
I wanted a setup where I could implement a game that could build for both native, such as Windows, and WASM, for the browsers.

For this I have been working on a pet project called Blueprint.
Intention with Blueprint was to create a Rust template that could be quickly generated using cargo-generate.
Features of the project included:
- 2d and 3d rendering.
- many thousands of sprites using VBO batching
- entity component setup using Hecs.
- pre-defined systems such as movement system, physics systems.

My primary motivation was a template where I could quickly prototype game ideas using Rust.
Prevously I have been using Typescript + PIXI.js or THREE.js for this.
But since I am a huge Rust fanatic I wanted to see if I could conjure up a similar setup using Rust + libraries such as winit, xx, yy, etc.

Recently however, I stumbled upon Bevy, a data driven game engine written in Rust.
Bevy more or less ticks all the boxes above, except for WASM support.
I want to build my games such that they can be quickly shared in the browser for other to see, thus WASM is a non-optional thing. 

However, it seems that WASM support is a focus area of Bevy and it seems it is currently possible to run Bevy in the browser using webgl plugins.

In any case.
I have decided to put my own Blueprint project on hold and fiddle a bit with Bevy before continuing down a path which seems to be well underway by the community!

If all goes well, I can ditch my efforts on my own brewed Blueprint and make a Bevy template!

