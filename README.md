# miniature

A **tiny** Scoop shim alternative, written in Rust.

Clocks in at about 13KB, compared to the current [better-shim-exe](https://github.com/kiennq/scoop-better-shimexe), which is about 133KB.
That's about 10% the size.

miniature doesn't rely on any standard libraries from Rust or C, or Microsoft's C runtime,
and only uses the Windows SDK, which comes bundled as part of every Windows installation.
So it's **very** portable.

Miniature also encodes the shim target and arguments directly in the binary's resources, so no more parsing an external `.shim` file.

**Made with 💗 by Juliette Cordor**
