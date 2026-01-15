import type { Task } from "./task";

export function hasDueDate(
    task: Task
): task is Task & { dueDate: string } {
    return task.dueDate != null;
}