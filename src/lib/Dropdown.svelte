<script lang='ts'>
    import { ChevronDown } from '@lucide/svelte';
    import { fade, fly } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { quartInOut } from 'svelte/easing';
    import { portal } from './misc/portal';
    import { getPageEl } from './misc/context';

    type DropdownOption = {
        label: string;
        value: string;
    }

    let {
        options,
        selected = $bindable(""),
        dropDisabled = false
    }: {
        options: DropdownOption[],
        selected: string,
        dropDisabled?: boolean
    } = $props();

    let open = $state(false);
    let triggerEl = $state<HTMLButtonElement>();
    let portalEl = $state<HTMLDivElement>();
    let pos = $state({top: 0, left: 0});
    const getPageElement = getPageEl();

    let selectedLabel = $derived.by(() =>
        options.find(o => o.value === selected)?.label ?? ""
    );

    function handleClickOutside(event: MouseEvent) {
        const target = event.target as Node;
        const insideTrigger = triggerEl?.contains(target);
        const insidePortal = portalEl?.contains(target);
        if (!insideTrigger && !insidePortal) {
            open = false;
        }
    }

    function selectOption(option: DropdownOption) 
    {
        selected = option.value;
        open = false;
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });

    function toggle() {
        open = !open;
        const pageEl = getPageElement();
        if (open && triggerEl && pageEl) {
            const rect = triggerEl.getBoundingClientRect();
            const pageRect = pageEl.getBoundingClientRect();
            pos = { 
                top: rect.bottom - pageRect.top + pageEl.scrollTop, 
                left: rect.left - pageRect.left + pageEl.scrollLeft
            };
        }
    }
</script>

{#snippet optionSnippet(option: DropdownOption)}
    <button class="option" class:selected-bar={selected === option.value} onclick={() => selectOption(option)}>
        {option.label}
    </button>
{/snippet}

<div class="container">
    <div class="dropdown-container">
        <button class="dropdown" onclick={toggle} bind:this={triggerEl}>
            <div style="position:relative; display: flex; width:100%; align-items: center;">
                {#if selected}
                    {#key selected}
                        <span class="selected-ob" transition:fade|global={{ duration: 300, easing: quartInOut}}>{selectedLabel}</span>
                    {/key}
                {/if}
                <span class="right-align"><ChevronDown size={20} strokeWidth={1}/></span>
            </div>
        </button>
        {#if open && !dropDisabled}
            <div 
                style='position:absolute; top: {pos.top}px; left: {pos.left}px; z-index: 999;' 
                use:portal={getPageElement}
                bind:this={portalEl}
            >
                <div transition:fly={{y: -15, duration: 150, easing: quartInOut}} class="options">
                    {#each options as option}
                        {#if option}
                            {@render optionSnippet(option)}
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
        border-radius: 15px;
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
        border-radius: 15px;
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