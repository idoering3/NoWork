<script lang='ts'>
    import { fly, slide } from "svelte/transition";
    import Button from "./Button.svelte";
    import Calendar from "@lucide/svelte/icons/calendar";
    import { onMount } from "svelte";
    import { ChevronLeft, ChevronRight } from "@lucide/svelte";
    import { quartInOut, quartOut } from "svelte/easing";

    let dropdownOpen = $state(false);

    let dropdownEl: HTMLElement;

    function handleClickOutside(event: MouseEvent) {
        const target = event.target as HTMLElement;
        
        if (!dropdownEl.contains(target)) {
            dropdownOpen = false;
            return;
        }

        if (target.closest('button')) {
            // Clicked button inside dropdown that should NOT close it
            return; // do nothing, keep dropdown open
        }
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });

    const dayLabels = [
        'Su',
        "Mo",
        "Tu",
        "We",
        "Th",
        "Fr",
        "Sa"
    ];

    type CalendarDay = {
        day: number;
        month: number; // 0-based
        year: number;
        date: Date;
        inCurrentMonth: boolean;
    };

    function getDaysInMonth(year: number, month: number) {
        return new Date(year, month + 1, 0).getDate();
    }

    function generateCalendarDays(year: number, month: number): CalendarDay[] {
        const firstDayIndex = new Date(year, month, 1).getDay();
        const totalDays = getDaysInMonth(year, month);

        // Previous month
        const prevMonth = (month - 1 + 12) % 12;
        const prevYear = month === 0 ? year - 1 : year;
        const prevMonthDays = getDaysInMonth(prevYear, prevMonth);

        // Leading days from previous month
        const leadingDays: CalendarDay[] = Array.from(
            { length: firstDayIndex },
            (_, i) => {
                const dayNum = prevMonthDays - firstDayIndex + i + 1;
                return {
                    day: dayNum,
                    month: prevMonth,
                    year: prevYear,
                    date: new Date(prevYear, prevMonth, dayNum),
                    inCurrentMonth: false
                };
            }
        );

        // Current month days
        const currentDays: CalendarDay[] = Array.from(
            { length: totalDays },
            (_, i) => {
                const dayNum = i + 1;
                return {
                    day: dayNum,
                    month,
                    year,
                    date: new Date(year, month, dayNum),
                    inCurrentMonth: true
                };
            }
        );

        // Next month
        const nextMonth = (month + 1) % 12;
        const nextYear = month === 11 ? year + 1 : year;

        // Fill trailing days so total = 42 (6 weeks)
        const trailingDaysCount = 42 - (leadingDays.length + currentDays.length);
        const trailingDays: CalendarDay[] = Array.from(
            { length: trailingDaysCount },
            (_, i) => {
                const dayNum = i + 1;
                return {
                    day: dayNum,
                    month: nextMonth,
                    year: nextYear,
                    date: new Date(nextYear, nextMonth, dayNum),
                    inCurrentMonth: false
                };
            }
        );

        return [...leadingDays, ...currentDays, ...trailingDays];
    }


    const getMonthName = (monthIndex: number): string => {
        const date = new Date();
        date.setDate(2);
        date.setMonth(monthIndex);
        return new Intl.DateTimeFormat('en-US', { month: 'long' }).format(date);
    };

    let currentDate = new Date();
    let year = $state(currentDate.getFullYear());
    let month = $state(currentDate.getMonth());
    let monthText = $derived(getMonthName(month));

    let days = $derived(generateCalendarDays(year, month));

    function increaseMonth() {
        if (month === 11) {
            year += 1;
            month = 0;
        } else {
            month += 1;
        }
    }

    function decreaseMonth() {
        if (month === 0) {
            year -= 1;
            month = 11;
        } else {
            month -= 1;
        }
    }

    function selectDate (date: CalendarDay) {
        selectedDate = date.date;
    }

    interface Props {
        selectedDate: Date | null,
        size?: 'xsmall' | 'small' | '',
        slowAnimation?: boolean
    }

    let { selectedDate = $bindable(), size = '', slowAnimation = true }: Props = $props();

    let dropsUp = $state(false);
    let posRight = $state(false);

    function updateDropDirection(inputEl: HTMLElement, dropdownHeight: number, dropdownWidth: number) {
        const rect = inputEl.getBoundingClientRect();
        const spaceBelow = window.innerHeight - rect.bottom; // space from input bottom to viewport bottom
        const spaceAbove = rect.top; // space from input top to viewport top
        const spaceRight = window.innerWidth - rect.left;
        const spaceLeft = rect.right;

        // flip dropsUp if there's not enough space below
        const dropsUp = spaceBelow < dropdownHeight && spaceAbove > dropdownHeight;
        const posRight = spaceRight > dropdownWidth && spaceLeft > dropdownWidth;
        return [dropsUp, posRight];
    }

    $effect(() => {
        if (dropdownOpen && calendarElement) {
            const calendarHeight = calendarElement.offsetHeight;
            const calendarWidth = calendarElement.offsetWidth;
            [dropsUp, posRight] = updateDropDirection(dropdownEl, calendarHeight, calendarWidth);
        }
    });

    let calendarElement: HTMLElement | undefined = $state();

