#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub message: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageType {
    OnCallState,
    OnIncomingCall,
    RegisterComplete,
    OnCallMediaStateActive,
}
