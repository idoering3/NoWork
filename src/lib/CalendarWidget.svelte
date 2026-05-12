<script lang="ts">
    import { fly } from "svelte/transition";
    import { dayKey, startClock } from "./stores.svelte";
    import { quartOut } from "svelte/easing";
    import type { Task } from "./types/task";

    interface Props {
        tasks?: Task[]
    }

    let { tasks }: Props = $props();

    function getWeekDays(date: Date, weekStartsOn: 0 | 1 = 0): Date[] {
        const d = new Date(date);
        const diff = (d.getDay() - weekStartsOn + 7) % 7;
        d.setDate(d.getDate() - diff);
        return Array.from({ length: 7 }, (_, i) => {
            const day = new Date(d);
            day.setDate(d.getDate() + i);
            return day;
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

    const dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    let currentDate = $state(new Date());
    startClock(date => currentDate = date);

    let days = $derived(getWeekDays(currentDate));

    function startOfDay(d: Date): Date {
        return new Date(d.getFullYear(), d.getMonth(), d.getDate());
    }

    function colorByDate(date: Date): string {
        const diffDays = Math.round(
            (startOfDay(date).getTime() - startOfDay(currentDate).getTime())
            / (1000 * 60 * 60 * 24)
        );
        if (diffDays < 0)  return "#f12618";
        if (diffDays === 0) return "#bd2c21";
        if (diffDays <= 3) return "#e08217";
        return "var(--primary-dark)";   // ← removed stray semicolon
    }
</script>

{#snippet dueDot(task: Task)}
    <span class="dot" style="background-color: {colorByDate(new Date(task.dueDate!))};"></span>
{/snippet}

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut }}>
    <h5 style="padding: 2rem; color: var(--hover-primary-dark);">This Week</h5>

    <div class="week">
        {#each days as day, j}
            <div
                class="day"
                class:current={dayKey(day) === dayKey(currentDate)}
                in:fly|global={{ y: 7, delay: 1000 + j * 50, duration: 1500, easing: quartOut }}
            >
                <div class="day-text">
                    <h5 class="faded">{dayNames[day.getDay()]}</h5>
                    <h3 
                        class:faded={day.getDay() === 0 || day.getDay() === 6}
                        class:current-day-color={dayKey(day) === dayKey(currentDate)}
                    >
                        {day.getDate()}
                    </h3>
                </div>

                <div class="hours">
                    {#each { length: 9 }}
                        <div class="hour"></div>
                    {/each}
                </div>

                {#if tasksByDay[dayKey(day)]}
                    <div class="dots">
                        {#each tasksByDay[dayKey(day)] as task}
                            {@render dueDot(task)}
                        {/each}
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .container {
        display: flex;    
        flex-direction: column;
        box-sizing: border-box;
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
        margin: 0.5rem 1rem 1rem;
        flex: 1;
        min-height: 0;
    }

    .day {
        display: flex;
        flex-direction: column;
        position: relative;
        min-height: 0;
    }

    .day-text {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding-top: 1rem;
        padding-bottom: 0.75rem;
    }

    .current-day-color {
        color: var(--highlight-color);
    }

    /* Hours container grows to fill available space */
    .hours {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    /* Each hour row grows equally, border-top gives the line */
    .hour {
        flex: 1;
        border-top: 1px solid var(--border-color);
        opacity: 0.5;
    }

    .hour:nth-child(even) {
        border-top: 1px solid var(--border-color);
        opacity: 0.2;
    }

    .dots {
        position: absolute;
        bottom: 0.25rem;
        left: 10%;
        width: 80%;
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
    }

    .dot {
        width: 0.3rem;
        height: 0.3rem;
        border-radius: 50%;
        margin: 0.05rem;
    }

    .current {
        background-color: var(--secondary-color);
        border-radius: 15px;
    }

    .faded {
        color: rgb(112, 112, 112);
    }
</style>