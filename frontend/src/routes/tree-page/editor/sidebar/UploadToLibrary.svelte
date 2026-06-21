<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a pop-up menu allowing the user to upload cards to the card library, to be used on the edit-tree page.

    ## Params
    - `title`  - The title of the card that the user wants for his/her selected card in the edit-tree page.
    - `tags`   - A list of tags that contains tags that the user wants to add to his selected card in the edit-tree page.
-->

<script lang="ts">
    import { minInputLengths, maxInputLengths } from '$lib/api-wrapper/trees';
    import { tryParseTags } from '$lib/treeMetadata';
    import { m } from '$lib/paraglide/messages';
    import type { CardSideBar, MenuSideBar } from './Sidebar.svelte';
    import { doAddCardToLibrary, followIndexNode, type TreeHistory } from '../history.svelte.ts';
    import type { Notice } from '$lib/Notices.svelte';
    import { CheckOutline } from 'flowbite-svelte-icons';

    let {
        sidebar = $bindable(),
        history,
        notices = $bindable()
    }: {
        sidebar: CardSideBar | MenuSideBar;
        history: TreeHistory;
        notices: Notice[];
    } = $props();

    let titleInput: string = $state('');
    let tagsInput: string = $state('');
    let tagsInputError: string = $state('');
    let titleChanged: boolean = $state(false);
    let tagsChanged: boolean = $state(false);

    let formChanged: boolean = $derived(titleChanged || tagsChanged);

    function close() {
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
            handleSave(event);
        }
    }

    async function handleSave(event: Event) {
        event.preventDefault();

        if (!formChanged || sidebar.kind !== 'CardEditor') return;

        const tags = tryParseTags(tagsInput);
        if (typeof tags === 'string') {
            tagsInputError = tags;
            return;
        }

        let cardId = followIndexNode(history.currentStructure, sidebar.location).cardId;

        await doAddCardToLibrary(history, cardId, titleInput, tags);

        notices.push({
            text: m.cardUploadedSuccessfully(),
            icon: checkIcon,
            textColor: 'var(--color-tree-610)',
            backgroundColor: 'var(--color-tree-100)'
        });
        close();
    }
</script>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

<form
    data-testid="card-upload-popup"
    onsubmit={handleSave}
    class={[formChanged && 'completed', 'flex w-full flex-col']}
>
    <div>
        <h2 class="page__heading">{m.uploadCard()}</h2>
    </div>
    <div>
        <label for="title">{m.title()} *</label>
        <div class="input__container">
            <input
                id="title"
                class={[!titleChanged && 'inactive', 'input__text w-full']}
                type="text"
                bind:value={titleInput}
                placeholder={m.examplePlaceholderRequired({
                    value: m.exampleTitleCard()
                })}
                oninput={() => {
                    titleChanged = true;
                }}
                minlength={minInputLengths.title}
                maxlength={maxInputLengths.title}
                required
            />
        </div>
    </div>
    <div>
        <label for="tags">{m.tags()}</label>
        <div class="input__container">
            <input
                id="tags"
                class={[
                    !tagsChanged && 'inactive',
                    tagsInputError !== '' && 'error',
                    'input__text',
                    'w-full'
                ]}
                type="text"
                bind:value={tagsInput}
                placeholder={m.examplePlaceholder({ value: m.exampleTags() })}
                oninput={() => {
                    tagsChanged = true;
                }}
            />
        </div>
        {#if tagsInputError !== ''}
            <p class="text__error" data-testid="tags-input-error">
                {tagsInputError}
            </p>
        {/if}
    </div>
    <div class="grid grid-cols-2 gap-1">
        <button type="submit" data-testid="confirm-button-upload">{m.confirmUpload()}</button>
        <button type="reset" class="cursor-pointer" onclick={close}>{m.close()}</button>
    </div>
</form>

{#snippet checkIcon()}
    <CheckOutline class="h-5 w-5 shrink-0" />
{/snippet}

<style>
    form {
        .inactive:not(.input__long_text) {
            background-color: var(--color-neutral-100);
        }
    }
</style>
