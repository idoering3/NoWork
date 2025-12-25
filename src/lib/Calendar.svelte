<script lang="ts">
    import { fly } from "svelte/transition";
    import ColorLine from "./ColorLine.svelte";
    import { startClock } from "./stores.svelte";
    import { quartOut } from "svelte/easing";

    function getWeekDays(date: Date, weekStartsOn: 0 | 1 = 0): Date[] {
        const d = new Date(date);
        const day = d.getDay();
        const diff = (day - weekStartsOn + 8) % 7;
        d.setDate(d.getDate() - diff);

        return Array.from({ length: 7 }, (_, i) => {
            const dayDate = new Date(d);
            dayDate.setDate(d.getDate() + i);
            return dayDate;
        });
    }

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
</script>

<div class="container" transition:fly={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    {#key currentDate.getDay()}
        {#each days as day, i}
            <div class="day" transition:fly={{ y: 30, delay: 300 + (i + 1) * 300, duration: 1500, easing: quartOut}}>
                <div class="day" class:current={day.getDay() === currentDate.getDay()}>
                    <p>
                        {dayNames[day.getDay()]}
                    </p>
                    <p class="faded">
                        {day.getDate()}
                    </p>
                </div>
            </div>
        {/each}
    {/key}
</div>

<style>
    .container {
        display: flex;
        background-color: var(--primary-light);
        border-radius: 15px;
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        justify-content: space-evenly;
    }

    .day {
        display: flex;
        flex-direction: column;
        align-items: baseline;
        justify-content: start;
        gap: 0.25rem;
        padding: 0.5rem;
        align-items: center;
        justify-content: center;
    }

    .current {
        gap: 0.25rem;
        background-color: var(--secondary-color);
        border-radius: 15px;
        border: 1px solid var(--border-color);
    }

    .faded {
        color: rgb(112, 112, 112);
    }
</style>