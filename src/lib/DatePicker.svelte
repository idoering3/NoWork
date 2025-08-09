<script lang='ts'>
    import { slide } from "svelte/transition";
    import Button from "./Button.svelte";
    import Calendar from "@lucide/svelte/icons/calendar";
    import { onMount } from "svelte";
    import { ArrowLeft, ArrowRight } from "@lucide/svelte";

    let dropdownOpen = $state(false);

        let dropdownEl: HTMLElement;

    function handleClickOutside(event: MouseEvent) {
        const target = event.target as HTMLElement;
        
        if (!dropdownEl.contains(target)) {
            dropdownOpen = false;
            return;
        }

        if (target.closest('button')) {
            // Clicked button inside dropdown that should NOT close it
            return; // do nothing, keep dropdown open
        }
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });

</script>

<div class='container' bind:this={dropdownEl}>
    <Button class="square" flavor="outline" Icon={Calendar} onclick={() => dropdownOpen = !dropdownOpen}/>
    {#if dropdownOpen}
        <div class='context-menu' transition:slide={{ duration: 150 }}>
            <div class="top">
                <Button flavor="ghost" class="square small rounded" Icon={ ArrowLeft } />
                <h8>
                    Date
                </h8>
                <Button flavor="ghost" class="square small rounded" Icon={ ArrowRight } />
            </div>
        </div>
    {/if}
</div>

<style>
    .top {
        display: flex;
        width: 100%;
        justify-content: space-between;
        align-items: center;
    }

    .container {
        display: flex;
        align-items:flex-start;
        justify-content: end;
        position: relative;
    }

    .context-menu {
        padding: 1rem;
        align-items: center;
        overflow: hidden;
        position: absolute;
        border-radius: 7px;
        background-color: var(--primary-color);
        border: 1px solid var(--border-color);
        box-shadow: 0px 0px 5px -2px #b8b8b8;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        width: 20rem;
        height: 20rem;
        bottom: 3.25rem;
        gap: 1rem;
    }
</style>