<script lang='ts'>
    import type { Snippet } from "svelte";


    interface Props {
        flavor?: 'default' | 'outline' | 'danger',
        children: Snippet
    }

    let { flavor ='default', children }: Props = $props();

    const flavorMap = {
        default: 'default',
        danger: 'danger',
        outline: 'outline',
    }
</script>

<div class={[`${flavorMap[flavor] ?? ''}`, 'badge']}>
    {@render children?.()}
</div>

<style>
    .badge {
        display: flex;
        font-size: 0.8rem;;
        padding: 0 0.5rem;
        height: 1.5rem;
        align-items: center;
        border-radius: 15px;
    }
    .badge.default {
        color: black;
        background-color: #e6e6e6;
    }

    .badge.secondary {
        color: black;
        background-color: var(--secondary-color);
    }

    .badge.danger {
        background-color: #ffb4b4;
        color: #86231c;
    }
    
    .badge.outline {
        background-color: var(--primary-color);
        border: 1.5px solid black;
    }
</style>