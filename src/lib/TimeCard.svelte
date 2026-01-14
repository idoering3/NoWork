<script lang='ts'>
    import { onMount, type Component } from "svelte";
    import { getSimpleTimeOfDay } from "./misc/timeofday";
    import { startClock } from "./stores.svelte";
    import { fly } from "svelte/transition";
    import { quartOut } from "svelte/easing";
    import { MoonStar, Sun, Sunrise, Sunset, type IconProps } from "@lucide/svelte";
    import { load } from "@tauri-apps/plugin-store";

    let timeOfDay = $state("");
    let currentDate: Date = $state(new Date());
    let minutesStudied = $state(0);

    onMount (async () => {
        
        startClock(date => currentDate = date);
        const store = await load(".settings.json");
        const value = await store.get<{ value: number }>("totalStudyTime");
        console.log(value);

        if (value?.value) {
            minutesStudied = value.value;
        }

        timeOfDay = await getSimpleTimeOfDay(currentDate);
    });

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    let Icons: Record<string, Component<IconProps>> = {
        "night": MoonStar,
        "dawn": Sunrise,
        "day": Sun,
        "sunset": Sunset,   
    };

    let CurrentIcon = $derived(Icons[timeOfDay]);
</script>

<div class="container" transition:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    <div style="display: flex; gap: 0.5rem;">
        <h4 transition:fly={{ y: 15, delay: 600, duration: 1500, easing: quartOut}}>
            {currentDate.getHours().toString().padStart(2, "0")}:{currentDate.getMinutes().toString().padStart(2, "0")} | 
        </h4>
        <h4 transition:fly={{ y: 15, delay: 900, duration: 1500, easing: quartOut}}>
            {months[currentDate.getMonth()]} {currentDate.getDate()} |
        </h4>
        <h4 style="padding-top:0.2rem;" transition:fly={{ y: 15, delay: 1200, duration: 1500, easing: quartOut}}>
            <CurrentIcon />
        </h4>
    </div>
</div>
<div class="container" transition:fly|global={{ y: 30, delay: 900, duration: 1500, easing: quartOut}}>
    <div style="display: flex; gap: 0.5rem;">
        <h4 transition:fly={{ y: 15, delay: 900, duration: 1500, easing: quartOut}}>
            Study time |
        </h4>
        <h4 transition:fly={{ y: 15, delay: 1200, duration: 1500, easing: quartOut}}>
           {Math.floor(minutesStudied / 60)}hr {minutesStudied % 60}min
        </h4>
    </div>
</div>

<style>
    .container {
        padding: 1rem;
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        border-radius: 15px;
        background-color: var(--primary-light);
        width: 100%;
        display: flex;
        gap: 0.5rem;
        flex-direction: column;
    }
</style>