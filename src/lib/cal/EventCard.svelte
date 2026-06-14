<script lang='ts'>
    import { Clock } from "@lucide/svelte";
import type { CalendarEvent } from "./calendar";
    import { stringToDate } from "./dateCalculations";
  import { quartOut } from "svelte/easing";
  import { fly } from "svelte/transition";

    
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
    let eventTop = $derived(startOffset * pixelsPerHour);
    let eventHeight = $derived(pixelsPerHour * duration);



    // if any of these conditions is false, we hide the event
    let visible = $derived(
        duration > 0 &&
        startOffset >= 0
        // startOffset + duration <= numHours
        );

    let isCompact = $derived( eventHeight < 36);

    let nameFontSize = $derived(
        isCompact
            ? `${Math.min(12.8, Math.max(9, eventHeight * 0.55))}px`
            : "0.8rem"
    );

    let padding = $state(2);
</script>

{#if visible}
    <div
        class="card" 
        style="
            width: {width - padding }px; 
            height: {eventHeight - padding}px; 
            top: {eventTop + padding / 2}px;
            left: {padding / 2}px; 
            border-radius:{isCompact ? '4px' : '5px'}">
        <div class="bar"></div>
        <div class="inner {isCompact ? 'centered' : ''}">
            <p class='xs' style="color: var(--primary-dark); font-size: {nameFontSize}">
                {calEvent.summary}
            </p>
            {#if !isCompact}
                <div style="color: rgb(112, 112, 112); display: flex; gap: 0.2rem; align-items: center;">
                    <Clock size={12} />
                    <p class="xs" style="color: rgb(112, 112, 112);">
                        {
                        new Intl.DateTimeFormat("en-GB", {
                            hour: "2-digit",
                            minute: "2-digit",
                            hour12: false,
                            }).format(stringToDate(calEvent.start))
                        } -
                        {
                        new Intl.DateTimeFormat("en-GB", {
                            hour: "2-digit",
                            minute: "2-digit",
                            hour12: false,
                            }).format(stringToDate(calEvent.end))
                        }
                    </p>
                </div>
            {/if}
        </div>
    </div>
{/if}

<style>
    .centered {
        display: flex;
        align-items: center;
    }
    .inner {
        padding: 0.3rem;
        background-color: color-mix(in srgb, var(--primary-color) 50%, transparent);
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
        margin: 0;
        padding: 0;
        overflow: hidden;
        word-wrap: break-word;
        text-overflow: ellipsis;
    }

    .bar {
        /* flex-shrink: 0; */
        /* flex-grow: 0; */
        width: 4px;
        margin: 0.2rem 0rem 0.2rem 0.2rem;
        border-radius: 15px;
        background: var(--border-color);
    }
</style>