# Maze generation with Rust(wasm) + JavaScript

## How to build

```
> cd maze
> wasm-pack build
> cd www
> npm install
```

## How to run

```
> cd maze/wwww
> npm run start
```

Open localhost:8080 with Browser (Chrome, etc)

<img src="scr.png">

## How to edit source and run again

- Rust

Edit .rs and do ```wasm-pack build``` again

- JavaScript

Just edit .js and save it. npm process will detect file update and reload .js automatically.
