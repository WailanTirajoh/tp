import { invoke } from "@tauri-apps/api/core";

export interface User {
  id: number;
  name: string;
  email: string;
  age?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateUserInput {
  name: string;
  email: string;
  age?: number;
}

export interface UpdateUserInput {
  name?: string;
  email?: string;
  age?: number;
}

export const useUsersApi = () => {
  const getUsers = async (): Promise<User[]> => {
    try {
      return await invoke<User[]>("get_users");
    } catch (error) {
      console.error("Failed to get users:", error);
      throw error;
    }
  };

  const getUser = async (id: number): Promise<User> => {
    try {
      return await invoke<User>("get_user", { id });
    } catch (error) {
      console.error(`Failed to get user ${id}:`, error);
      throw error;
    }
  };

  const createUser = async (input: CreateUserInput): Promise<User> => {
    try {
      return await invoke<User>("create_user", { input });
    } catch (error) {
      console.error("Failed to create user:", error);
      throw error;
    }
  };

  const updateUser = async (
    id: number,
    input: UpdateUserInput,
  ): Promise<User> => {
    try {
      return await invoke<User>("update_user", { id, input });
    } catch (error) {
      console.error(`Failed to update user ${id}:`, error);
      throw error;
    }
  };

  const deleteUser = async (id: number): Promise<void> => {
    try {
      await invoke<void>("delete_user", { id });
    } catch (error) {
      console.error(`Failed to delete user ${id}:`, error);
      throw error;
    }
  };

  return {
    getUsers,
    getUser,
    createUser,
    updateUser,
    deleteUser,
  };
};
