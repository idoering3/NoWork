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
    let ForecastIcons: Component[] | undefined = $state();
    let isLoadingWeather = $state(false);

    async function refreshWeather() {
        if (!currentLocation.lat || !currentLocation.lon) 
            return;
        isLoadingWeather = true;
        try {
            currentWeather = await getCurrentWeather(currentLocation.lat, currentLocation.lon);
            WeatherIcon = getWeatherIcon(currentWeather.current.weather_code, currentWeather.current.is_day == 1);

            ForecastIcons = currentWeather.daily.weather_code
                ? Array.from(currentWeather.daily.weather_code, (code) => getWeatherIcon(code, true))
                : undefined;
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

<!-- make the snippet for the daily forecast -->
{#snippet dayForecast(day: number)}
    <div style="
        display: flex; 
        flex-direction: column; 
        flex-grow: 1;
        justify-content:center;
        align-items: center;
        border: 1px solid var(--border-color);
        border-radius: 15px;
        padding: 0.5rem;
        {day != 0 ? "background-color:none" : "background-color: var(--secondary-color)"}
        "
        in:fly|global={{ y: 10, delay: 1200 + day * 75, duration: 1500, easing: quartOut}}
    >
        <div style="padding: 0.2rem;">
            {#if ForecastIcons?.[day]}
                {@const Icon = ForecastIcons[day]}
                <Icon size={24} absoluteStrokeWidth={true}/>
            {/if}
        </div>
        <p class="faded">
            H {Math.round(currentWeather!.daily.temperature_2m_max![day])}
        </p>
        <p class="faded">
            L {Math.round(currentWeather!.daily.temperature_2m_min![day])}
        </p>
    </div>
{/snippet}

<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>
    <div style="display: flex; flex-direction: column;" class="inner">
        <div 
            style="display: flex; align-items:center;" 
            in:fly|global={{ y: 10, delay: 800, duration: 1500, easing: quartOut}}
        >
            <div>
                <h1 style="padding-top:0.2rem;">
                    {#if currentWeather}
                        <span style="position: relative; display: inline-block; padding-right: 18px;">
                            {Math.round(currentWeather.current.temperature_2m)}
                            <CircleSmall 
                                size={14} 
                                style="position: absolute; top: 1rem; right: 0.5rem; color: var(--hover-primary-dark);" 
                            />
                        </span>
                    {/if}
                </h1>
    
                <p class="faded" in:fly|global={{ y: 10, delay: 1000, duration: 1500, easing: quartOut}}>
                    H 
                    {#if currentWeather?.daily.temperature_2m_max}
                        {Math.round(currentWeather.daily.temperature_2m_max[0])}<span>&#176;</span> 
                    {/if}
                    L
                    {#if currentWeather?.daily.temperature_2m_min}
                        {Math.round(currentWeather.daily.temperature_2m_min[0])}<span>&#176;</span> 
                    {/if}
                </p>
            </div>

            <div class="weather-icon">
                {#if WeatherIcon}
                    <div class="weather-icon-circle">
                        <WeatherIcon size={40} absoluteStrokeWidth={true}/>
                    </div>
                {/if}
            </div>
        </div>
        <div>
            <hr style="">
            <p style="display: flex; align-items: center; gap: 0.5rem;" class="faded"
                in:fly|global={{ y: 10, delay: 1200, duration: 1500, easing: quartOut}}
            >
                <Wind strokeWidth={1.1} size={20}/>
                {#if currentWeather?.current.wind_speed_10m}
                    {Math.round(currentWeather.current.wind_speed_10m)}
                {/if}
                mph
            </p>
            <p style="display: flex; align-items: center; gap: 0.5rem;" class="faded"
                in:fly|global={{ y: 10, delay: 1400, duration: 1500, easing: quartOut}}
            >
                <Droplet strokeWidth={1.1} size={20}/>
                {#if currentWeather?.current.precipitation}
                    {Math.round(currentWeather.current.precipitation * 100) / 100}
                {:else}
                    0.00
                {/if}
                in
            </p>
        </div>
        <div style="display: flex; align-items: center; justify-content: center; gap: 0.5rem; flex-grow: 1;">
            {#if currentWeather?.daily.temperature_2m_max}
                {#each { length: currentWeather?.daily.temperature_2m_max?.length}, day}
                    {@render dayForecast(day)}
                {/each}
            {/if}
        </div>
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
        flex: 1;
        margin-top:0.2rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    
    .weather-icon-circle {
        border: solid 1px var(--border-color);
        border-radius: 9999px;
        background-color: var(--secondary-color);
        padding:1rem;
    }

    .inner {
        padding: 0 1rem;
        flex: 1;
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