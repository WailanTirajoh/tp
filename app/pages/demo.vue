<script setup lang="ts">
import {
  useDocumentStorage,
  removeDocument,
  getDocumentKeys,
  clearDocumentStorage,
  getRegistryKeys,
} from "@/composables/useDocumentStorage";

// Demo 1: Simple reactive user object
const user = useDocumentStorage(
  "demo:user",
  {
    name: "Guest",
    email: "guest@example.com",
    count: 0,
  },
  { collection: "demo" },
);

// Demo 2: Same key, different component (shows registry sharing)
const userCopy = useDocumentStorage(
  "demo:user",
  {
    name: "Default",
    email: "default@example.com",
    count: 0,
  },
  { collection: "demo" },
);

// Demo 3: Settings with manual control
const settings = useDocumentStorage(
  "demo:settings",
  {
    theme: "dark",
    notifications: true,
    language: "en",
  },
  {
    collection: "demo",
    manual: true, // Manual save mode
  },
);

// Demo 4: Counter with custom debounce
const counter = useDocumentStorage(
  "demo:counter",
  {
    value: 0,
    lastUpdated: new Date().toISOString(),
  },
  {
    collection: "demo",
    debounce: 1000, // 1 second debounce
  },
);

// Demo 5: Array data
const todos = useDocumentStorage(
  "demo:todos",
  [
    { id: 1, text: "Try useDocumentStorage", done: true },
    { id: 2, text: "Build something awesome", done: false },
  ],
  { collection: "demo" },
);

// Registry and keys state
const registryKeys = ref<string[]>([]);
const documentKeys = ref<string[]>([]);
const isLoading = ref(false);

// Actions
const incrementUser = () => {
  user.value.count++;
};

const incrementCounter = () => {
  counter.value.value++;
  counter.value.lastUpdated = new Date().toISOString();
};

const addTodo = () => {
  const newTodo = {
    id: Date.now(),
    text: `Todo #${todos.value.length + 1}`,
    done: false,
  };
  todos.value.push(newTodo);
};

const toggleTodo = (id: number) => {
  const todo = todos.value.find((t) => t.id === id);
  if (todo) {
    todo.done = !todo.done;
  }
};

const deleteTodo = (id: number) => {
  const index = todos.value.findIndex((t) => t.id === id);
  if (index !== -1) {
    todos.value.splice(index, 1);
  }
};

const deleteUser = async () => {
  await removeDocument("demo:user");
  // Reset to default
  user.value = {
    name: "Guest",
    email: "guest@example.com",
    count: 0,
  };
};

const refreshKeys = async () => {
  isLoading.value = true;
  try {
    registryKeys.value = getRegistryKeys();
    documentKeys.value = await getDocumentKeys();
  } finally {
    isLoading.value = false;
  }
};

const clearAll = async () => {
  if (confirm("Clear all demo documents from storage?")) {
    await clearDocumentStorage();
    // Reset to defaults
    user.value = { name: "Guest", email: "guest@example.com", count: 0 };
    settings.value = { theme: "dark", notifications: true, language: "en" };
    counter.value = { value: 0, lastUpdated: new Date().toISOString() };
    todos.value = [];
    await refreshKeys();
  }
};

// Load keys on mount
onMounted(() => {
  refreshKeys();
});
</script>

