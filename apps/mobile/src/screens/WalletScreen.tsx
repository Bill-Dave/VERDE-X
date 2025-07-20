import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import LinearGradient from 'react-native-linear-gradient';

export default function WalletScreen() {
  return (
    <LinearGradient colors={['#8B0000', '#000']} style={styles.container}>
      <Text style={styles.title}>Wallets</Text>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, alignItems: 'center', justifyContent: 'center' },
  title: { fontSize: 24, color: '#FFC107' },
});
