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
    import { addCoAuthor, type TreeId } from '$lib/api-wrapper/trees';
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
    let successfullyAddedCoAuthor: boolean = $state(false);
    let usernameDoesNotExist: boolean = $state(false);
    let userAlreadyAuthorized: boolean = $state(false);

    async function handleAddCoAuthor() {
        if (coAuthorName === '') return;
        successfullyAddedCoAuthor = false;
        usernameDoesNotExist = false;
        userAlreadyAuthorized = false;

        let response = await addCoAuthor(treeId, coAuthorName);

        if (response.type === 'Success') {
            successfullyAddedCoAuthor = true;
        } else if (response.type === 'UsernameDoesNotExist') {
            usernameDoesNotExist = true;
        } else if (response.type === 'UserAlreadyAuthorized') {
            userAlreadyAuthorized = true;
        } else {
            throw new Error('ERROR: Unexpected output from API.');
        }

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
        <div class="mb-3 flex gap-3" data-testid="co-author-popup">
            <div class="w-64">
                <label for="coAuthorName">{m.addCoAuthor()}</label>
                <input
                    id="coAuthorName"
                    class="w-full rounded border p-1"
                    bind:value={coAuthorName}
                    placeholder={m.examplePlaceholder({ value: m.exampleName() })}
                />
            </div>
        </div>
        {#if successfullyAddedCoAuthor}
            <p class="text__success" data-testid="authorization-success">
                {m.successfulAuthorization()}
            </p>
        {/if}
        {#if usernameDoesNotExist}
            <p class="text__error" data-testid="error">{m.invalidAuthorisationTarget()}</p>
        {/if}
        {#if userAlreadyAuthorized}
            <p class="text__error" data-testid="error">{m.userAlreadyAuthorized()}</p>
        {/if}
        <button
            class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white"
            onclick={handleAddCoAuthor}>{m.addCoAuthor()}</button
        >
        <button class="cursor-pointer" onclick={closeEdit}>{m.close()}</button>
    </Popup>
{/if}
