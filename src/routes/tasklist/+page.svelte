<script lang='ts'>
    import { invoke } from "@tauri-apps/api/core";
    import Card from "$lib/Card.svelte";
    import type { Task } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";

    let tasks: Task[] = [
        {
            id: 1,
            name: "Finish NoWork UI",
            dueDate: "2025-08-10T12:00:00Z",
            createdAt: "2025-08-06T10:00:00Z",
            completed: false,
            completedAt: null,
            tags: "work"
        },
        {
            id: 2,
            name: "Review async Rust",
            dueDate: null,
            createdAt: "2025-08-04T14:00:00Z",
            completed: true,
            completedAt: "2025-08-05T15:00:00Z",
            tags: "study"
        }
    ];

</script>

<h1>
    Task List
</h1>

<Card expanded>
    {#each tasks as task (task.id)}
        <TaskCard {task} />
    {/each}
</Card>