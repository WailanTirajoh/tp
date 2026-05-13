<template>
  <div class="container mx-auto p-6 max-w-4xl">
    <h1 class="text-4xl font-bold mb-6">Welcome to Noken POS</h1>

    <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
      <h2 class="text-2xl font-semibold mb-4">Getting Started</h2>
      <p class="text-gray-700 mb-4">
        This is a Tauri + Nuxt application with SQLite integration.
      </p>

      <div class="space-y-4">
        <div class="border-l-4 border-blue-500 pl-4">
          <h3 class="font-semibold text-lg mb-2">Features</h3>
          <ul class="list-disc list-inside text-gray-700 space-y-1">
            <li>SQLite database integration</li>
            <li>Full CRUD operations for Users</li>
            <li>Tauri backend commands</li>
            <li>Responsive Nuxt frontend</li>
          </ul>
        </div>

        <div class="border-l-4 border-green-500 pl-4">
          <h3 class="font-semibold text-lg mb-2">Available Pages</h3>
          <div class="space-y-2">
            <div>
              <NuxtLink
                to="/users"
                class="text-blue-600 hover:text-blue-800 font-medium"
              >
                → Users Management
              </NuxtLink>
              <p class="text-sm text-gray-600">
                View, create, edit, and delete users
              </p>
            </div>
          </div>
        </div>

        <div class="border-l-4 border-purple-500 pl-4">
          <h3 class="font-semibold text-lg mb-2">Tech Stack</h3>
          <div class="grid grid-cols-2 gap-4 text-gray-700">
            <div>
              <strong>Frontend:</strong>
              <ul class="list-disc list-inside ml-2 text-sm">
                <li>Nuxt 3</li>
                <li>Vue 3</li>
                <li>TypeScript</li>
                <li>Tailwind CSS</li>
              </ul>
            </div>
            <div>
              <strong>Backend:</strong>
              <ul class="list-disc list-inside ml-2 text-sm">
                <li>Tauri v2</li>
                <li>Rust</li>
                <li>SQLite (rusqlite)</li>
              </ul>
            </div>
          </div>
        </div>
      </div>

      <div class="mt-6 pt-6 border-t border-gray-200">
        <NuxtLink
          to="/users"
          class="inline-block px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition"
        >
          Go to Users Management →
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>
