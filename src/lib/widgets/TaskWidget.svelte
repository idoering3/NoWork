<script lang='ts'>
  import TaskCard from "$lib/TaskCard.svelte";
  import type { Task } from "$lib/types/task";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { quartOut } from "svelte/easing";
    import { fly } from "svelte/transition";

    let tasks: Task[] = $state([]);

    onMount(async () => {
        tasks = await invoke("get_incomplete_tasks");
        tasks = tasks?.slice(0,7);
    });

    async function completeTask (taskId: number) {
        await invoke('complete_task', { taskId: taskId });
        await getIncompleteTasks();
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
    }

    
</script>

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    {#if tasks.length > 0}
        {#each tasks as task}
            <TaskCard {task} allowsEdit={false} size={"small"} onComplete={completeTask}/>
        {/each}
    {:else}
        No tasks!
    {/if}

</div>

<style>
    .container {
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        border-radius: 15px;
        background-color: var(--primary-light);
        width: 100%;
        display: flex;
        flex-direction: column;
        height: 100%;
        justify-content: space-evenly;
    }
</style>