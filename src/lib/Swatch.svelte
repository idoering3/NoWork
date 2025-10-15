<script lang='ts'>
    import { onMount } from "svelte";
    import Card from "./Card.svelte";
    import { fly, slide } from "svelte/transition";
    import { quartInOut } from "svelte/easing";
    import Textbox from "./Textbox.svelte";

	// Use $props() to define the component's properties in Svelte 5.
	let { color = 'transparent' } = $props();

    let swatchEl: HTMLElement;

    function handleClickOutside(event: MouseEvent) {
        const target = event.target as HTMLElement;
        
        if (!swatchEl.contains(target)) {
            expanded = false;
            return;
        }

        if (target.closest('button')) {
            // Clicked button inside dropdown that should NOT close it
            return; // do nothing, keep dropdown open
        }
    }

    function switchColor () {
        expanded = !expanded;
    }

    let expanded = $state(false);
    
    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });

    const root = document.documentElement;
    let colorText = $state(color.substring(color.indexOf('--'), color.lastIndexOf(')')));
    let colorHex: string | undefined = $state();

    $effect(() => {
        if (expanded) {
            const rootStyles = getComputedStyle(root);
            colorText = color.substring(color.indexOf('--'), color.lastIndexOf(')'));
            colorHex = rootStyles.getPropertyValue(colorText);
        }
    });

    function isValidColor(colorString: string) {
    // Create a temporary element
    const style = new Option().style;
    
    // Try to set the background color
    style.backgroundColor = colorString;
    
    // Check if the browser successfully computed the color
    // A color is valid if the browser doesn't return an empty string
    return style.backgroundColor !== '';
    }

    $effect(() => {
        // check if the color is valid
        if (colorHex) {
            if (isValidColor(colorHex)) {
                root.style.setProperty(colorText, colorHex);
            }
        }
    });

</script>

<div class='container' bind:this={swatchEl}>
    <button aria-label="color-swatch" class="swatch" style:background-color={color} onclick={switchColor}>
    
    </button>
    <div class='dropdown'>
        {#if expanded}
            <div transition:fly={{ y: 15, duration: 150, easing: quartInOut }}>
                <Card>
                    <div>
                        <Textbox bind:value={colorHex} placeholders={['']}>
                            <input type="color" bind:value={colorHex}>
                        </Textbox>
                    </div>
                </Card>
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
    }

    .dropdown {
        position: absolute;
        display: flex;
        bottom: 2.5rem;
        width: 15rem;
        z-index: 50;
    }

    button {
        padding: 0;
        margin: 0;
        border: none;
    }
    button:hover {
        cursor: pointer;
    }

	.swatch {
		width: 3rem;
		height: 3rem;
	}
</style>