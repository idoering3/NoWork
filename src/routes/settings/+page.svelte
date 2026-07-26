<script lang='ts'>
    import AlertDialog from "$lib/AlertDialog.svelte";
    import Button from "$lib/Button.svelte";
    import Dropdown from "$lib/Dropdown.svelte";
    import Swatch from "$lib/Swatch.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { load } from "@tauri-apps/plugin-store";
    import { getCalendarNumHours, getCalendarStartTime, selectedDateFormat, theme, themes, updateCalendarNumHours, updateCalendarStartTime, username, type ThemeName } from "$lib/stores.svelte";
    import { onMount } from "svelte";
    import Textbox from "$lib/Textbox.svelte";
    import { fly } from "svelte/transition";
    import { quartOut } from "svelte/easing";
    import { dateFormatOptions, type DateFormatName } from "$lib/misc/datePrints";
    import { loadCredentials, saveCredentials } from "$lib/cal/calendarCredentialStorage";
    import { setPageEl } from "$lib/misc/context";
    import NumberInput from "$lib/NumberInput.svelte";

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
    let calStartTime = $state(7);
    let minCalEndTime = $derived(calStartTime + 1);
    let calEndTime = $state(19);
    let calNumHours = $derived(calEndTime - calStartTime);
    let loaded = $state(false);
    let mounted = $state(false);
    let email = $state("");
    let password = $state("");
    
    let selectedTheme = $state<ThemeName>("pinkDark");
    const themeOptions = (Object.keys(themes) as ThemeName[]).map((key) => ({
        value: key,
        label: themes[key].name
    }));
    
    async function updateTheme(newThemeName: ThemeName) {
        theme.name = newThemeName;
        theme.theme = themes[newThemeName];

        const store = await load(".settings.json");
        await store.set("theme", { value: newThemeName });
        await store.save();
    }

    async function updateDateFormat(newDateFormatFunctionName: DateFormatName) {
        const store = await load(".settings.json");
        await store.set("dateFormat", { value: newDateFormatFunctionName });
        await store.save();
    }

    $effect(() => {
        if (!mounted) return;
        if (selectedTheme) {
            updateTheme(selectedTheme);
        }
    });

    $effect(() => {
        if (!mounted) return;
        if (selectedDateFormat.name) {
            updateDateFormat(selectedDateFormat.name);
        }
    });
    
    onMount(async () => {
        const store = await load(".settings.json");

        const value = await store.get<{ value: ThemeName }>("theme");

        if (value?.value) {
            theme.name = value.value;
            theme.theme = themes[value.value];
            selectedTheme = value.value;
        }


        const dateFormat = await store.get<{ value: DateFormatName }>("dateFormat");
        const emailName = await store.get<{ value: string}>("email");

        calNumHours = await getCalendarNumHours();
        calStartTime = await getCalendarStartTime();
        calEndTime = calStartTime + calNumHours;

        if (emailName?.value) {
            email = emailName.value;
            password = await loadCredentials(email);
        }

        selectedDateFormat.name = dateFormat?.value ?? "dayOfWeekAndMonth";
        loaded = true;
        mounted = true;
    });

    $effect(() => {
        if (!mounted) return;
        if (username.name) {
            updateName(username.name);
        }
    });

    async function updateName(newName: string) {
        const store = await load(".settings.json");
        await store.set("username", { value: newName });
        await store.save();
    }

    $effect(() => {
        if (!mounted) return;
        if (calEndTime && loaded) updateCalendarNumHours(calNumHours);
        if (calStartTime && loaded) updateCalendarStartTime(calStartTime);
    });

    export async function removeCredentials(email: string): Promise<void> {
        await invoke("delete_credentials", { email });
        const store = await load(".settings.json");
        await store.set("email", { value: "" });
        await store.save();
        password = "";
        email = "";
    }

    let pageEl = $state<HTMLElement>();
    setPageEl( () => pageEl );
</script>

