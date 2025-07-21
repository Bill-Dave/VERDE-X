import React from 'react';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import HomeScreen from '../screens/HomeScreen';
import CreateEscrowScreen from '../screens/CreateEscrowScreen';
import DisputeChatScreen from '../screens/DisputeChatScreen';

export type RootStackParamList = {
  Home: undefined;
  CreateEscrow: undefined;
  DisputeChat: { escrowId: string };
};

const Stack = createNativeStackNavigator<RootStackParamList>();

export default function RootStack() {
  return (
    <Stack.Navigator screenOptions={{ headerShown: false }}>
      <Stack.Screen name="Home" component={HomeScreen} />
      <Stack.Screen name="CreateEscrow" component={CreateEscrowScreen} />
      <Stack.Screen name="DisputeChat" component={DisputeChatScreen} />
    </Stack.Navigator>
  );
}
