# ur-wasm-js

WASM/JS bindings for the ur-rust rust library

## Getting started

### Installation

Either build the library yourself with wasm-pack or install for npm

```sh
# Install from npm
npm i ur-wasm-js
```

```sh
# Or build from source
git clone https://github.com/lightning-digital-entertainment/ur-wasm-js
cd ur-wasm-js
wasm-pack build
# In your project dir
npm i path/to/pkg/inside/ur-wasm-js
```

### Use it in Javascript

```js
import * as ur from "ur-wasm-js";

const textdecoder = new TextDecoder();

const encoder = new ur.UrEncoder(
  "This is pretty awesome generating qr codes like this, isnt it?!?!?!?!?!?!?!?",
  5,
);
const decoder = new ur.UrDecoder();

while (!decoder.complete()) {
  const part = encoder.next_value();
  decoder.receive(part);
}

const messageAsBytes = decoder.message();
const messageAsString = textdecoder.decode(messageAsBytes);

console.log(messageAsString);
```
