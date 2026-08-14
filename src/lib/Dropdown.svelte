<script lang='ts'>
    import { ChevronDown } from '@lucide/svelte';
    import { fade, fly, slide } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { quartInOut } from 'svelte/easing';
    import { portal } from './misc/portal';
    import { getPageEl } from './misc/context';

    type DropdownOption = {
        label: string;
        value: string | null;
    }

    let {
        options,
        selected = $bindable(""),
        dropDisabled = false,
    }: {
        options: DropdownOption[],
        selected: string | null,
        dropDisabled?: boolean,
    } = $props();

    let open = $state(false);
    let triggerEl = $state<HTMLButtonElement>();
    let portalEl = $state<HTMLDivElement>();
    let pos = $state({top: 0, left: 0});
    let indexLen = $derived(options.length);
    const getPageElement = getPageEl();
    
    let selectedLabel = $derived(
        options.find(o => o.value === selected)?.label ?? ""
    );
    
    let selectedIndex = $derived(
        options.findIndex(option => option.value === selected)
    );
    
    // really messed up but it works
    let slideDirection = $derived.by(() => {
        if(selectedIndex < Math.floor(indexLen / 2)) {
            return "down";
        } else if (selectedIndex > Math.floor(indexLen / 2)) {
            return "up";
        } else if (indexLen % 2 === 1) {
            return "both";
        } else {
            return "up"
        }
    });

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
                // we need the top to also account for the index of the current selected index
                top: rect.top - pageRect.top + pageEl.scrollTop - selectedIndex * rect.height,
                left: rect.left - pageRect.left + pageEl.scrollLeft
            };
        }
    }

    //custom slide transition

    function dynamicSlide(node: HTMLElement, {
        duration = 150,
        easing = quartInOut,
        direction = 'both', // 'up' | 'down' | 'both'
        completion = 0.6,
    } = {}) {
        const height = node.offsetHeight;

        return {
            duration,
            easing,
            css: (t: number, u: number) => {
                let top = 0;
                let bottom = 0;

                let progress = (1 - completion) * u;

                if (direction === 'up') {
                    top = height * (progress);
                }

                if (direction === 'down') {
                    bottom = height * (progress);
                }

                if (direction === 'both') {
                    top = height / 2 * (progress);
                    bottom = height / 2 * (progress);
                }

                return `
                    clip-path: inset(${top}px 0 ${bottom}px 0);
                `;
            }
        };
    }
</script>

{#snippet optionSnippet(option: DropdownOption)}
    <button class="option" class:selected-option={selected === option.value} onclick={() => selectOption(option)}>
        {#if selected == option.value}
            <span class="selected-bar"></span>
        {/if}
        {option.label}
    </button>
{/snippet}

<div class="container">
    <div class="dropdown-container">
        <button class="dropdown" onclick={toggle} bind:this={triggerEl}>
            <div style="position:relative; display: flex; width:100%; align-items: center;">
                {#if selected}
                    {#key selected}
                        <span class="selected-ob" transition:fade|global={{ duration: 150, easing: quartInOut}}>{selectedLabel}</span>
                    {/key}
                {/if}
                <span class="right-align"><ChevronDown size={20} strokeWidth={1}/></span>
            </div>
        </button>
        {#if open && !dropDisabled}
            <div 
                style='position:absolute; top: {pos.top}px; left: {pos.left}px; z-index: 999; width: 100%;' 
                use:portal={getPageElement}
                bind:this={portalEl}
            >
                <div in:dynamicSlide={{duration: 75, direction: slideDirection}} class="options">
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
        position: absolute;
        background-color: var(--highlight-color);
        height: 1.75rem;
        width: 3px;
        margin-left: -8px;
        border-radius: 15px;
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
        height: 3rem;
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
        z-index: 5000;
        align-items: start;
        justify-content: start;
        overflow: hidden;
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
        position:relative;
        display:flex;
        align-items: center;
        text-align: left;
        transition: 0.15s ease-in-out;
        height: 3rem;
        padding: 1rem;
        width: 100%;
        color: var(--primary-dark);
    }
    .option:hover {
        background-color: var(--secondary-color);
    }
</style>