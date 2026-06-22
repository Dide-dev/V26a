<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a pop-up allowing the user to add new co-authors, to be used on the edit-tree page.

    ## Params
    - `treeId`  - The id of the tree to which the co-authors will be added, corresponding to the tree currently open in the edit-tree page.
-->

<script lang="ts">
    import {
        addCoAuthor,
        type TreeId,
        type CoAuthorizeResponse,
        removeCoAuthor
    } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages';
    import iconTrash from '$lib/assets/icons/trash.svg?raw';
    import type { ManageSideBar, MenuSideBar } from './Sidebar.svelte';

    let {
        treeId,
        coAuthors = $bindable(),
        sidebar = $bindable()
    }: {
        treeId: TreeId;
        coAuthors: string[];
        sidebar: ManageSideBar | MenuSideBar;
    } = $props();

    function closeEdit(event: Event) {
        event.preventDefault();
        coAuthorName = '';
        apiResponse = null;
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    function onkeydown(event: KeyboardEvent) {
        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        if (event.ctrlKey) event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            handleAddButton(event);
        }
    }

    let coAuthorName: string = $state('');
    let apiResponse: CoAuthorizeResponse | null = $state(null);

    async function handleAddButton(event: Event) {
        event.preventDefault();

        if (coAuthorName === '') return;

        apiResponse = await addCoAuthor(treeId, coAuthorName);

        if (apiResponse.type === 'Success') {
            coAuthors.push(coAuthorName);
        }
        coAuthorName = '';
    }

    async function handleRemoveButton(coAuthorName: string) {
        coAuthors.splice(coAuthors.indexOf(coAuthorName), 1);

        await removeCoAuthor(treeId, coAuthorName);
    }
</script>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

<form class={[coAuthorName && 'completed', 'flex w-full flex-col']} data-testid="co-author-popup">
    <div>
        <h2 class="page__heading">{m.coAuthors()}</h2>

        {#if coAuthors.length === 0}
            <em>{m.none()}</em>
        {:else}
            <div class="flex flex-col border-b border-neutral-200">
                {#each coAuthors as coAuthor (coAuthor)}
                    <div
                        class={[
                            'flex w-full justify-between gap-1',
                            'border-t border-neutral-200 bg-neutral-50 p-2 transition-colors duration-200 hover:bg-neutral-100'
                        ]}
                    >
                        <p>{coAuthor}</p>
                        <button
                            type="button"
                            class="cursor-pointer"
                            onclick={() => handleRemoveButton(coAuthor)}
                        >
                            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                            {@html iconTrash}
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <div>
        <label for="co-author-name">{m.addCoAuthor()}</label>
        <input
            id="co-author-name"
            class="input__text"
            type="text"
            name="username"
            bind:value={coAuthorName}
            placeholder={m.examplePlaceholder({ value: m.exampleName() })}
        />
        {#if apiResponse?.type === 'Success'}
            <p class="text__success" data-testid="authorization-success">
                {m.successfulAuthorization()}
            </p>
        {/if}
        {#if apiResponse?.type === 'UsernameDoesNotExist'}
            <p class="text__error" data-testid="error">{m.invalidAuthorisationTarget()}</p>
        {/if}
        {#if apiResponse?.type === 'UserAlreadyAuthorized'}
            <p class="text__error" data-testid="error">{m.userAlreadyAuthorized()}</p>
        {/if}
    </div>
    <div class="grid grid-cols-2 gap-1">
        <button type="submit" onclick={handleAddButton}>{m.addCoAuthor()}</button>
        <button type="reset" class="cursor-pointer" onclick={closeEdit}>{m.close()}</button>
    </div>
</form>
