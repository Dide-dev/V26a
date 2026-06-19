<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a language for in a list.

    ## Params
    - `language: Language` - The language to display.
    - `creating: boolean` - Whether the entry is for creating a new language.
    - `onUpdate: () => Promise<string | null>` - A callback for when the language is to be updated.
    - `onCreate: () => Promise<string | null>` - A callback for when the language is to be created.
    - `onCancel: () => void` - A callback for when language creation is cancelled.
-->

<script lang="ts">
    import { type Language } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages.js';
    import { CloseOutline, EditOutline, ArrowLeftToBracketOutline } from 'flowbite-svelte-icons';

    let {
        language = $bindable(),
        creating = false,
        onUpdate = async () => null,
        onCreate = async () => null,
        onCancel = () => {}
    }: {
        language: Language;
        creating?: boolean;
        onUpdate?: () => Promise<string | null>;
        onCreate?: () => Promise<string | null>;
        onCancel?: () => void;
    } = $props();

    let editing: boolean = $state(false);
    let errorMessage: string | null = $state(null);
    let oldLanguage: Language = $state.snapshot(language);

    async function update() {
        errorMessage = await onUpdate();
        if (errorMessage === null) {
            oldLanguage = language;
            editing = false;
        }
    }

    function cancel() {
        language = oldLanguage;
        editing = false;
    }

    const minInputLengths = {
        code: 2,
        name: 1
    };
    const maxInputLengths = {
        code: 2,
        name: 32
    };
</script>

<div class="list__entry">
    <div class="flex w-full flex-col justify-between gap-2">
        {#if creating}
            <form
                class="flex w-full flex-row justify-between gap-2"
                onsubmit={async () => (errorMessage = await onCreate())}
            >
                <input
                    id="name"
                    class="input__text min-w-6"
                    type="text"
                    bind:value={language.name}
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    placeholder={m.examplePlaceholder({ value: 'English' })}
                    required
                />
                <input
                    id="code"
                    class="input__text mt-0 min-w-6"
                    type="text"
                    bind:value={language.code}
                    minlength={minInputLengths.code}
                    maxlength={maxInputLengths.code}
                    placeholder={m.examplePlaceholder({ value: 'en' })}
                    required
                />
                <div class="flex flex-row gap-1">
                    <button
                        class="button__icon cursor-pointer"
                        type="submit"
                        aria-label="save"
                        title={m.saveLanguage()}
                    >
                        <ArrowLeftToBracketOutline class="mb-1 h-5 w-5 shrink-0" />
                    </button>
                    <button
                        class="button__icon cursor-pointer"
                        type="button"
                        aria-label="cancel"
                        title={m.cancel()}
                        onclick={onCancel}
                    >
                        <CloseOutline class="mb-1 h-5 w-5 shrink-0" />
                    </button>
                </div>
            </form>
            {#if errorMessage !== null}
                <p class="text__error">
                    {errorMessage}
                </p>
            {/if}
        {:else if editing}
            <form class="flex w-full flex-row justify-between gap-2" onsubmit={update}>
                <input
                    id="name"
                    class="input__text"
                    type="text"
                    bind:value={language.name}
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    placeholder={m.examplePlaceholder({ value: 'English' })}
                    required
                />
                <div class="flex flex-row gap-1">
                    <button
                        class="button__icon cursor-pointer"
                        type="submit"
                        aria-label="save"
                        title={m.saveLanguage()}
                    >
                        <ArrowLeftToBracketOutline class="visible mb-1 h-5 w-5 shrink-0" />
                    </button>
                    <button
                        class="button__icon cursor-pointer"
                        type="button"
                        aria-label="cancel"
                        title={m.cancel()}
                        onclick={cancel}
                    >
                        <CloseOutline class="mb-1 h-5 w-5 shrink-0" />
                    </button>
                </div>
            </form>
            {#if errorMessage !== null}
                <p class="text__error">
                    {errorMessage}
                </p>
            {/if}
        {:else}
            <div class="flex w-full flex-row justify-between gap-2">
                <strong>{language.name} ({language.code})</strong>
                <button
                    class="cursor-pointer"
                    type="button"
                    aria-label="edit"
                    title={m.editLanguage()}
                    onclick={() => (editing = true)}
                >
                    <EditOutline class="mb-1 h-5 w-5 shrink-0" />
                </button>
            </div>
        {/if}
    </div>
</div>
