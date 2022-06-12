# Permen Remuk

If you familiar with "Candy Crush", it is similar, but created using Vanilla Javascript and Web Assembly compiled with Rust.

Why you ask? It's just a fun project to learn Web Assembly and Rust 😁.

## How to build

1. Build the wasm file
    ```
    $ wasm-pack build --not-typescript
    ```
2. Bundle the webfiles
    ```
    $ node_modules/.bin/webpack
    ```
    note: to install js dependencies use `$ npm install`

## Serve the webfiles

```
$ serve -s dist
```