<template>
  <div class="min-h-screen bg-gray-50 py-8 px-4">
    <div class="max-w-7xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-2">
          useDocumentStorage Demo
        </h1>
        <p class="text-lg text-gray-600">
          Interactive demonstration of reactive document storage with Rust +
          SQLite persistence
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Demo 1: Simple Reactive Object -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h2 class="text-2xl font-semibold text-gray-900 mb-4">
            1. Simple Reactive Object
          </h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Name
              </label>
              <input
                v-model="user.name"
                type="text"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Email
              </label>
              <input
                v-model="user.email"
                type="email"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <p class="text-sm text-gray-700 mb-2">
                Count: <span class="font-mono font-bold">{{ user.count }}</span>
              </p>
              <button
                @click="incrementUser"
                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
              >
                Increment Count
              </button>
            </div>
            <div class="pt-4 border-t border-gray-200">
              <p class="text-xs text-gray-500 mb-2">
                💡 Changes auto-save to SQLite after 300ms debounce
              </p>
              <pre class="text-xs bg-gray-100 p-3 rounded overflow-x-auto">{{
                JSON.stringify(user, null, 2)
              }}</pre>
            </div>
          </div>
        </div>

        <!-- Demo 2: Registry Sharing (Same Key) -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h2 class="text-2xl font-semibold text-gray-900 mb-4">
            2. Registry Sharing (Same Key)
          </h2>
          <div class="space-y-4">
            <p class="text-sm text-gray-600">
              This uses the
              <span class="font-mono bg-gray-100 px-2 py-1 rounded"
                >demo:user</span
              >
              key. Changes here reflect in Demo 1 instantly!
            </p>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Name (Shared)
              </label>
              <input
                v-model="userCopy.name"
                type="text"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-green-500"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Email (Shared)
              </label>
              <input
                v-model="userCopy.email"
                type="email"
                class="w-full px-4 py-2 border border-gray-300 rounded-lg bg-white text-gray-900 focus:ring-2 focus:ring-green-500"
              />
            </div>
            <div>
              <p class="text-sm text-gray-700 mb-2">
                Count:
                <span class="font-mono font-bold">{{ userCopy.count }}</span>
              </p>
              <button
                @click="userCopy.count++"
                class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition"
              >
                Increment (Shared)
              </button>
            </div>
            <div class="pt-4 border-t border-gray-200">
              <p class="text-xs text-gray-500 mb-2">
                💡 Both refs point to same reactive object via storageRegistry
              </p>
              <pre class="text-xs bg-gray-100 p-3 rounded overflow-x-auto">{{
                JSON.stringify(userCopy, null, 2)
              }}</pre>
            </div>
          </div>
        </div>

        <!-- Demo 3: Custom Debounce -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h2 class="text-2xl font-semibold text-gray-900 mb-4">
            4. Custom Debounce (1s)
          </h2>
          <div class="space-y-4">
            <p class="text-sm text-gray-600">
              Changes are saved after 1 second of inactivity (custom debounce).
            </p>
            <div>
              <p class="text-6xl font-bold text-center text-orange-600 my-8">
                {{ counter.value }}
              </p>
              <button
                @click="incrementCounter"
                class="w-full px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition text-xl font-semibold"
              >
                + 1
              </button>
            </div>
            <div class="pt-4 border-t border-gray-200">
              <p class="text-xs text-gray-500">
                Last Updated:
                <span class="font-mono">{{
                  new Date(counter.lastUpdated).toLocaleTimeString()
                }}</span>
              </p>
              <p class="text-xs text-gray-500 mt-2">
                💡 Saves 1 second after you stop clicking
              </p>
            </div>
          </div>
        </div>

        <!-- Demo 5: Array Data -->
        <div class="bg-white rounded-lg shadow-md p-6 lg:col-span-2">
          <h2 class="text-2xl font-semibold text-gray-900 mb-4">
            5. Array Data (Todo List)
          </h2>
          <div class="space-y-4">
            <button
              @click="addTodo"
              class="w-full px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition"
            >
              + Add Todo
            </button>
            <ul class="space-y-2">
              <li
                v-for="todo in todos"
                :key="todo.id"
                class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg"
              >
                <input
                  :checked="todo.done"
                  type="checkbox"
                  @change="toggleTodo(todo.id)"
                  class="w-5 h-5 text-indigo-600 rounded focus:ring-indigo-500"
                />
                <span
                  class="flex-1 text-gray-900"
                  :class="{ 'line-through opacity-50': todo.done }"
                >
                  {{ todo.text }}
                </span>
                <button
                  @click="deleteTodo(todo.id)"
                  class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition text-sm"
                >
                  Delete
                </button>
              </li>
            </ul>
            <div class="pt-4 border-t border-gray-200">
              <p class="text-xs text-gray-500 mb-2">
                💡 Array mutations are reactive and auto-persisted
              </p>
              <pre class="text-xs bg-gray-100 p-3 rounded overflow-x-auto">{{
                JSON.stringify(todos, null, 2)
              }}</pre>
            </div>
          </div>
        </div>

        <!-- Storage Inspector -->
        <div class="bg-white rounded-lg shadow-md p-6 lg:col-span-2">
          <h2 class="text-2xl font-semibold text-gray-900 mb-4">
            🔍 Storage Inspector
          </h2>
          <div class="space-y-4">
            <div class="flex gap-2">
              <button
                @click="refreshKeys"
                :disabled="isLoading"
                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition disabled:opacity-50"
              >
                {{ isLoading ? "🔄 Loading..." : "🔄 Refresh Keys" }}
              </button>
              <button
                @click="deleteUser"
                class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition"
              >
                🗑️ Delete User Document
              </button>
              <button
                @click="clearAll"
                class="px-4 py-2 bg-red-700 text-white rounded-lg hover:bg-red-800 transition"
              >
                ⚠️ Clear All Demo Storage
              </button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                  Registry Keys (In Memory)
                </h3>
                <p class="text-xs text-gray-500 mb-2">
                  Active reactive refs in the current session
                </p>
                <ul
                  class="space-y-1 bg-gray-50 p-4 rounded-lg max-h-48 overflow-y-auto"
                >
                  <li
                    v-for="key in registryKeys"
                    :key="key"
                    class="text-sm font-mono text-gray-900"
                  >
                    • {{ key }}
                  </li>
                  <li
                    v-if="registryKeys.length === 0"
                    class="text-sm text-gray-500 italic"
                  >
                    No keys in registry
                  </li>
                </ul>
              </div>

              <div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                  Document Keys (In SQLite)
                </h3>
                <p class="text-xs text-gray-500 mb-2">
                  Persisted documents in the database
                </p>
                <ul
                  class="space-y-1 bg-gray-50 p-4 rounded-lg max-h-48 overflow-y-auto"
                >
                  <li
                    v-for="key in documentKeys"
                    :key="key"
                    class="text-sm font-mono text-gray-900"
                  >
                    • {{ key }}
                  </li>
                  <li
                    v-if="documentKeys.length === 0"
                    class="text-sm text-gray-500 italic"
                  >
                    No documents in storage
                  </li>
                </ul>
              </div>
            </div>

            <div class="pt-4 border-t border-gray-200">
              <h3 class="text-lg font-semibold text-gray-900 mb-3">
                💡 Key Features Demonstrated
              </h3>
              <ul class="space-y-2 text-sm text-gray-700">
                <li>
                  ✅ <strong>Auto-persistence:</strong> Changes automatically
                  saved to SQLite after debounce
                </li>
                <li>
                  ✅ <strong>Reactivity:</strong> Vue 3 reactive refs with full
                  TypeScript support
                </li>
                <li>
                  ✅ <strong>Registry sharing:</strong> Same key = same reactive
                  reference across components
                </li>
                <li>
                  ✅ <strong>Custom debounce:</strong> Configure delay per
                  document (default: 300ms)
                </li>
                <li>
                  ✅ <strong>Multi-window sync:</strong> Changes emit
                  <span class="font-mono">storage:changed</span> events
                </li>
                <li>
                  ✅ <strong>Collection support:</strong> Group documents with
                  <span class="font-mono">collection</span> option
                </li>
                <li>
                  ✅ <strong>Array mutations:</strong> Deep reactivity for
                  arrays and nested objects
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="mt-8 text-center text-sm text-gray-500">
        <p>🦀 Powered by Rust + Diesel + SQLite + Vue 3 + Nuxt 3</p>
        <p class="mt-2">
          Open multiple windows to see multi-window synchronization in action!
          🪟
        </p>
      </div>
    </div>
  </div>
</template>
