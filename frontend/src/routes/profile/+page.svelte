<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { locales } from '$lib/paraglide/runtime.js';
    import DeleteButton from './DeleteButton.svelte';
    import {
        getOwnUserData,
        getUserInfo,
        updateUser,
        changePassword,
        type UserData,
        type ChangePasswordData
    } from '$lib/api-wrapper/users';
    import {
        checkSamePassword,
        checkLowercase,
        checkUppercase,
        checkNumber,
        checkSpecialCharacter
    } from '$lib/passwords';
    import { m } from '$lib/paraglide/messages.js';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';

    let userDataPromise = $state(getOwnUserData());

    let changePasswordData: ChangePasswordData = $state({
        oldPassword: '',
        newPassword: ''
    });
    let repeatPassword = $state('');

    let incorrectPassword: boolean = $state(false);
    let passwordMissingChar: boolean = $state(false);
    let sameOldAndNewPassword: boolean = $state(false);
    let usernameAlreadyExists: boolean = $state(false);
    let emailAlreadyExists: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let successfulPasswordChange: boolean = $state(false);

    let inputsChanged = $state({
        emailChanged: false,
        nameChanged: false,
        nicknameChanged: false,
        aboutMeChanged: false,
        languageChanged: false,
        emailVisibleChanged: false,
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
            inputsChanged.emailVisibleChanged ||
            (changePasswordData.oldPassword.length > 0 &&
                changePasswordData.newPassword.length > 0 &&
                repeatPassword.length > 0)
    );

    const minInputLengths = {
        email: 3,
        name: 4,
        nickname: 2,
        aboutMe: 0,
        password: 8,
        oldPassword: 1
    };

    const maxInputLengths = {
        email: 128,
        name: 32,
        nickname: 32,
        aboutMe: 512,
        password: 72
    };

    function resetInputStates() {
        inputsChanged.emailChanged = false;
        inputsChanged.nameChanged = false;
        inputsChanged.nicknameChanged = false;
        inputsChanged.aboutMeChanged = false;
        inputsChanged.languageChanged = false;
        inputsChanged.emailVisibleChanged = false;
        inputsChanged.error = false;
    }

    function resetInputResponses() {
        incorrectPassword = false;
        passwordMissingChar = false;
        sameOldAndNewPassword = false;
        usernameAlreadyExists = false;
        emailAlreadyExists = false;
        passwordNotSame = false;
        successfulPasswordChange = false;
    }

    async function handleSubmit(event: SubmitEvent, userData: UserData) {
        event.preventDefault();

        if (!formChanged) {
            return;
        }

        await handlePasswordChange();

        const userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }

        usernameAlreadyExists = false;
        resetInputStates();

        const result = await updateUser({
            name: userData.name,
            nickname: userData.nickname,
            aboutMe: userData.aboutMe,
            emailVisible: userData.emailVisible,
            language: userData.language
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

        // Effectively reloads the page. This is not really necessary, but prevents
        // the displayed data from being inconsistent with the database in case something
        // goes wrong.
        userDataPromise = getOwnUserData();
    }

    async function handlePasswordChange() {
        successfulPasswordChange = false;

        // Return if the user has not entered all required fields.
        if (!(changePasswordData.oldPassword && changePasswordData.newPassword && repeatPassword)) {
            return;
        }

        passwordMissingChar = false;
        passwordNotSame = false;
        incorrectPassword = false;
        sameOldAndNewPassword = false;

        if (changePasswordData.oldPassword == changePasswordData.newPassword) {
            sameOldAndNewPassword = true;
            return;
        }

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

        let changePasswordResponse = await changePassword(changePasswordData);

        if (!changePasswordResponse.succeeded) {
            incorrectPassword = true;
            return;
        }
        if (changePasswordResponse.succeeded) {
            successfulPasswordChange = true;
        }

        changePasswordData.oldPassword = '';
        changePasswordData.newPassword = '';
        repeatPassword = '';
    }

    function handleCancel(e: Event) {
        e.preventDefault();
        if (!formChanged) return;
        resetInputStates();
        resetInputResponses();
        changePasswordData.oldPassword = '';
        changePasswordData.newPassword = '';
        repeatPassword = '';
        userDataPromise = getOwnUserData();
    }
</script>

<div>
    {#await userDataPromise}
        <p>{m.loading()}</p>
    {:then userData}
        <form
            data-testid="profile-form"
            class={[formChanged && 'completed']}
            onsubmit={(event) => handleSubmit(event, userData.data)}
        >
            <h1 class="page__title">{m.profile()}</h1>
            <h2 class="page__heading">{m.general()}</h2>
            <div>
                <label for="email">{m.email()}</label>
                <input
                    id="email"
                    class={[
                        !inputsChanged.emailChanged && 'inactive',
                        emailAlreadyExists && 'error',
                        'input__text'
                    ]}
                    name="email"
                    type="email"
                    data-testid="email"
                    minlength={minInputLengths.email}
                    maxlength={maxInputLengths.email}
                    value={userData.email}
                    oninput={() => {
                        inputsChanged.emailChanged = true;
                    }}
                    disabled
                />
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
                        bind:value={userData.data.name}
                        name="username"
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
                        bind:value={userData.data.nickname}
                        name="name"
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
                    bind:value={userData.data.aboutMe}
                    name="aboutMe"
                    placeholder={m.optional()}
                    minlength={minInputLengths.aboutMe}
                    maxlength={maxInputLengths.aboutMe}
                    rows="2"
                    data-testid="about-me"
                    oninput={() => {
                        inputsChanged.aboutMeChanged = true;
                    }}
                ></textarea>
            </div>
            <h2 class="page__heading">{m.accountSettings()}</h2>
            <div>
                <label for="language">{m.language()}</label>
                <select
                    id="language"
                    class={[!inputsChanged.languageChanged && 'inactive', 'input__select']}
                    bind:value={userData.data.language}
                    oninput={() => {
                        inputsChanged.languageChanged = true;
                    }}
                >
                    {#each locales as value (value)}
                        <option class={['input__select_option']}>
                            {value}
                        </option>
                    {/each}
                </select>
            </div>
            <div class="checkbox__container">
                <input
                    id="emailVisible"
                    class={[!inputsChanged.emailVisibleChanged && 'inactive', 'input__checkbox']}
                    bind:checked={userData.data.emailVisible}
                    name="emailVisible"
                    type="checkbox"
                    oninput={() => {
                        inputsChanged.emailVisibleChanged = true;
                    }}
                />
                <label for="emailVisible">
                    {m.emailVisible()}
                </label>
                <div class="info">
                    <sup>🛈</sup>
                    <div class="info__content">{m.emailVisibleInfo()}</div>
                </div>
            </div>
            <h2 class="page__heading">{m.changePassword()}</h2>
            <div>
                <label for="oldPassword">{m.oldPassword()}</label>
                <input
                    id="oldPassword"
                    class={[(incorrectPassword || sameOldAndNewPassword) && 'error', 'input__text']}
                    bind:value={changePasswordData.oldPassword}
                    name="oldPassword"
                    type="password"
                    autocomplete="current-password"
                    data-testid="oldPassword"
                    placeholder={m.requiredPlaceholder({ value: m.oldPassword() })}
                    minlength={minInputLengths.oldPassword}
                />
                {#if incorrectPassword}
                    <p class="text__error">{m.incorrectPassword()}</p>
                {/if}
                <label for="newPassword">{m.newPassword()}</label>
                <input
                    id="newPassword"
                    class={[
                        (sameOldAndNewPassword || passwordMissingChar || passwordNotSame) &&
                            'error',
                        'input__text'
                    ]}
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
                {#if sameOldAndNewPassword}
                    <p class="text__error" data-testid="same-old-and-new-password-error">
                        {m.sameOldAndNewPassword()}
                    </p>
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
            <DeleteButton />
        </form>
    {/await}
</div>
