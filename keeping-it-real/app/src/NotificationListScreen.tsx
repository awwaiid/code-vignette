import React, { useEffect, useState, useCallback, useRef } from 'react';
import {
  FlatList,
  View,
  Text,
  Pressable,
  StyleSheet,
  Linking,
  ActivityIndicator,
  Alert,
} from 'react-native';
import {
  getNotifications,
  firePendingIntent,
  isPermissionGranted,
  notificationEmitter,
  type NotificationRecord,
} from './NativeNotificationModule';
import NotificationRow from './NotificationRow';

const ROW_HEIGHT = 80; // approximate fixed height for getItemLayout

export default function NotificationListScreen() {
  const [records, setRecords]         = useState<NotificationRecord[]>([]);
  const [loading, setLoading]         = useState(true);
  const [hasPermission, setHasPermission] = useState<boolean | null>(null);
  const knownKeys = useRef(new Set<string>());

  // Check permission + load history on mount
  useEffect(() => {
    let mounted = true;

    (async () => {
      const granted = await isPermissionGranted();
      if (!mounted) return;
      setHasPermission(granted);

      if (granted) {
        const data = await getNotifications();
        if (!mounted) return;
        for (const r of data) knownKeys.current.add(r.key);
        setRecords(data);
      }
      setLoading(false);
    })();

    return () => { mounted = false; };
  }, []);

  // Subscribe to realtime events
  useEffect(() => {
    const sub = notificationEmitter.addListener(
      'onNotificationPosted',
      (record: NotificationRecord) => {
        if (knownKeys.current.has(record.key)) return; // duplicate guard
        knownKeys.current.add(record.key);
        setRecords(prev => [record, ...prev]);
      }
    );
    return () => sub.remove();
  }, []);

  const handlePress = useCallback(async (key: string) => {
    const fired = await firePendingIntent(key);
    if (!fired) {
      Alert.alert(
        'Cannot open',
        'The original notification action is no longer available. ' +
        'This happens after the app restarts or the notification expires.'
      );
      // Update hasPendingIntent in state so the row dims
      setRecords(prev =>
        prev.map(r => r.key === key ? { ...r, hasPendingIntent: false } : r)
      );
    }
  }, []);

  const openSettings = useCallback(() => {
    Linking.sendIntent('android.settings.ACTION_NOTIFICATION_LISTENER_SETTINGS');
  }, []);

  if (loading) {
    return (
      <View style={styles.center}>
        <ActivityIndicator size="large" color="#4af" />
      </View>
    );
  }

  return (
    <View style={styles.root}>
      {hasPermission === false && (
        <Pressable style={styles.permBanner} onPress={openSettings}>
          <Text style={styles.permText}>
            Tap to grant Notification Access — required to capture notifications
          </Text>
        </Pressable>
      )}

      <FlatList
        data={records}
        keyExtractor={item => item.key}
        renderItem={({ item }) => (
          <NotificationRow record={item} onPress={handlePress} />
        )}
        getItemLayout={(_, index) => ({
          length: ROW_HEIGHT,
          offset: ROW_HEIGHT * index,
          index,
        })}
        initialNumToRender={20}
        maxToRenderPerBatch={10}
        windowSize={10}
        ListEmptyComponent={
          <View style={styles.empty}>
            <Text style={styles.emptyText}>
              {hasPermission
                ? 'No notifications captured yet.\nNew ones will appear here in realtime.'
                : 'Grant notification access to begin.'}
            </Text>
          </View>
        }
        contentContainerStyle={records.length === 0 ? styles.emptyContainer : undefined}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  root: {
    flex: 1,
    backgroundColor: '#111',
  },
  center: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#111',
  },
  permBanner: {
    backgroundColor: '#7a3500',
    paddingHorizontal: 16,
    paddingVertical: 10,
  },
  permText: {
    color: '#ffd',
    fontSize: 13,
    lineHeight: 18,
  },
  empty: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 32,
  },
  emptyContainer: {
    flexGrow: 1,
  },
  emptyText: {
    color: '#555',
    fontSize: 15,
    textAlign: 'center',
    lineHeight: 22,
  },
});
