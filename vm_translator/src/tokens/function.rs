use crate::tokenizer::TryParse;

pub enum Function {}

impl TryParse for Function {
    fn try_parse(value: &str, iter: &mut impl Iterator<Item = super::raw::RawToken>) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
