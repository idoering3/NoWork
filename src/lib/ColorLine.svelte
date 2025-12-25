<script lang='ts'>
    import type { Snippet } from "svelte";
    import { quartInOut } from "svelte/easing";
    import { scale } from "svelte/transition";
    import { flavorMap } from "./stores.svelte";

    interface Props {
        flavor?: 'default' | 'outline' | 'danger' | 'blue',
        noPadding?: boolean,
    }

    let { 
        flavor = 'default', 
        noPadding = false
    }: Props = $props();
</script>

<div class={[`${flavorMap[flavor].name ?? ''}`, 'badge', noPadding ? 'no-padding' : '']}>
</div>

<style>
    .badge.no-padding {
        padding: 0;
    }

    .badge {
        padding: 0 0.75rem;
        height: 0.2rem;
        align-items: center;
        border-radius: 15px;
    }
    .badge.default {
        color: black;
        background-color: #e6e6e6;
    }

    .badge.secondary {
        background-color: var(--secondary-color);
    }

    .badge.danger {
        background-color: #e81506;
    }

    .badge.blue {
        background-color: #363cda;
    }
    
    .badge.outline {
        border: 1.5px solid var(--border-color);
    }
</style>