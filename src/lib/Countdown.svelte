<script lang='ts'>
    import Button from "./Button.svelte";
    import Pause from '@lucide/svelte/icons/pause';
    import Play from '@lucide/svelte/icons/play';
    import IterationCw from '@lucide/svelte/icons/iteration-cw';
    import { timerStore, msToMinutesSeconds } from '$lib/types/timerStore.svelte';

    // Props from parent component
    let { repetitions, breakTime, studyTime } = $props();

    let timerEnabled = $state(true);

    const FULL_DASH_ARRAY = 283;

    function calculateTimeFraction() {
        const limit = timerStore.timeLimit;
        if (limit <= 0) return 0;
        return timerStore.timeLeft / limit;
    }
    
    // This derived value will automatically update when timerStore values change
    let circleDasharray = $derived(`${
        (calculateTimeFraction() * FULL_DASH_ARRAY).toFixed(2)
    } 283`);
</script>

<div class="timer">
    <svg viewBox="0 0 100 100">
        <g>
            <circle fill="none" stroke="var(--secondary-color)" stroke-width="3" cx="50" cy="50" r="45"></circle>
            <path
                stroke="var(--highlight-color)" stroke-width="3"
                fill="none"
                stroke-dasharray={circleDasharray}
                stroke-linecap="round"
                transform="rotate(90 50 50)"
                d="M 50, 50 m -45, 0 a 45,45 0 1,0 90,0 a 45,45 0 1,0 -90,0"
            ></path>
        </g>
    </svg>
    <div class="content">
        <h1>
            {msToMinutesSeconds(timerStore.timeLeft)}
        </h1>
        <h7>
            {timerStore.isStudying ? "Study" : "Break"} | Session {timerStore.sessionNum} / {repetitions}
        </h7>
        <div class="timer-buttons">
            {#if timerStore.isRunning}
                <Button onclick={timerStore.pause} Icon={Pause} class={"square"} />
            {:else}
                <Button onclick={timerStore.start} Icon={Play} class={"square"}/>
                <Button onclick={timerStore.reset} Icon={IterationCw} class={"square"}/>
            {/if}
        </div>
    </div>
</div>

<style>

    path {
        transition: stroke-dasharray 0.1s linear;
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