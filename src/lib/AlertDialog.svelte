<script>
    import { scale } from "svelte/transition";
    import Button from "./Button.svelte";
    import Card from "./Card.svelte";
    import { quartInOut } from "svelte/easing";

    let {title, message, affirmation = "Continue", negative = "Cancel", result = $bindable(), open = $bindable(false)} = $props();
    
	function cancel() {
		result = "cancel";
		open = false;
	}

	function confirm() {
		result = "continue";
		open = false;
	}
</script>

{#if open}
    <div class="container">
        <div transition:scale={{ duration: 75, easing: quartInOut, start: 0.75, opacity: 0 }}>
            <div class="mica-card">
                <div class="card">
                    <div>
                        <h6>
                            {title}
                        </h6>
                        <p>
                            {message}
                        </p>
                    </div>
                    <div class='options'>
                        <div class='button'>
                            <Button flavor="outline" onclick={cancel}>{negative}</Button>
                        </div>
                        <div class='button'>
                            <Button flavor="primary" onclick={confirm}>{affirmation}</Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    h6 {
        padding-bottom: 0.5rem;
    }

    p {
        padding-bottom: 1rem;
        color: var(--primary-dark);
    }

    .mica-card {
        padding: 1rem;
        position: relative;
        overflow: hidden;
        box-shadow: 0px 0px 5px -2px var(--border-color);
        background: 
        linear-gradient(
            190deg,
            color-mix(in srgb, var(--accent-color) 5%, transparent),
            transparent 60%
        ),
        color-mix(
            in srgb,
            var(--primary-light) 75%,
            transparent
        );
        border-radius: 15px;
        border: 1px solid color-mix(
            in srgb,
            var(--border-color) 50%,
            transparent
        );
        width: fit-content;
        box-shadow:
            0 1px 2px rgb(0 0 0 / 4%),
            0 8px 24px rgb(0 0 0 / 6%);
        
        backdrop-filter: blur(20px);
        display: flex;
        flex-direction: column;
        align-content: center;
        gap: 0.5rem;
    }

    .options {
        display: flex;
        justify-content: end;
        gap: 0.5rem;
    }
    .container {
        z-index: 1000;
        top: -3rem;
        left: 0;
        position: absolute;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .card {
        padding: 1rem;
        width: 30rem;
        height: 10rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
    .button {
        width: 6rem;
    }
</style>