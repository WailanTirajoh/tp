<script setup lang="ts">
import { ref } from "vue";
import {
  useRustStorage,
  getRustStorageKeys,
  clearRustStorage,
} from "@/composables/useRustStorage";

interface User {
  name: string;
  email: string;
  age: number;
}

interface Settings {
  darkMode: boolean;
  notifications: boolean;
  language: string;
}

// Reactive storage - automatically synced with Rust
const user = useRustStorage<User>("user", {
  name: "Guest",
  email: "guest@example.com",
  age: 18,
});

const user2 = useRustStorage<User>("user");

const settings = useRustStorage<Settings>("app_settings", {
  darkMode: false,
  notifications: true,
  language: "en",
});

const counter = useRustStorage<number>("shared_counter", 0);

// Debug utilities
const allKeys = ref<string[]>([]);

async function showAllKeys() {
  allKeys.value = await getRustStorageKeys();
}

async function clearStorage() {
  if (confirm("Are you sure you want to clear all storage?")) {
    await clearRustStorage();
    allKeys.value = [];
    alert("Storage cleared!");
  }
}
</script>

<template>
  <div class="p-8 max-w-4xl mx-auto">
    <h1 class="text-3xl font-bold mb-8">Reactive Storage Demo</h1>

    <!-- User Example -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-2xl font-semibold mb-4">User Profile (useRustStorage)</h2>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Name:</label>
        <input
          v-model="user.name"
          type="text"
          class="w-full px-4 py-2 border rounded-lg"
          placeholder="Enter name"
        />
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Email:</label>
        <input
          v-model="user.email"
          type="email"
          class="w-full px-4 py-2 border rounded-lg"
          placeholder="Enter email"
        />
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Age:</label>
        <input
          v-model.number="user.age"
          type="number"
          class="w-full px-4 py-2 border rounded-lg"
          placeholder="Enter age"
        />
      </div>

      <div class="bg-gray-100 p-4 rounded">
        <p class="text-sm font-mono">{{ JSON.stringify(user, null, 2) }}</p>
        <p class="text-sm font-mono">{{ JSON.stringify(user2, null, 2) }}</p>
      </div>

      <p class="text-sm text-gray-600 mt-4">
        ℹ️ Changes are automatically synced to Rust and persisted to SQLite
      </p>
    </div>

    <!-- Settings Example -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-2xl font-semibold mb-4">App Settings</h2>

      <div class="mb-4">
        <label class="flex items-center">
          <input v-model="settings.darkMode" type="checkbox" class="mr-2" />
          <span>Dark Mode</span>
        </label>
      </div>

      <div class="mb-4">
        <label class="flex items-center">
          <input
            v-model="settings.notifications"
            type="checkbox"
            class="mr-2"
          />
          <span>Enable Notifications</span>
        </label>
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Language:</label>
        <select
          v-model="settings.language"
          class="w-full px-4 py-2 border rounded-lg"
        >
          <option value="en">English</option>
          <option value="id">Indonesian</option>
          <option value="zh">中文</option>
        </select>
      </div>

      <div class="bg-gray-100 p-4 rounded">
        <p class="text-sm font-mono">{{ JSON.stringify(settings, null, 2) }}</p>
      </div>
    </div>

    <!-- Multi-Window Test -->
    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-2xl font-semibold mb-4">Multi-Window Sync Test</h2>

      <p class="text-sm text-gray-600 mb-4">
        Open another window of this app and change values - they will sync
        automatically!
      </p>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-2">Shared Counter:</label>
        <div class="flex items-center gap-4">
          <button
            @click="counter--"
            class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
          >
            -
          </button>
          <span class="text-3xl font-bold">{{ counter }}</span>
          <button
            @click="counter++"
            class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
          >
            +
          </button>
        </div>
      </div>

      <div class="flex gap-4">
        <button
          @click="showAllKeys"
          class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Show All Keys
        </button>

        <button
          v-if="!$config.public.production"
          @click="clearStorage"
          class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Clear Storage (Dev Only)
        </button>
      </div>

      <div v-if="allKeys.length > 0" class="mt-4 bg-gray-100 p-4 rounded">
        <p class="text-sm font-semibold mb-2">All Storage Keys:</p>
        <ul class="text-sm font-mono">
          <li v-for="key in allKeys" :key="key">• {{ key }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>
