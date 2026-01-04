<script lang="ts">
    import { Blocks, Clock, House, Info, ListTodo, Menu, Settings } from "@lucide/svelte";
    import { onMount, setContext, type Component } from "svelte";
    import SidebarButton from "./SidebarButton.svelte";
    import { page } from "$app/state";
    import { fly, slide } from "svelte/transition";
    import { expoIn, expoInOut, expoOut, quadInOut, quartInOut } from "svelte/easing";

    let sidebarExpanded = $state({ state: false });
    setContext('sidebarExpanded', sidebarExpanded);

    interface SidebarInfo {
        name: string;
        href: string;
        Icon: Component;
        iconSize?: number;
        isTop: boolean;
    }

    const sidebarInfo: SidebarInfo[] = [
        {
            name: "Home",
            href: "/",
            Icon: House,
            iconSize: 20,
            isTop: true
        },
        {
            name: "Study",
            href: "/study",
            Icon: Clock,
            isTop: true
        },
        {
            name: "Tasks",
            href: "/tasklist",
            Icon:ListTodo,
            isTop: true
        },
        {
            name: "Info",
            href: "/info",
            Icon: Info,
            isTop: false
        },
        {
            name: "Settings",
            href: "/settings",
            Icon: Settings,
            isTop: false
        }
    ];
    
    let selectedIndex = $state(0);
    let previousIndex = 0;

    function selectPage(index: number) {
        previousIndex = selectedIndex;
        selectedIndex = index;
    }


    function switchPage(
        node: HTMLElement,
        params: { delay?: number; duration?: number; easing?: (t: number) => number; dir: number }
    ) {
        const o = +getComputedStyle(node).opacity;
        const distance = 25;
        
        return {
            delay: params.delay || 0,
            duration: params.duration || 500,
            easing: params.easing || expoInOut  ,
            css: (t: number, u: number) => {
                const progress = u === 0 ? (1 - t) : u;
                const blur = progress * 2;


            return `
                transform: translateY(${params.dir * distance * progress}px)
                        scaleY(${0.8 + 0.2 * t});
                filter: blur(${blur}px);
                opacity: ${t * o};
            `;
            }
        };
    }


    onMount(() => {
        //check if the selected page equals the name, then set that as the selected index
        let pageUrl = page.url.pathname;
        selectedIndex = sidebarInfo.findIndex(p => p.href === pageUrl);
    });
</script>

<aside class="aside">
    <div>
        {#each sidebarInfo as sidebar, i}
            {#if sidebar.isTop}
                <SidebarButton onclick={() => selectPage(i)} iconSize={sidebar.iconSize} Icon={sidebar.Icon} href={sidebar.href} name={sidebar.name}>
                    {#if selectedIndex === i}
                        <div
                            in:switchPage={{ duration: 500, dir: previousIndex < i ? -1 : 1 }}
                            out:switchPage={{ duration: 500, dir: i < selectedIndex ? 1 : -1 }}
                            class="selected-bar"
                        >
                        </div>
                    {/if}
                </SidebarButton>
            {/if}
        {/each}
    </div>
    <div>
        {#each sidebarInfo as sidebar, i}
            {#if !sidebar.isTop}
                <SidebarButton onclick={() => selectPage(i)} iconSize={sidebar.iconSize} Icon={sidebar.Icon} href={sidebar.href} name={sidebar.name}>
                    {#if selectedIndex === i}
                        <div
                            in:switchPage={{ duration: 300, dir: previousIndex < i ? -1 : 1 }}
                            out:switchPage={{ duration: 300, dir: i < selectedIndex ? 1 : -1 }}
                            class="selected-bar"
                        >
                        </div>
                    {/if}
                </SidebarButton>
            {/if}
        {/each}
    </div>
</aside>

<style>
    .aside {
        margin-top: 0.25rem;
        width: 3rem;
        display: flex;
        justify-content: space-between;
        flex-direction: column;
        transition: 0.15s ease-in-out;
        height: calc(100vh - 3.25rem);
    }
    div {
        width: min-content;
    }
    .selected-bar {
        position: absolute;
        z-index: 1;
        background-color: var(--highlight-color);
        height: 1.75rem;
        width: 3px;
        margin-left: 4px;
        border-radius: 7px;
    }
</style>