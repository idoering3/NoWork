<script lang='ts'>
    import Button from "./Button.svelte";
    import { CircleSmall, FlagTriangleRight } from "@lucide/svelte";
    import type { TaskPriority } from "./types/task";
    import { onMount } from "svelte";
  import { quartInOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import Badge from "./Badge.svelte";
  import { flavorMap } from "./stores.svelte";
  import { getPriorityColor } from "./misc/priority";

	let dropdownOpen = $state(false);
    
    interface Props {
        priority?: TaskPriority;
        size?: 'small' | 'normal'
    }
    
    let { priority = $bindable(null), size="normal" }: Props = $props();
    
    let yTransitionAmt = $derived(size === "small" ? -15 : 15);
	let dropdownEl: HTMLElement;

	// Component Logic
	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	});

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;

		if (!dropdownEl.contains(target)) {
			dropdownOpen = false;
			return;
		}

		if (target.closest('button')) {
			return; // Clicked button inside dropdown, do nothing
		}
	}
</script>
<div bind:this={dropdownEl} class="dropdown-container">
    <Button class={size === "normal" ? "square" : "square small"} flavor="outline" onclick={() => dropdownOpen =!dropdownOpen}>
        {#if priority}
            <CircleSmall size={14} strokeWidth={1.1} color={getPriorityColor(priority)} fill={getPriorityColor(priority)}/>
        {:else}
            <CircleSmall size={14} strokeWidth={1.1}/>
        {/if}
    </Button>
    {#if dropdownOpen}
        <div class="dropdown-content" class:small={size === "small"} transition:fly={{ y: yTransitionAmt, easing: quartInOut, duration: 150 }}>
            {#if size === "normal"}
                <Button class="square" flavor="ghost" onclick={() => priority = "high"}>
                    <CircleSmall size={14} strokeWidth={1.1} color="#D64540" fill="#D64540"  />
                </Button>
                <Button class="square" flavor="ghost" onclick={() => priority = "medium"}>
                    <CircleSmall size={14} strokeWidth={1.1} color="orange" fill="orange" />
                </Button>
                <Button class="square" flavor="ghost" onclick={() => priority = "low"}>
                    <CircleSmall size={14} strokeWidth={1.1} color={flavorMap['green'].bgcolor} fill={flavorMap['green'].bgcolor} />
                </Button>
                <Button class="square" flavor="ghost" onclick={() => priority = null}>
                    <CircleSmall size={14} strokeWidth={1.1}/>
                </Button>
            {:else if size === "small"}
                <Button class="square small" flavor="ghost" onclick={() => priority = null}>
                    <CircleSmall size={14} strokeWidth={1.1}/>
                </Button>
                <Button class="square small" flavor="ghost" onclick={() => priority = "low"}>
                    <CircleSmall size={14} strokeWidth={1.1} color={flavorMap['green'].bgcolor} fill={flavorMap['green'].bgcolor} />
                </Button>
                <Button class="square small" flavor="ghost" onclick={() => priority = "medium"}>
                    <CircleSmall size={14} strokeWidth={1.1} color="orange" fill="orange" />
                </Button>
                <Button class="square small" flavor="ghost" onclick={() => priority = "high"}>
                    <CircleSmall size={14} strokeWidth={1.1} color="#D64540" fill="#D64540"  />
                </Button>
            {/if}
        </div>
    {/if}
</div>

<style>
    .dropdown-content {
		align-items: center;
		overflow: hidden;
		position: absolute;
		border-radius: 15px;
		background-color: var(--primary-light);
		border: 1px solid var(--border-color);
		box-shadow: 0px 0px 5px -2px #b8b8b8;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		bottom: 3.25rem;
    }

    .dropdown-content.small {
        top: 2.25rem;
        bottom:auto;
    }

    .dropdown-container {
        display: flex;
		align-items: flex-start;
		justify-content: end;
		position: relative;
    }
</style>