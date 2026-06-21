<script lang='ts'>
    import Badge from "$lib/Badge.svelte";
    import { getIncompleteTasksDueThisWeek, getTasksDueThisWeek, getTasksDueToday, type Task } from "$lib/types/task";
    import { CalendarCheck2, CalendarClock, CircleCheck } from "@lucide/svelte";
    import { onMount } from "svelte";
    import { quartOut } from "svelte/easing";
    import { fly } from "svelte/transition";

    let tasksDueToday: Task[] = $state([]);
    let tasksDueThisWeek: Task[] = $state([]);
    let incompleteTasksDueThisWeek: Task[] = $state([]);

    onMount(async () => {
        tasksDueToday = await getTasksDueToday();
        tasksDueThisWeek = await getTasksDueThisWeek();
        incompleteTasksDueThisWeek = await getIncompleteTasksDueThisWeek();
    });

    let numTasksDueToday = $derived(tasksDueToday.length);
    let numTasksDueThisWeek = $derived(tasksDueThisWeek.length);
    let incompleteNumTasksDueThisWeek = $derived(incompleteTasksDueThisWeek.length);
</script>

<div 
    style="margin-top: 1rem; display: flex; gap: 1rem;"
    in:fly={{ y: 15, delay: 1400, duration: 2500, easing: quartOut}}
>
    <Badge flavor="defaultoutline">
        <CircleCheck size={16} class={numTasksDueToday == 0 ? "no-tasks-icon" : ""} />
        <div class={numTasksDueToday == 0 ? "no-tasks-text" : ""}>
            {numTasksDueThisWeek - incompleteNumTasksDueThisWeek} / {numTasksDueThisWeek} task{numTasksDueToday > 1 || numTasksDueToday < 1 ? "s" : ''} this week
        </div>
    </Badge>

    <Badge flavor="greenoutline">
        {#if numTasksDueToday == 0}
            <CalendarCheck2 size={16} class={numTasksDueToday == 0 ? "no-tasks-icon" : ""} />
        {:else}
            <CalendarClock size={16} class={numTasksDueToday == 0 ? "no-tasks-icon" : ""} />
        {/if}
        <div class={numTasksDueToday == 0 ? "no-tasks-text" : ""}>
            {numTasksDueToday} task{numTasksDueToday > 1 || numTasksDueToday < 1 ? "s" : ''} today
        </div>
    </Badge>
</div>

<style>

</style>