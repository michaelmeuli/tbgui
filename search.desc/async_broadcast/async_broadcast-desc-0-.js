searchState.loadedDescShard("async_broadcast", 0, "Async broadcast channel\nThe channel is closed.\nThe channel is empty and closed.\nThe channel is empty and closed.\nThe channel is empty but not closed.\nThe channel is full but not closed.\nThere are currently no active receivers, only inactive …\nAn inactive  receiver.\nThe channel has overflowed since the last element was …\nThe channel has overflowed since the last element was …\nThe receiving side of a channel.\nA future returned by <code>Receiver::recv()</code>.\nAn error returned from <code>Receiver::recv()</code>.\nA future returned by <code>Sender::broadcast()</code>.\nAn error returned from <code>Sender::broadcast()</code>.\nThe sending side of the broadcast channel.\nAn error returned from <code>Receiver::try_recv()</code>.\nAn error returned from <code>Sender::try_broadcast()</code>.\nConvert to an activate <code>Receiver</code>.\nCreate an activate <code>Receiver</code> for the associated channel.\nIf sender will wait for active receivers.\nIf sender will wait for active receivers.\nIf sender will wait for active receivers.\nCreate a new broadcast channel.\nBroadcasts a message on the channel.\nBroadcasts a message on the channel using the blocking …\nBroadcasts a message on the channel without pinning the …\nReturns the channel capacity.\nReturns the channel capacity.\nReturns the channel capacity.\nProduce a clone of this Receiver that has the same …\nCloses the channel.\nCloses the channel.\nCloses the channel.\nDowngrade to a <code>InactiveReceiver</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the number of inactive receivers for the channel.\nReturns the number of inactive receivers for the channel.\nReturns the number of inactive receivers for the channel.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUnwraps the message that couldn’t be sent.\nUnwraps the message that couldn’t be sent.\nReturns <code>true</code> if the channel is closed.\nReturns <code>true</code> if the channel is closed.\nReturns <code>true</code> if the channel is closed.\nReturns <code>true</code> if the channel is empty and closed.\nReturns <code>true</code> if the channel is closed.\nReturns <code>true</code> if there are currently no active receivers, …\nReturns <code>true</code> if the channel is empty.\nReturns <code>true</code> if the channel is empty.\nReturns <code>true</code> if the channel is empty but not closed.\nReturns <code>true</code> if the channel is empty.\nReturns <code>true</code> if the channel is full.\nReturns <code>true</code> if the channel is full.\nReturns <code>true</code> if the channel is full but not closed.\nReturns <code>true</code> if the channel is full.\nReturns <code>true</code> if this error indicates the receiver missed …\nReturns the number of messages in the channel.\nReturns the number of messages in the channel.\nReturns the number of messages in the channel.\nProduce a new Receiver for this channel.\nProduce a new Receiver for this channel.\nProduce a new Sender for this channel.\nIf overflow mode is enabled on this channel.\nIf overflow mode is enabled on this channel.\nIf overflow mode is enabled on this channel.\nA low level poll method that is similar to <code>Receiver::recv()</code>…\nReturns the number of receivers for the channel.\nReturns the number of receivers for the channel.\nReturns the number of receivers for the channel.\nReceives a message from the channel.\nReceives a message from the channel using the blocking …\nReceives a message from the channel without pinning the …\nReturns the number of senders for the channel.\nReturns the number of senders for the channel.\nReturns the number of senders for the channel.\nSpecify if sender will wait for active receivers.\nSpecify if sender will wait for active receivers.\nSpecify if sender will wait for active receivers.\nSet the channel capacity.\nSet the channel capacity.\nSet the channel capacity.\nSet overflow mode on the channel.\nSet overflow mode on the channel.\nSet overflow mode on the channel.\nAttempts to broadcast a message on the channel.\nAttempts to receive a message from the channel.")