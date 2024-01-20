## About
If you have not read through the [Yew tutorial](https://yew.rs/docs/tutorial), please do so.
This example starts where they leave off.

## Dependecies
Webatui handles most of the interactions with Yew, so you can remove it as a dependency.
Replace the Yew dependency with a dependency on Ratatui and Webatui.

## The index
The base index.html that Yew provide is a good starting point; however, TUIs need a monospace font.
For this, we'll be using Adobe's Source Code Pro font.
See this example's index file.

## The main function
Instead of needing to create a component and render HTML, the webatui's `WebTerminal` will handle all of that are you.
The `WebTerminal` wraps around any type that implements the `TerminalApp` trait.
This is the core abstraction that webatui revolves around.
Implementing it is fairly straightforward.
See `src/main.rs`
