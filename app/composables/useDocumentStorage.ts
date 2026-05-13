/**
 * Reactive Document Storage Composable - useDocumentStorage
 *
 * Provides VueUse-style localStorage-like API backed by Rust reactive storage
 * with automatic SQLite persistence.
 *
 * Example:
 * ```ts
 * const user = useDocumentStorage<User>('user', { name: 'Guest' })
 *
 * // Automatically syncs to Rust and persists to SQLite
 * user.value.name = 'Wailan'
 * ```
 */

import { ref, type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import type {
  Document,
  StorageChangeEvent,
  StorageOptions,
  SetDocumentRequest,
  DeleteDocumentRequest,
} from "@/types/storage";
import { storageRegistry, WINDOW_ID } from "@/utils/storageRegistry";

/**
 * Reactive document storage composable
 *
 * @param key - Unique document key
 * @param initialValue - Default value if key doesn't exist
 * @param options - Storage options
 * @returns Reactive ref synced with Rust storage
 */
export function useDocumentStorage<T>(
  key: string,
  initialValue?: T,
  options: StorageOptions = {},
): Ref<T> {
  const {
    collection = undefined,
    debounce = 300,
    shallow = false,
    manual = false,
    onError,
  } = options;

  // Check if already exists in registry (shared instance)
  if (storageRegistry.hasRef(key)) {
    return storageRegistry.getRef<T>(key)!;
  }

  // Create new reactive ref
  const data = ref<T>(initialValue as T) as Ref<T>;

  // Error handler
  const handleError = (error: unknown, context: string) => {
    const err = error instanceof Error ? error : new Error(String(error));
    console.error(`[useDocumentStorage] ${context} "${key}":`, err);
    if (onError) onError(err);
  };

  // Load initial value from Rust storage
  const loadInitialValue = async () => {
    try {
      const doc = await invoke<Document | null>("storage_get", { key });

      if (doc) {
        data.value = doc.value;
        storageRegistry.setVersion(key, doc.version);
      } else if (initialValue !== undefined) {
        // Set initial value in storage
        await invoke<Document>("storage_set", {
          request: {
            key,
            value: initialValue,
            collection,
            expected_version: null,
          } as SetDocumentRequest,
          origin: WINDOW_ID,
        });
        storageRegistry.setVersion(key, 1);
      }
    } catch (error) {
      handleError(error, "Failed to load");
    }
  };

  // Setup event listener for changes
  const setupListener = async () => {
    if (manual) return;

    try {
      const unlisten = await listen<StorageChangeEvent>(
        "storage:changed",
        (event) => {
          const changeEvent = event.payload;

          // Only update if it's for this key
          if (changeEvent.key !== key) return;

          // Skip if this window originated the change
          if (changeEvent.origin === WINDOW_ID) return;

          // Check version to prevent loops
          const lastVersion = storageRegistry.getVersion(key);
          if (changeEvent.version <= lastVersion) return;

          // Update reactive ref
          if (changeEvent.change_type === "delete") {
            data.value = initialValue as T;
            storageRegistry.setVersion(key, 0);
          } else {
            data.value = changeEvent.value;
            storageRegistry.setVersion(key, changeEvent.version);
          }

          console.log(
            `[useDocumentStorage] Synced "${key}" from ${changeEvent.origin}`,
          );
        },
      );

      storageRegistry.setListener(key, unlisten);
    } catch (error) {
      handleError(error, "Failed to setup listener");
    }
  };

  // Sync to Rust with debounce
  const syncToRust = useDebounceFn(async (newValue: T) => {
    try {
      const currentVersion = storageRegistry.getVersion(key);

      const doc = await invoke<Document>("storage_set", {
        request: {
          key,
          value: newValue,
          collection,
          expected_version: currentVersion || null,
        } as SetDocumentRequest,
        origin: WINDOW_ID,
      });

      storageRegistry.setVersion(key, doc.version);
    } catch (error) {
      handleError(error, "Failed to sync");
    }
  }, debounce);

  // Watch for changes using VueUse's watchDebounced
  watchDebounced(
    data,
    (newValue: T) => {
      syncToRust(newValue);
    },
    {
      debounce,
      deep: !shallow,
    },
  );

  // Initialize
  loadInitialValue();
  setupListener();

  // Register in global registry
  storageRegistry.setRef(key, data);

  // Cleanup on unmount using VueUse's tryOnUnmounted
  tryOnUnmounted(() => {
    storageRegistry.removeListener(key);
  });

  return data;
}

/**
 * Remove a document from storage
 */
export async function removeDocument(key: string): Promise<void> {
  try {
    const currentVersion = storageRegistry.getVersion(key);

    await invoke("storage_delete", {
      request: {
        key,
        expected_version: currentVersion || null,
      } as DeleteDocumentRequest,
      origin: WINDOW_ID,
    });

    storageRegistry.removeRef(key);
    storageRegistry.removeVersion(key);
    storageRegistry.removeListener(key);
  } catch (error) {
    console.error(`[removeDocument] Failed to remove "${key}":`, error);
    throw error;
  }
}

/**
 * Get all document keys (for debugging)
 */
export async function getDocumentKeys(): Promise<string[]> {
  try {
    return await invoke<string[]>("storage_keys");
  } catch (error) {
    console.error("[getDocumentKeys] Failed:", error);
    return [];
  }
}

/**
 * Clear all documents (for testing)
 */
export async function clearDocumentStorage(): Promise<void> {
  if (import.meta.env.PROD) {
    throw new Error("clearDocumentStorage is not available in production");
  }

  try {
    await invoke("storage_clear");
    storageRegistry.clear();
  } catch (error) {
    console.error("[clearDocumentStorage] Failed:", error);
    throw error;
  }
}

/**
 * Get registry keys (local only)
 */
export function getRegistryKeys(): string[] {
  return storageRegistry.getKeys();
}

// Re-export utilities for convenience
export { storageRegistry, WINDOW_ID } from "@/utils/storageRegistry";
