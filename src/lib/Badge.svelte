<script lang='ts'>
    import type { Snippet } from "svelte";
    import { quartOut } from "svelte/easing";
    import { scale } from "svelte/transition";
    import { flavorMap } from "./stores.svelte";
    import type { TagColor } from "./types/task";

    interface Props {
        flavor?: TagColor,
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
    style="
        --badge-bg: {flavorMap[flavor].bgcolor};
        --badge-color: {flavorMap[flavor].color};
        --badge-border: {flavorMap[flavor].border ?? 'none'};
    "
    class={[flavorMap[flavor].name, 'badge']}
>
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
        color: var(--badge-color);
        border: var(--badge-border);
    }
</style>