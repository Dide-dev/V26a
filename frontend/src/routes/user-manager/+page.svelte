<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import {
        getUserAllData,
        type AdminUpdateData,
        updateUserData,
        adminChangePassword,
        type AdminChangePasswordData,
        type AdminGetUserData
    } from '$lib/api-wrapper/admin';
    import {
        checkLowercase,
        checkUppercase,
        checkNumber,
        checkSpecialCharacter,
        checkSamePassword
    } from '$lib/passwords';
    import { m } from '$lib/paraglide/messages.js';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';
    import { goto } from '$app/navigation';
    import { appLink, timeFormatString } from '$lib';
    import { onMount } from 'svelte';

    let id: number = $state(0);
    let userDataPromise: Promise<AdminGetUserData> = $state(new Promise(() => {}));

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');
        if (!pageId) {
            await goto(appLink('/'));
        }
        id = Number(pageId);
        if (Number.isNaN(id)) {
            await goto(appLink('/'));
        }
        userDataPromise = getUserAllData(id);
    });

    let repeatPassword = $state('');
    let changePasswordData: AdminChangePasswordData = $state({
        newPassword: ''
    });
    let passwordMissingChar: boolean = $state(false);
    let usernameAlreadyExists: boolean = $state(false);
    let emailAlreadyExists: boolean = $state(false);
    let successfulPasswordChange: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);

    let inputsChanged = $state({
        enabledChanged: false,
        accountTypeChanged: false,
        emailChanged: false,
        nameChanged: false,
        nicknameChanged: false,
        aboutMeChanged: false,
        languageChanged: false,
        passwordChanged: false,
        error: false
    });

    let formChanged: boolean = $derived(
        inputsChanged.error ||
            inputsChanged.emailChanged ||
            inputsChanged.nameChanged ||
            inputsChanged.nicknameChanged ||
            inputsChanged.aboutMeChanged ||
            inputsChanged.languageChanged ||
            inputsChanged.enabledChanged ||
            inputsChanged.accountTypeChanged ||
            (changePasswordData.newPassword.length > 0 && repeatPassword.length > 0)
    );

    const minInputLengths = {
        name: 2,
        nickname: 2,
        aboutMe: 0,
        password: 8
    };

    const maxInputLengths = {
        email: 256,
        name: 128,
        nickname: 128,
        aboutMe: 160,
        password: 72
    };

    function resetInputStates() {
        inputsChanged.enabledChanged = false;
        inputsChanged.accountTypeChanged = false;
        inputsChanged.emailChanged = false;
        inputsChanged.nameChanged = false;
        inputsChanged.nicknameChanged = false;
        inputsChanged.aboutMeChanged = false;
        inputsChanged.languageChanged = false;
        inputsChanged.error = false;
    }

    async function handleSubmit(event: SubmitEvent, userData: AdminUpdateData) {
        event.preventDefault();

        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            await goto(appLink('/'));
            return;
        }

        let id = Number(pageId);

        if (Number.isNaN(id)) {
            await goto(appLink('/'));
            return;
        }

        if (!formChanged) {
            return;
        }

        await handlePasswordChange(id);

        const userInfo = getUserAllData(id);
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }

        usernameAlreadyExists = false;
        resetInputStates();

        const result = await updateUserData(id, {
            email: userData.email,
            data: userData.data,
            enabled: userData.enabled,
            accountType: userData.accountType
        });

        if (result.type === 'Success') {
            emailAlreadyExists = false;
            usernameAlreadyExists = false;
        } else if (result.type === 'EmailAlreadyUsed') {
            emailAlreadyExists = true;
            inputsChanged.error = true;
            return;
        } else if (result.type === 'UsernameAlreadyUsed') {
            usernameAlreadyExists = true;
            inputsChanged.error = true;
            return;
        } else {
            throw new Error('ERROR: Unexpected output from API.');
        }

        await goto(appLink('/user-list'));
    }

    async function handlePasswordChange(id: number) {
        successfulPasswordChange = false;

        // Return if the user has not entered all required fields.
        if (!(changePasswordData.newPassword && repeatPassword)) {
            return;
        }

        passwordMissingChar = false;
        passwordNotSame = false;

        if (
            !checkLowercase(changePasswordData.newPassword) ||
            !checkUppercase(changePasswordData.newPassword) ||
            !checkNumber(changePasswordData.newPassword) ||
            !checkSpecialCharacter(changePasswordData.newPassword)
        ) {
            passwordMissingChar = true;
            return;
        }

        if (!checkSamePassword(changePasswordData.newPassword, repeatPassword)) {
            passwordNotSame = true;
            return;
        }

        let changePasswordResponse = await adminChangePassword(id, changePasswordData);

        if (changePasswordResponse.succeeded) {
            successfulPasswordChange = true;
        }

        changePasswordData.newPassword = '';
        repeatPassword = '';
    }

    async function handleCancel(e: Event) {
        e.preventDefault();

        await goto(appLink('/user-list'));
    }
</script>

