use lombok_macros::*;

#[allow(dead_code)]
#[derive(Lombok, Debug, Default)]
pub struct Worker {
    pub(super) id: usize,
}
