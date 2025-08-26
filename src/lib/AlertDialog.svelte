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
        <div transition:scale={{ duration: 150, easing: quartInOut, start: 0.75, opacity: 0 }}>
            <Card>
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
            </Card>
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

    .options {
        display: flex;
        justify-content: end;
        gap: 0.5rem;
    }
    .container {
        top: 0;
        left: 0;
        position: absolute;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: color-mix(in srgb, var(--primary-dark), transparent 50%);
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