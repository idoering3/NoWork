<script lang='ts'>
    import type { TaskModification } from "$lib/tasks/taskParser";
    import { onMount, type Snippet } from "svelte";

    interface Props {
        value?: string,
        modifications: TaskModification[],
        placeholders: string[],
        preamble?: boolean,
        onkeydown?: ((event: KeyboardEvent) => void) | undefined;
        children?: Snippet
    }

    let { value = $bindable(), placeholders, modifications, preamble = true, onkeydown, children }: Props = $props();

    let placeholder = $derived(placeholders[Math.floor(Math.random() * placeholders.length)]);
    onMount(() => {
        if (preamble) {
            placeholder = `Type something, like '${placeholder}'`; 
        }
    })

    function onInput() {
        if (value === '') {
        // Randomize placeholder
        const randomIndex = Math.floor(Math.random() * placeholders.length);
        placeholder = placeholders[randomIndex];
            if (preamble) {
                placeholder = `Type something, like '${placeholder}'`; 
            }
        }
    }

    interface textIndex {
        index: number;
        isStart: boolean;
    }

    let textIndices: textIndex[] = $derived(
        [
            ...modifications.flatMap(
                mod => [
                    {
                        index: mod.start,
                        isStart: true
                    }, 
                    {
                        index: mod.end,
                        isStart: false
                    }
            ]).sort((a, b) => a.index - b.index)
        ]
    );

    interface textIndexPair {
        text: string;
        isMod: boolean;
    }

    function chunkText(text: string, textIndices: textIndex[]): textIndexPair[] {
        let final: textIndexPair[] = [];

        if (textIndices.length === 0) {
            return [{
                text,
                isMod: false
            }];
        }

        // we want a slice from zero to first index if not one
        if(textIndices[0].index !== 0) {
            final.push({
                text: text.slice(0, textIndices[0].index),
                isMod: false
            });
        }

        for(let i = 1; i < textIndices.length; i++) {
            // we create a pair (start, end) to use for slicing later
            const chunk = [textIndices[i - 1].index, textIndices[i].index];
            let s = text.slice(chunk[0], chunk[1]);
            let isMod = false;
            if (textIndices[i - 1].isStart && !textIndices[i].isStart) {
                isMod = true;
            }
            // create the new textIndexPair
            const textIndexPair = {
                text: s,
                isMod: isMod
            };
            final.push(textIndexPair);
        }
        // one more push for the end
        final.push({
            text: text.slice(textIndices[textIndices.length - 1].index),
            isMod: false
        });

        // this will have the form string, isMod
        return final;
    }

    let indexPairs = $derived.by<textIndexPair[]>(() => chunkText(value ? value: "", textIndices)); 

</script>


<div class="custom-input">
    <input autocomplete="off" id='input' class='input' bind:value placeholder={placeholder} oninput={onInput} onkeydown={onkeydown}/>
    <div class='absolutely'>
        {#each indexPairs as chunk}
            {#if chunk.isMod}
                <span class='mod'>{chunk.text}</span>
            {:else}
                <span>{chunk.text}</span>
            {/if}
        {/each}
    </div>
</div>

<style>

    .absolutely {
        position: absolute;
        inset: 0;
        display: flex;
        align-items: center;
        overflow: hidden;
        white-space: pre;
        pointer-events: none;
        padding: 1rem;
    }

    .custom-input {
        position: relative;
        gap: 0.5rem;
        width: 100%;
        height: 100%;
    }

    .custom-input input {
        font: inherit;
        background: transparent;
        z-index: 50;
        width: 100%;
        height: 100%;
        border: none;
        padding: 1rem 1rem;
        font-size: 1rem;
        transition: 0.3s ease-in-out;
        box-sizing: border-box;
        color: transparent;
        caret-color: var(--primary-dark);
    }
    .custom-input input:focus {
        outline: none;
    }

    input::placeholder {
        opacity: 1;
        transition: 150ms ease-in-out;
        color: var(--hover-primary-dark);
    }

    input:focus::placeholder {
        color: var(--highlight-color);
    }

    .mod {
        color: var(--highlight-color);
    }
</style>