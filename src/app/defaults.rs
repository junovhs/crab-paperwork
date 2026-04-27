pub const DEFAULT_MARKDOWN: &str = r#"# Markdown syntax guide

## Headers

# This is a Heading h1
## This is a Heading h2
###### This is a Heading h6

## Emphasis

*This text will be italic*  
_This will also be italic_

**This text will be bold**  
__This will also be bold__

_You **can** combine them_

## Lists

### Unordered

* Item 1
* Item 2
* Item 2a
* Item 2b
    * Item 3a
    * Item 3b

### Ordered

1. Item 1
2. Item 2
3. Item 3
    1. Item 3a
    2. Item 3b

## Links

You may be using [Markdown Live Preview](https://markdownlivepreview.com/).

## Blockquotes

> Markdown is a lightweight markup language with plain-text-formatting syntax.
>
>> Markdown is often used to format readme files and messages.

## Tables

| Left columns | Right columns |
| ------------ | :-----------: |
| left foo     | right foo     |
| left bar     | right bar     |
| left baz     | right baz     |

## Blocks of code

```rust
fn main() {
    println!("Hello world");
}
```

## Inline code

This app renders Markdown with `pulldown-cmark` and sanitizes preview HTML.
"#;
