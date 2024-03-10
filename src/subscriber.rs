
struct Subscriber<ValueT> {
    node: &Node,    // reference to the owner node
    topic: &Topic,  // reference to the topic
    callback: fn(Message<ValueT>)
}

