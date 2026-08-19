<script lang='ts'>

    let { children, onClick, selected = $bindable(), ...others } = $props();

</script>

<button class="badge-button {others.class}" style={others.style} onclick={onClick}>
    {@render children()}
</button>

<style>
    .badge-button {
        font-size: 0.8rem;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0 0.5rem;
        height: 1.75rem;
        border-radius: 25px;
        background-color: transparent;
        border: 1px solid var(--border-color);
        color: var(--primary-dark);
        gap: 0.25rem;
        transition: 150ms ease-in-out;
    }

    .badge-button:hover {
        color: var(--highlight-color) !important;
    }

    .badge-button:hover {
        cursor: pointer;
    }
</style>