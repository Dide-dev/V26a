<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a subject for in a list.

    ## Params
    - `subject: Subject` - The subject to display.
    - `creating: boolean` - Whether the entry is for creating a new subject.
    - `onUpdate: () => Promise<string | null>` - A callback for when the subject is to be updated.
    - `onCreate: () => Promise<string | null>` - A callback for when the subject is to be created.
    - `onCancel: () => void` - A callback for when subject creation is cancelled.
-->

<script lang="ts">
    import { type Subject } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages.js';
    import { CloseOutline, EditOutline, ArrowLeftToBracketOutline } from 'flowbite-svelte-icons';

    let {
        subject = $bindable(),
        creating = false,
        onUpdate = async () => null,
        onCreate = async () => null,
        onCancel = () => {}
    }: {
        subject: Subject;
        creating?: boolean;
        onUpdate?: () => Promise<string | null>;
        onCreate?: () => Promise<string | null>;
        onCancel?: () => void;
    } = $props();

    let editing: boolean = $state(false);
    let errorMessage: string | null = $state(null);
    let oldSubject: Subject = $state.snapshot(subject);

    async function update() {
        errorMessage = await onUpdate();
        if (errorMessage === null) {
            oldSubject = subject;
            editing = false;
        }
    }

    function cancel() {
        subject = oldSubject;
        editing = false;
        errorMessage = null;
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
                    bind:value={subject.data.name}
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    placeholder={m.examplePlaceholder({ value: 'Mathematics' })}
                    required
                />
                <input
                    id="languageCode"
                    class="input__text mt-0 min-w-6"
                    type="text"
                    bind:value={subject.data.languageCode}
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
                        title={m.saveSubject()}
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
                    class="input__text min-w-6"
                    type="text"
                    bind:value={subject.data.name}
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    placeholder={m.examplePlaceholder({ value: 'English' })}
                    required
                />
                <input
                    id="languageCode"
                    class="input__text mt-0 min-w-6"
                    type="text"
                    bind:value={subject.data.languageCode}
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
                        title={m.saveSubject()}
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
                <div class="flex flex-row gap-1">
                    <strong>{subject.data.name} ({subject.data.languageCode})</strong>
                    <span class="font-semibold text-neutral-500">#{subject.id}</span>
                </div>
                <button
                    class="cursor-pointer"
                    type="button"
                    aria-label="edit"
                    title={m.editSubject()}
                    onclick={() => (editing = true)}
                >
                    <EditOutline class="mb-1 h-5 w-5 shrink-0" />
                </button>
            </div>
        {/if}
    </div>
</div>
