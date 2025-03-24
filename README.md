## C# (Roslyn)

This is a Zed extension to provide C# through the new Roslyn LSP. Currently it uses [roslyn-language-server](https://github.com/SofusA/roslyn-language-server), which is a wrapper around the official Roslyn Language Server.

It doesn't support Razor-specific services as of now, since it requires another LSP (rzls) and it will be supported in the future.

### Installation Guide

> Note: This extension is not yet published in the Zed Extension Marketplace. So you need to sideload it as a dev extension.

- Clone this repository: `git clone https://github.com/Sathiyaraman-M/csharp-roslyn.git`.
- Go to Zed Extensions.
- Click "Install Dev Extension" and select the repository directory.
- Enjoy!
