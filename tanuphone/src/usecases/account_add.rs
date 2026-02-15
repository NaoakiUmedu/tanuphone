use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn account_add(user: &str, pass: &str, domain: &str, pjsua: &Box<dyn TPjsuaWrapper>) -> i32 {
    pjsua.account_add(user, pass, domain)
}

// TODO TPjsuaWrapperを実装したスタブを作って、account_add()をテストする
#[cfg(test)]
mod test {
    use crate::pjsua_wrapper::{self, test_util::PjsuaStub};
    use super::*;

    #[test]
    fn test_account_add() {
        let pjsua_stub : Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        let _ = super::account_add("1001", "@1001", "test.invalid", &pjsua_stub);
        let _ = super::account_add("1002", "@1002", "test.invalid", &pjsua_stub);

        let accs = pjsua_wrapper::test_util::get_added_accounts();
        assert_eq!(2 as usize, accs.len());
        assert_eq!("1001", accs[0].user);
        assert_eq!("@1001", accs[0].pass);
        assert_eq!("test.invalid", accs[0].domain);
        assert_eq!("1002", accs[1].user);
    }
}
