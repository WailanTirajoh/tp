/**
 * Type definitions for Reactive Rust Storage
 */

import type { Ref } from "vue";

/**
 * Document stored in Rust reactive storage
 */
export interface Document {
  key: string;
  collection: string | null;
  value: any;
  version: number;
  created_at: string;
  updated_at: string;
}

/**
 * Event emitted when storage changes
 */
export interface StorageChangeEvent {
  key: string;
  collection: string | null;
  value: any;
  version: number;
  timestamp: string;
  origin: string;
  change_type: "set" | "delete";
}

/**
 * Options for storage initialization
 */
export interface StorageOptions {
  /**
   * Collection name for grouping documents
   */
  collection?: string;

  /**
   * Debounce delay for writes in milliseconds
   * @default 300
   */
  debounce?: number;

  /**
   * Use shallow reactivity (better performance for large objects)
   * @default false
   */
  shallow?: boolean;

  /**
   * Disable automatic event listening
   * @default false
   */
  manual?: boolean;

  /**
   * Custom error handler
   */
  onError?: (error: Error) => void;
}

/**
 * Request to set a document in Rust storage
 */
export interface SetDocumentRequest {
  key: string;
  value: any;
  collection: string | null;
  expected_version: number | null;
}

/**
 * Request to delete a document from Rust storage
 */
export interface DeleteDocumentRequest {
  key: string;
  expected_version: number | null;
}

/**
 * Storage registry for shared reactive refs
 */
export interface StorageRegistry {
  refs: Map<string, Ref<any>>;
  versions: Map<string, number>;
  listeners: Map<string, () => void>;
}

/**
 * Return type for useDocumentStorage composable
 */
export interface UseDocumentStorageReturn<T> {
  /**
   * Reactive ref to the storage value
   */
  data: Ref<T>;

  /**
   * Current version number
   */
  version: Ref<number>;

  /**
   * Whether storage is syncing
   */
  syncing: Ref<boolean>;

  /**
   * Last error if any
   */
  error: Ref<Error | null>;

  /**
   * Force sync to Rust immediately
   */
  sync: () => Promise<void>;

  /**
   * Remove from storage
   */
  remove: () => Promise<void>;
}
