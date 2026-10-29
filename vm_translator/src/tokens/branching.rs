use crate::tokenizer::TryParse;

pub enum Branching {}

impl TryParse for Branching {
    fn try_parse(value: &str, iter: &mut impl Iterator<Item = super::raw::RawToken>) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
