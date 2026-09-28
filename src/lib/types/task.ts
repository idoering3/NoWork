import { flavorMap } from "$lib/stores.svelte";
import { invoke } from "@tauri-apps/api/core";
import { completedTaskCount, taskState } from "./taskStore.svelte";

export const taskPriorityOptions = [
    { label: "No priority", value: null },
    { label: "Low", value: "low" },
    { label: "Medium", value: "medium" },
    { label: "High", value: "high" },
];


export type TaskPriority = "low" | "medium" | "high" | null;

export interface Task {
  id: number;
  name: string;
  dueDate?: string | null;
  createdAt: string;
  completed: boolean;
  completedAt?: string | null;
  priority: TaskPriority;
  tags?: Tag[] | null;
}

export interface CreateTask {
  name: string;
  dueDate: Date | null;
  priority: TaskPriority;
  tags: Tag[];
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

export async function getCompletedTaskCount (): Promise<number> {
  return await invoke<number>('get_completed_task_count');
}


export async function getIncompleteTasks(): Promise<Task[]> {
  return await invoke('get_incomplete_tasks');
}

// actually modify the task in some sort of way

export async function submitTask (task: CreateTask) {
  if (task.name) {
      await invoke('add_database_task', {name: task.name, dueDate: task.dueDate, priority: task.priority, tags: task.tags});
      refreshTasks();
  }
}

export async function completeTask (taskId: number) {
  await invoke('complete_task', { taskId: taskId });
  refreshTasks();
  refreshCompletedTaskCount();
}

export async function deleteTask (taskId: number) {
  await invoke("delete_task", {taskId: taskId});
  refreshTasks();
}

// to refresh the task list must be done whenever modifying
// TODO better setup...
async function refreshTasks() {
  taskState.tasks = await getIncompleteTasks();
}

async function refreshCompletedTaskCount() {
  completedTaskCount.completed = await getCompletedTaskCount();
}

// tag functions
async function getAllTags(): Promise<Tag[]> {
    return await invoke<Tag[]>('get_all_tags'); 
}

