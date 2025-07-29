<script lang='ts'>
    import type { Component, Snippet } from "svelte";

    interface Props {
        Icon?: Component | null;
        iconSize?: number;
        iconStroke?: number;
        flavor?: 'ghost' | 'danger' | 'default' | 'disabled';
        href?: string | undefined;
        expanded?: boolean;
        onclick?: (event: MouseEvent) => void;
        children?: Snippet;
    }

    let {
        Icon = null,
        iconSize = 18,
        iconStroke = 1.1,
        flavor = 'default',
        href = undefined,
        expanded = false,
        children,
        ...props
    } = $props();
    // Determine which element to render
    let isLink = $derived(href !== undefined);
</script>


{#if isLink}
    <a href={href} class:red={flavor === 'danger'} class:ghost={flavor === 'ghost'} class:disabled={flavor === 'disabled'} {...props}>
        {#if Icon}
            <span class="icon">
                <Icon size={iconSize} strokeWidth={iconStroke}></Icon>
            </span>
        {/if}
        {@render children?.()}
    </a>
{:else}
    <button class:red={flavor === 'danger'} class:ghost={flavor === 'ghost'} class:disabled={flavor === 'disabled'} {...props}>
        {#if Icon}
            <span class="icon">
                <Icon size={iconSize} strokeWidth={iconStroke}></Icon>
            </span>
        {/if}
        {@render children?.()}
    </button>
{/if}




<style>
    /* Default variant */
    button, a {
        user-select: none;
        border-radius: 7px;
        border: 1px solid var(--border-color);
        text-decoration: none;
        display: flex;
        justify-content: center;
        vertical-align: middle;
        align-items: center;
        height: 3rem;
        width: 100%;
        transition: 0.15s ease-in-out;
        color: black;
        font-family: 'Inter', sans-serif;
        font-size: 1rem;
        background-color: transparent;
        box-shadow: 0px 0px 5px -2px var(--border-color);
    }
    button:hover, a:hover {
        background-color: var(--secondary-color);
        cursor:pointer;
    }
    button:active, a:active {
        color: #8d6577;
    }

    button.disabled, a.disabled {
        pointer-events: none;
        box-shadow: none;
        border: none;
        border-radius: 0;
        color: var(--border-color);
    }

    button.ghost, a.ghost {
        box-shadow: none;
        background-color: transparent;
        border-radius: 0;
        border: none;
        aspect-ratio: 1 / 1;
    }
    button.ghost:hover, a.ghost:hover {
        background-color: var(--hover-color);
        cursor:pointer;
    }
    button.ghost:active, a.ghost:active {
        color: #8d6577;
    }

    /* Red variant */
    button.red, a.red {
        background-color: transparent;
        color: #D64540;
        border-radius: 0;
        border: none;
        aspect-ratio: 1 / 1;
        box-shadow: none;
    }
    button.red:hover, a.red:hover
    {
        background-color: red;
        color:white;
    }
</style>