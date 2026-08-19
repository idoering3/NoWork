import type { Tag, Task, TaskPriority } from "./task";

export type TaskFilter = {
    tags: Tag[],
    priorities: TaskPriority[]
}

export function matchesFilter(task: Task, filter: TaskFilter): boolean {
    // first, check if there even if a filtered tag
    const matchesTags =
        filter.tags.length === 0 ||
        filter.tags.some(filterTag =>
            task.tags?.some(taskTag => taskTag.id === filterTag.id) ?? false
        );

    const matchesPriority = 
        filter.priorities.length === 0 ||
        filter.priorities.includes(task.priority);

    return matchesTags && matchesPriority;
}