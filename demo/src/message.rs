#[derive(Debug, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    OnCallState,
    OnIncomingCall,
}
