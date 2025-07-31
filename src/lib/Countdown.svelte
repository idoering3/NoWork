<script lang='ts'>
    import Button from "./Button.svelte";
    import Pause from '@lucide/svelte/icons/pause';
    import Play from '@lucide/svelte/icons/play';
    import IterationCw from '@lucide/svelte/icons/iteration-cw';
    import { onMount } from "svelte";

    let { repetitions = $bindable(), time, timerEnabled = $bindable() } = $props();

    let timeLeft = $state(time);
    let timer: ReturnType<typeof setInterval> | null = null;
    let isRunning = $state(false);

    onMount(() => {
        startTimer();
    })

    function startTimer() {
        isRunning = true;
        timer = setInterval(() => {
            if (timeLeft > 0) {
                timeLeft -= 100;
            } else {
                pauseTimer();
                timerEnabled = false;
            }
        }, 100);
    }


    function pauseTimer () {
        if (timer) {
            clearInterval(timer);
            timer = null;
            isRunning = false;
        }
    }

    function resetTimer () {
        pauseTimer()
        timerEnabled = false;
    }

    function msToMinutesSeconds(ms: number): string {
        const totalSeconds = Math.floor(ms / 1000);
        const minutes = Math.floor(totalSeconds / 60);
        const seconds = totalSeconds % 60;
        return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }

    let pathRemaining: SVGPathElement;
    const FULL_DASH_ARRAY = 283;

    function calculateTimeFraction() {
        return timeLeft / time;
    }

    let circleDasharray = $derived(`${(
            calculateTimeFraction() * FULL_DASH_ARRAY
        ).toFixed(0)} 283`);
</script>
<div class="timer">
    <svg viewBox="0 0 100 100">
        <g>
        <circle fill="none" cx="50" cy="50" r="45"></circle>
        <path
            bind:this={pathRemaining}
            stroke="var(--highlight-color)" stroke-width="3"
            fill="none"
            stroke-dasharray={circleDasharray}
            d="
            M 50, 50
            m -45, 0
            a 45,45 0 1,0 90,0
            a 45,45 0 1,0 -90,0
            "
        ></path>
        </g>
    </svg>
    <div class="content">
        <h1>
            {msToMinutesSeconds(timeLeft)}
        </h1>
        <h7>
            Study
        </h7>
        <div class="timer-buttons">
            {#if isRunning}
                <Button onclick={pauseTimer} Icon={Pause} class={"square"} />
            {:else}
                <Button onclick={startTimer} Icon={Play} class={"square"}/>
                <Button onclick={resetTimer} Icon={IterationCw} class={"square"}/>
            {/if}
        </div>
    </div>
</div>
<style>
    path {
        transition: stroke-dasharray 0.001s linear;
    }

    svg {
        position: absolute;
        width: 25rem;
        height: 25rem;
    }

    .content {
        z-index: 50;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        justify-content: center;
        align-items: center;
    }

    .timer {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 1rem;
    }
    .timer-buttons {
        display: flex;
        gap: 1rem;
        justify-content: center;
    }
    h1 {
        margin-bottom: 0;
        line-height: 0.7;
        font-family: "Inter", sans-serif;
    }
</style>