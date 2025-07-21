import React, { useEffect } from 'react';
import { View, StyleSheet } from 'react-native';
import { Camera } from 'expo-camera';

export default function QRScanner({ onRead }: { onRead: (data: string) => void }) {
  const [permission, requestPermission] = Camera.useCameraPermissions();

  useEffect(() => {
    if (!permission?.granted) requestPermission();
  }, [permission]);

  if (!permission?.granted) return null;

  return (
    <Camera
      style={styles.camera}
      onBarCodeScanned={({ data }) => onRead(data)}
      barCodeScannerSettings={{ barCodeTypes: ['qr'] }}
    />
  );
}

const styles = StyleSheet.create({
  camera: { flex: 1, width: '100%' },
});
