/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/
const wasm = import('./bindgenhello');

wasm
    .then(h => h.hello("world!"))
    .catch(console.error);

