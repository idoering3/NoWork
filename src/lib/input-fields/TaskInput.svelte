<script lang='ts'>
  import Badge from "$lib/Badge.svelte";
  import Button from "$lib/Button.svelte";
  import Card from "$lib/Card.svelte";
  import DatePicker from "$lib/dropdowns/DatePicker.svelte";
  import PrioritySelector from "$lib/dropdowns/PrioritySelector.svelte";
  import TagSelector from "$lib/dropdowns/TagSelector.svelte";
  import { quartInOut, quartOut } from "svelte/easing";
  import Textbox from "./Textbox.svelte";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { ArrowUp, X } from "@lucide/svelte";
  import { submitTask, taskPriorityOptions, type CreateTask, type Task } from "$lib/types/task";
  import { getAllTags } from "$lib/stores.svelte";


    interface Props {
        show: boolean;
        taskBar: HTMLDivElement | undefined;
    }

    let placeholders = [
        'steal grandma\'s bagel',
        'read War and Peace',
        'get absolutely wasted on a Tuesday morning',
        'scroll on social media for 6 hours',
        'add an item to my task list',
        'make a sad peanut butter jelly sandwich',
        'cry myself to sleep in a fetal position',
        'sigh heavily and gaze forlornly out the window',
        'talk to myself like a crazy person for an hour',
        'debate with my coffee whether it\'s time to quit or keep going',
        'become a conspiracy theorist',
        'run laps in the swivel chair around the cubicle',
        'make a playlist called \'songs to pretend you\'re working to\'',
        'try drinking a glass of milk while updside down',
        'bring a fishing pole to the aquarium',
        'call John and tell him I can\'t talk right now',
        'drive around in a clown costume like the clown I am',
        'streak through the streets, shouting Eureka!',
        'write a strongly worded email',
        'impersonate a drunk Lyndon B. Johnson looking for his car keys',
        'delete those incriminating Watergate tapes',
        'go for a run',
        'pretend to do your work'
    ]

    let proposedTask = $state<CreateTask>(
        // proposed task must always exist in some form, even if vals set to nothing
        {
            name: "",
            dueDate: null,
            tags: [],
            priority: null
        }
    );

    let { show, taskBar }: Props = $props();

    async function removeDate() {
        proposedTask.dueDate = null;
    }

    function removeTagFromTask(tag: string) {
        proposedTask.tags = proposedTask.tags.filter(t => t.name !== tag);
    }

    async function trySubmitTask() {
        submitTask(proposedTask);
        resetProposedTask();
    }

    function resetProposedTask() {
        proposedTask = {
            name: "",
            dueDate: null,
            tags: [],
            priority: null
        };
    }

</script>

{#if show}
    <div class="task-bar" bind:this={taskBar} in:fly|global={{ duration: 1500, delay:600, y:15, easing: quartOut }}>
        <Card expanded class="short">
            <Textbox bind:value={proposedTask.name} {placeholders} />
            {#snippet tagsn(name: string, color: 'default' | 'outline' | 'danger' | 'blue')}
                <Badge flavor={color} noPadding>
                    <span style="padding-left: 0.5rem">
                        {name}
                    </span>
                    <Button flavor="badge" class="square xsmall circular" Icon={X}
                        onclick={() => {
                            removeTagFromTask(name);
                        }}
                    />
                </Badge>
            {/snippet}
            {#key proposedTask.tags.length}
                <div
                    style="display: flex;"
                >
                {#each proposedTask.tags as tag (tag.name)}
                    <div animate:flip|global={{ duration: 300, easing: quartInOut }} style="padding: 0.25rem;">
                        <div style=""
                        >
                            {@render tagsn(tag.name, tag?.color)}
                        </div>
                    </div>
                {/each}
                </div>
            {/key}
            {#if proposedTask.dueDate !== null}
                {proposedTask.dueDate.toLocaleDateString()}
                <Button class="square xsmall" Icon={X} flavor='outline' onclick={removeDate}/>
            {/if}
            <PrioritySelector bind:priority={proposedTask.priority}/>
            <TagSelector bind:selectedTags={proposedTask.tags} />
            <DatePicker bind:selectedDate={proposedTask.dueDate}/>
            <div in:fly|global={{ duration: 1500, delay:800, y:5, easing: quartOut }}>
                <Button onclick={trySubmitTask} class="square circular" flavor="primary" Icon={ArrowUp} />
            </div>
        </Card>
    </div>
{/if}