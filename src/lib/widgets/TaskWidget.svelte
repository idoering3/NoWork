<script>
  import { quartOut } from "svelte/easing";
  import { fly } from "svelte/transition";

</script>
<div class="container" in:fly|global={{ y: 30, delay: 600, duration: 1500, easing: quartOut}}>


</div>

<style>
    .container {
        border: 1px solid var(--border-color);
        margin: 0.5rem;
        border-radius: 15px;
        background-color: var(--primary-light);
        width: 100%;
        display: flex;
        gap: 0.5rem;
        flex-direction: column;
        height: 100%;
    }
</style>