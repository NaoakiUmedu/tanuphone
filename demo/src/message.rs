#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    OnCallState,
    OnIncomingCall,
}
