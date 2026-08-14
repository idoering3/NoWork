import { flavorMap } from "$lib/stores.svelte";
import type { TaskPriority } from "$lib/types/task";

export function getPriorityColor(priority: TaskPriority) {
    switch (priority) {
        case 'low':
            return flavorMap['green'].bgcolor;
        case 'medium':
            return 'orange';
        case 'high':
            return '#D64540';
        default:
            return 'var(--primary-dark)';
    }
}