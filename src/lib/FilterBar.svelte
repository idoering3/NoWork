<script lang='ts'>
    import { CircleSmall, ListFilter } from "@lucide/svelte";
    import BadgeButton from "./BadgeButton.svelte";
    import type { TaskFilter } from "./types/filter";
  import type { Tag, TaskPriority } from "./types/task";
  import { flavorMap } from "./stores.svelte";

    interface Props {
        filter: TaskFilter;
        tags: Tag[];
        saveFilter: (filter: TaskFilter
        ) => void;
    }

    let { saveFilter = $bindable(), tags = $bindable(), filter = $bindable()}: Props = $props();

    async function toggleFilterTag(tag: Tag) {
        if (filter.tags.some(selected => selected.id === tag.id)) {
            filter.tags = filter.tags.filter(t => t.id !== tag.id);
        } else {
            filter.tags = [...filter.tags, tag];
        }

        await saveFilter(filter);
    }

    async function toggleFilterPriority(priority: TaskPriority) {
        if (filter.priorities.includes(priority)) {
            filter.priorities = filter.priorities.filter(p => p !== priority);
        } else {
            filter.priorities = [...filter.priorities, priority];
        }

        await saveFilter(filter);
    }

    async function clearFilterTags() {
        filter.tags = [];
        filter.priorities = [];

        await saveFilter(filter);
    }
</script>

<div class="filter-container">
    <div class="filter-container-child">
        {#each tags as tag}
            <BadgeButton 
                onClick={() => toggleFilterTag(tag)}
                style={filter.tags.some(selected => selected.id === tag.id) ? `
                    border: 1px solid ${flavorMap[tag.color].bgcolor};
                    background-color: color-mix(in srgb, var(--primary-dark), transparent 90%);
                ` : ""}
            >
                <CircleSmall size={14} strokeWidth={1.1} color={flavorMap[tag.color].bgcolor} fill={flavorMap[tag.color].bgcolor} />
                {tag.name}
            </BadgeButton>
        {/each}
        

        <!-- DO ALL THE PRIORITIES HERE -->
        <div class="priority-button-group">
            <BadgeButton
                onClick={() => toggleFilterPriority("high")}
                style={`
                    border-radius: 25px 0px 0px 25px;
                    ${filter.priorities.includes("high") ? `
                        background-color: color-mix(in srgb, #D64540, transparent 80%);
                    ` : ""}
                `}
            >
                <CircleSmall size={14} strokeWidth={1.1} color="#D64540" fill="#D64540" />
                High
            </BadgeButton>
            <BadgeButton
                style={`
                    border-radius: 0px; border-style: solid transparent; border-width: 1px;
                    ${filter.priorities.includes("medium") ? `
                        border-style: solid;
                        background-color: color-mix(in srgb, 
                        orange, transparent 80%);
                    ` : ""}
                `}
                onClick={() => toggleFilterPriority("medium")}
            >
                <CircleSmall size={14} strokeWidth={1.1} color="orange" fill="orange" />
                Med
            </BadgeButton>
            <BadgeButton
                style={`
                    border-radius: 0px 25px 25px 0px;
                    ${filter.priorities.includes("low") ? `
                        background-color: color-mix(in srgb, 
                        #86e28c, transparent 80%);
                    ` : ""}
                `}
                onClick={() => toggleFilterPriority("low")}
            >
                <CircleSmall size={14} strokeWidth={1.1} color={flavorMap['green'].bgcolor} fill={flavorMap['green'].bgcolor} />
                Low
            </BadgeButton>
        </div>
        {#if filter.tags.length > 0 || filter.priorities.length > 0}
            <BadgeButton 
                style="color: var(--faded-text)"
                onClick={() => clearFilterTags()}
            >
                Clear
            </BadgeButton>
        {/if}
    </div>
    <!-- RIGHT FILTER (DETAILED FILTER) -->
        <div class="filter-container-child">
        <BadgeButton
            onClick={() => console.log("")}
        >
            <ListFilter size={14}/>
            Filters
        </BadgeButton>
        </div>
</div>


<style>
    .filter-container-child {
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
        margin: 1rem 0rem;
    }

    .filter-container {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .priority-button-group {
        display: flex;
        border-radius: 25px;
        overflow: hidden;
        margin-left: 1rem;
    }
</style>