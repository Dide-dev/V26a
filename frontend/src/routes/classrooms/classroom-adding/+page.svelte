<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { addClassroom, type ClassroomAddData } from '$lib/backend/classrooms';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { onMount } from 'svelte';
    import { redirectLogin } from '$lib/backend';
    import { getUserInfo, type UserInfo } from '$lib/backend/users';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';

    let userInfo: UserInfo | null = $state(null);

    onMount(async () => {
        userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }
    });

    let classroomData: ClassroomAddData = $state({
        name: '',
        description: ''
    });

    let formCompleted: boolean = $derived(classroomData.name !== '');

    const minInputLengths = {
        name: 0,
        description: 0
    };

    const maxInputLengths = {
        name: 32,
        description: 256
    };

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        await addClassroom(classroomData);

        await goto(appLink('/classrooms/classroom-dashboard'));
    }
</script>

<div>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">{m.classroomAdd()}</h1>
        </div>
        <div>
            <label for="name">
                {m.classroomName()} *
            </label>
            <input
                id="name"
                class={['input__text']}
                bind:value={classroomData.name}
                name="name"
                type="text"
                data-testid="name"
                placeholder={m.examplePlaceholderRequired({ value: m.exampleClassroomName() })}
                minlength={minInputLengths.name}
                maxlength={maxInputLengths.name}
                required
            />
        </div>
        <div>
            <label for="description">
                {m.classroomDescription()}
            </label>
            <textarea
                id="description"
                class="input__long_text min-h-20"
                bind:value={classroomData.description}
                name="description"
                data-testid="description"
                placeholder={m.examplePlaceholderOptional({
                    value: m.exampleClassroomDescription()
                })}
                minlength={minInputLengths.description}
                maxlength={maxInputLengths.description}
            ></textarea>
        </div>
        <button data-testid="add-button">{m.classroomAdd()}</button>
    </form>
</div>
