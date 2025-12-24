<script lang="ts">
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import { Trash } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";
    import { scale } from "svelte/transition";
    import { quartInOut } from "svelte/easing";

    interface Props {
        task: Task;
        onComplete: (id: number) => void;
        onDelete: (id: number) => void;
    }

    let { task = $bindable(), onComplete = $bindable(), onDelete = $bindable() }: Props = $props();

    function complete() {
        onComplete(task.id);
    }

    function deleted() {
        onDelete(task.id);
    }

    const dueDate: Date | null = task.dueDate ? new Date(task.dueDate) : null;

    // Local date comparison helpers
    function isSameLocalDate(a: Date, b: Date): boolean {
        return (
            a.getFullYear() === b.getFullYear() &&
            a.getMonth() === b.getMonth() &&
            a.getDate() === b.getDate()
        );
    }

    function isPastDue(a: Date, b: Date): boolean {
        const today = new Date(b.getFullYear(), b.getMonth(), b.getDate());
        const check = new Date(a.getFullYear(), a.getMonth(), a.getDate());
        return check < today;
    }

    const now = new Date();
    const dueToday = dueDate && isSameLocalDate(dueDate, now);
    const overdue = dueDate && isPastDue(dueDate, now);
</script>

<div
    class="task-container"
    transition:scale={{ duration: 150, easing: quartInOut, start: 0.75, opacity: 0 }}
>
    <div class="task-card" class:overdue={overdue} class:due-today={dueToday}>
        <Button onclick={complete} Icon={Check} flavor="outline" class="square small" />
        <div class="stacked">
            <p style="font-size: 1rem">{task.name}</p>
            <p class="date">
                {dueDate ? dueDate.toLocaleDateString(undefined, { month: 'long', day: 'numeric' }) : ''}
            </p>
        </div>
        <div class="tags">
            {#if task.tags}
                {#each task.tags as tag}
                    <Badge flavor={tag.color}>{tag.name}</Badge>
                {/each}
            {/if}
        </div>
    </div>
    <Button onclick={deleted} Icon={Trash} flavor="outline" class="square small" />
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
        border-radius: 0.5rem;
        transition: background-color 0.3s ease, color 0.3s ease;
    }

    .task-card.due-today .date{
        color: #86231c;
    }

    .task-card.overdue .date {
        color: #86231c;
    }

    .date {
        font-size: 0.8rem;
    }

    .stacked {
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .tags {
        display: flex;
        gap: 0.5rem;
    }
</style>
