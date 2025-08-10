<script lang='ts'>
    import type { Component, Snippet } from "svelte";

    interface Props {
        Icon?: Component | null;
        iconSize?: number;
        iconStroke?: number;
        flavor?: 'ghost' | 'danger' | 'default' | 'disabled' | 'outline' | 'primary';
        href?: string | undefined;
        expanded?: boolean;
        onclick?: (event: MouseEvent) => void;
        onmousedown?: (event: MouseEvent) => void;
        onmouseup?: (event: MouseEvent) => void;
        onmouseleave?: (event: MouseEvent) => void;
        children?: Snippet;
        class?: string;
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
    }: Props = $props();
    // Determine which element to render
    let isLink = $derived(href !== undefined);

    const extraClass = props.class ?? '';
    delete props.class;

    const flavorMap = {
        danger: 'red',
        ghost: 'ghost',
        outline: 'outline',
        disabled: 'disabled',
        default: 'default',
        primary: 'primary'
    }

    const flavorName = `${flavorMap[flavor] ?? ''} ${extraClass}`.trim();
</script>


{#if isLink}
    <a href={href} class={flavorName} {...props}>
        {#if Icon}
            <span class="icon">
                <Icon size={iconSize} strokeWidth={iconStroke}></Icon>
            </span>
        {/if}
        {#if children}
            {@render children?.()}
        {/if}
    </a>
{:else}
    <button class={flavorName} {...props}>
        {#if Icon}
            <span class="icon">
                <Icon size={iconSize} strokeWidth={iconStroke}></Icon>
            </span>
        {/if}
        {#if children}
            {@render children?.()}
        {/if}
    </button>
{/if}

<style>
    span {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    button.square, a.square {
        width: 3rem;
        height: 3rem;
        flex-shrink: 0;
    }

    button.square.xsmall, a.square.xsmall {
        width: 1.5rem;
        height: 1.5rem;
    }

    button.square.small, a.square.small {
        width: 2rem;
        height: 2rem;
    }

    /* Default variant */
    button, a {
        user-select: none;
        border-radius: 7px;
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
        background-color: var(--secondary-color);
        border: 1px solid var(--secondary-color);
    }
    button:hover, a:hover {
        background-color: var(--hover-color);
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

    button.primary, a.primary {
        background-color: var(--primary-dark);
        color: var(--primary-light);

    }

    button.primary:hover, button.primary:hover {
        background-color: var(--hover-primary-dark);
    }

    button.primary:active, button.primary:active {
        color: var(--border-color);
    }

    button.outline, a.outline {
        background-color: white;
        border: 1px solid var(--border-color);
    }
    button.outline:hover, a.outline:hover {
        background-color: var(--secondary-color);
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

    button.rounded, a.rounded {
        border-radius: 7px;
    }

    button.circular, a.circular {
        border-radius: 15px;
    }
</style>