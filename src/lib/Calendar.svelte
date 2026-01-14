<script lang="ts">
    import { fly } from "svelte/transition";
    import { startClock } from "./stores.svelte";
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
        "Su",
        "Mo",
        "Tu",
        "We",
        "Th",
        "Fr",
        "Sa"
    ];


    let days = $state(getWeekDays(new Date()));
    let currentDate = $state(new Date());

    startClock(date => currentDate = date);

    function dayKey(d: Date): string {
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, "0");
        const day = String(d.getDate()).padStart(2, "0");
        return `${y}-${m}-${day}`;
    }

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
            getWeekDays(new Date(start.getTime() + 7 * 24 * 60 * 60 * 1000)),
            getWeekDays(new Date(start.getTime() + 14 * 24 * 60 * 60 * 1000)),
        ];
    });

</script>

{#snippet dueDot(task: Task)}
    <span class="dot" style="background-color: {colorByDate(new Date(task.dueDate!))};">
    </span>
{/snippet}

<div class="container" transition:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut }}>
    <h4 style="padding: 1rem;">
        Calendar
    </h4>
    {#each weeks as week, i}
        <div class="week">
            {#each week as day, j}
                <div 
                    class="day" class:current={dayKey(day) === dayKey(currentDate)}
                    transition:fly|global={{ y: 7, delay: 600 + (j * (i + 1) * 50), duration: 1500, easing: quartOut }}
                >
                    {#if i == 0}
                        <p>{dayNames[day.getDay()]}</p>
                    {/if}
                    <p class="faded">{day.getDate()}</p>

                    {#if tasksByDay[dayKey(day)]} <!-- you now have the tasks for this day --> 
                        <div style="">
                            <div style="position: absolute; width: 80%; left: 10%; bottom: 0.25rem;"> 
                                <div style="display: flex; align-items: center; justify-content: center; flex-wrap: wrap; overflow:"> 
                                    {#each tasksByDay[dayKey(day)] as task} 
                                        {@render dueDot(task)} 
                                    {/each} 
                                </div> 
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
        margin: 0.5rem;
        display: flex;
        flex-direction: column;
        background-color: var(--primary-light);
        border-radius: 15px;
        border: 1px solid var(--border-color);
    }

    .week {
        display: flex;
        justify-content: space-evenly;
        align-items: center;
    }

    .day {
        position: relative;
        display: flex;
        padding-bottom: 0.5rem;
        width: 2.5rem;
        height: 3.25rem;
        flex-direction: column;
        align-items: baseline;
        gap: 0rem;
        align-items: center;
        justify-content: center;
    }

    .current {
        background-color: var(--secondary-color);
        border-radius: 15px;
        border: 1px solid var(--border-color);
    }

    .faded {
        color: rgb(112, 112, 112);
    }
</style>