
struct Topic<MessageT> {
    name: String,
    messages: VecDeque<MessageT> // message queue. the message is removed from the queue after it is published to all subscribers
}

