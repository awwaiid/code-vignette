import React, { memo } from 'react';
import {
  View,
  Text,
  Image,
  Pressable,
  StyleSheet,
} from 'react-native';
import type { NotificationRecord } from './NativeNotificationModule';

interface Props {
  record: NotificationRecord;
  onPress: (key: string) => void;
}

function formatRelativeTime(postTime: number): string {
  const diffMs = Date.now() - postTime;
  const diffSec = Math.floor(diffMs / 1000);
  if (diffSec < 60)   return `${diffSec}s ago`;
  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60)   return `${diffMin}m ago`;
  const diffHr = Math.floor(diffMin / 60);
  if (diffHr < 24)    return `${diffHr}h ago`;
  const diffDay = Math.floor(diffHr / 24);
  return `${diffDay}d ago`;
}

function NotificationRow({ record, onPress }: Props) {
  const canOpen = record.hasPendingIntent;

  return (
    <Pressable
      style={[styles.row, !canOpen && styles.rowDimmed]}
      onPress={() => canOpen && onPress(record.key)}
      android_ripple={{ color: '#2a2a2a' }}
    >
      <View style={styles.iconWrap}>
        {record.appIconBase64 ? (
          <Image
            style={styles.icon}
            source={{ uri: `data:image/png;base64,${record.appIconBase64}` }}
            resizeMode="contain"
          />
        ) : (
          <View style={[styles.icon, styles.iconPlaceholder]} />
        )}
      </View>

      <View style={styles.body}>
        <View style={styles.headerRow}>
          <Text style={styles.appName} numberOfLines={1}>{record.appName}</Text>
          <Text style={styles.time}>{formatRelativeTime(record.postTime)}</Text>
        </View>
        {record.title ? (
          <Text style={styles.title} numberOfLines={1}>{record.title}</Text>
        ) : null}
        {record.text ? (
          <Text style={styles.text} numberOfLines={2}>{record.text}</Text>
        ) : null}
      </View>

      {canOpen && <View style={styles.intentDot} />}
    </Pressable>
  );
}

export default memo(NotificationRow, (prev, next) =>
  prev.record.key === next.record.key &&
  prev.record.hasPendingIntent === next.record.hasPendingIntent
);

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderBottomColor: '#2a2a2a',
    backgroundColor: '#111',
  },
  rowDimmed: {
    opacity: 0.5,
  },
  iconWrap: {
    width: 40,
    alignItems: 'center',
    marginRight: 12,
    paddingTop: 2,
  },
  icon: {
    width: 32,
    height: 32,
    borderRadius: 6,
  },
  iconPlaceholder: {
    backgroundColor: '#333',
  },
  body: {
    flex: 1,
  },
  headerRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 2,
  },
  appName: {
    fontSize: 11,
    color: '#888',
    textTransform: 'uppercase',
    letterSpacing: 0.5,
    flex: 1,
    marginRight: 8,
  },
  time: {
    fontSize: 11,
    color: '#555',
  },
  title: {
    fontSize: 14,
    fontWeight: '600',
    color: '#eee',
    marginBottom: 2,
  },
  text: {
    fontSize: 13,
    color: '#aaa',
    lineHeight: 18,
  },
  intentDot: {
    width: 6,
    height: 6,
    borderRadius: 3,
    backgroundColor: '#4af',
    marginTop: 6,
    marginLeft: 8,
  },
});
