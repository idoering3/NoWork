<script lang='ts'>
    import type { Snippet } from "svelte";

    interface Props {
        value?: string,
        placeholders: string[],
        preamble?: string,
        onkeydown?: ((event: KeyboardEvent) => void) | undefined;
        children?: Snippet
    }

    let { value = $bindable(), placeholders, preamble = "Type something, like ", onkeydown, children }: Props = $props();

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
    <input class='input' bind:value placeholder={preamble + placeholder} oninput={onInput} onkeydown={onkeydown}/>
    <div class='absolutely'>
        {@render children?.()}
    </div>
</div>

<style>

    .absolutely {
        position: absolute;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .custom-input {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: end;
        gap: 0.5rem;
        width: 100%;
        height: 100%;
    }

    .custom-input input {
        background: transparent;
        z-index: 50;
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