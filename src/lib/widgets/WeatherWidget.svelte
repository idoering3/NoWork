<script lang='ts'>
    import { onMount, type Component } from "svelte";
    import { currentLocation, startClock } from "../stores.svelte";
    import { fly } from "svelte/transition";
    import { quartOut } from "svelte/easing";
    import { CircleSmall, Droplet, Wind} from "@lucide/svelte";
    import { getCurrentWeather, getWeatherIcon, type WeatherData } from "$lib/misc/weather";

    const WEATHER_POLL_INTERVAL = 10* 60 * 1000; // 10 min in ms
    let currentWeather: WeatherData | undefined = $state();
    let WeatherIcon: Component | undefined = $state();
    let isLoadingWeather = $state(false);

    async function refreshWeather() {
        if (!currentLocation.lat || !currentLocation.lon) 
            return;
        isLoadingWeather = true;
        try {
            currentWeather = await getCurrentWeather(currentLocation.lat, currentLocation.lon);
            WeatherIcon = getWeatherIcon(currentWeather.current.weather_code, currentWeather.current.is_day == 1);
        } catch (e) {
            console.error("Failed to fetch weather:", e);
        } finally {
            isLoadingWeather = false;
        }
    }

    onMount(() => {
        const intervalId = setInterval(refreshWeather, WEATHER_POLL_INTERVAL);

        return () => {
            clearInterval(intervalId);
        };
    });

    $effect(() => {
        void (async () => {
            if (currentLocation.lat && currentLocation.lon) {
                void refreshWeather();
            }
        })();
    });
</script>

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    <div style="" class="inner">
        <div 
            style="display: flex; align-items:center;" 
            in:fly|global={{ y: 15, delay: 800, duration: 1500, easing: quartOut}}
        >
            <h1 style="padding-top:0.2rem;">
                {#if currentWeather}
                    <span style="position: relative; display: inline-block; padding-right: 14px;">
                        {Math.round(currentWeather.current.temperature_2m)}
                        <CircleSmall 
                            size={14} 
                            style="position: absolute; top: 1rem; right: 0.5rem; color: var(--hover-primary-dark);" 
                        />
                    </span>
                {/if}
            </h1>
            <div class="weather-icon">
                {#if WeatherIcon}
                    <WeatherIcon size={40} absoluteStrokeWidth={true}/>
                {/if}
            </div>
        </div>
        <p class="faded" in:fly|global={{ y: 15, delay: 1000, duration: 1500, easing: quartOut}}>
            H 
            {#if currentWeather?.daily.temperature_2m_max}
                {Math.round(currentWeather.daily.temperature_2m_max[0])}<span>&#176;</span> 
            {/if}
            L
            {#if currentWeather?.daily.temperature_2m_min}
                {Math.round(currentWeather.daily.temperature_2m_min[0])}<span>&#176;</span> 
            {/if}
        </p>
        <hr>
        <p style="display: flex; align-items: center; gap: 0.5rem;" class="faded">
            <Wind strokeWidth={1.1} size={20}/>
            {#if currentWeather?.current.wind_speed_10m}
                {Math.round(currentWeather.current.wind_speed_10m)}
            {/if}
            mph
        </p>
        <p style="display: flex; align-items: center; gap: 0.5rem;" class="faded">
            <Droplet strokeWidth={1.1} size={20}/>
            {#if currentWeather?.current.precipitation}
                {Math.round(currentWeather.current.precipitation * 100) / 100}
            {:else}
                0.00
            {/if}
            in
        </p>
    </div>
</div>

<style>
    hr {
        border-color: var(--border-color);
    }

    .container {
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        border-radius: 15px;
        background-color: var(--primary-light);
        width: 100%;
        display: flex;
        gap: 0.5rem;
        flex-direction: column;
        height: 100%;
    }

    .weather-icon {
        margin: 1rem;
        flex: 1;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    
    .inner {
        margin-left: 1rem;
    }

    .faded {
        color: rgb(112, 112, 112);
    }

    h1 {
        margin: 0;
        padding: 0;
    }

    p {
        margin: 0;
        padding: 0;
    }
</style>