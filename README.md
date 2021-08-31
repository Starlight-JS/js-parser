# js-parser

JavaScript parser based on V8 (basically RIIR of V8 parser into Rust). 

## Why? 

[Starlight](https://github.com/starlight-js/starlight) requires advanced parser and AST that provides scope information after parsing for producing effective bytecode which is not possible with SWC so I've decided to start this project.