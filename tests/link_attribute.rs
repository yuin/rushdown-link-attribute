use rushdown::{
    new_markdown_to_html_string,
    parser::{self},
    renderer::html,
    test::{MarkdownTestCase, MarkdownTestCaseOptions},
};
use rushdown_link_attribute::link_attribute_parser_extension;

#[test]
fn test_link_attribute() {
    let markdown_to_html = new_markdown_to_html_string(
        parser::Options::default(),
        html::Options {
            allows_unsafe: true,
            xhtml: false,
            ..html::Options::default()
        },
        link_attribute_parser_extension(),
        html::NO_EXTENSIONS,
    );
    {
        let source = r#"
[aaa](https://example.com/aaa){.myclass}
"#;
        MarkdownTestCase::new(
            1,
            "ok",
            source,
            r#"<p><a href="https://example.com/aaa" class="myclass">aaa</a></p>
"#,
            MarkdownTestCaseOptions::default(),
        )
        .execute(&markdown_to_html);
    }
    {
        let source = r#"
[aaa]{.myclass}

[bbb][aaa]{target="_blank"}

[aaa]: https://example.com/aaa
"#;
        MarkdownTestCase::new(
            2,
            "reference link",
            source,
            r#"<p><a href="https://example.com/aaa" class="myclass">aaa</a></p>
<p><a href="https://example.com/aaa" target="_blank">bbb</a></p>
"#,
            MarkdownTestCaseOptions::default(),
        )
        .execute(&markdown_to_html);
    }
    {
        let source = r#"
[aaa](https://example.com/aaa) {.myclass}
"#;
        MarkdownTestCase::new(
            3,
            "ng",
            source,
            r#"<p><a href="https://example.com/aaa">aaa</a> {.myclass}</p>
"#,
            MarkdownTestCaseOptions::default(),
        )
        .execute(&markdown_to_html);
    }
}

#[test]
fn test_image_attribute() {
    let markdown_to_html = new_markdown_to_html_string(
        parser::Options::default(),
        html::Options {
            allows_unsafe: true,
            xhtml: false,
            ..html::Options::default()
        },
        link_attribute_parser_extension(),
        html::NO_EXTENSIONS,
    );
    {
        let source = r#"
![aaa](https://example.com/aaa){.myclass width=100}
"#;
        MarkdownTestCase::new(
            1,
            "ok",
            source,
            r#"<p><img src="https://example.com/aaa" alt="aaa" class="myclass" width="100"></p>
"#,
            MarkdownTestCaseOptions::default(),
        )
        .execute(&markdown_to_html);
    }
    {
        let source = r#"
![aaa](https://example.com/aaa) {.myclass}
"#;
        MarkdownTestCase::new(
            2,
            "ng",
            source,
            r#"<p><img src="https://example.com/aaa" alt="aaa"> {.myclass}</p>
"#,
            MarkdownTestCaseOptions::default(),
        )
        .execute(&markdown_to_html);
    }
}
