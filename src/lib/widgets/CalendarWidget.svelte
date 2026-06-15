<script lang="ts">
    import { fly } from "svelte/transition";
    import { dayKey, startClock } from "../stores.svelte";
    import { quartOut } from "svelte/easing";
    import type { Task } from "../types/task";
    import EventCard from "$lib/cal/EventCard.svelte";
    import { onMount } from "svelte";
    import { load } from "@tauri-apps/plugin-store";
    import { invoke } from "@tauri-apps/api/core";
    import type { CalendarEvent } from "$lib/cal/calendar";

    interface Props {
        showCurrentTime: boolean;
    }

    let { showCurrentTime = true }: Props = $props();

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

    let events: CalendarEvent[] | undefined = $state();

    let days = $derived(getWeekDays(currentDate));

    type CachedCalendarEvents = {
        eventsList: CalendarEvent[];
        lastUpdated: number;
    }

    onMount(async () => {
        const store = await load(".settings.json");
        
        // get the cached events
        let calEvents = await store.get<{ value: CachedCalendarEvents }>('calendarEvents');
        
        
        if (calEvents?.value) {
            let cachedEvents = calEvents.value;
            
            // check if the data is stale
            const age = Date.now() - cachedEvents.lastUpdated;
            // compare against 5 min
            if (age > 60000 * 5) {
                console.log("Date is stale");
                
                // we need to go to the api to get calendar events
                const emailName = await store.get<{ value: string }>("email");
                if (emailName?.value) {
                    let email = emailName.value;
                    events = await invoke("test_auth", {email: email});
                    // make sure to update cachedEvents lastmodified to the current time
                    if (events) {
                        cachedEvents = {
                            eventsList: events,
                            lastUpdated: Date.now()
                        }
                    }

                    await store.set('calendarEvents', { value: cachedEvents});
                }
            } else {
                // else the cached events are ok
                events = cachedEvents.eventsList;
                console.log(cachedEvents);
            }
        }
    });
    
    $effect(() => {
        $inspect(events);
    });
    const eventsByDay = $derived.by(() => {
        const grouped: Record<string, CalendarEvent[]> = {};

        if(events) {
            console.log(events);
            for (const event of events) {
                const key = dayKey(new Date(event.start));
    
                if (!grouped[key]) {
                    grouped[key] = [];
                }
    
                grouped[key].push(event);
            }
    
            return grouped;
        } 
        return undefined;
    });

    let startingHour = $state(7);
    let numHours = $state(12);

    let dayWidth = $state(0);
    let dayHeight = $state(0);

    let currentTimePixels = $derived(
        ((currentDate.getHours() + currentDate.getMinutes() / 60) - startingHour)
         / (numHours) * dayHeight
    );
</script>

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut }}>
    <h5 style="padding: 2rem; color: var(--hover-primary-dark);">This Week</h5>
    <div 
        class="week"
    >
        {#each days as day, j}
            <div
                class="day"
                bind:clientWidth={dayWidth}
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

                {#if dayKey(day) === dayKey(currentDate)}
                    <!-- add a red line for the current time -->
                    {#if showCurrentTime}
                        <div style="position: relative;">
                            <div class="current-time" style="top: {currentTimePixels}px">
                                
                            </div>
                        </div>
                    {/if}
                {/if}
                
                <div class="hours"
                    bind:clientHeight={dayHeight}
                >

                {#each { length: numHours }}
                    <div class="hour"></div>
                {/each}
                    <div style="position: absolute;">
                        {#if eventsByDay}
                            {#if eventsByDay[dayKey(day)]}
                                {#each eventsByDay[dayKey(day)] as event, i}
                                    <div                             
                                        in:fly|global={{ y: 15, delay: 1000 + 300 * i, duration: 1500, easing: quartOut}}    
                                    >
                                        <EventCard 
                                            calEvent={event} 
                                            {numHours} 
                                            {startingHour} 
                                            width={dayWidth} 
                                            height={dayHeight}
                                        />
                                    </div>
                                {/each}
                            {/if}
                        {/if}
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
        color: var(--highlight-color) !important;
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

    .current {
        background-color: var(--secondary-color);
        border-radius: 15px;
    }

    .faded {
        color: rgb(112, 112, 112);
    }

    .current-time {
        position: absolute;
        width: 100%;
        height:0.1rem;
        background-color: red;
        border-radius: 15px;
    }
</style>