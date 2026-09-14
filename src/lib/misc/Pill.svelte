<script lang='ts'>

    let { text } = $props();

</script>

<div>
    {text}
</div>

<style>
    div {
        border-radius: 50%;
        background-color: var(--primary-dark);
        color: var(--primary-light);
        width: 1rem;
        height: 1rem;
        text-align: center;
        display: flex;
        justify-content: center;
        align-items: center;
        border: none;
        font-size: 0.7rem;
    }
</style>