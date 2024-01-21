[![Crates.io](https://img.shields.io/crates/v/webatui.svg)](https://crates.io/crates/webatui)
[![Documentation](https://docs.rs/webatui/badge.svg)](https://docs.rs/webatui/)
![GitHub Workflows](https://github.com/TylerBloom/webatui/actions/workflows/ar_ci.yml/badge.svg)
[![Coverage Status](https://codecov.io/gh/TylerBloom/webatui/branch/main/graph/badge.svg)](https://codecov.io/gh/TylerBloom/webatui)
![Maintenance](https://img.shields.io/badge/Maintenance-Actively%20Developed-brightgreen.svg)

## About
Webatui is an integration between the [Yew](https://github.com/yewstack/yew) and [Ratatui](https://github.com/ratatui-org/ratatui) for building TUI-themed webapps.
As much as possible, webatui is built to be "plug and play".
That is, if you already have a TUI app that you want to display in the browser, you should be able to do so with minimal refactoring.

The crate was inspired by the creation of my blog, [The Avid Rustacean](https://avid-rustacean.shuttleapp.rs/).
A write up on how Yew and Ratatui are integrated can be found in the blog [inital post](https://avid-rustacean.shuttleapp.rs/blog/About-This-Blog).

What webatui does:
 - Renders the text to HTML
 - Supports Ratatui's index colors via [base16-palettes](https://github.com/TylerBloom/base16-palettes)
 - Supports hyperlinks
 - Supports mouse events (clicks)
 - Supports automatic screen resizing
 - Supports scrolling (on PC and mobile)

## Basic Usage
This crate has examples that you can use as a template for your own project.
To get a more complete understanding, I would recommend following the [Yew tutorial](https://yew.rs/docs/tutorial).
Once you have wrapped your head around that (or just gotten it to run), come back here.

For basic apps, add Ratatui and Webatui as dependencies.
Webatui handles nearly all of the Yew-related details, but your app will use ratatui to render the text.
See the hello world example to see how to turn your Yew hello world example into a Webatui hello world.

There are a few important details.
Webatui handrolls most of its own CSS; however, you must use a monospace font.
Pick a font that you like, but every example uses Adobe's Source Code Pro font.
Many monospace fonts will render well on PC but do not render in a strickly monospace fashion on mobile.
Take this into consideration when picking your font.

## Contributing
This project is licensed under LGPL-2.1.
You may use this crate as a dependency on a closed-source/properity codebase; however, if you improve up or derivate another library crate from this crate, please license that crate under the LGPL-2.1 or a strong GPL license.

If you discover a bug, would like a feature supported, or have some way of improving this crate, please feel free to open an issue or a PR.

## Future Plans
This crate was born from an abstraction that I needed for my blog.
As such, there are surely ways to improve upon it.
I will avoid needlessly adding breaking changes; however, they still many occur for improved ergonomics, effieceny, final binary size, and more.
