<template>
  <div>
    <NuxtRouteAnnouncer />
    <nav class="bg-gray-800 text-white p-4">
      <div class="container mx-auto flex items-center justify-between">
        <h1 class="text-xl font-bold">Noken POS</h1>
        <div class="space-x-4">
          <NuxtLink to="/" class="hover:text-gray-300">Home</NuxtLink>
          <NuxtLink to="/users" class="hover:text-gray-300">Users</NuxtLink>
        </div>
      </div>
    </nav>
    <NuxtPage />
  </div>
</template>

<style>
body {
  margin: 0;
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue",
    Arial, sans-serif;
}
</style>
