<script lang='ts'>
    import { onMount } from "svelte";
    import type { CalendarEvent } from "./calendar";
    import { stringToDate } from "./dateCalculations";

    
    interface Props {
        calEvent: CalendarEvent
        width: number
        height: number
        numHours: number
        startingHour: number
    }

    let {
        calEvent,
        width,
        height,
        numHours,
        startingHour
    }: Props = $props();

    // to calculate the actual height, we take the viewport height and then we need to find the duration of the event
    // then we also need the total hours the calendar covers
    // we also need the starting time covered
    function getEventDuration(event: CalendarEvent): number {
        let start = stringToDate(event.start);
        let end = stringToDate(event.end);

        // diff in ms
        return subtractTimes(start, end)

    }

    function getEventStartTime(event: CalendarEvent, startTime: number): number {
        let start = stringToDate(event.start);

        let startHours = Math.floor(startTime);
        let startMinutes = Math.round((startTime - startHours) * 60);
        // we need to clone the start but then change the time
        let clone = new Date(start);
        clone.setHours(startHours, startMinutes, 0, 0);

        let elapsedTime = subtractTimes(clone, start);

        return elapsedTime;
    }

    // returns in hour diff
    function subtractTimes(start: Date, end: Date): number {
        let elapsed = end.getTime() - start.getTime();
        return elapsed / 1000 / 60 / 60;
    }

    let duration    = $derived(getEventDuration(calEvent));
    let startOffset = $derived(getEventStartTime(calEvent, startingHour));
    let pixelsPerHour = $derived(height / numHours);
    let eventTop = $derived(startOffset * pixelsPerHour)
    let eventHeight = $derived(pixelsPerHour * duration)



    // if any of these conditions is false, we hide the event
    let visible = $derived(
        duration > 0 &&
        startOffset >= 0 &&
        startOffset + duration <= numHours
        );
</script>

{#if visible}
    <div class="card" style="width: {width}px; height: {eventHeight}px; top: {eventTop}px;">
        <div class="bar"></div>
        <div class="inner">
            <p class="xs" style="color: rgb(112, 112, 112);">
                {
                new Intl.DateTimeFormat("en-GB", {
                    hour: "2-digit",
                    minute: "2-digit",
                    hour12: false,
                    }).format(stringToDate(calEvent.start))
                }
            </p>
            <p class='xs' style="color: var(--primary-dark);">
                {calEvent.summary}
            </p>
        </div>
    </div>
{/if}

<style>

    .inner {
        padding: 0.3rem;
        opacity: 0.7;
        background-color: var(--primary-color);
        width: 100%;
    }
    .xs {
        font-size: 0.8rem;
        text-overflow: ellipsis;
        overflow: hidden;
    }

    .card {
        display: flex;
        position:absolute;
        box-sizing: border-box;
        border: 1px solid var(--border-color);
        border-radius: 7px;
        margin: 0;
        padding: 0;
        overflow: hidden;
        word-wrap: break-word;
        text-overflow: ellipsis;
    }

    .bar {
        flex-shrink: 0;
        flex-grow: 0;
        width: 4px;
        background: var(--border-color);
    }
</style>