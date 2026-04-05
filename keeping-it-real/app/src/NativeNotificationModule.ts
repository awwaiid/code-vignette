import { NativeModules, NativeEventEmitter, Platform } from 'react-native';

const { NotificationModule } = NativeModules;

if (!NotificationModule) {
  throw new Error(
    'NotificationModule native module not found. ' +
    'This app requires Android. Ensure the module is registered in MainApplication.kt ' +
    'and you are running a real build (not Expo Go).'
  );
}

export interface NotificationRecord {
  key: string;
  packageName: string;
  appName: string;
  title: string | null;
  text: string | null;
  postTime: number;         // epoch ms
  appIconBase64: string | null;
  hasPendingIntent: boolean;
}

/** Load all stored notifications from the local DB, most-recent first. */
export function getNotifications(): Promise<NotificationRecord[]> {
  return NotificationModule.getNotifications();
}

/**
 * Fire the original app's PendingIntent for the given notification key.
 * Returns true if the intent fired, false if it was expired or not found.
 * After a process restart, all intents are gone — check hasPendingIntent first.
 */
export function firePendingIntent(key: string): Promise<boolean> {
  return NotificationModule.firePendingIntent(key);
}

/**
 * Returns true if the user has granted Notification Listener access to this app.
 * If false, direct the user to Settings > Special App Access > Notification Access.
 */
export function isPermissionGranted(): Promise<boolean> {
  return NotificationModule.isPermissionGranted();
}

/** Emitter for realtime events. Subscribe to "onNotificationPosted". */
export const notificationEmitter = new NativeEventEmitter(NotificationModule);
