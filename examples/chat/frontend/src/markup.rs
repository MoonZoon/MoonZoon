use zoon::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../markup.pest"]
struct MarkupParser;

#[derive(Debug)]
pub enum Object<'a> {
    Text(&'a str),
    Smile,
    SlightSmile,
}

pub fn parse_markup_objects(markup: &str) -> impl Iterator<Item = Object> {
    let objects = MarkupParser::parse(Rule::objects, markup).unwrap_throw();
    objects.filter_map(|object| {
        Some(match object.as_rule() {
            Rule::text => Object::Text(object.as_str()),
            Rule::smile => Object::Smile,
            Rule::slight_smile => Object::SlightSmile,
            Rule::EOI => None?,
            _ => unreachable!(),
        })
    })
}
