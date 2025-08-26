<script lang='ts'>
    import Card from "$lib/Card.svelte";
    import { username } from "$lib/stores.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from '@tauri-apps/plugin-store';
    import { onMount } from "svelte";
    import { sineInOut } from "svelte/easing";
    import { fly } from "svelte/transition";
    import type { Task } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import Skeleton from "$lib/Skeleton.svelte";

    let message = $state();
    let tasks: Task[] | null = $state(null);
    let tasksDueToday = $state(false);
    let gifPath: string | null = $state(null);
    let error: string | null = $state(null);

    onMount (async () => {
        message = await invoke( 'greet' );


        const store = await load(".settings.json");
        const name = await store.get<{ value: string}>("username");
        if (name?.value) {
            username.name = name.value;
        } else {
            username.name = "User";
        }

        await refreshTasks();
        await fetchGif();
    });

    async function completeTask (taskId: number) {
        await invoke('complete_task', { taskId: taskId });
        getIncompleteTasks();
    }

    async function deleteTask (taskId: number) {
        await invoke("delete_task", {taskId: taskId});
        getIncompleteTasks();
    }

    async function refreshTasks() {
        await getTasksDueToday();
        tasksDueToday = true;
        if (tasks?.length === 0) {
            getIncompleteTasks();
            tasksDueToday = false;
        }
    }

    async function getTasksDueToday() {
        tasks = await invoke("get_tasks_due_today");
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
    }

    let oldUrl: string | null = $state(null);

    async function fetchGif() {
        try {
            // Revoke previous blob URL
            if (oldUrl) {
            URL.revokeObjectURL(oldUrl);
            oldUrl = null;
            }

            const bytesLike = await invoke<Uint8Array>('random_gif');

            // Ensure proper ArrayBuffer backing
            const bytes = new Uint8Array(bytesLike);

            const blob = new Blob([bytes], { type: 'image/gif' });
            oldUrl = URL.createObjectURL(blob);

            gifPath = oldUrl;
            console.log(gifPath);
            error = null;
        } catch (e) {
            error = String(e);
            gifPath = null;
        }
    }

    let chief_messages = [
        "Chief needs you to finish this fight!",
        "Time to give the Covenant back their bomb.",
        "Sir, finishing this fight.",
        "You have a job to do.",
        "There won't be a homework if we don't stop the banished.",
        "Don\'t make yourself a study plan… if you know you won\'t keep it.",
        "Reach didn't fall in a day",
        "For a brick… he flew pretty good!",
        "Wake up, spartan."
    ];

    let chief_message = $state(chief_messages[Math.floor(Math.random() * chief_messages.length)]);
</script>


<h1>Welcome, {username.name}!</h1>
<h5 transition:fly={{ y: 10, delay: 150, duration: 1000, easing: sineInOut }}>{message}</h5>
<div class="cardholder">
    <div class="taskview">
        <Card class="expanded">
            <div style="padding: 1rem;">
                {#if tasks}
                    <h6>There's stuff due today!</h6>
                    {#each tasks as task (task.id)}
                        <TaskCard {task} onComplete={completeTask} onDelete={deleteTask}/>
                    {/each}
                {:else}
                    <h6>Wow, you're so boring...</h6>
                {/if}
            </div>
        </Card>
    </div>
    <div>
        <Card class="expanded no-padding">
            <div style="display:flex; justify-content:center; position: relative; overflow:hidden; height: 25rem;">
                {#if error}
                    <p style="color: red;">Error: {error}</p>
                {:else if gifPath}
                    <img class="chief" src={gifPath} alt="Random Chief GIF" />
                    <p style="position:absolute; bottom: 1rem; color:white;">{chief_message}</p>
                {:else}
                    <Skeleton/>
                {/if}
            </div>
        </Card>
    </div>
</div>

<style>
    div {
        width: 100%;
    }

    img.chief {
        border-radius: 15px;
        object-fit: cover;
        width: 100%;
        height: 100%;
    }
    .taskview {
        height: 25rem;
        width: 100%;
    }
    .cardholder {
        margin-top: 2rem;
        display: flex;
        gap: 2rem;
    }
</style>