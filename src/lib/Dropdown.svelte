<script lang='ts'>
    import { ChevronDown } from '@lucide/svelte';
    import { fade, fly, slide } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { quartInOut } from 'svelte/easing';

    let { options, selected=$bindable("")} = $props();

    let dropdownOpen = $state(false);

    let dropdownEl: HTMLElement;

    function handleClickOutside(event: MouseEvent) {
        if (dropdownEl && !dropdownEl.contains(event.target as Node)) {
            dropdownOpen = false;
        }
    }

    function selectOption(text: string) 
    {
        selected = text;
        dropdownOpen = false;
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });
</script>

{#snippet option(text: string)}
    <button class="option" class:selected-bar={selected === text} onclick={() => selectOption(text)}>
        {text}
    </button>
{/snippet}

<div class="container">
    <div class="dropdown-container" bind:this={dropdownEl}>
        <button class="dropdown" onclick={() => dropdownOpen = !dropdownOpen}>
            <div style="position:relative; display: flex; width:100%; align-items: center;">
                {#if selected}
                    {#key selected}
                        <span class="selected-ob" transition:fade={{ duration: 300, easing: quartInOut}}>{selected}</span>
                    {/key}
                {/if}
                <span class="right-align"><ChevronDown size={20} strokeWidth={1}/></span>
            </div>
        </button>
        {#if dropdownOpen}
            <div style='position:relative; display: flex; align-items: center; justify-content: center;'>
                <div transition:fly={{y: -15, duration: 150, easing: quartInOut}} class="options">
                    {#each options as thing}
                        {#if thing}
                            {@render option(thing)}
                        {/if}
                    {/each}
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    .selected-ob {
        position: absolute;
    }

    .right-align {
        margin-left: auto;
    }

    .selected-bar {
        text-decoration-line: underline;
        font-weight: bold;
        color: var(--highlight-color);
    }

    .dropdown-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .container {
        align-items: center;
        gap: 1rem;
        user-select: none;
    }

    div, button {
        font-family: "Inter", sans-serif;
        font-size: 1rem;
        padding: 0;
        border: none;
        background-color: transparent;
    }
    .dropdown {
        position:relative;
        vertical-align: middle;
        align-items: center;
        display: flex;
        justify-content: space-between;
        min-width: 15rem;
        gap: 1rem;
        padding: 1rem;
        border-radius: 7px;
        border: 1px solid var(--border-color);
        transition: 0.15s ease-in-out;
        box-shadow: 0px 0px 5px -2px var(--border-color);
        color: var(--primary-dark);
        background-color: var(--primary-light);
    }
    .dropdown:hover {
        background-color: var(--secondary-color);
    }
    .options {
        z-index: 50;
        align-items: center;
        overflow: hidden;
        top: 0.5rem;
        position: absolute;
        border-radius: 7px;
        background-color: var(--primary-light);
        border: 1px solid var(--border-color);
        box-shadow: 0px 0px 5px -2px var(--border-color);
        display: flex;
        flex-direction: column;
        min-width: 15rem;
        color: var(--primary-dark);
    }
    .option {
        transition: 0.15s ease-in-out;
        padding: 1rem;
        width: 100%;
        color: var(--primary-dark);
    }
    .option:hover {
        background-color: var(--secondary-color);
    }
</style>