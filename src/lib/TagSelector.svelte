<script lang='ts'>
    import { slide } from "svelte/transition";
    import Button from "./Button.svelte";
    import Tags from "@lucide/svelte/icons/tags";
    import { quartInOut } from "svelte/easing";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Textbox from "./Textbox.svelte";
    import { ArrowUp, Plus, X } from "@lucide/svelte";
    import Badge from "./Badge.svelte";

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

    function handleKeydown(event: KeyboardEvent, tag: string) {
        if (event.key === 'Enter') {
            addTag(tag);
            tagName = '';
        }
    }


    let { selectedTags = $bindable([]) } = $props();

    let allTags: string[] = $state([]);
    let search = "";

    // Fetch tags from Tauri backend
    async function loadTags() {
        try {
            const tags = await invoke("get_all_tags") as { id: number, name: string }[];
            allTags = tags.map(t => t.name);
        } catch (err) {
            console.error("Failed to load tags:", err);
        }
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        loadTags();
        return () => document.removeEventListener('click', handleClickOutside);
    });

    function addTagToTask(tag:string) {
        if (tag && !selectedTags.includes(tag)) {
            selectedTags = [...selectedTags, tag];
        }
    }

    async function addTag(tag: string) {
        tag = tag.toLowerCase();
        tag = tag.trim();
            if (tag && !selectedTags.includes(tag)) {
                if (!allTags.includes(tag)) {
                    allTags = [...allTags, tag];
                    await invoke("add_tag", { tagName: tag });
                    await loadTags();
                }
            }
        search = "";
    }

    async function removeTag(tag: string) {
        allTags = allTags.filter(t => t !== tag);
        selectedTags = selectedTags.filter(t => t !== tag);
        await invoke("remove_tag", { tagName: tag });
        await loadTags();
    }

    let tagName = $state('');

    const placeholders = [
        'home',
        'work',
        'study',
        'help me!',
        'car',
        'bills',
    ];
</script>


<div class="container">
    <div class="dropdown-container" bind:this={dropdownEl}>
        <Button class="square" flavor="outline" Icon={Tags} onclick={() => dropdownOpen = !dropdownOpen}>
        </Button>
        {#if dropdownOpen}
            <div class="context-menu" transition:slide={{ duration: 150 }}>
                <h8>Tags</h8>
                <div class="tag-container">
                    {#each allTags as tag}
                        <Badge noPadding flavor="outline">
                            <Button flavor="ghost" class="square xsmall circular" Icon={X} 
                            onclick={(event) => {
                                event.stopPropagation(); 
                                removeTag(tag);
                            }}
                            />
                                {tag}
                            <Button flavor="ghost" class="square xsmall circular" Icon={Plus}
                                onclick={(event) => {
                                    event.stopPropagation();
                                    addTagToTask(tag);
                                }}    
                            />
                        </Badge>
                    {/each}
                </div>
                <div class='task-creator'>
                    <Textbox bind:value={tagName} onkeydown={(event: KeyboardEvent) => handleKeydown(event, tagName)} {placeholders} />
                    <Button class="small square" flavor="primary" Icon={ArrowUp} onclick={() => addTag(tagName)} />
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    .tag-container {
        display: flex;
        flex-wrap: wrap;
        gap: 0.25rem;
        justify-content: center;
    }

    .task-creator {
        border-radius: 7px;
        align-items: center;
        padding: 0 0.5rem;
        display: flex;
        width: 100%;
        justify-content: space-between;
        border: 1px solid var(--border-color);
    }

    .dropdown-container {
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
        min-width: 20rem;
        min-height: 12rem;
        bottom: 3.25rem;
        gap: 1rem;
    }
</style>