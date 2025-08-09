<script lang='ts'>
    import type { Task } from "$lib/types/task";
    import Check from "@lucide/svelte/icons/check";
    import Button from "./Button.svelte";
    import Badge from "./Badge.svelte";

    interface Props {
        task: Task
        onComplete: (id: number) => void;
    }

    let { task = $bindable(), onComplete = $bindable()}: Props = $props();

    function complete() {
        onComplete(task.id);
    }
</script>

<div class="task-card">
    <Button onclick={complete} Icon={Check} flavor="outline" class="square small"/>
    <div class="stacked">
        <h7>{task.name}</h7>
        <p class="date">{task.dueDate ?? ''}</p>
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

    .tags {
        display: flex;
        gap: 0.5rem;
    }
</style>