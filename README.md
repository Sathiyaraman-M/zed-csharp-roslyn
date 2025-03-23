## C# (Roslyn)

> Experimental: Needs a local build of [roslyn-language-server](https://github.com/SofusA/roslyn-language-server) as an executable.

This is a Zed extension to provide C# through the new Roslyn LSP.

Currently, it relies on a manual installation of the Roslyn Language Server.
Moreover, it doesn't support Razor-specific services currently as it requires another LSP (rzls).

## TODO

- Provide Installation Guide
- Integrate Razor support with `rzls`
- Improve the approach of starting the `roslyn-language-server` executable.
