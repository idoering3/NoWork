<script lang='ts'>
    import type { Snippet } from "svelte";
    import { quartInOut, quartOut } from "svelte/easing";
    import { scale } from "svelte/transition";
    import { flavorMap } from "./stores.svelte";

    interface Props {
        flavor?: 'default' | 'outline' | 'danger' | 'blue',
        children: Snippet,
        noPadding?: boolean,
    }

    let { 
        flavor = 'default', 
        children, 
        noPadding = false
    }: Props = $props();
</script>

<div
    transition:scale={{ delay: 50, duration: 300, start: 0.75, opacity: 0, easing: quartOut }}
    class={[`${flavorMap[flavor].name ?? ''}`, 'badge', noPadding ? 'no-padding' : '']}>
    {@render children?.()}
</div>

<style>
    .badge.no-padding {
        padding: 0;
    }

    .badge {
        box-sizing: border-box;
        gap: 0.25rem;
        display: flex;
        font-size: 0.8rem;
        padding: 0 0.5rem;
        height: 1.5rem;
        align-items: center;
        border-radius: 15px;
        background-color: var(--badge-bg);
    }
    .badge.default {
        color: black;
        --badge-bg: #e6e6e6;
    }

    .badge.secondary {
        color: black;
        --badge-bg: var(--secondary-color);
    }

    .badge.danger {
        --badge-bg: #ffb4b4;
        color: #86231c;
    }

    .badge.blue {
        --badge-bg: #749cdc;
        color: #363cda;
    }
    
    .badge.outline {
        border: 1px solid var(--border-color);
        --badge-bg: var(--primary-light);
        overflow: hidden;
    }
</style>