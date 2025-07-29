<script>
    import { Blocks, Clock, House, Info, ListTodo, Menu, Settings } from "@lucide/svelte";
    import { setContext } from "svelte";
    import Button from "./Button.svelte";
    import SidebarButton from "./SidebarButton.svelte";

    let sidebarExpanded = $state({ state: false });
    setContext('sidebarExpanded', sidebarExpanded);

</script>

<aside class="aside {sidebarExpanded.state ? 'expanded' : ''}">
    <div>
        <!-- <Button Icon={Menu} iconSize={20} onclick={() => sidebarExpanded.state = !sidebarExpanded.state} expanded={sidebarExpanded.state}/> -->
        <SidebarButton Icon={House} iconSize={20} href="/" name="Home"/>
        <SidebarButton Icon={Clock} href="/study" name="Study"/>
        <SidebarButton Icon={ListTodo} iconSize={20} href="/tasklist" name="Tasks"/>
    </div>
    <div>
        <SidebarButton Icon={Info} iconSize={20} href="/info" name="Info"/>
        <SidebarButton Icon={Settings} iconSize={20} href="/settings" name="Settings"/>
    </div>
</aside>

<style>
    .aside {
        margin-top: 0.25rem;
        width: 3rem;
        background-color: transparent;
        display: flex;
        justify-content: space-between;
        align-items: left;
        flex-direction: column;
        transition: 0.15s ease-in-out;
    }
    .aside.expanded {
        width: 9rem;
    }
</style>