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
    <Button Icon={Check} class="square small"/>
    <div class="stacked">
        <h7>{task.name}</h7>
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
        padding: 0rem 0.5rem;
        height: 4rem;
    }
    .date {
        font-size: 0.85rem;
    }

    .stacked {
        display: flex;
        flex-direction: column;
    }
</style>