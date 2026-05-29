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
    import { addCoAuthor, type TreeId, type CoAuthorizeResponse } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

    let { treeId }: { treeId: TreeId } = $props();

    let visible = $state(false);

    function onEditCoAuthorsButtonClick() {
        visible = true;
    }

    function closeEdit() {
        coAuthorName = '';
        visible = false;
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            handleAddCoAuthor();
        }
    }

    let coAuthorName: string = $state('');
    let apiResponse: CoAuthorizeResponse | null = $state(null);

    async function handleAddCoAuthor() {
        if (coAuthorName === '') return;

        apiResponse = await addCoAuthor(treeId, coAuthorName);

        coAuthorName = '';
    }
</script>

<button class="cursor-pointer rounded-sm bg-emerald-500 p-1" onclick={onEditCoAuthorsButtonClick}>
    {m.changeCoAuthors()}
</button>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

{#if visible}
    <Popup>
        <div class="mb-3 flex w-64 flex-col" data-testid="co-author-popup">
            <div class="w-64">
                <label for="coAuthorName">{m.addCoAuthor()}</label>
                <input
                    id="coAuthorName"
                    class="w-full rounded border p-1"
                    bind:value={coAuthorName}
                    placeholder={m.examplePlaceholder({ value: m.exampleName() })}
                />
            </div>
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
        <button
            class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white"
            onclick={handleAddCoAuthor}>{m.addCoAuthor()}</button
        >
        <button class="cursor-pointer" onclick={closeEdit}>{m.close()}</button>
    </Popup>
{/if}
