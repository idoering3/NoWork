<script lang='ts'>
    import { ChevronDown } from '@lucide/svelte';
    import { fade, slide } from 'svelte/transition';
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
            {#if selected}
                <span transition:fade={{ duration: 300, easing: quartInOut}}>{selected}</span>
            {/if}
            <span class="right-align"><ChevronDown size={20} strokeWidth={1}/></span>
        </button>
        {#if dropdownOpen}
            <div transition:slide={{ duration: 300, easing: quartInOut}} class="options">
                {#each options as thing}
                    {#if thing}
                        {@render option(thing)}
                    {/if}
                {/each}
                </div>
        {/if}
    </div>
</div>

<style>
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
        box-shadow: 0px 0px 5px -2px #b8b8b8;
    }
    .dropdown:hover {
        background-color: var(--secondary-color);
    }
    .options {
        align-items: center;
        overflow: hidden;
        position: absolute;
        border-radius: 7px;
        background-color: var(--primary-color);
        border: 1px solid var(--border-color);
        box-shadow: 0px 0px 5px -2px #b8b8b8;
        display: flex;
        flex-direction: column;
        min-width: 15rem;
    }
    .option {
        transition: 0.15s ease-in-out;
        padding: 1rem;
        width: 100%;
    }
    .option:hover {
        background-color: var(--secondary-color);
    }
</style>