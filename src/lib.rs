#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use rushdown::ast::Arena;
use rushdown::ast::NodeRef;
use rushdown::ast::Text;
use rushdown::context::ContextKey;
use rushdown::context::ContextKeyRegistry;
use rushdown::context::ObjectValue;
use rushdown::matches_kind;
use rushdown::parser;
use rushdown::parser::parse_attributes;
use rushdown::parser::parser_extension;
use rushdown::parser::AnyInlineParser;
use rushdown::parser::InlineParser;
use rushdown::parser::NoParserOptions;
use rushdown::parser::Parser;
use rushdown::parser::ParserExtension;
use rushdown::text;

// Parser {{{

#[derive(Debug)]
struct LinkAttributeParser {
    list: ContextKey<ObjectValue>,
}

impl LinkAttributeParser {
    fn new(reg: Rc<RefCell<ContextKeyRegistry>>) -> Self {
        Self {
            list: reg.borrow_mut().create::<ObjectValue>(),
        }
    }
}

impl InlineParser for LinkAttributeParser {
    fn trigger(&self) -> &[u8] {
        b"{"
    }

    fn parse(
        &self,
        arena: &mut Arena,
        parent_ref: NodeRef,
        reader: &mut text::BlockReader,
        ctx: &mut parser::Context,
    ) -> Option<NodeRef> {
        let prev = arena[parent_ref].last_child()?;
        if !matches_kind!(arena[prev], Link) && !matches_kind!(arena[prev], Image) {
            return None;
        }
        let attributes = parse_attributes(reader)?;
        arena[prev].attributes_mut().extend(attributes);
        let n = arena.new_node(Text::new(text::Value::Empty));

        let mut list_opt = ctx.get_mut(self.list);
        if list_opt.is_none() {
            ctx.insert(self.list, Box::new(Vec::<NodeRef>::new()));
            list_opt = ctx.get_mut(self.list);
        }
        let list = list_opt.unwrap().downcast_mut::<Vec<NodeRef>>().unwrap();
        list.push(n);

        Some(n)
    }

    fn close_block(
        &self,
        arena: &mut Arena,
        _node_ref: NodeRef,
        _reader: &mut text::BlockReader,
        ctx: &mut parser::Context,
    ) {
        if let Some(list) = ctx.get_mut(self.list) {
            let list = list.downcast_mut::<Vec<NodeRef>>().unwrap();
            for n in list.drain(..) {
                n.delete(arena);
            }
        }
    }
}

impl From<LinkAttributeParser> for AnyInlineParser {
    fn from(p: LinkAttributeParser) -> Self {
        AnyInlineParser::Extension(Box::new(p))
    }
}

// }}}

// Extension {{{

/// Returns a parser extension that parses link attributes.
pub fn link_attribute_parser_extension() -> impl ParserExtension {
    parser_extension(|p: &mut Parser| {
        p.add_inline_parser(LinkAttributeParser::new, NoParserOptions, 500);
    })
}

// }}}
