# Miniature

A **tiny** Scoop shim alternative, written in Rust.

Clocks in at about 20KB, compared to the current [better-shim-exe](https://github.com/71/scoop-better-shimexe), which is about 113KB.
That's about 17% the size.

Miniature also encodes the shim target and arguments directly in the binary's resources, so no more parsing an external `.shim` file.

**Made with 💗 by Juliette Cordor**
