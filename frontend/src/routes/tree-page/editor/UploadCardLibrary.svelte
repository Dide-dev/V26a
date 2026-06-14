<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { minInputLengths, maxInputLengths } from '$lib/api-wrapper/trees';
    import { tryParseTags } from '$lib/treeMetadata';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

    let {
        visible = $bindable(false),
        onConfirm
    }: {
        visible: boolean;
        onConfirm: (title: string, tags: string[]) => void;
    } = $props();

    let titleInput: string = $state('');
    let tagsInput: string = $state('');
    let tagsInputError: string = $state('');
    let titleChanged: boolean = $state(false);
    let tagsChanged: boolean = $state(false);
    let uploaded: boolean = $state(false);

    let formChanged: boolean = $derived(titleChanged || tagsChanged);

    function closeEdit() {
        titleInput = '';
        tagsInput = '';
        titleChanged = false;
        tagsChanged = false;
        tagsInputError = '';
        uploaded = false;
        visible = false;
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            handleSave(new SubmitEvent('submit'));
        }
    }

    function handleSave(e: SubmitEvent) {
        e.preventDefault();

        if (!formChanged) return;

        const result = tryParseTags(tagsInput);
        if (typeof result === 'string') {
            tagsInputError = result;
            return;
        }

        onConfirm(titleInput, result);

        titleInput = '';
        tagsInput = '';
        titleChanged = false;
        tagsChanged = false;
        tagsInputError = '';
        uploaded = true;
    }
</script>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

{#if visible}
    <Popup>
        {#if uploaded}
            <div class="mb-3 flex w-64 flex-col items-center text-center" data-testid="card-upload-success">
                <p class="text-black">{m.cardUploadedSuccessfully()}</p>
                <button type="submit" onclick={closeEdit}>{m.close()}</button>
            </div>
        {:else}
            <form
                data-testid="card-upload-popup"
                onsubmit={handleSave}
                class={[formChanged && 'completed', 'mb-3 flex w-64 flex-col']}
            >
                <div class="w-64">
                    <label for="title">{m.title()} *</label>
                    <div class="input__container">
                        <input
                            id="title"
                            class={[!titleChanged && 'inactive', 'input__text', 'w-full']}
                            type="text"
                            bind:value={titleInput}
                            placeholder={m.examplePlaceholderRequired({ value: m.exampleTitleCard({ name: '' }) })}
                            oninput={() => {
                                titleChanged = true;
                            }}
                            minlength={minInputLengths.title}
                            maxlength={maxInputLengths.title}
                            required
                        />
                    </div>
                </div>
                <div class="w-64">
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
                <div>
                    <button
                        type="submit"
                        data-testid="confirm-button-upload">{m.confirmUpload()}
                    </button>
                </div>
                <button type="reset" class="cursor-pointer" onclick={closeEdit}>{m.close()}</button>
            </form>
        {/if}
    </Popup>
{/if}