<div>
    {#await userDataPromise}
        <p>{m.loading()}</p>
    {:then userData}
        <form
            data-testid="profile-form"
            class={[formChanged && 'completed']}
            onsubmit={(event) => handleSubmit(event, userData.updateableData)}
        >
            <h1 class="page__title">
                {m.profileOf({ username: userData.updateableData.data.name })}
            </h1>
            <h2 class="page__heading">{m.general()}</h2>
            <p class="italic">
                {m.registeredOn()}:
                {new Intl.DateTimeFormat(timeFormatString).format(new Date(userData.createdAt))}
            </p>
            <p class="italic">
                {m.lastLogin()}:
                {new Intl.DateTimeFormat(timeFormatString).format(new Date(userData.lastLogin))}
            </p>
            <div>
                <label for="email">{m.email()}</label>
                <div class="input__container">
                    <input
                        id="email"
                        class={[
                            !inputsChanged.emailChanged && 'inactive',
                            emailAlreadyExists && 'error',
                            'input__text'
                        ]}
                        bind:value={userData.updateableData.email}
                        name="email"
                        type="email"
                        data-testid="email"
                        maxlength={maxInputLengths.email}
                        oninput={() => {
                            inputsChanged.emailChanged = true;
                        }}
                        required
                    />
                </div>
                {#if emailAlreadyExists}
                    <p class="text__error" data-testid="error">{m.emailAlreadyExists()}</p>
                {/if}
            </div>
            <div>
                <label for="name">{m.name()}</label>
                <div class="input__container">
                    <input
                        id="name"
                        class={[
                            !inputsChanged.nameChanged && 'inactive',
                            usernameAlreadyExists && 'error',
                            'input__text'
                        ]}
                        bind:value={userData.updateableData.data.name}
                        name="name"
                        type="text"
                        data-testid="name"
                        placeholder={m.examplePlaceholderRequired({ value: m.exampleName() })}
                        minlength={minInputLengths.name}
                        maxlength={maxInputLengths.name}
                        oninput={() => {
                            inputsChanged.nameChanged = true;
                        }}
                        required
                    />
                </div>
                {#if usernameAlreadyExists}
                    <p class="text__error" data-testid="error">{m.usernameAlreadyExists()}</p>
                {/if}
            </div>
            <div>
                <label for="nickname">{m.nickname()}</label>
                <div class="input__container">
                    <input
                        id="nickname"
                        class={[!inputsChanged.nicknameChanged && 'inactive', 'input__text']}
                        bind:value={userData.updateableData.data.nickname}
                        name="nickname"
                        type="text"
                        placeholder={m.examplePlaceholderOptional({ value: m.exampleNickname() })}
                        minlength={minInputLengths.nickname}
                        maxlength={maxInputLengths.nickname}
                        oninput={() => {
                            inputsChanged.nicknameChanged = true;
                        }}
                    />
                </div>
            </div>
            <div>
                <label for="aboutMe">{m.aboutMe()}</label>
                <textarea
                    id="aboutMe"
                    class={[!inputsChanged.aboutMeChanged && 'inactive', 'input__long_text']}
                    bind:value={userData.updateableData.data.aboutMe}
                    name="aboutMe"
                    placeholder={m.optional()}
                    minlength={minInputLengths.aboutMe}
                    maxlength={maxInputLengths.aboutMe}
                    rows="2"
                    data-testid="about-me"
                    oninput={() => {
                        inputsChanged.aboutMeChanged = true;
                    }}></textarea>
            </div>
            <h2 class="page__heading">{m.accountSettings()}</h2>
            <div>
                <label for="accountType">{m.account()}</label>
                <select
                    id="acccountType"
                    class={[!inputsChanged.accountTypeChanged && 'inactive', 'input__select']}
                    bind:value={userData.updateableData.accountType}
                    oninput={() => {
                        inputsChanged.accountTypeChanged = true;
                    }}
                >
                    <option class={['input__select_option']}>admin </option>
                    <option class={['input__select_option']}>author </option>
                </select>
            </div>
            <div class="checkbox__container">
                <input
                    id="enabled"
                    class={[!inputsChanged.enabledChanged && 'inactive', 'input__checkbox']}
                    bind:checked={userData.updateableData.enabled}
                    name="enabled"
                    type="checkbox"
                    oninput={() => {
                        inputsChanged.enabledChanged = true;
                    }}
                />
                <label for="enabled">
                    {m.enabled()}
                </label>
            </div>
            <h2 class="page__heading">{m.changePassword()}</h2>
            <div>
                <label for="newPassword">{m.newPassword()}</label>
                <input
                    id="newPassword"
                    class={[passwordMissingChar && 'error', 'input__text']}
                    bind:value={changePasswordData.newPassword}
                    name="newPassword"
                    type="password"
                    autocomplete="new-password"
                    data-testid="newPassword"
                    placeholder={m.requiredPlaceholder({ value: m.newPassword() })}
                    minlength={minInputLengths.password}
                    maxlength={maxInputLengths.password}
                />
                {#if passwordMissingChar}
                    <p class="text__error" data-testid="missing-char-error">
                        {m.missingCharacter()}
                    </p>
                {/if}
                {#if changePasswordData.newPassword}
                    <input
                        class={[passwordNotSame && 'error', 'input__text']}
                        bind:value={repeatPassword}
                        name="repeatPassword"
                        type="password"
                        autocomplete="new-password"
                        data-testid="repeatPassword"
                        placeholder={m.requiredPlaceholder({ value: m.repeatPassword() })}
                        minlength={minInputLengths.password}
                        maxlength={maxInputLengths.password}
                        required
                    />

                    {#if passwordNotSame}
                        <p class="text__error" data-testid="differing-password-error">
                            {m.passwordsNotTheSame()}
                        </p>
                    {/if}
                {/if}
                {#if successfulPasswordChange}
                    <p class="text__success" data-testid="password-change-success">
                        {m.successfulPasswordChange()}
                    </p>
                {/if}
            </div>
            <div class="mb-0 flex flex-row gap-1 *:w-1/2">
                <button type="submit" data-testid="confirm-button">{m.confirmChanges()}</button>
                <button type="reset" onclick={handleCancel}>{m.cancel()}</button>
            </div>
        </form>
    {/await}
</div>
