<script lang='ts'>
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";

    interface Props {
        task: Task
    }

    let { task = $bindable() }: Props = $props();
</script>

<div class="task-card">
    <Button Icon={Check} class="square"/>
    <div class="stacked">
        <p>{task.name}</p>
        <p class="date">{task.dueDate ?? ''}</p>
    </div>
    {#if task.tags}
        <Badge flavor="outline">{task.tags}</Badge>
    {/if}
</div>

<style>
    .task-card {
        gap: 1rem;
        display: flex;
        align-items: center;
        width: auto;
        padding: 0 0.5rem;
        height: 4rem;
        border-radius: 7px;
        border: 1px solid var(--border-color);
    }
    .date {
        font-size: 0.85rem;
    }

    .stacked {
        display: flex;
        flex-direction: column;
    }
</style>