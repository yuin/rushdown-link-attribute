# rushdown-link-attribute
rushdown-link-attribute is an extension for the rushdown that allows you to add attributes to links and images in markdown.

## Installation
Add dependency to your `Cargo.toml`:

```toml
[dependencies]
rushdown-link-attribute = "x.y.z"
```

rushdown-link-attribute can also be used in `no_std` environments. To enable this feature, add the following line to your `Cargo.toml`:

```toml
rushdown-link-attribute = { version = "x.y.z", default-features = false, features = ["no-std"] }
```

## Syntax

```markdown
[aaa](https://example.com/aaa){.myclass}
```

You can add attributes to links and images by using curly braces `{}` after the link or image. Inside the curly braces, you can specify classes, ids, and other attributes.

Attribute syntax is same as the one used in rushdown.

## Usage
### Example

```rust
use core::fmt::Write;
use rushdown::{
    new_markdown_to_html,
    parser::{self, ParserExtension},
    renderer::html::{self, RendererExtension},
    Result,
};
use rushdown_link_attribute::link_attribute_parser_extension;

let markdown_to_html = new_markdown_to_html(
    parser::Options::default(),
    html::Options::default(),
    link_attribute_parser_extension(),
    html::NO_EXTENSIONS,
);
let mut output = String::new();
let input = r#"
[aaa](https://example.com/aaa){.myclass}
"#;
match markdown_to_html(&mut output, input) {
    Ok(_) => {
        println!("HTML output:\n{}", output);
    }
    Err(e) => {
        println!("Error: {:?}", e);
    }
}
```

## Donation
BTC: 1NEDSyUmo4SMTDP83JJQSWi1MvQUGGNMZB

Github sponsors also welcome.

## License
MIT

## Author
Yusuke Inuzuka
