<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
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

    function cancelEdit() {
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

    let coAuthorName: string = $state(null);
    let successfullyAddedCoAuthor: boolean = $state(false);

    async function handleAddCoAuthor() {
        let response = await addCoAuthor(treeId, coAuthorName);

        if (response.type === 'Success') {
            successfullyAddedCoAuthor = true;
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
                <label
                    >{m.addCoAuthor()}<input
                        class="w-full rounded border p-1"
                        bind:value={coAuthorName}
                        placeholder={m.examplePlaceholder({ value: m.exampleName() })}
                    /></label
                >
            </div>
        </div>
        {#if successfullyAddedCoAuthor}
            <p class="text__success" data-testid="authorization-success">
                {m.successfulAuthorization()}
            </p>
        {/if}
        <button
            class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white"
            onclick={handleAddCoAuthor}>{m.addCoAuthor()}</button
        >
        <button class="cursor-pointer" onclick={cancelEdit}>{m.cancel()}</button>
    </Popup>
{/if}
