<script>
    import Logo from "$lib/Logo.svelte";
    import WindowButtons from "./WindowButtons.svelte";


</script>

<header data-tauri-drag-region>
    <navbar data-tauri-drag-region>
        <Logo/>
    </navbar>
    <div data-tauri-drag-region>
        <p data-tauri-drag-region>NoWork</p>
    </div>
    <aside>
        <WindowButtons />
    </aside>
</header>

<style>
    navbar {
        display: flex;
        vertical-align: center;
        align-items: center;
        align-content: center;
        margin: 0;
    }
    header {
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