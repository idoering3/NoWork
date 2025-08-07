<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { sineInOut } from "svelte/easing";
    import { fly } from "svelte/transition";

    let userName = 'Ian';
    let message = $state();

    onMount (async () => {
        message = await invoke( 'greet' );
    });
</script>


<h1>Welcome, {userName}!</h1>
<h6 transition:fly={{ y: 25, delay: 150, duration: 1000, easing: sineInOut }}>{message}</h6>