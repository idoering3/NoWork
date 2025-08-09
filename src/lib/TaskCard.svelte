<script lang='ts'>
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";
    import { scale } from "svelte/transition";
    import { quartInOut } from "svelte/easing";

    interface Props {
        task: Task
        onComplete: (id: number) => void;
    }

    let { task = $bindable(), onComplete = $bindable()}: Props = $props();

    function complete() {
        onComplete(task.id);
    }

    const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;
    const now: Date = new Date();
</script>

<div class="task-card" transition:scale={{ duration: 150, easing: quartInOut, start: 0.75, opacity: 0 }}>
    <Button onclick={complete} Icon={Check} flavor="outline" class="square small"/>
    <div class="stacked">
        <h7>{task.name}</h7>
        <p class="date" 
            class:due-today={
                dueDate?.toLocaleDateString() === now.toLocaleDateString()
                }
        >
            {dueDate?.toLocaleDateString() ?? ''}
        </p>
    </div>
    <div class="tags">
        {#if task.tags}
            {#each task.tags as tag}
                <Badge flavor="outline">{tag}</Badge>
            {/each}
        {/if}
    </div>
</div>

<style>
    .task-card {
        gap: 1rem;
        display: flex;
        align-items: center;
        width: fit-content;
        padding: 0rem 0.5rem;
        height: 3rem;
        min-height: 0;
    }
    .date {
        font-size: 0.85rem;
    }

    .date.due-today {
        color: #86231c;
    }


    .stacked {
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .tags {
        display: flex;
        min-height: 0;
        gap: 0.5rem;
    }
</style>