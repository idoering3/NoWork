<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let userName = 'Ian';
    let message = $state();

    onMount (async () => {
        message = await invoke( 'greet' );
    });
</script>


<h1>Welcome, {userName}!</h1>
<h6>{message}</h6>