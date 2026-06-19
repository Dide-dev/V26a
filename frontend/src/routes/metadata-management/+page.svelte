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
        addLanguage,
        addSubject,
        updateLanguage,
        updateSubject,
        type Language,
        type Subject
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import LanguageListEntry from '$lib/lists/LanguageListEntry.svelte';
    import SubjectListEntry from '$lib/lists/SubjectListEntry.svelte';
    import { PlusOutline } from 'flowbite-svelte-icons';

    let languages: Language[] | null = $state(null);
    let subjects: Subject[] | null = $state(null);
    let newLanguage: Language | null = $state(null);
    let newSubject: Subject | null = $state(null);

    onMount(async () => {
        languages = await getAllLanguages();
        languages!.sort((a, b) => a.code.localeCompare(b.code));
        const subjectMap = await getAllSubjects();
        subjects = Object.values(subjectMap).flat().sort(compareSubjects);
    });

    function validateLanguage(language: Language): string | null {
        if (languages!.some((l) => language !== l && language!.code === l.code))
            return m.languageCodeExists({ code: language!.code });
        if (languages!.some((l) => language !== l && language!.name === l.name))
            return m.languageNameExists({ name: language!.name });
        return null;
    }

    async function tryAddLanguage(): Promise<string | null> {
        const error = validateLanguage(newLanguage!);
        if (error !== null) return error;
        await addLanguage(newLanguage!);
        languages!.push(newLanguage!);
        languages!.sort((a, b) => a.code.localeCompare(b.code));
        newLanguage = null;
        return null;
    }

    async function tryUpdateLanguage(language: Language): Promise<string | null> {
        const error = validateLanguage(language);
        if (error !== null) return error;
        await updateLanguage(language.code, language.name);
        return null;
    }

    function compareSubjects(a: Subject, b: Subject): number {
        const cmp = a.data.languageCode.localeCompare(b.data.languageCode);
        return cmp !== 0 ? cmp : a.data.name.localeCompare(b.data.name);
    }

    function validateSubject(subject: Subject): string | null {
        if (!languages!.some((l) => subject.data.languageCode === l.code))
            return m.missingLanguage({ code: subject.data.languageCode });
        const exists = subjects!.some(
            (s) =>
                subject !== s &&
                subject.data.languageCode === s.data.languageCode &&
                subject.data.name === s.data.name
        );
        if (exists) {
            return m.subjectExists({
                languageCode: subject.data.languageCode,
                name: subject.data.name
            });
        }
        return null;
    }

    async function tryAddSubject(): Promise<string | null> {
        const error = validateSubject(newSubject!);
        if (error !== null) return error;
        newSubject!.id = await addSubject(newSubject!.data);
        subjects!.push(newSubject!);
        subjects!.sort(compareSubjects);
        newSubject = null;
        return null;
    }

    async function tryUpdateSubject(subject: Subject): Promise<string | null> {
        const error = validateSubject(subject);
        if (error !== null) return error;
        await updateSubject(subject.id, subject.data);
        subjects!.sort(compareSubjects);
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
                        onCreate={tryAddLanguage}
                        onCancel={() => (newLanguage = null)}
                        bind:language={newLanguage}
                    />
                {/if}
                {#each languages as language, index (languages[index].code)}
                    <LanguageListEntry
                        onUpdate={async () => tryUpdateLanguage(languages![index])}
                        bind:language={languages[index]}
                    />
                {/each}
            </div>
        </div>
    {/if}

    <div class="flex w-full flex-row items-center justify-between gap-2">
        <h1 class="page__heading m-0">{m.subjectList()}</h1>
        <button
            class="cursor-pointer"
            type="button"
            aria-label="edit"
            title={m.addSubject()}
            onclick={() => (newSubject = { id: 0, data: { languageCode: '', name: '' } })}
        >
            <PlusOutline class="mb-1 h-5 w-5 shrink-0" />
        </button>
    </div>
    {#if subjects === null}
        <p>{m.loading()}</p>
    {:else}
        <div class="flex w-full flex-col items-center justify-center overflow-auto">
            <div class="list__container">
                {#if newSubject !== null}
                    <SubjectListEntry
                        creating={true}
                        onCreate={tryAddSubject}
                        onCancel={() => (newSubject = null)}
                        bind:subject={newSubject}
                    />
                {/if}
                {#each subjects as subject, index (subjects[index].id)}
                    <SubjectListEntry
                        onUpdate={async () => tryUpdateSubject(subjects![index])}
                        bind:subject={subjects[index]}
                    />
                {/each}
            </div>
        </div>
    {/if}
</div>
