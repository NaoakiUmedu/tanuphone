use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn answer(call_id_i32: i32, pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.answer(call_id_i32);
}

// TODO ユニットテストで着信を模擬する仕組みを作る
#[cfg(test)]
mod test {
    use crate::pjsua_wrapper::{self, test_util::{self, PjsuaStub}};
    use super::*;

    #[test]
    fn test_answer() {
        let pjsua_stub : Box<dyn TPjsuaWrapper> = Box::new(PjsuaStub {});
        pjsua_stub.init();

        test_util::make_incoming(0);
        let call = test_util::get_incomming_calls()[0].clone();
        assert_eq!(test_util::TestCallState::Incomming, call.state);

        answer(0, &pjsua_stub);
        let call = test_util::get_incomming_calls()[0].clone();
        assert_eq!(test_util::TestCallState::Talking, call.state);

        pjsua_stub.destroy();
    }
}
