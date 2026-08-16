use crate::pjsua_wrapper::TPjsuaWrapper;

fn hold(call_id_i32: i32, pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.hold(call_id_i32);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pjsua_wrapper::test_util::{self, PjsuaStub};
    use crate::usecases::answer;
    #[test]
    fn test_hold() {
        let pjsua_stub: Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        let pjsua_stub: Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        let call_id = test_util::make_incoming();
        answer::answer(call_id, &pjsua_stub);
        hold(call_id, &pjsua_stub);

        let call = test_util::get_calls()[0].clone();
        assert_eq!(test_util::TestCallState::Holding, call.state);
    }
}
