<script lang="ts">
    import { fly } from "svelte/transition";
    import { dayKey, startClock } from "./stores.svelte";
    import { quartOut } from "svelte/easing";
    import { onMount } from "svelte";
    import type { CalendarEvent } from "./cal/calendar";
    import { invoke } from "@tauri-apps/api/core";
    import EventCard from "./cal/EventCard.svelte";
    import { stringToDate } from "./cal/dateCalculations";
    import { load } from "@tauri-apps/plugin-store";

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

    const dayNames = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    let currentDate = $state(new Date());
    startClock(date => currentDate = date);

    let days = $derived(getWeekDays(currentDate));

    let events: CalendarEvent[] | undefined = $state();
    let dayElWidth = $state(0);
    let dayElHeight = $state(0);
    let numHours = $state(10);

    onMount(async () => {
        const store = await load(".settings.json");
        const calendarNumHours = await store.get<{ value: number} >("calendarNumHours");
        const calendarStartDate = await store.get<{ value: number}>("calendarStartTime");

        // assigned the saved settings to the variables.
        if (calendarNumHours?.value) {
            numHours = calendarNumHours.value;
        }
        if (calendarStartDate?.value) {
            startingHour = calendarStartDate.value;
        }
        events = await invoke("test_auth");

        console.log(events);
    });

    // starting at 7 am as a fallback
    let startingHour = $state(7);

</script>

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut }}>
    <h5 style="padding: 2rem 2rem 1rem 2rem; color: var(--hover-primary-dark);">This Week</h5>
    <div class="week">
        {#each days as day, j}
            <div
                bind:clientWidth={dayElWidth}
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
                <div class="timeline" bind:clientHeight={dayElHeight}>
                    {#each { length: numHours }, ij}
                        <div class="hour-line"
                            in:fly|global={{ y: 7, delay: 1000 + ij * 50, duration: 1500, easing: quartOut }}
                        ></div>
                    {/each}

                    <div class="events">
                        {#each (events ?? []) as event, k}
                            {#if stringToDate(event.start).toDateString() === days[j].toDateString()}
                                <div class="event"
                                    in:fly|global={{ y: 7, delay: 1000 + k * 50, duration: 1500, easing: quartOut }}
                                >
                                    <EventCard calEvent={event} width={dayElWidth} height={dayElHeight} numHours={numHours} startingHour={startingHour}/>
                                </div>
                            {/if}
                        {/each}
                    </div>

                </div>
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
    .timeline {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    /* Each hour row grows equally, border-top gives the line */
    .hour-line {
        flex: 1;
        border-top: 1px solid var(--border-color);
        opacity: 0.5;
    }

    .hour-line:nth-child(even) {
        border-top: 1px solid var(--border-color);
        opacity: 0.2;
    }

    .events {
        position:absolute;
    }

    .current {
        background-color: var(--secondary-color);
        border-radius: 15px;
    }

    .faded {
        color: rgb(112, 112, 112);
    }
</style>