<script lang='ts'>
    import Card from "$lib/Card.svelte";
    import { selectedDateFormat, startClock, username } from "$lib/stores.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from '@tauri-apps/plugin-store';
    import { onMount } from "svelte";
    import { quartOut } from "svelte/easing";
    import { fly } from "svelte/transition";
    import type { Task } from "$lib/types/task";
    import { getSimpleTimeOfDay } from "$lib/misc/timeofday";
    import { hasDueDate } from "$lib/types/taskStore.svelte";
    import { getDayOfWeekAndTextStandardDateShort, dateFormats, type DateFormatName } from "$lib/misc/datePrints";
    import CalendarWidget from "$lib/widgets/CalendarWidget.svelte";
    import TimeWidget from "$lib/widgets/TimeWidget.svelte";
    import type { CalendarEvent } from "$lib/cal/calendar";

    type WidgetType = "calendar" | "time";

    // let's store the grid data as data
    let items: {
        id: string;
        component: WidgetType;
        x: number;
        y: number;
        w: number;
        h: number;
    }[] = [
        { id: "calendar", component: "calendar", x: 0, y: 0, w: 6, h: 4 },
        { id: "time", component: "time", x: 6, y: 0, w: 2, h: 1 }
    ];

    // registry of components
    const registry: Record<WidgetType, any> = {
        calendar: CalendarWidget,
        time: TimeWidget
    };

    let message = $state();
    let tasks: Task[] | null = $state(null);
    let sortedTasks: Task[] | undefined = $state(undefined);
    let currentDate: Date = $state(new Date());
    let dateFormatFunction = $state<(date: Date) => string>(
        getDayOfWeekAndTextStandardDateShort
    );

    type Location = {
        city: string;
        region: string;
        country: string;
    }

    let location: Location | undefined = $state();
    let timeOfDay = $state();

    onMount (async () => {
        message = await invoke( 'greet' );
        
        const store = await load(".settings.json");
        const name = await store.get<{ value: string}>("username");
        if (name?.value) {
            username.name = name.value;
        } else {
            username.name = "User";
        }

        const dateFormat = await store.get<{ value: DateFormatName }>("dateFormat");
        if (dateFormat?.value) {
            dateFormatFunction = dateFormats[dateFormat.value];
        } else {
            const store = await load(".settings.json");
            await store.set("dateFormat", { value: "dayOfWeekAndMonth" as DateFormatName });
            await store.save();
        }

        location = await getLocation();
        await refreshTasks();

        timeOfDay = await getSimpleTimeOfDay(new Date());
    });

    onMount(() => {
        return startClock(date => currentDate = date);
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
        sortedTasks = tasks?.slice().sort((a, b) => {
            if (!a.dueDate && !b.dueDate) return 0;
            if (!a.dueDate) return 1;
            if (!b.dueDate) return -1;

            return new Date(a.dueDate).getTime() - new Date(b.dueDate).getTime();
        });
    }

    let tasksWithDueDates = $derived(
        (tasks ?? []).filter(hasDueDate)
    );



    export async function getLocation(): Promise<Location> {
        const res = await fetch("https://ipapi.co/json/");
        const data = await res.json();

        console.log(data);

        return {
            city: data.city,
            region: data.region,
            country: data.country
        };
    }
</script>



<div class="container">
    <!-- username hello -->
    <div style="display: flex; justify-content: space-between">
        <!-- left side -->
        <div>
            <h5 in:fly={{ y: 30, delay: 50, duration: 1500, easing: quartOut}}
                style="color: var(--hover-primary-dark);"
            >
                {dateFormatFunction(currentDate)}
            </h5>
            <h1 in:fly={{ y: 30, delay: 200, duration: 1500, easing: quartOut}}>Hello, {username.name}.</h1>
            <h6 in:fly={{ y: 10, delay: 1200, duration: 2500, easing: quartOut}} style="font-style: italic;">{message}</h6>    

        </div>
        <!-- right side -->
        <div style="display: flex; justify-content: flex-end; flex-direction: column; align-items: end;">
            <h1 
                in:fly={{ y: 15, delay: 600, duration: 1500, easing: quartOut}}    
            >
                {currentDate.getHours().toString().padStart(2, "0")}:{currentDate.getMinutes().toString().padStart(2, "0")} 
            </h1>
            <p style="color: var(--hover-primary-dark);">
                {#if location}
                    {location.city}, {location.region}
                {:else}
                    Getting location...
                {/if}
            </p>
        </div>
    </div>

    <hr in:fly={{ y: 10, delay: 1600, duration: 2500, easing: quartOut}} style="margin-top: 3rem; margin-bottom: 3rem; border-color: var(--border-color); border-width: 0.5px;"/>
    <div class="grid">
        {#each items as item (item.id)}
            {@const Component = registry[item.component]}

            <div
                class="widget"
                style="
                    grid-column: {item.x + 1} / span {item.w};
                    grid-row: {item.y + 1} / span {item.h};
                "
            >
                {#if Component}
                    <Component
                        {sortedTasks}
                        {tasksWithDueDates}
                        {currentDate}
                        {completeTask}
                        {deleteTask}
                    />
                {:else}
                    <Card>
                        <p>Component not found: {item.component}</p>
                    </Card>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .container {
        padding: 3rem;
    }
    .grid {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        grid-auto-rows: 130px;
        gap: 2rem;
        margin-right: 3rem;
    }

    .widget {
        width: 100%;
        height: 100%;
    }
</style>