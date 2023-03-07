# `@napi-rs/clipboard`

Manipulate Clipboard in Node.js via native API.

It's a Node.js binding for [1Password/aboard](https://github.com/1Password/arboard) with additions from [rgwood/clipboard-anywhere](https://github.com/rgwood/clipboard-anywhere)

[![install size](https://packagephobia.com/badge?p=@napi-rs/clipboard)](https://packagephobia.com/result?p=@napi-rs/clipboard)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/clipboard.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/clipboard?minimal=true)

## API

### Text

```js
import { Clipboard } from '@napi-rs/clipboard'

const clipboard = new Clipboard()

clipboard.setText('😅')
clipboard.getText() // '😅'
```

### Image

```js
import { join } from 'path'
import { fileURLToPath } from 'url'

import { Clipboard } from '@napi-rs/clipboard'
import { Transformer } from '@napi-rs/image'

const image = new Transformer(
  readFileSync(join(fileURLToPath(import.meta.url), '..', 'test.png'))
)
const { width, height } = await image.metadata()
const rawPixels = await image.rawPixels()
// Only accept raw RGBA pixels
clipboard.setImage(width, height, image)

// You can paste image now
```
