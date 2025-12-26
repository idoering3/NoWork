<script lang='ts'>
    import Card from "$lib/Card.svelte";
    import { startClock, username } from "$lib/stores.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from '@tauri-apps/plugin-store';
    import { onMount } from "svelte";
    import { quadInOut, quartOut, sineInOut } from "svelte/easing";
    import { fly } from "svelte/transition";
    import type { Task } from "$lib/types/task";
    import TaskCard from "$lib/TaskCard.svelte";
    import Calendar from "$lib/Calendar.svelte";
    import { getSimpleTimeOfDay } from "$lib/misc/timeofday";
    import TimeCard from "$lib/TimeCard.svelte";

    let message = $state();
    let tasks: Task[] | null = $state(null);
    let currentDate: Date = $state(new Date());

    onMount (async () => {
        startClock(date => currentDate = date);

        timeOfDay = await getSimpleTimeOfDay(new Date());

        message = await invoke( 'greet' );

        const store = await load(".settings.json");
        const name = await store.get<{ value: string}>("username");
        if (name?.value) {
            username.name = name.value;
        } else {
            username.name = "User";
        }

        await refreshTasks();
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
        getIncompleteTasks();
    }

    async function getIncompleteTasks() {
        tasks = await invoke('get_incomplete_tasks');
    }


    const dayNames = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursay",
        "Friday",
        "Saturday"
    ];

    let timeOfDay = $state("");
</script>

<div class="container">
    <h1 transition:fly={{ y: 30, delay: 150, duration: 1500, easing: quartOut}}>Hello, {username.name}.</h1>
    <h6 transition:fly={{ y: 10, delay: 1200, duration: 2500, easing: quartOut}}>{message}</h6>
    <div class="cardholder">
        <div class="taskview">
            {#each tasks?.slice(0, 3) as task, i}
                <div transition:fly={{ y: 30, delay: 300 + (i + 1) * 300, duration: 1500, easing: quartOut}} style="margin: 0.5rem;">
                    <Card class="expanded">
                        <TaskCard {task} onComplete={completeTask}/>
                    </Card>
                </div>
            {/each}
        </div>
        <div class="calendar">
            <Calendar></Calendar>
        </div>
        <div class="timecard">
            <TimeCard />
        </div>
    </div>
</div>

<style>
    .container {
        overflow: hidden;
        padding: 3rem;
        margin-right: 3rem;
    }
    .calendar {
        flex: 1;
    }
    .timecard {
        flex: 1;
    }
    .taskview {
        height: 13rem;
        display: flex;
        flex-direction: column;
        flex: 1;
    }
    .cardholder {
        display: flex;
        margin-top: 2rem;
        display: flex;
        gap: 2rem;
    }
</style>