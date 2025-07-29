<script>
    import '../app.css';
    import Header from '$lib/Header.svelte';
    import Sidebar from '$lib/Sidebar.svelte';
    import { page } from '$app/state';
    import { fade, fly } from 'svelte/transition';
    import { circInOut, quartInOut } from 'svelte/easing';
    function toggleSidebar () {
        const sidebar = document.getElementById("navbar-toggle");
        sidebar?.classList.toggle("collapsed");
        sidebar?.classList.toggle("expanded");
    }

    let { children } = $props();
</script>



<div class="grid">
    <Header />
    <main>
        <Sidebar />
        <div class="main">
            {#key page.url.pathname}
                <content in:fly={{ duration: 300, y: 150, easing: circInOut}} out:fade={{ duration: 150, easing: quartInOut}}>
                    {@render children?.()}
                </content>
            {/key}
        </div>
    </main>
</div>




<style>
    .grid {
        display: flex;
        flex-wrap: wrap;
        background-color: var(--secondary-color);
        height: 100%;
        width: 100%;
    }
    /* I KNOW it's a bad name! */
    .main {
        display: grid;
        overflow: hidden;
        width: 100%;
        background-color: var(--primary-color);
        border-radius: 7px 0 0 0;
        border-top: 1px solid var(--border-color);
        border-left: 1px solid var(--border-color);
        padding: 3rem;
        display: grid;
    }
    main {
        margin-top: 3rem;
        display: flex;
        width: 100%;
    }
    content {
        width: 100%;
        height: 100%;
        grid-column-start: 1;
        grid-column-end: 2;
        grid-row-start: 1;
        grid-row-end: 2;
    }
</style>