<script lang="ts">
    import { fly } from "svelte/transition";
    import { dayKey, startClock } from "./stores.svelte";
    import { quartOut } from "svelte/easing";
    import type { Task } from "./types/task";

    interface Props {
        tasks?: Task[]
    }

    let { tasks } : Props = $props();

    let dueDates = $derived(tasks!.map(task => new Date(task.dueDate!)));

    function getWeekDays(date: Date, weekStartsOn: 0 | 1 = 0): Date[] {
        const d = new Date(date);
        const day = d.getDay();
        const diff = (day - weekStartsOn + 7) % 7;
        d.setDate(d.getDate() - diff);

        return Array.from({ length: 7 }, (_, i) => {
            const dayDate = new Date(d);
            dayDate.setDate(d.getDate() + i);
            return dayDate;
        });
    }

    let tasksByDay = $derived.by(() => {
        const record: Record<string, Task[]> = {};

        for (const task of tasks ?? []) {
            const key = dayKey(new Date(task.dueDate!));
            (record[key] ??= []).push(task);
        }

        return record;
    });


    const dayNames = [
        "Sun",
        "Mon",
        "Tue",
        "Wed",
        "Thu",
        "Fri",
        "Sat"
    ];


    let days = $state(getWeekDays(new Date()));
    let currentDate = $state(new Date());

    startClock(date => currentDate = date);

    function startOfDay(d: Date): Date {
        return new Date(d.getFullYear(), d.getMonth(), d.getDate());
    }

    function colorByDate(date: Date): string {
        const today = startOfDay(currentDate);
        const target = startOfDay(date);

        const diffMs = target.getTime() - today.getTime();
        const diffDays = Math.round(diffMs / (1000 * 60 * 60 * 24));

        if (diffDays < 0) return "#f12618";        // past
        if (diffDays === 0) return "#bd2c21";        // today
        if (diffDays <= 3) return "#e08217";          // within 3 days
        return "var(--primary-dark);";                           // chill, not urgent
    }

    let weeks = $derived.by(() => {
        const start = startOfDay(currentDate);

        return [
            getWeekDays(start),
            // getWeekDays(new Date(start.getTime() + 7 * 24 * 60 * 60 * 1000)),
            // getWeekDays(new Date(start.getTime() + 14 * 24 * 60 * 60 * 1000)),
        ];
    });

</script>

{#snippet dueDot(task: Task)}
    <span class="dot" style="background-color: {colorByDate(new Date(task.dueDate!))};">
    </span>
{/snippet}

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut }}>
    <h5 style="padding: 1rem; color: var(--hover-primary-dark);">
        This Week
    </h5>
    {#each weeks as week, i}
        <div class="week">
            {#each week as day, j}
            <!-- this is each day -->
                <div 
                    class="day" class:current={dayKey(day) === dayKey(currentDate)}
                    in:fly|global={{ y: 7, delay: 1000 + (j * (i + 1) * 50), duration: 1500, easing: quartOut }}
                >
                    <div class="day-text">
                        {#if i == 0}
                            <h5 class="faded" style="padding-top: 1rem;">{dayNames[day.getDay()]}</h5>
                        {/if}
                        <h3 class:faded={day.getDate() === 1 || day.getDate() === 7}>{day.getDate()}</h3>
                    </div>

                    <!-- <hr style="margin-top: 1rem; border-color: var(--border-color); border-width: 0.5px; width: 100%;"/> -->

                    <div>
                        {#each {length: 10}}
                            <div class="hour"></div>
                        {/each}
                    </div>

                    {#if tasksByDay[dayKey(day)]} <!-- you now have the tasks for this day --> 
                        <div style="position: absolute; width: 80%; left: 10%;"> 
                            <div style="display: flex; align-items: center; justify-content: center; flex-wrap: wrap; overflow:"> 
                                {#each tasksByDay[dayKey(day)] as task} 
                                    {@render dueDot(task)} 
                                {/each} 
                            </div> 
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/each}
</div>

<style>
    .dot {
        margin: 0.05rem;
        width: 0.3rem;
        height: 0.3rem;
        border-radius: 50%;
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .container {
        height: 100%;
        margin: 0.5rem;
        background-color: var(--primary-light);
        border-radius: 15px;
        border: 1px solid var(--border-color);
        padding-bottom: 1rem;
        overflow: hidden;
    }

    .week {
        display: grid;
        grid-template-columns: repeat(7, 1fr);
        gap: 0.75rem;
        margin: 0.5rem 1rem 1rem 1rem;
        height: 100%;
    }

    .day {
        display: grid;
        grid-template-rows: repeat(10, 1fr);
        gap: 0.25rem;
        gap: 0rem;
        height: 100%;
    }

    .day-text {
        display: flex;
        flex-direction: column;
        padding-bottom: 0.5rem;
        align-items: center;
    }

    .hour {
        border-top: 1px solid var(--border-color);
    }

    .current {
        background-color: var(--secondary-color);
        border-radius: 15px;
        /* border: 1px solid var(--border-color); */
    }

    .faded {
        color: rgb(112, 112, 112) !important;
    }
</style>