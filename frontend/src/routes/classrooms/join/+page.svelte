<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import {
        classroomLogin,
        type ClassroomLoginResponse,
        type ClassroomUserLoginData
    } from '$lib/api-wrapper/classrooms';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';

    let loginData: ClassroomUserLoginData = $state({
        joinCode: '',
        username: ''
    });

    let joinCodeIncorrect: boolean = $state(false);
    let missingUser: boolean = $state(false);

    let formCompleted: boolean = $derived(loginData.joinCode !== '' && loginData.username !== '');

    const minInputLengths = {
        joinCode: 0,
        username: 0
    };

    const maxInputLengths = {
        joinCode: 6,
        username: 32
    };

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        let classroomLoginResponse: ClassroomLoginResponse = await classroomLogin(loginData);

        switch (classroomLoginResponse.type) {
            case 'Success':
                joinCodeIncorrect = false;
                missingUser = false;
                await goto(appLink(`/classrooms/${classroomLoginResponse.id}`));
                break;
            case 'JoinCodeIncorrect':
                missingUser = false;
                joinCodeIncorrect = true;
                break;
            case 'MissingUser':
                joinCodeIncorrect = false;
                missingUser = true;
                break;
        }
    }
</script>

<div>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">{m.joinClassroom()}</h1>
        </div>
        <div>
            <label for="join-code">
                {m.classroomJoinCode()}
            </label>
            <input
                id="join code"
                class={['input__text']}
                bind:value={loginData.joinCode}
                name="join code"
                type="text"
                data-testid="join code"
                placeholder={m.examplePlaceholderRequired({ value: '11A2B3' })}
                minlength={minInputLengths.joinCode}
                maxlength={maxInputLengths.joinCode}
                required
            />
        </div>
        <div>
            <label for="username">
                {m.classroomUserName()}
            </label>
            <input
                id="username"
                class={['input__text']}
                bind:value={loginData.username}
                name="user name or number"
                type="text"
                data-testid="username"
                placeholder={m.examplePlaceholderRequired({ value: 'Lorax' })}
                minlength={minInputLengths.username}
                maxlength={maxInputLengths.username}
                required
            />
        </div>
        <button type="submit" data-testid="join-button">{m.joinClassroom()}</button>
        {#if joinCodeIncorrect}
            <p class="text__error">{m.classroomJoinCodeIncorrect()}</p>
        {:else if missingUser}
            <p class="text__error">{m.classroomMissingUser()}</p>
        {/if}
    </form>
</div>
