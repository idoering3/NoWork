<script lang='ts'>
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import Button from "./Button.svelte";

    interface Props {
        task: Task
    }

    let { task = $bindable() }: Props = $props();
</script>

<div class="task-card">
    <Button Icon={Check} class="square"/>
    <p>{task.name}</p>
    <h8>{task.dueDate ?? ''}</h8>
    {#if task.tags}
        <p>{task.tags}</p>
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
</style>