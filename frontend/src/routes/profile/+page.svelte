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
        /*changePassword,*/ type UserGetData /*, ChangePasswordData*/
    } from '$lib/backend/users';
    import { m } from '$lib/paraglide/messages.js';
    import { redirectLogin } from '$lib/backend';
    import { page } from '$app/state';

    let userDataPromise: Promise<UserGetData> = $state(getOwnUserData());

    type ChangePasswordData = {
        oldPassword: string;
        newPassword: string;
    };

    let changePasswordData: ChangePasswordData = $state({
        oldPassword: '',
        newPassword: ''
    });
    let repeatPassword = $state('');
    const passwordChangeAttempt = $derived(
        (changePasswordData.oldPassword || changePasswordData.newPassword || repeatPassword) &&
            !(!changePasswordData.oldPassword || !changePasswordData.newPassword || !repeatPassword)
    );
    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);

    function checkSamePassword(): boolean {
        return repeatPassword == changePasswordData.newPassword;
    }

    function checkLowercase(): boolean {
        let passwordChecker = changePasswordData.newPassword.toUpperCase();
        return passwordChecker != changePasswordData.newPassword;
    }

    function checkUppercase(): boolean {
        let passwordChecker = changePasswordData.newPassword.toLowerCase();
        return passwordChecker != changePasswordData.newPassword;
    }

    function checkNumber(): boolean {
        let regex = /\d/;
        return regex.test(changePasswordData.newPassword);
    }

    function checkSpecialCharacter(): boolean {
        let regex = /[?!@#$%^&*()+=.,<>{}[\]:;"'|~`_/\\-]/;
        return regex.test(changePasswordData.newPassword);
    }

    async function handleSubmit(event: SubmitEvent, userData: UserGetData) {
        event.preventDefault();

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

        handlePasswordChange();

        // Effectively reloads the page. This is not really necessary, but prevents
        // the displayed data from being inconsistent with the database in case something
        // goes wrong.
        userDataPromise = getOwnUserData();
    }

    async function handlePasswordChange() {
        if (!changePasswordData.oldPassword && !changePasswordData.newPassword && !repeatPassword) {
            return;
        }

        passwordMissingChar = false;
        passwordNotSame = false;

        if (!checkLowercase() || !checkUppercase() || !checkNumber() || !checkSpecialCharacter()) {
            passwordMissingChar = true;
            return;
        }
        if (!checkSamePassword()) {
            passwordNotSame = true;
            return;
        }

        // await changePassword(changePasswordData);
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
                <label>
                    <p class="font-bold">{m.oldPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={changePasswordData.oldPassword}
                        name="oldPassword"
                        type="password"
                        placeholder={m.oldPassword()}
                        maxlength="128"
                    />
                </label>
                <label>
                    <p class="font-bold">{m.newPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={changePasswordData.newPassword}
                        name="newPassword"
                        type="password"
                        placeholder={m.newPassword()}
                        maxlength="128"
                    />
                </label>
                {#if passwordMissingChar}
                    <p class="text-red-600">{m.missingCharacter()}</p>
                {/if}
                <label>
                    <p class="font-bold">{m.repeatPassword()}</p>
                    <input
                        class="w-full rounded-md border-2"
                        bind:value={repeatPassword}
                        name="repeatPassword"
                        type="password"
                        placeholder={m.repeatPassword()}
                        maxlength="128"
                    />
                </label>
                {#if passwordNotSame}
                    <p class="text-red-600">{m.passwordsNotTheSame()}</p>
                {/if}
                {#if passwordChangeAttempt}
                    <p class="text-red-600">{m.changePasswordAttempt()}</p>
                {/if}

                <button class="rounded-sm bg-emerald-500 p-1">{m.confirmChanges()}</button>
            </form>
        </div>
    </div>
{/await}
