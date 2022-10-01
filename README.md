# Highlight Converter

Converts exported kindle highlights to other formats.

## Motivation

After reader finishes a Kindle book it is handy to place highlights from the book
into some readable format for revisiting it later.
The `highlights` utility is created to convert exported highlights to other formats
more suitable for placing into note-taking software.

## Usage

```shell
highlight [input] <output>
```

### Examples

Read highlights from the bookcision json file and render to the output stream.

```shell
highlight input.json
```

Read highlights from the bookcision json file and render to the file.

```shell
example kasparov.json kasparov.md
```

## Supported output formats

### Markdown

The highlights are suitable for importing into the Obsidian or similar software that works with Markdown format.

## Supported input formats

### Bookcision JSON

A free [bookcision](https://readwise.io/bookcision) service provides a way to export
kindle highlights to the json file.
Highlight support those files as an input.
