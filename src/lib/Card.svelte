<script>

    let { expanded = false, children = null, ...props } = $props();
</script>

<div class={[expanded ? "expanded" : "", props.class]} {...props}>
    {#if children}
        {@render children?.()}
    {/if}
</div>

<style>
    .evenly-space-vert {
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
    }

    div {
        box-shadow: 0px 0px 5px -2px #b8b8b8;
        border: 1px solid #b8b8b8;
        border-radius: 15px;
        padding: 1rem;
        margin: 1rem;
        width: 25rem;
        min-height: 25rem;
        display: flex;
        flex-direction: column;
        align-content: center;
        gap: 0.5rem;
    }

    div.expanded {
        width: auto;
    }
</style>