# Batcher

This is my batcher. It's a hot mess of continous/shutgun design patterns. Its very light irl ram wise so it can squeeze out a bit more worker scripts before crashing the game.

The first argument sets a fixed target. With a fixed target the Batcher does not rely on the Formulas API. If not target is passed the Formulas API will be used to calculate the most profitable target.

This batcher wont use any RAM on `home` by default (cause that stops you from running other scripts). By adding the `-h` flag home RAM will also be filled with batches.

`run Batcher.js <target> -h`
