import React, { useEffect, useState } from 'react';
import { GiftedChat, IMessage } from 'react-native-gifted-chat';
import { io } from 'socket.io-client';
import { useRoute } from '@react-navigation/native';

const socket = io('http://localhost:3000');

export default function DisputeChatScreen() {
  const route = useRoute<any>();
  const { escrowId } = route.params;
  const [messages, setMessages] = useState<IMessage[]>([]);

  useEffect(() => {
    socket.emit('join_dispute', escrowId);
    socket.on('new_message', (msg: IMessage) =>
      setMessages((prev) => GiftedChat.append(prev, [msg]))
    );
    return () => socket.off('new_message');
  }, [escrowId]);

  const onSend = (newMsgs: IMessage[]) => {
    const [msg] = newMsgs;
    socket.emit('send_message', { ...msg, escrowId });
  };

  return (
    <GiftedChat
      messages={messages}
      onSend={onSend}
      user={{ _id: 1, name: 'Me' }}
      renderUsernameOnMessage
    />
  );
}
