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
    in:scale={{ duration: 150, delay: 1000, easing: quartOut, start: 0.75, opacity: 1}}
    out:scale={{ duration: 150, easing: quartOut, start: 0.75, opacity: 1}}
    class={[`${flavorMap[flavor].name ?? ''}`, 'badge', noPadding ? 'no-padding' : '']}>
    {@render children?.()}
</div>

<style>
    .badge.no-padding {
        padding: 0;
    }

    .badge {
        gap: 0.25rem;
        display: flex;
        font-size: 0.8rem;
        padding: 0 0.5rem;
        height: 1.5rem;
        align-items: center;
        border-radius: 15px;
    }
    .badge.default {
        color: black;
        background-color: #e6e6e6;
    }

    .badge.secondary {
        color: black;
        background-color: var(--secondary-color);
    }

    .badge.danger {
        background-color: #ffb4b4;
        color: #86231c;
    }

    .badge.blue {
        background-color: #749cdc;
        color: #363cda;
    }
    
    .badge.outline {
        border: 1.5px solid var(--border-color);
    }
</style>