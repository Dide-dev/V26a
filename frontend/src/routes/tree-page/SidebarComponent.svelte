<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This sidebar is currently only used for the card editing, but could be used for more later.
 -->
<script lang="ts">
    import type { CardGetData } from '$lib/backend/cards';
    import { m } from '$lib/paraglide/messages';

    let { sidebar = $bindable() }: { sidebar: SideBar | null } = $props();

    export type SideBar = {
        location: number[];
        editingVersion: CardGetData;
    };

    export const fullwidth: number = 350;
</script>

{#if sidebar !== null}
    <div class="pointer-events-none absolute top-0 right-0 h-full" style="width: {fullwidth}px;">
        <div
            class="pointer-events-auto sticky top-0 z-50 flex w-full flex-col gap-2 bg-blue-50 p-2 shadow-[-5px_5px_5px_-5px_rgba(0_0_0/0.8)]"
        >
            <h1>{m.editThisCard()}</h1>

            <p>Front</p>
            <input class="border" bind:value={sidebar.editingVersion.front} />

            <p>Back</p>
            <textarea rows="10" bind:value={sidebar.editingVersion.back}></textarea>
            <button class="cursor-pointer" onclick={() => (sidebar = null)}>
                {m.cancel()}
            </button>
            <button class="cursor-pointer" onclick={() => alert('todo')}>
                {m.save()}
            </button>
        </div>
    </div>
{/if}
