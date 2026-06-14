<script>
    import Button from "./Button.svelte";
    import { page } from "$app/state";

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