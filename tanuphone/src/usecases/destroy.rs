use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn destroy(pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.destroy();
}

#[cfg(test)]
mod test {
    use crate::pjsua_wrapper::{test_util::PjsuaStub};
    use super::*;

    #[test]
    fn test_destroy() {
        let pjsua_stub : Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        // no assertion!

        pjsua_stub.destroy();
    }
}
