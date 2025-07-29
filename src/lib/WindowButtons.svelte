<script>
    import { Minus, Square, X } from "@lucide/svelte";
    import Button from "./Button.svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let appWindow = getCurrentWindow();

    let isMaximized = false;

    function closeApplication ()
    {
        appWindow.close();
    }
    function minimizeApplication ()
    {
        appWindow.minimize();
    }
    function toggleFullscreenApplication ()
    {
        appWindow.toggleMaximize();
    }
</script>

<header>
    <Button flavor="ghost" Icon={Minus} onclick={minimizeApplication}/>
    <Button flavor="ghost" Icon={Square} iconSize={14} onclick={toggleFullscreenApplication}/>
    <Button Icon={X} flavor="danger" onclick={closeApplication}/>
</header>

<style>
    header {
        display: flex;
    }
</style>