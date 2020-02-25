/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/
fetch('./func_test.wasm').then(response =>
    response.arrayBuffer()
  ).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
    console.log("Loaded wasm module");
    instance = results.instance;
    console.log("instance", instance);
  
    var white = 2;
    var black = 1;
    var crowned_white = 6;
    var crowned_black = 5;
  
    console.log("Calling offset");
    var offset = instance.exports.offsetForPosition(3,4);
    console.log("Offset for 3,4 is ",offset);
  
    console.debug("White is white?", instance.exports.isWhite(white));
    console.debug("Black is black?", instance.exports.isBlack(black));
    console.debug("Black is white?", instance.exports.isWhite(black));
    console.debug("Uncrowned white",
      instance.exports.isWhite(instance.exports.withoutCrown(crowned_white)));
    console.debug("Uncrowned black",
      instance.exports.isBlack(instance.exports.withoutCrown(crowned_black)));
    console.debug("Crowned is crowned",
      instance.exports.isCrowned(crowned_black));
    console.debug("Crowned is crowned (b)",
      instance.exports.isCrowned(crowned_white));
  });