<script lang='ts'>
    import type { Snippet } from "svelte";
    import { quartInOut } from "svelte/easing";
    import { scale } from "svelte/transition";


    interface Props {
        flavor?: 'default' | 'outline' | 'danger',
        children: Snippet,
        noPadding?: boolean;
    }

    let { flavor ='default', children, noPadding = false }: Props = $props();

    const flavorMap = {
        default: 'default',
        danger: 'danger',
        outline: 'outline',
    }
</script>

<div transition:scale={{ duration: 150, easing: quartInOut, start: 0.75, opacity: 0 }} class={[`${flavorMap[flavor] ?? ''}`, 'badge', noPadding ? 'no-padding' : '']}>
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
    
    .badge.outline {
        background-color: var(--primary-color);
        border: 1.5px solid black;
    }
</style>