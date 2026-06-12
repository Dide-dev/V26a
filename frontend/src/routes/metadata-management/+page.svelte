<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        getAllLanguages,
        getAllSubjects,
        minInputLengths,
        maxInputLengths,
        updateTreeMetaData,
        addLanguage,
        type Language,
        type Subject,
        type TreeData,
        type TreeId,
        type TreeLinkedData
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import LanguageListEntry from '$lib/lists/LanguageListEntry.svelte';
    import { PlusOutline } from 'flowbite-svelte-icons';

    let languages: Language[] | null = $state(null);
    let subjects: Record<string, Subject[]> | null = $state(null);
    let creating: boolean = $state(false);
    let newLanguage: Language | null = $state(null);

    onMount(async () => {
        languages = await getAllLanguages();
        languages!.sort((a, b) => a.code.localeCompare(b.code));
        subjects = await getAllSubjects();
    });

    async function createLanguage() {
        if (languages!.some((l) => newLanguage!.code === l.code))
            return m.languageExists({ code: newLanguage!.code });
        await addLanguage(newLanguage!);
        languages!.push(newLanguage!);
        languages!.sort((a, b) => a.code.localeCompare(b.code));
        newLanguage = null;
        return null;
    }
</script>

<div class="flex w-7/8 max-w-lg flex-col">
    <div class="flex w-full flex-row items-center justify-between gap-2">
        <h1 class="page__heading m-0">{m.languageList()}</h1>
        <button
            class="cursor-pointer"
            type="button"
            aria-label="edit"
            title={m.addLanguage()}
            onclick={() => (newLanguage = { code: '', name: '' })}
        >
            <PlusOutline class="mb-1 h-5 w-5 shrink-0" />
        </button>
    </div>
    {#if languages === null}
        <p>{m.loading()}</p>
    {:else}
        <div class="flex w-full flex-col items-center justify-center overflow-auto">
            <div class="list__container">
                {#if newLanguage !== null}
                    <LanguageListEntry
                        creating={true}
                        onCreate={createLanguage}
                        onCancel={() => (newLanguage = null)}
                        bind:language={newLanguage}
                    />
                {/if}
                {#each languages.entries() as entry}
                    <LanguageListEntry bind:language={languages[entry[0]]} />
                {/each}
            </div>
        </div>
    {/if}
</div>
