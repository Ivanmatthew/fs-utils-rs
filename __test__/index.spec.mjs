import test from 'ava'
import fs from 'node:fs'

import { copyFolderRecursive, deleteFolderRecursive } from '../index.js'

test.beforeEach(() => {
  fs.mkdirSync('./test', { recursive: true })
  fs.writeFileSync('./test/test.txt', 'Hello World!')

  fs.mkdirSync('./test/test', { recursive: true })
  fs.writeFileSync('./test/test/test.txt', 'Hello World!')
})

test('Copies succesfully, recursively.', (t) => {
  copyFolderRecursive('./test', './test2')

  t.true(fs.existsSync('./test2/test.txt'))
  t.true(fs.existsSync('./test2/test/test.txt'))
})

test('Deletes succesfully, recursively.', (t) => {
  deleteFolderRecursive('./test')
  t.false(fs.existsSync('./test'))

  deleteFolderRecursive('./test2')
  t.false(fs.existsSync('./test2'))
})
