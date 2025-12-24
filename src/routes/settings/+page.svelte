<script lang='ts'>
    import AlertDialog from "$lib/AlertDialog.svelte";
    import Button from "$lib/Button.svelte";
    import Card from "$lib/Card.svelte";
    import Dropdown from "$lib/Dropdown.svelte";
    import Swatch from "$lib/Swatch.svelte";
    import { setColors, type Theme } from "$lib/theme";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from "@tauri-apps/plugin-store";
    import { theme, themes, username } from "$lib/stores.svelte";
    import { onMount } from "svelte";
    import Textbox from "$lib/Textbox.svelte";

    async function resetDatabase() {
        await invoke('reset_database');
        databaseConfirmDialogOpen = false;
        confirmDialog = null;
    }

    async function confirmResetDatabase() {
        databaseConfirmDialogOpen = true;
    }

    $effect(() => {
        if (confirmDialog === "continue") {
            resetDatabase();
        }
    });

    let databaseConfirmDialogOpen = $state(false);
    let confirmDialog = $state();
    
    let selectedTheme: string = $derived("");
     
    async function updateTheme(newThemeName: string) {
        if (newThemeName) {
            theme.theme = themes[newThemeName]
            const store = await load(".settings.json");
            await store.set("theme", { value: themes[newThemeName] });
            await store.save();
        }
    }

    $effect(() => {
        if (selectedTheme) {
            updateTheme(selectedTheme);
        }
    });

    onMount(async () => {
        if (theme.theme) {
            selectedTheme = theme.theme.name;
        } else {
            const store = await load(".settings.json");
            const value = await store.get<{ value: Theme }>("theme");

            if (value?.value) {
                theme.theme = value.value;
                selectedTheme = theme.theme.name;
            }
        }
    })

    $effect(() => {
        if(username.name) {
            updateName(username.name);
        }
    });

    async function updateName(newName: string) {
        const store = await load(".settings.json");
        await store.set("username", { value: newName });
        await store.save();
    }

</script>
<div style="padding: 3rem;">
    <h1>
        Settings
    </h1>
    <div class='stretch'>
        <div class="column">
            <div>
                <h5>
                    General
                </h5>
                <Card expanded>
                    <div style="padding: 1rem;">
                        <h6>
                            Change Name
                        </h6>
                        <p>Change the homepage's display name!</p>
                        <div style="border: 1px solid var(--border-color); border-radius: 15px;">
                            <Textbox placeholders={["Enter your name"]} preamble={false} bind:value={username.name} />
                        </div>
                    </div>
                </Card>
            </div>
            <div>
                <h5>
                    Theme
                </h5>
                <Card expanded>
                    <div style="padding: 1rem;">
                        <h6>
                           Change Theme 
                        </h6>
                        <p>
                            Choose between a preselected theme or make your own!
                        </p>
                        <Dropdown options={Object.keys(themes)} bind:selected={selectedTheme}/>
                        <p>
                            Theme colors
                        </p>
                        <div class='swatches'>
                            <Swatch color={"var(--primary-light)"}/>
                            <Swatch color={"var(--primary-dark)"}/>
                            <Swatch color={"var(--hover-primary-dark)"}/>
                            <Swatch color={"var(--primary-color)"}/>
                            <Swatch color={"var(--secondary-color)"}/>
                            <Swatch color={"var(--highlight-color)"}/>
                            <Swatch color={"var(--border-color)"}/>
                            <Swatch color={"var(--hover-color)"}/>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
        <div style="margin-top: 2rem;">
            <h5>
                Tasks
            </h5>
            <Card expanded>
                <div style="padding: 1rem;">
                    <div class='stretch'>
                        <AlertDialog 
                            bind:open={databaseConfirmDialogOpen}
                            bind:result={confirmDialog}
                            title="Are you absolutely sure?"
                            message="This action cannot be undone. This will permanently delete both completed and incompleted tasks you've created, as well as any tags."
                        />
                        <h6>
                            Reset Database
                        </h6>
                        <p>
                            This removes <span class='red'>all</span> tasks, including completed and incompleted tasks as well as any created tags!
                        </p>
                        <div class='reset-database'>
                            <Button flavor="danger" class='rounded border' onclick={confirmResetDatabase}>
                                Reset Database
                            </Button>
                        </div>
                    </div>
                </div>
            </Card>
        </div>
    </div>
</div>

<style>
    .column {
        display: flex;
        gap: 2rem;
    }
    .stretch {
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    h5 {
        padding: 1rem 0;
    }
    h6 {
        padding-bottom: 0.5rem;
    }
    p {
        padding-bottom: 1rem;
    }

    .red {
        color: #D64540;
    }

    .reset-database {
        width: 10rem;
    }

    .swatches {
        display: flex;
        border: 1px solid var(--border-color);
        width: fit-content;
    }

</style>