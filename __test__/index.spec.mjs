import { readFileSync } from 'fs'
import { join } from 'path'
import { fileURLToPath } from 'url'

import { Transformer } from '@napi-rs/image'
import test from 'ava'

import { Clipboard } from '../index.js'

test('clipboard text', (t) => {
  const c = new Clipboard()
  c.setText('😅')
  t.is(c.getText(), '😅')
})

test('clipboard image', async (t) => {
  const c = new Clipboard()
  const image = new Transformer(
    readFileSync(join(fileURLToPath(import.meta.url), '..', 'test.png'))
  )
  const { width, height } = await image.metadata()
  const rawPixels = await image.rawPixels()
  t.notThrows(() => {
    c.setImage(width, height, rawPixels)
  })
})
