<script>
    import Button from "./Button.svelte";
    import { page } from "$app/state";
    import { slide } from "svelte/transition";
    import { quartInOut } from "svelte/easing";
    import { getContext } from "svelte";

    let { children = null, href, name = 'test', ...props} = $props();

    let pageHref = $derived(page.url.pathname);

</script>

<div class="container">
    {@render children?.()}
    <Button flavor="ghost" {href} {...props}>
    </Button>
</div>

<style>
    .container {
        position:relative;
        display: flex;
        align-items: center;
    }
</style>