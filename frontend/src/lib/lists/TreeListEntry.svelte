<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a tree for in a list.

    ## Params
    - `tree: TreeMetadata` - The data of the tree to display.
    - `expanded: boolean` - Bindable flag whether the entry displays all its contents.
-->

<script lang="ts">
    import {
        ArrowUpRightFromSquareOutline,
        BookOutline,
        ChalkboardOutline,
        EditOutline,
        FileCloneOutline,
        LanguageOutline,
        LockSolid,
        TrashBinSolid
    } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import { removeTree, type TreeMetadata } from '$lib/api-wrapper/trees';
    import { appLink, timeFormatString } from '$lib';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { getUserInfo } from '$lib/api-wrapper/sessions';
    import { duplicateThisTree } from '$lib/treePage';
    import Popup from '$lib/Popup.svelte';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);
    let userInfo = $state(getUserInfo());

    let {
        tree,
        expanded = $bindable(),
        handleDelete,
        usedForClassroom
    }: {
        tree: TreeMetadata;
        expanded: boolean;
        handleDelete: () => void;
        usedForClassroom: boolean;
    } = $props();

    let deleteConfirmation = $state(false);

    function askDeleteConfirmation() {
        deleteConfirmation = true;
    }

    function cancelDelete() {
        deleteConfirmation = false;
    }

    async function deleteThisTree() {
        deleteConfirmation = false;

        await removeTree(tree.genData.id);

        handleDelete();
    }
</script>

<div class="list__entry">
    <div class="flex w-full flex-row justify-between gap-2">
        <div class="flex max-w-2/3 flex-col">
            <span class="flex gap-1 *:*:inline">
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                    <a
                        href={appLink(`/trees/${tree.genData.id}`)}
                        target="_blank"
                        class={[
                            !tree.data.published && 'text-neutral-600 italic',
                            'line-clamp-1 w-fit flex-1 break-all text-ellipsis'
                        ]}
                        title={tree.data.published ? m.viewTree() : m.unpublishedTree()}
                    >
                        <strong>{tree.data.title}</strong>
                    </a>
                </AnimatedUnderline>
                {#if !tree.data.published}
                    <span title={m.unpublishedTree()}>
                        <LockSolid class="mb-1 h-5 w-5 shrink-0" />
                    </span>
                {/if}
                <span class="font-semibold text-neutral-500">#{tree.genData.id}</span>
                <a
                    href={appLink(`/trees/${tree.genData.id}`)}
                    aria-label="open"
                    target="_blank"
                    title={m.viewTree()}
                >
                    <ArrowUpRightFromSquareOutline class="mb-1 h-5 w-5 shrink-0" />
                </a>
                {#if userInfo?.userId === tree.genData.ownerId || tree.linkedData.coAuthors.includes(userInfo?.name ?? '') || userInfo?.accountType === 'admin'}
                    <a
                        href={appLink(`/trees/${tree.genData.id}/edit`)}
                        aria-label="edit"
                        target="_blank"
                        title={m.editTree()}
                    >
                        <EditOutline class="mb-1 h-5 w-5 shrink-0" />
                    </a>
                {/if}
                {#if userInfo !== null}
                    <button
                        onclick={() => duplicateThisTree(tree.genData.id)}
                        aria-label="duplicate"
                        title={m.duplicateTree()}
                        class="cursor-pointer"
                    >
                        <FileCloneOutline class="mb-1 h-5 w-5 shrink-0" />
                    </button>
                {/if}
                {#if !usedForClassroom}
                    {#if userInfo?.userId === tree.genData.ownerId || userInfo?.accountType === 'admin'}
                        <button
                            onclick={askDeleteConfirmation}
                            aria-label="delete"
                            title={m.deleteTree()}
                            class="cursor-pointer"
                        >
                            <TrashBinSolid class="mb-1 h-5 w-5 shrink-0" />
                        </button>
                    {/if}
                {/if}
                <span class="flex-auto grow-100"></span>
            </span>
            <em class="-mt-1 text-neutral-700">{m.madeBy()} {tree.linkedData.ownerName}</em>

            <div
                class={[
                    (expanded || tree.data.description.length <= 40) && 'inline',
                    !expanded && 'flex overflow-hidden'
                ]}
            >
                {#if tree.data.description.length > 0}
                    <span
                        class={[
                            !expanded && 'line-clamp-1',
                            'w-fit flex-1 wrap-break-word text-ellipsis'
                        ]}
                    >
                        {tree.data.description}
                    </span>
                {/if}
                {#if tree.data.description.length > 40 || tree.data.bookIsbn !== null || tree.data.subjectId !== null || tree.data.tags.length > 2}
                    <button
                        onclick={() => {
                            expanded = !expanded;
                        }}
                        class={[tree.data.description.length <= 40 && 'block', 'text__link']}
                    >
                        {expanded ? m.showLess() : m.showMore()}
                    </button>
                {/if}
            </div>
        </div>

        <div class="max-w-1/3 text-right">
            <p class="font-semibold">{timeFormat.format(new Date(tree.genData.createdAt))}</p>
            <span class={[!expanded && 'line-clamp-1', 'text-neutral-700']}>
                {tree.data.tags.map((tag) => `#${tag}`).join(' ')}
            </span>
            <div class="mt-0.5 grid w-fit grid-cols-[auto_auto] gap-0.5 justify-self-end text-left">
                <LanguageOutline class="inline h-5 w-5 shrink-0" />
                {tree.linkedData.languageName}
                {#if expanded}
                    {#if tree.data.bookIsbn !== null}
                        <BookOutline class="inline h-5 w-5 shrink-0" />
                        <span>
                            {tree.data.bookName + (tree.data.bookAuthor !== '' ? ',' : '')}
                            {tree.data.bookAuthor}
                        </span>
                    {/if}
                    {#if tree.data.subjectId !== null}
                        <ChalkboardOutline class="inline h-5 w-5 shrink-0" />
                        {tree.linkedData.subjectName}
                    {/if}
                {/if}
            </div>
        </div>
    </div>
</div>

{#if deleteConfirmation}
    <Popup>
        <form class="completed flex flex-col" onsubmit={deleteThisTree} onreset={cancelDelete}>
            <p>{m.deleteTreeConfirmation({ title: tree.data.title })}</p>

            <div class="grid grid-cols-2 gap-1">
                <button type="reset">{m.cancel()}</button>
                <button type="submit" class="danger">{m.confirm()}</button>
            </div>
        </form>
    </Popup>
{/if}
