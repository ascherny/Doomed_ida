use crate::metadata::Metadata;

pub trait FileFormat {
    fn name(&self) -> &'static str;
    fn metadata(&self) -> Metadata;
}
