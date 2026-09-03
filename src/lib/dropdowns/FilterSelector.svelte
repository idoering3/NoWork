<script lang='ts'>
	import { DateFilter } from "$lib/misc/dateFilter";
	import { quartInOut } from "svelte/easing";
	import { fly } from "svelte/transition";

	interface Props {
		dateFilter: DateFilter;
		onchange: (filter: DateFilter) => void;
	}

	let { dateFilter, onchange }: Props = $props();

    function changeDateFilter(filter: DateFilter) {
        onchange(filter === dateFilter ? DateFilter.None : filter);
    }
</script>

<div class="container">
    <div class="context-menu" transition:fly={{ duration: 150, y:-15, easing: quartInOut }}>
        <h5 style="color: var(--faded-text)">Due date</h5>
		<div class="button-holder">
			<button 
				class="due-button"
				class:selected={dateFilter === DateFilter.Today}
				style="border-radius: 0px 0px 0 0; border-bottom: 1px solid var(--border-color);"
				onclick={() => changeDateFilter(DateFilter.Today)}
			>
				Due Today
			</button>
			<button 
				class="due-button"
				class:selected={dateFilter === DateFilter.ThisWeek}
				onclick={() => changeDateFilter(DateFilter.ThisWeek)}
			>
				Due This Week
			</button>
			<button 
				class="due-button"
				class:selected={dateFilter  === DateFilter.NotDue}
				onclick={() => changeDateFilter(DateFilter.NotDue)}
				style="border-top: 1px solid var(--border-color)"
			>
				No Due Date
			</button>
		</div>
    </div>
</div>


<style>
	.due-button {
		background-color: transparent;
		border: none;
		border-color: var(--border-color);
		color: var(--primary-dark);
		height: 3rem;
		transition: 150ms ease-in-out;
	}

	.button-holder {
		border: 1px solid var(--border-color);
		display: flex;
		flex-direction: column;
		border-radius: 15px;
		overflow: hidden;
	}

	.due-button.selected {
		background-color: var(--hover-color);
	}

	.due-button:hover {
		color: var(--highlight-color);
		cursor: pointer;
	}

    .container {
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
		border-radius: 15px;
		background-color: var(--primary-light);
		border: 1px solid var(--border-color);
		box-shadow: 0px 0px 5px -2px #b8b8b8;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		min-width: 8rem;
		min-height: 12rem;
		top: 0.5rem;
		gap: 1rem;
        z-index: 50;
	}
</style>