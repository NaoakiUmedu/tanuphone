use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn hangup(pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.hangup();
}

#[cfg(test)]
mod test {
    use crate::{pjsua_wrapper::{self, test_util::PjsuaStub}, usecases};
    use super::*;

    #[test]
    fn test_hangup() {
        let pjsua_stub : Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        usecases::account_add::account_add("1001", "@1001", "test.invalid", &pjsua_stub);
        usecases::callto::callto("1002", "test.invalid", &pjsua_stub);

        let calls = pjsua_wrapper::test_util::get_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!("1002".to_string(), calls[0].callee);
        assert_eq!("test.invalid".to_string(), calls[0].domain);

        hangup(&pjsua_stub);

        let calls = pjsua_wrapper::test_util::get_calls();
        assert_eq!(calls.len(), 0);

        pjsua_stub.destroy();
    }
}
