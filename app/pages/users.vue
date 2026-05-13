<template>
  <div class="container mx-auto p-6 max-w-6xl">
    <div class="mb-6 flex items-center justify-between">
      <h1 class="text-3xl font-bold">Users Management</h1>
      <button
        @click="openCreateModal"
        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
      >
        Add New User
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mb-4 p-4 bg-red-100 text-red-700 rounded-lg">
      {{ error }}
    </div>

    <!-- Success Message -->
    <div
      v-if="successMessage"
      class="mb-4 p-4 bg-green-100 text-green-700 rounded-lg"
    >
      {{ successMessage }}
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-8">
      <div
        class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"
      ></div>
      <p class="mt-4 text-gray-600">Loading users...</p>
    </div>

    <!-- Users Table -->
    <div v-else class="bg-white rounded-lg shadow overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              ID
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              Name
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              Email
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              Age
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              Created At
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              Actions
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-if="users.length === 0">
            <td colspan="6" class="px-6 py-8 text-center text-gray-500">
              No users found. Click "Add New User" to create one.
            </td>
          </tr>
          <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ user.id }}
            </td>
            <td
              class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
            >
              {{ user.name }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ user.email }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ user.age || "-" }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ formatDate(user.created_at) }}
            </td>
            <td
              class="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2"
            >
              <button
                @click="openEditModal(user)"
                class="text-blue-600 hover:text-blue-900"
              >
                Edit
              </button>
              <button
                @click="handleDelete(user.id!)"
                class="text-red-600 hover:text-red-900"
              >
                Delete
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Modal -->
    <div
      v-if="showModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      @click.self="closeModal"
    >
      <div class="bg-white rounded-lg p-6 w-full max-w-md">
        <h2 class="text-2xl font-bold mb-4">
          {{ editingUser ? "Edit User" : "Create New User" }}
        </h2>

        <form @submit.prevent="handleSubmit" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Name *
            </label>
            <input
              v-model="form.name"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter name"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Email *
            </label>
            <input
              v-model="form.email"
              type="email"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter email"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Age
            </label>
            <input
              v-model.number="form.age"
              type="number"
              min="1"
              max="150"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter age (optional)"
            />
          </div>

          <div class="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              @click="closeModal"
              class="px-4 py-2 text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition"
            >
              Cancel
            </button>
            <button
              type="submit"
              :disabled="submitting"
              class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ submitting ? "Saving..." : editingUser ? "Update" : "Create" }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { User } from "~/composables/useUsersApi";

const { getUsers, createUser, updateUser, deleteUser } = useUsersApi();

const users = ref<User[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const successMessage = ref<string | null>(null);

const showModal = ref(false);
const editingUser = ref<User | null>(null);
const submitting = ref(false);

const form = ref({
  name: "",
  email: "",
  age: undefined as number | undefined,
});

// Load users on mount
const loadUsers = async () => {
  try {
    loading.value = true;
    error.value = null;
    users.value = await getUsers();
  } catch (e) {
    error.value = "Failed to load users";
    console.error(e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadUsers();
});

// Modal functions
const openCreateModal = () => {
  editingUser.value = null;
  form.value = { name: "", email: "", age: undefined };
  showModal.value = true;
};

const openEditModal = (user: User) => {
  editingUser.value = user;
  form.value = {
    name: user.name,
    email: user.email,
    age: user.age,
  };
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
  editingUser.value = null;
  form.value = { name: "", email: "", age: undefined };
};

const handleSubmit = async () => {
  try {
    submitting.value = true;
    error.value = null;

    if (editingUser.value) {
      // Update
      await updateUser(editingUser.value.id!, {
        name: form.value.name,
        email: form.value.email,
        age: form.value.age,
      });
      successMessage.value = "User updated successfully!";
    } else {
      // Create
      await createUser({
        name: form.value.name,
        email: form.value.email,
        age: form.value.age,
      });
      successMessage.value = "User created successfully!";
    }

    closeModal();
    await loadUsers();

    // Clear success message after 3 seconds
    setTimeout(() => {
      successMessage.value = null;
    }, 3000);
  } catch (e: any) {
    error.value = e.message || "Failed to save user";
  } finally {
    submitting.value = false;
  }
};

const handleDelete = async (id: number) => {
  if (!confirm("Are you sure you want to delete this user?")) {
    return;
  }

  try {
    error.value = null;
    await deleteUser(id);
    successMessage.value = "User deleted successfully!";
    await loadUsers();

    // Clear success message after 3 seconds
    setTimeout(() => {
      successMessage.value = null;
    }, 3000);
  } catch (e: any) {
    error.value = e.message || "Failed to delete user";
  }
};

const formatDate = (dateStr?: string) => {
  if (!dateStr) return "-";
  try {
    return new Date(dateStr).toLocaleString();
  } catch {
    return dateStr;
  }
};
</script>
