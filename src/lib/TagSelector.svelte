<script lang='ts'>
	import { fade, fly, slide } from "svelte/transition";
	import { quartInOut } from "svelte/easing";
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { Tags, ArrowUp, Plus, X } from "@lucide/svelte";
	import Button from "./Button.svelte";
	import Textbox from "./Textbox.svelte";
	import Badge from "./Badge.svelte";
	import { flavorMap } from "./stores.svelte";
	import type { Tag } from "$lib/types/task";

	// State
	let dropdownOpen = $state(false);
	let tagColor: 'default' | 'outline' | 'danger' | 'blue' = $state('default');
	let tagName = $state('');

	// Props
	let {
		selectedTags = $bindable([] as Tag[]),
		refreshTags = $bindable(() => {}),
		allTags = $bindable([] as Tag[])
	} = $props();

	// Element Refs
	let dropdownEl: HTMLElement;

	// Component Logic
	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		loadTags();
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

	function handleKeydown(event: KeyboardEvent, tag: string) {
		if (event.key === 'Enter') {
			addTag(tag);
		}
	}

	async function loadTags() {
		try {
			const tags = (await invoke("get_all_tags")) as Tag[];
			allTags = tags;
		} catch (err) {
			console.error("Failed to load tags:", err);
		}
	}

	function addTagToTask(tag: Tag) {
		if (tag.name && !selectedTags.map(t => t.name).includes(tag.name)) {
			selectedTags = [...selectedTags, tag];
		}
	}

	async function addTag(name: string) {
		name = name.toLowerCase().trim();
		if (!name) return; // Don't add empty tags

		// Check if tag name already exists
		const tagExists = allTags.some((t) => t.name === name);

		if (!tagExists) {
			let newTag: Tag = {
				name: name,
				color: tagColor
			};
			try {
				await invoke("add_tag", { newTag });
				await loadTags(); // Refresh from DB as single source of truth
			} catch (err) {
				console.error("Failed to add tag:", err);
			}
		}
		
		tagName = ''; // Clear input
		// We don't auto-add to task here, user must click '+'
		// If you *want* to auto-add, you could call addTagToTask(newTag)
	}

	async function removeTag(tag: Tag) {
		// Filter local selected tags immediately for UI responsiveness
		selectedTags = selectedTags.filter(t => t.name !== tag.name)
		
		try {
			await invoke("remove_tag", { tagName: tag.name });
			await loadTags(); // Refresh from DB
			refreshTags(); // Refresh parent component
		} catch (err) {
			console.error("Failed to remove tag:", err)
		}
	}

	function setTagColor(name: 'default' | 'outline' | 'danger' | 'blue') {
		tagColor = name;
	}

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
		<Button class="square" flavor="outline" Icon={Tags} onclick={() => (dropdownOpen = !dropdownOpen)}>
		</Button>
		{#if dropdownOpen}
			<div class="context-menu" transition:fly={{ y: 15, easing: quartInOut, duration: 150 }}>
				<h8>Tags</h8>
				<div class="tag-container">
					{#each allTags as tag (tag.name)}
						<Badge noPadding flavor={tag.color ?? "default"}>
							<Button
								flavor="ghost"
								class="square xsmall circular"
								Icon={X}
								onclick={(event) => {
									event.stopPropagation();
									removeTag(tag);
								}}
							/>
							{tag.name}
							<Button
								flavor="ghost"
								class="square xsmall circular"
								Icon={Plus}
								onclick={(event) => {
									event.stopPropagation();
									addTagToTask(tag);
								}}
							/>
						</Badge>
					{/each}
				</div>
				<div class="task-creator">
					<Textbox
						bind:value={tagName}
						onkeydown={(event: KeyboardEvent) => handleKeydown(event, tagName)}
						{placeholders}
					/>
					<div style="display: flex;">
						<div style="border-radius: 7px; display: flex; border: 1px solid var(--border-color); overflow: hidden; margin-right:1rem;">
							{#each Object.values(flavorMap) as color}
								<button
									style="background-color: {color.bgcolor};
									width: 2rem;
									height: 2rem;
									border: none;
									"
									onclick={() => setTagColor(color.name)}
									aria-label="whatever"
									>
									{#if tagColor === color.name}
										<p transition:slide style="color: {color.color}">s</p>
									{/if}
								</button>
							{/each}
						</div>

						<Button
							class="small square"
							flavor="primary"
							Icon={ArrowUp}
							onclick={() => addTag(tagName)}
						/>
					</div>
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
		align-items: flex-start;
		justify-content: end;
		position: relative;
	}

	.context-menu {
		padding: 1rem 1.5rem;
		align-items: center;
		overflow: hidden;
		position: absolute;
		border-radius: 7px;
		background-color: var(--primary-light);
		border: 1px solid var(--border-color);
		box-shadow: 0px 0px 5px -2px #b8b8b8;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		min-width: 27rem;
		min-height: 12rem;
		bottom: 3.25rem;
		gap: 1rem;
	}
</style>