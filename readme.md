# ZeroSync WASM Verifier
The Giza STARK verifier and a ZeroSync proof parser compiled to WebAssembly.


## Build

```sh
npm run build
```


## Development Server

```sh
cd pkg
python -m http.server 
```

The demo expects a chain proof named `proof.bin` in `pkg`.