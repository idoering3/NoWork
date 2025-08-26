<script lang='ts'>
    import Logo from "$lib/Logo.svelte";
    import WindowButtons from "./WindowButtons.svelte";
    
</script>

<div class="header" data-tauri-drag-region>
    <navbar data-tauri-drag-region>
        <Logo/>
    </navbar>
    <div data-tauri-drag-region>
        <p id="gradient-text" data-tauri-drag-region>NoWork</p>
    </div>
    <aside>
        <WindowButtons />
    </aside>
</div>

<style>
    #gradient-text {
        font-weight: bold;
        display: inline-block;
        background: linear-gradient(270deg, var(--primary-dark), #8d6577, var(--highlight-color));
        background-size: 600% 600%; /* bigger than text for smooth animation */
        -webkit-text-fill-color: transparent;
        animation: move-gradient 6s ease infinite;
        background-clip: text;
    }

    @keyframes move-gradient {
        0% { background-position: 0% 50%; }
        50% { background-position: 100% 50%; }
        100% { background-position: 0% 50%; }
    }

    navbar {
        display: flex;
        vertical-align: center;
        align-items: center;
        align-content: center;
        margin: 0;
    }
    .header {
        user-select: none;
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 3rem;
        position: fixed;
        top: 0;
        width: 100%;
    }
    p {
        font-family: 'Playfair Display Italic', sans-serif;
    }
</style>