<div class="page" bind:this={pageEl}>
    <h1 in:fly={{ y: 20, delay: 100, duration: 1500, easing: quartOut }}>
        Settings
    </h1>

    <div class="settings-columns">
        
        <div class="column">
            <!-- Appearance -->
            <section class="card" in:fly={{ y: 30, delay: 250, duration: 1500, easing: quartOut}}>
                <h5 class="card-title">Appearance</h5>

                <div class="field">
                    <p class="field-label">Display Name</p>
                    <p class="field-hint">Shown on the homepage greeting.</p>
                    <div class="input-wrap">
                        <Textbox placeholders={["Enter your name"]} preamble={false} bind:value={username.name} />
                    </div>
                </div>

                <div class="divider"></div>

                <div class="field">
                    <p class="field-label">Theme</p>
                    <p class="field-hint">Choose a preset or build your own.</p>
                    <Dropdown options={themeOptions} bind:selected={selectedTheme} />
                    <div style="display: flex; justify-content: center;">
                        <div class="swatches">
                            <Swatch color={"var(--primary-light)"} />
                            <Swatch color={"var(--primary-dark)"} />
                            <Swatch color={"var(--hover-primary-dark)"} />
                            <Swatch color={"var(--primary-color)"} />
                            <Swatch color={"var(--secondary-color)"} />
                            <Swatch color={"var(--highlight-color)"} />
                            <Swatch color={"var(--border-color)"} />
                            <Swatch color={"var(--hover-color)"} />
                        </div>
                    </div>
                </div>

                <div class="divider"></div>

                <div class="field">
                    <p class="field-label">Date Format</p>
                    <p class="field-hint">Controls how dates appear throughout the app.</p>
                    <Dropdown options={dateFormatOptions} bind:selected={selectedDateFormat.name} />
                </div>
            </section>

            <!-- Danger Zone -->
            <section class="card danger-card" in:fly={{ y: 20, delay: 400, duration: 1500, easing: quartOut }}>
                <h5 class="card-title">Danger Zone</h5>

                <div class="field">
                    <p class="field-label">Reset Database</p>
                    <p class="field-hint">
                        Permanently deletes <strong>all</strong> tasks (completed and incomplete) and all tags. This cannot be undone.
                    </p>
                    <Button flavor="danger" class="circular border" onclick={confirmResetDatabase}>
                        Reset Database
                    </Button>
                </div>
            </section>
        </div>
    
        <div class="column">

            <!-- Calendar -->
            <section class="card" in:fly={{ y: 20, delay: 325, duration: 1500, easing: quartOut }}>
                <h5 class="card-title">Calendar</h5>
    
                <!-- actual calendar customization -->
                <div class="field">
                    <p class="field-label">Calendar</p>
                    <p class="field-hint">Modify the homepage calendar</p>
                    <!-- Calendar Start Time -->
                    <div>
                        <NumberInput 
                            bind:num={calStartTime} 
                            label="Start Time"
                            upperLimitNum={23}
                            lowerLimitNum={0}
                            increment={1}
                        /> 
                        <NumberInput 
                            bind:num={calEndTime} 
                            label="End Time"
                            upperLimitNum={24}
                            bind:lowerLimitNum={minCalEndTime}
                            increment={1}
                        /> 
                    </div>
                </div>
    
                <div class="divider"></div>
                <!-- Icloud account fields -->
    
                <div class="field">
                    <p class="field-label">iCloud Account</p>
                    <p class="field-hint">Use an app-specific password from appleid.apple.com.</p>
                    <div class="input-wrap">
                        <Textbox placeholders={["Email"]} preamble={false} bind:value={email} />
                    </div>
                    <div class="input-wrap">
                        <Textbox placeholders={["App-specific password"]} preamble={false} bind:value={password} />
                    </div>
                    <div class="button-row">
                        <Button flavor="primary" class="circular" onclick={() => saveCredentials(email, password)}>
                            Save
                        </Button>
                        <Button flavor="danger" class="circular border" onclick={() => removeCredentials(email)}>
                            Clear
                        </Button>
                    </div>
                </div>
            </section>
        </div>

    </div>
</div>

<AlertDialog
    bind:open={databaseConfirmDialogOpen}
    bind:result={confirmDialog}
    title="Are you absolutely sure?"
    message="This action cannot be undone. This will permanently delete both completed and incompleted tasks you've created, as well as any tags."
/>

<style>
    .page {
        position:relative;
        padding: 3rem;
        max-width: 860px;
    }

    h1 {
        margin-bottom: 2rem;
    }

    /* Grid: two columns on wide viewports, single column otherwise */
    .settings-columns {
        display: flex;
        gap: 1.5rem;
    }

    .column {
        display: flex;
        gap: 1.5rem;
        flex-direction: column;
        width: calc((100vw - 3rem) / 4);
    }

    /* Cards */
    .card {
        border: 1px solid var(--border-color);
        background-color: var(--primary-light);
        border-radius: 15px;
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 0;
    }

    .card-title {
        margin-bottom: 1.25rem;
    }

    .danger-card {
        border-color: color-mix(in srgb, #D64540 40%, var(--border-color));
    }

    /* Fields */
    .field {
        display: flex;
        flex-direction: column;
        gap: 0.375rem;
    }

    .field-label {
        font-weight: 600;
        font-size: 0.875rem;
    }

    .field-hint {
        font-size: 0.8rem;
        opacity: 0.6;
        margin: 0;
        padding: 0;
        padding-bottom: 0.5rem;
    }

    .divider {
        height: 1px;
        background: var(--border-color);
        margin: 1.25rem 0;
    }

    /* Inputs */
    .input-wrap {
        border: 1px solid var(--border-color);
        border-radius: 15px;
        margin-bottom: 0.5rem;
    }

    /* Buttons */
    .button-row {
        display: flex;
        gap: 0.5rem;
        margin-top: 0.25rem;
    }

    /* Swatches */
    .swatches {
        display: flex;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        overflow: hidden;
        width: fit-content;
        margin-top: 0.75rem;
    }
</style>