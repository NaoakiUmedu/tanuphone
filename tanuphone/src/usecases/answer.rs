use crate::pjsua_wrapper::TPjsuaWrapper;

pub fn answer(call_id_i32: i32, pjsua: &Box<dyn TPjsuaWrapper>) {
    pjsua.answer(call_id_i32);
}

// TODO ユニットテストで着信を模擬する仕組みを作る
