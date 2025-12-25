<script lang='ts'>
    import { onMount } from "svelte";
    import { getSimpleTimeOfDay } from "./misc/timeofday";
    import { startClock } from "./stores.svelte";
    import { fly } from "svelte/transition";
    import { quartOut } from "svelte/easing";

    let timeOfDay = $state("");
    let currentDate: Date = $state(new Date());

    onMount (async () => {
        timeOfDay = await getSimpleTimeOfDay(new Date());

        startClock(date => currentDate = date);
    });
</script>

<div class="container" transition:fly={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    <h1>
        {currentDate.getHours().toString().padStart(2, "0")}:{currentDate.getMinutes().toString().padStart(2, "0")}
    </h1>
    <h4>
        {timeOfDay.charAt(0).toUpperCase() + timeOfDay.slice(1)}
    </h4>
</div>

<style>
    .container {
        padding: 1rem;
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        border-radius: 15px;
        background-color: var(--primary-light);
        width: 100%;
    }
</style>