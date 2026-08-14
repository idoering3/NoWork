import { flavorMap } from "$lib/stores.svelte";
import { invoke } from "@tauri-apps/api/core";

export const taskPriorityOptions = [
    { label: "No priority", value: null },
    { label: "Low", value: "Low" },
    { label: "Medium", value: "Medium" },
    { label: "High", value: "High" },
];

export type TaskPriority = "Low" | "Medium" | "High" | null;

export interface Task {
  id: number;
  name: string;
  dueDate?: string | null;
  createdAt: string;
  completed: boolean;
  completedAt?: string | null;
  priority?: TaskPriority | null;
  tags?: Tag[] | null;
}
export interface Tag {
  id: number;
  name: string;
  color: TagColor
}

export interface NewTag {
  name: string;
  color: TagColor
}

export type TagColor = keyof typeof flavorMap;

export async function getTasksDueToday(): Promise<Task[]> {
  let taskContainer = await invoke<Task[]>('get_tasks_due_today');
  return taskContainer.filter(task => !task.completed);
}

export async function getTasksDueThisWeek(): Promise<Task[]> {
  let taskContainer = await invoke<Task[]>('get_tasks_due_this_week');
  return taskContainer;
}

export async function getIncompleteTasksDueThisWeek(): Promise<Task[]> {
  let taskContainer = await invoke<Task[]>('get_tasks_due_this_week');
  return taskContainer.filter(task => !task.completed);
}