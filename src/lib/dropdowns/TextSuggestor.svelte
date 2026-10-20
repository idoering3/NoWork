<script lang='ts'>
    interface Props {
        xpos: number,
        text: string[]
        appendText: (value: string) => void;
    }

    let { 
        xpos,
        text,
        appendText,
    }: Props = $props();

    function selectText(text: string) {
        appendText(text);
    }
</script>

<div class="context-menu" style="position: absolute; left: {xpos + 25}px; bottom: 3rem;">
    {#each text as t}
        <button onclick={() => selectText(t)}>{t}</button>
    {/each}
</div>

<style>
    .context-menu {
		overflow: hidden;
		position: absolute;
		border-radius: 15px;
		background-color: var(--primary-light);
		border: 1px solid var(--border-color);
		box-shadow: 0px 0px 5px -2px #b8b8b8;
		display: flex;
		flex-direction: column;
	}

    button {
        padding: 0.5rem 1.5rem;
        font-family: "Inter", sans-serif;
        font-size: 1rem;
        border: none;
        background-color: transparent;
        color: var(--primary-dark);
        transition: 0.15s ease-in-out;
        text-align: start;
    }

    button:hover {
        background-color: var(--hover-color);
        cursor:pointer;
    }
</style>