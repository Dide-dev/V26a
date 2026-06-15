<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { Snippet } from 'svelte';
    import { Alert } from 'flowbite-svelte';

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
        <Alert
            dismissable
            onclick={() => dismissNotice(i)}
            class="rounded-md border"
            style="color: {notice.textColor}; background-color: {notice.backgroundColor};"
            icon={notice.icon}
        >
            <p>{notice.text}</p>
        </Alert>
    {/each}
</div>
