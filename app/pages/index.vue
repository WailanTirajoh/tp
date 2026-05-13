<script setup lang="ts"></script>
<template>
  <div class="container mx-auto p-6 max-w-4xl">
    <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
      <h2 class="text-2xl font-semibold mb-4">Reactive Storage System</h2>
      <p class="text-gray-700 mb-4">
        This is a Tauri + Nuxt application with reactive SQLite-backed storage.
      </p>

      <div class="space-y-4">
        <div class="border-l-4 border-blue-500 pl-4">
          <h3 class="font-semibold text-lg mb-2">Architecture</h3>
          <ul class="list-disc list-inside text-gray-700 space-y-1">
            <li>Frontend: Vue 3 + Nuxt 3 with TypeScript</li>
            <li>Backend: Rust + Tauri (persistence transport only)</li>
            <li>Storage: Reactive document store with SQLite persistence</li>
            <li>Business Logic: Lives in the frontend</li>
          </ul>
        </div>

        <div class="border-l-4 border-green-500 pl-4">
          <h3 class="font-semibold text-lg mb-2">Features</h3>
          <div class="space-y-2 text-gray-700">
            <div>
              <strong>✨ Reactive Storage</strong>
              <p class="text-sm">
                VueUse-style API backed by Rust with automatic SQLite
                persistence
              </p>
            </div>
            <div>
              <strong>🔄 Multi-window Sync</strong>
              <p class="text-sm">
                Changes automatically sync across all app windows
              </p>
            </div>
            <div>
              <strong>⚡ In-Memory Performance</strong>
              <p class="text-sm">
                SQLite is just a persistence layer, not the source of truth
              </p>
            </div>
            <div>
              <strong>🎯 Frontend-First</strong>
              <p class="text-sm">
                All business logic lives in Vue composables and components
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
