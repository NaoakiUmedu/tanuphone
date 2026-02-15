use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn callto(callee: &str, domein: &str, pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.callto(callee, domein);
}

#[cfg(test)]
mod test {
    use crate::pjsua_wrapper::{self, test_util::PjsuaStub};
    use super::*;

    #[test]
    fn test_callto() {
        let pjsua_stub : Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        callto("1002", "test.invalid", &pjsua_stub);

        let calls = pjsua_wrapper::test_util::get_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!("1002".to_string(), calls[0].callee);
        assert_eq!("test.invalid".to_string(), calls[0].domain);

        pjsua_stub.destroy();
    }
}
