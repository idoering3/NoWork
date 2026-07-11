<script lang='ts'>
    import TaskCard from "$lib/TaskCard.svelte";
    import type { Task } from "$lib/types/task";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { flip } from "svelte/animate";
    import { quartInOut, quartOut } from "svelte/easing";
    import { fly } from "svelte/transition";

    let tasks: Task[] = $state([]);
        let runCollapse = $state(true);


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
        tasks = tasks?.slice(0,7);
    }

    
</script>

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    {#if tasks.length > 0}
        {#each tasks as task, i (task.id)}
            <div animate:flip|global={{ duration: 300, easing: quartInOut }}>
                <div 
                    in:fly|global={{ duration: 1000, y: 15, easing: quartOut, delay: runCollapse ? 450 + 75 * (i + 1) : 0 }}
                    onintroend={() => runCollapse ? runCollapse = false : ""}
                >
                    <TaskCard {task} allowsEdit={false} size={"small"} onComplete={() => completeTask(task.id)}/>
                </div>
            </div>
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