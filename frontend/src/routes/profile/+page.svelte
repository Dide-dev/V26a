<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { locales } from '$lib/paraglide/runtime.js';
    import {
        getOwnUserData,
        getUserInfo,
        updateUser,
        changePassword,
        type UserGetData,
        type ChangePasswordData
    } from '$lib/backend/users';
    import {
        checkSamePassword,
        checkLowercase,
        checkUppercase,
        checkNumber,
        checkSpecialCharacter
    } from '$lib/passwords';
    import { m } from '$lib/paraglide/messages.js';
    import { redirectLogin } from '$lib/backend';
    import { page } from '$app/state';

    let userDataPromise: Promise<UserGetData> = $state(getOwnUserData());

    let changePasswordData: ChangePasswordData = $state({
        oldPassword: '',
        newPassword: ''
    });
    let repeatPassword = $state('');

    // Variable to check whether a user tries to change the password. Returns true if at least one of the change-password fields are filled in, returns false if not or if all three password change fields are filled in.
    const passwordChangeAttempt = $derived(
        (changePasswordData.oldPassword || changePasswordData.newPassword || repeatPassword) &&
            (!changePasswordData.oldPassword || !changePasswordData.newPassword || !repeatPassword)
    );

    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let unsuccesfulPasswordChange: boolean = $state(false);
    let succesfulPasswordChange: boolean = $state(false);
    let sameOldAndNewPassword: boolean = $state(false);

    async function handleSubmit(event: SubmitEvent, userData: UserGetData) {
        event.preventDefault();

        handlePasswordChange();

        const userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }
        await updateUser(userInfo.sub, {
            name: userData.name,
            nickname: userData.nickname,
            aboutMe: userData.aboutMe,
            emailVisible: userData.emailVisible,
            language: userData.language,
            accountType: null,
            enabled: null
        });

        // Effectively reloads the page. This is not really necessary, but prevents
        // the displayed data from being inconsistent with the database in case something
        // goes wrong.
        userDataPromise = getOwnUserData();
    }

    async function handlePasswordChange() {
        // Return if the user is not attempting to change the password at all.
        if (!changePasswordData.oldPassword && !changePasswordData.newPassword && !repeatPassword) {
            return;
        }
        // Return if not all fields are filled in.
        if (!changePasswordData.oldPassword || !changePasswordData.newPassword || !repeatPassword) {
            return;
        }

        passwordMissingChar = false;
        passwordNotSame = false;
        unsuccesfulPasswordChange = false;
        succesfulPasswordChange = false;
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
            unsuccesfulPasswordChange = true;
            return;
        }
        if (changePasswordResponse.succeeded) {
            succesfulPasswordChange = true;
        }

        changePasswordData.oldPassword = '';
        changePasswordData.newPassword = '';
        repeatPassword = '';
    }
</script>

{#await userDataPromise}
    <p>{m.loading()}</p>
{:then userData}
    <div class="flex flex-col items-center">
        <div>
            <h1 class="mb-4 text-3xl font-bold">{m.profile()}</h1>
            <form
                class="flex w-md flex-col gap-2"
                onsubmit={(event) => handleSubmit(event, userData)}
            >
                <div>
                    <p class="font-bold">{m.email()}</p>
                    <p>{userData.email}</p>
                </div>
                <label>
                    <p class="font-bold">{m.name()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={userData.name}
                        name="name"
                        type="text"
                        data-testid="name"
                        maxlength="128"
                        required
                    />
                </label>
                <label>
                    <p class="font-bold">{m.nickname()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={userData.nickname}
                        name="nickname"
                        type="text"
                        placeholder={m.optional()}
                        maxlength="128"
                    />
                </label>
                <label>
                    <p class="font-bold">{m.aboutMe()}</p>
                    <textarea
                        class="w-full rounded-md border-2"
                        bind:value={userData.aboutMe}
                        name="aboutMe"
                        placeholder={m.optional()}
                        maxlength="160"
                        rows="4"
                        data-testid="about-me"
                    ></textarea>
                </label>
                <label>
                    <p class="font-bold">{m.language()}</p>
                    <select class="rounded-md border-2" bind:value={userData.language}>
                        {#each locales as value (value)}
                            <option {value}>{value}</option>
                        {/each}
                    </select>
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        bind:checked={userData.emailVisible}
                        name="emailVisible"
                        type="checkbox"
                        maxlength="128"
                    />
                    {m.emailVisible()}
                </label>
                <h1 class="mt-2 text-3xl font-bold">{m.changePassword()}</h1>
                <label>
                    <p class="font-bold">{m.oldPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={changePasswordData.oldPassword}
                        name="oldPassword"
                        type="password"
                        data-testid="oldPassword"
                        placeholder={m.oldPassword()}
                        minlength="8"
                        maxlength="72"
                    />
                </label>
                <label>
                    <p class="font-bold">{m.newPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={changePasswordData.newPassword}
                        name="newPassword"
                        type="password"
                        data-testid="newPassword"
                        placeholder={m.newPassword()}
                        minlength="8"
                        maxlength="72"
                    />
                </label>
                {#if passwordMissingChar}
                    <p class="text-red-600" data-testid="missing-char-error">
                        {m.missingCharacter()}
                    </p>
                {/if}
                <label>
                    <p class="font-bold">{m.repeatPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={repeatPassword}
                        name="repeatPassword"
                        type="password"
                        data-testid="repeatPassword"
                        placeholder={m.repeatPassword()}
                        minlength="8"
                        maxlength="72"
                    />
                </label>
                {#if passwordNotSame}
                    <p class="text-red-600" data-testid="differing-password-error">
                        {m.passwordsNotTheSame()}
                    </p>
                {/if}
                {#if passwordChangeAttempt}
                    <p class="text-red-600" data-testid="change-attempt-error">
                        {m.changePasswordAttempt()}
                    </p>
                {/if}
                {#if sameOldAndNewPassword}
                    <p class="text-red-600" data-testid="same-old-and-new-password-error">
                        {m.sameOldAndNewPassword()}
                    </p>
                {/if}
                {#if succesfulPasswordChange}
                    <p class="text-green-700" data-testid="password-change-success">
                        {m.succesfulPasswordChange()}
                    </p>
                {/if}
                <button class="mt-4 mb-4 rounded-sm bg-emerald-500 p-1" data-testid="confirm-button"
                    >{m.confirmChanges()}</button
                >
                {#if unsuccesfulPasswordChange}
                    <p class="text-red-600">{m.unsuccesfulPasswordChange()}</p>
                {/if}
            </form>
        </div>
    </div>
{/await}
