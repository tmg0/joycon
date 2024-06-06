use crate::core::options::Options;

pub struct Context {
    pub options: Options,
}

pub fn create_context(options: Options) -> Context {
    Context { options }
}
