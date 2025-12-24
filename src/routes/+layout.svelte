<script lang='ts'>
    import '../app.css';
    import Header from '$lib/Header.svelte';
    import Sidebar from '$lib/Sidebar.svelte';
    import { page } from '$app/state';
    import { fade, fly } from 'svelte/transition';
    import { circInOut, quartInOut } from 'svelte/easing';
    import MiniCountdown from '$lib/MiniCountdown.svelte';
    import { timerStore } from '$lib/types/timerStore.svelte';
    import { load } from '@tauri-apps/plugin-store';
    import { setColors, type Theme } from '$lib/theme';
    import { onDestroy, onMount } from 'svelte';
    import { theme, themes } from "$lib/stores.svelte";
    import { cssVarToRGBArray, ShaderRenderer } from '$lib/shaders/shader';

    let { children } = $props();
    
    const root = document.documentElement;
    let renderer: ShaderRenderer | null = $state(null);
    let canvas: HTMLCanvasElement;

    onMount(async () => {
        // shader shennanigans

        const innerColor = cssVarToRGBArray("--accent-color");
        const outerColor = cssVarToRGBArray("--primary-color");

        renderer = new ShaderRenderer(canvas, innerColor, outerColor);
        await renderer.startUpdatingSun(60000);
        renderer.start();

        const store = await load(".settings.json");
        const value = await store.get<{ value: Theme }>("theme");

        if (value?.value) {
            theme.theme = value.value;
            setColors(root, theme.theme, renderer);
        }
    });

    $effect(() => {
        if (theme.theme && renderer)
            setColors(root, theme.theme, renderer);
    });

    onDestroy(() => {
        renderer?.stop();
    });

</script>

{#if timerStore.isEnabled && page.url.pathname !== "/study"}
    <MiniCountdown />
{/if}

<div class="grid">
    <canvas bind:this={canvas} id="gl-canvas" style="position:absolute; width:100vw; height:100vh; z-index:-1;"></canvas>
    <Header />
    <main>
        <Sidebar />
        <div class="main">
            {#key page.url.pathname}
                <content in:fly={{ duration: 300, y: 150, easing: circInOut}} out:fade={{ duration: 150, easing: quartInOut}}>
                    {@render children?.()}
                </content>
            {/key}
        </div>
    </main>
</div>

<style>
    .grid {
        display: flex;
        flex-wrap: wrap;
        height: 100vh;
        width: 100vw;
    }
    /* I KNOW it's a bad name! */
    .main {
        width: 100%;
        height: calc(100vh - 3rem);
        border-radius: 7px 0 0 0;
        border-top: 1px solid var(--border-color);
        border-left: 1px solid var(--border-color);
        display: grid;
    }
    main {
        margin-top: 3rem;
        display: flex;
        width: 100%;
    }
    content {
        /* padding: 3rem; */
        overflow-y: auto;
        width: calc(100vw - 3rem);
        height: calc(100vh - 3rem);
        grid-column-start: 1;
        grid-column-end: 2;
        grid-row-start: 1;
        grid-row-end: 2;
    }
</style>