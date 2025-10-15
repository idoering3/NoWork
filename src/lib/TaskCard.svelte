<script lang='ts'>
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";
    import { scale } from "svelte/transition";
    import { quartInOut } from "svelte/easing";
    import { Trash } from "@lucide/svelte";

    interface Props {
        task: Task,
        onComplete: (id: number) => void,
        onDelete: (id: number) => void
    }

    let { task = $bindable(), onComplete = $bindable(), onDelete = $bindable()}: Props = $props();

    function complete() {
        onComplete(task.id);
    }

    function deleted() {
        onDelete(task.id);
    }

    const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;
    const now: Date = new Date();
</script>

<div class="task-container" transition:scale={{ duration: 150, easing: quartInOut, start: 0.75, opacity: 0 }}>
    <div class='task-card'>
        <Button onclick={complete} Icon={Check} flavor="outline" class="square small"/>
        <div class="stacked">
            <p style="font-size: 1rem">{task.name}</p>
            <p class="date" 
                class:due-today={
                    dueDate && dueDate.toISOString().split("T")[0] <= new Date().toISOString().split("T")[0]
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
    <Button onclick={deleted} Icon={Trash} flavor="outline" class="square small"/>
</div>

<style>
    .task-container {
        gap: 1rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: fit-content;
        padding: 0rem 0.5rem;
        height: 3rem;
        min-height: 0;
    }

    .task-card {
        gap: 1rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: fit-content;
        padding: 0rem 0.5rem;
        height: 3rem;
        min-height: 0;
    }
    .date {
        font-size: 0.8rem;
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
        position: relative;
        display: flex;
        min-height: 0;
        gap: 0.5rem;
    }
</style>