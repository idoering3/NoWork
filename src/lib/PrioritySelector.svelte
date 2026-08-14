<script lang='ts'>
    import Button from "./Button.svelte";
    import { FlagTriangleRight } from "@lucide/svelte";
    import type { TaskPriority } from "./types/task";

	let dropdownOpen = $state(false);

    interface Props {
        priority?: TaskPriority;
    }

    let { priority = $bindable(null) }: Props = $props();
</script>
<Button class="square" flavor="outline" Icon={FlagTriangleRight}>

</Button>

{#if dropdownOpen}
{/if}