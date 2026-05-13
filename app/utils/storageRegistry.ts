/**
 * Storage registry for managing shared reactive refs
 */

import type { Ref } from "vue";
import type { StorageRegistry } from "@/types/storage";

class StorageRegistryManager {
  private refs = new Map<string, Ref<any>>();
  private versions = new Map<string, number>();
  private listeners = new Map<string, () => void>();

  /**
   * Get a ref from registry
   */
  getRef<T>(key: string): Ref<T> | undefined {
    return this.refs.get(key);
  }

  /**
   * Set a ref in registry
   */
  setRef<T>(key: string, ref: Ref<T>): void {
    this.refs.set(key, ref);
  }

  /**
   * Check if key exists in registry
   */
  hasRef(key: string): boolean {
    return this.refs.has(key);
  }

  /**
   * Remove a ref from registry
   */
  removeRef(key: string): void {
    this.refs.delete(key);
  }

  /**
   * Get version for a key
   */
  getVersion(key: string): number {
    return this.versions.get(key) || 0;
  }

  /**
   * Set version for a key
   */
  setVersion(key: string, version: number): void {
    this.versions.set(key, version);
  }

  /**
   * Remove version for a key
   */
  removeVersion(key: string): void {
    this.versions.delete(key);
  }

  /**
   * Get listener for a key
   */
  getListener(key: string): (() => void) | undefined {
    return this.listeners.get(key);
  }

  /**
   * Set listener for a key
   */
  setListener(key: string, unlisten: () => void): void {
    this.listeners.set(key, unlisten);
  }

  /**
   * Remove and call listener for a key
   */
  removeListener(key: string): void {
    const unlisten = this.listeners.get(key);
    if (unlisten) {
      unlisten();
      this.listeners.delete(key);
    }
  }

  /**
   * Clear all listeners
   */
  clearListeners(): void {
    for (const unlisten of this.listeners.values()) {
      unlisten();
    }
    this.listeners.clear();
  }

  /**
   * Clear entire registry
   */
  clear(): void {
    this.clearListeners();
    this.refs.clear();
    this.versions.clear();
  }

  /**
   * Get all keys
   */
  getKeys(): string[] {
    return Array.from(this.refs.keys());
  }
}

// Singleton instance
export const storageRegistry = new StorageRegistryManager();

// Window origin identifier
export const WINDOW_ID = `window-${Date.now()}-${Math.random().toString(36).slice(2)}`;
