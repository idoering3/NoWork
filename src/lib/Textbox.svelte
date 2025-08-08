<script lang='ts'>
    let { value = $bindable(), placeholders } = $props();

    let placeholder = $state(placeholders[Math.floor(Math.random() * placeholders.length)]);

    function onInput(event: Event) {
        value = (event.target as HTMLInputElement).value;
        if (value === '') {
        // Randomize placeholder
        const randomIndex = Math.floor(Math.random() * placeholders.length);
        placeholder = placeholders[randomIndex];
        }
    }
</script>

<div class="custom-input">
    <input bind:value placeholder={`Type something, like '${placeholder}'`} oninput={onInput}/>
</div>

<style>
    .custom-input {
        width: 100%;
        height: 100%;
    }

    .custom-input input {
        width: 100%;
        height: 100%;
        border: none;
        padding: 1rem 1rem;
        font-size: 1rem;
        transition: 0.3s ease-in-out;
        box-sizing: border-box;
    }
    .custom-input input:focus {
        outline: none;
    }

    input::placeholder {
        opacity: 1;
        transition: 150ms ease-in-out;
    }

    input:focus::placeholder {
        color: var(--highlight-color);
    }
</style>