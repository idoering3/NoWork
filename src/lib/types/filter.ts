import { DateFilter } from "$lib/misc/dateFilter";
import type { Tag, Task, TaskPriority } from "./task";

export type TaskFilter = {
    tags: Tag[],
    priorities: TaskPriority[],
    date: DateFilter,
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

    const matchesDate = filterByDate(task, filter.date);

    return matchesTags && matchesPriority && matchesDate;
}

// need to filter by date, essentially need to make sure the date exists but also is greater than the filter.date
function filterByDate(task: Task, dateFilter: DateFilter): boolean {
    if (dateFilter === DateFilter.None) 
        return true;
    if (dateFilter === DateFilter.NotDue) 
        return !task.dueDate;

    if (!task.dueDate) 
        return false;

    const taskDueDate = new Date(task.dueDate);
    const now = new Date();

    switch (dateFilter) {
        case DateFilter.Today: {
            const startOfToday = new Date(now);
            startOfToday.setHours(0, 0, 0, 0);

            const startOfTomorrow = new Date(startOfToday);
            startOfTomorrow.setDate(startOfToday.getDate() + 1);

            return taskDueDate >= startOfToday && taskDueDate < startOfTomorrow;
        }
        case DateFilter.ThisWeek: {
            const startOfToday = new Date(now);
            startOfToday.setHours(0, 0, 0, 0);

            const dayOfWeek = startOfToday.getDay();
            const daysSinceMonday = (dayOfWeek + 6) % 7;

            const startOfWeek = new Date(startOfToday);
            startOfWeek.setDate(startOfToday.getDate() - daysSinceMonday);

            const startOfNextWeek = new Date(startOfWeek);
            startOfNextWeek.setDate(startOfWeek.getDate() + 7);

            return taskDueDate >= startOfWeek && taskDueDate < startOfNextWeek;
        }
    }
}