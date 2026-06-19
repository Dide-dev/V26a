<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { Snippet } from 'svelte';
    import { CloseOutline } from 'flowbite-svelte-icons';

    let {
        notices = $bindable(),
        maxNumberOfNotices = 5
    }: {
        notices: Notice[];
        maxNumberOfNotices?: number;
    } = $props();

    export type Notice = {
        text: string;
        icon: Snippet;
        textColor: string;
        backgroundColor: string;
    };

    function dismissNotice(index: number) {
        notices.splice(index, 1);
    }
</script>

<div class="fixed right-2 bottom-2 z-40 flex flex-col gap-1">
    {#each notices.slice(0, maxNumberOfNotices) as notice, i (i)}
        <div
            class="flex flex-row gap-2 rounded-md border p-4"
            style="color: {notice.textColor}; background-color: {notice.backgroundColor};"
        >
            <div class="flex items-center">
                {@render notice.icon()}
            </div>
            <p>{notice.text}</p>
            <button
                class="flex cursor-pointer items-center"
                type="button"
                onclick={() => dismissNotice(i)}
            >
                <CloseOutline class="h-5 w-5 shrink-0" />
            </button>
        </div>
    {/each}
</div>
