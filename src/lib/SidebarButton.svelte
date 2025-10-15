<script>
    import Button from "./Button.svelte";
    import { page } from "$app/state";
    import { slide } from "svelte/transition";
    import { quartInOut } from "svelte/easing";
    import { getContext } from "svelte";

    let { href, name = 'test', ...props} = $props();

    let pageHref = $derived(page.url.pathname);

    let sidebarExpanded = getContext("sidebarExpanded");
</script>

<div class="container">
    {#if pageHref === href}
        <div transition:slide={{ duration: 300, easing: quartInOut }} class="selected-bar"></div>
    {/if}
    <Button flavor="ghost" {href} {...props} expanded={sidebarExpanded.state}>
        {#if sidebarExpanded.state}
            {name}
        {/if}
    </Button>
</div>

<style>
    .container {
        position:relative;
        display: flex;
        align-items: center;
    }
    .selected-bar {
        position: absolute;
        z-index: 1;
        background-color: var(--highlight-color);
        height: 1.75rem;
        width: 3px;
        margin-left: 4px;
        border-radius: 7px;
    }
</style>