import { getCompletedTaskCount, type Tag, type Task } from "./task";

export function hasDueDate(
    task: Task
): task is Task & { dueDate: string } {
    return task.dueDate != null;
}

export const taskState = $state({
    tasks: [] as Task[]
});

export const completedTaskCount = $state({
    completed: 0
});

getCompletedTaskCount().then(count => {
    completedTaskCount.completed = count;
});

export const tagState = $state({
    tags: [] as Tag[]
});