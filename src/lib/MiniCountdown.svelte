<script lang='ts'>
    import { IterationCw, Pause, Play } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import { msToMinutesSeconds, timerStore } from "./types/timerStore.svelte";
    import { fly } from "svelte/transition";
    import { quartInOut } from "svelte/easing";

    let expanded = $state(false);

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

<div class="container" transition:fly={{ x: 150, duration: 300, easing: quartInOut}}>
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
        <h5 class="time">
            {msToMinutesSeconds(timerStore.timeLeft)}
        </h5>
    </div>

    <div class='timer-info'>
        <h5>
            {timerStore.isStudying ? "Study" : "Break"}
        </h5>
        <p>
            Session {timerStore.sessionNum} / {timerStore.totalSessions}
        </p>
        <div class="timer-buttons">
            {#if timerStore.isRunning}
                <Button flavor={"outline"} onclick={timerStore.pause} Icon={Pause} class={"square small"} />
            {:else}
                <Button flavor={"outline"} onclick={timerStore.start} Icon={Play} class={"square small"}/>
                <Button flavor={"outline"} onclick={timerStore.reset} Icon={IterationCw} class={"square small"}/>
            {/if}
        </div>
    </div>
</div>

<style>
    .timer-buttons {
        margin-top: 0.5rem;
        display: flex;
        gap: 0.5rem;
    }

    .timer {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .time {
        position: absolute;
    }

    .timer-info {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.25rem;
        flex: 1;
    }

    .container {
        position: absolute;
        right: 0;
        top: 3.5rem;
        width: 17rem;
        height: 8.75rem;
        box-shadow: 0px 0px 5px -2px var(--border-color);
        border: 1px solid var(--border-color);
        border-top-left-radius: 15px;
        border-bottom-left-radius: 15px;
        background-color: var(--primary-light);
        z-index: 50;
        display: flex;
        align-items: center;
        padding: 0 2rem;
    }
    svg {
        width: 7.25rem;
        height: 7.25rem;
    }
</style>