</script>

<div class='container' bind:this={dropdownEl} 
    in:fly|global={{ duration: slowAnimation ? 1500 : 300, delay: slowAnimation? 900 : 0, y:7, easing: quartOut }}
    style={posRight ? 'justify-content: start' : 'justify-content:end'}
    >
    <Button class="square {size}" flavor="outline" Icon={Calendar} onclick={() => dropdownOpen = !dropdownOpen}/>
    {#if dropdownOpen}
        <div 
            bind:this={calendarElement}
            class='context-menu' 
            transition:fly={{ y: dropsUp ? 15 : -15, duration: 300, easing: quartOut }}
            style={dropsUp ? 'bottom: 3.25rem;' : 'top: 3.25rem;'}  
        >
            <div class="top">
                <Button flavor="ghost" class="square small rounded" Icon={ ChevronLeft } onclick={decreaseMonth} />
                    <h8>
                        {monthText} {year}
                    </h8>
                <Button flavor="ghost" class="square small rounded" Icon={ ChevronRight } onclick={increaseMonth} />
            </div>
            <div class="calendar">
                {#each dayLabels as day}
                    <div class="calendar-obj faded">
                        {day}
                    </div>
                {/each}
                {#key month}
                    {#each days as day}
                        <div class="calendar-obj">

                            <Button class="square small rounded" flavor={
                                (
                                    currentDate.getFullYear() === day.year &&
                                    currentDate.getMonth() === day.month &&
                                    currentDate.getDate() === day.day
                                ) ? "default" :"ghost"}
                                onclick={() => selectDate(day)}
                            >
                                <span class:faded={!day.inCurrentMonth}>
                                    {day.day}
                                </span>
                            </Button>
                        </div>
                    {/each}
                {/key}
            </div>
        </div>
    {/if}
</div>

<style>
    .faded {
        color: rgb(112, 112, 112);
    }

    .calendar-obj {
        display: flex;
        font-size: 0.85rem;
        aspect-ratio: 1 / 1;
        align-items: center;
        justify-content: center;
    }

    .calendar {
        flex-grow: 1;
        gap: 0.5rem;
        display: grid;
        grid-template-columns: repeat(7, 1fr);
        grid-template-rows: repeat(7, 1fr);
    }

    .top {
        display: flex;
        width: 100%;
        justify-content: space-between;
        align-items: center;
    }

    .container {
        display: flex;
        align-items:flex-start;
        justify-content: end;
        position: relative;
        z-index: 100;
    }

    .context-menu {
        padding: 1rem;
        align-items: center;
        overflow: hidden;
        position: absolute;
        border-radius: 15px;
        background-color: var(--primary-light);
        border: 1px solid var(--border-color);
        box-shadow: 0px 0px 5px -2px #b8b8b8;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        min-width: 15rem;
        min-height: 15rem;
        gap: 1rem;
    }
</style>