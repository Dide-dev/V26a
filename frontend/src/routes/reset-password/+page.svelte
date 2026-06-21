<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { type PasswordResetData, resetPassword } from '$lib/api-wrapper/users';
    import { onMount } from 'svelte';
    import {
        checkLowercase,
        checkNumber,
        checkSamePassword,
        checkSpecialCharacter,
        checkUppercase,
        genPassword
    } from '$lib/passwords';

    let passwordResetSuccess: boolean | null = $state(null);
    let passwordResetData: PasswordResetData = $state({
        code: '',
        newPassword: ''
    });
    let repeatPassword = $state('');
    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let formCompleted: boolean = $derived(
        passwordResetData.newPassword !== '' && repeatPassword !== ''
    );

    const minInputLengths = {
        password: 8
    };

    const maxInputLengths = {
        password: 72
    };

    onMount(async () => {
        let passwordResetCode = page.url.searchParams.get('code');

        if (!passwordResetCode) {
            await goto(appLink('/login'));
        }

        passwordResetData.code = passwordResetCode!;
    });

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        passwordMissingChar = false;
        passwordNotSame = false;

        if (
            !checkLowercase(passwordResetData.newPassword) ||
            !checkUppercase(passwordResetData.newPassword) ||
            !checkNumber(passwordResetData.newPassword) ||
            !checkSpecialCharacter(passwordResetData.newPassword)
        ) {
            passwordMissingChar = true;
            return;
        }
        if (!checkSamePassword(passwordResetData.newPassword, repeatPassword)) {
            passwordNotSame = true;
            return;
        }

        let resetResponse = await resetPassword(passwordResetData);
        if (resetResponse.type === 'Success') {
            passwordResetSuccess = true;
        } else if (resetResponse.type === 'InvalidCode') {
            passwordResetSuccess = false;
        } else {
            throw new Error('ERROR: Unexpected output from API.');
        }
    }

    async function gotoLogin(e: Event) {
        e.preventDefault();
        await goto(appLink('/login'));
    }

    async function gotoForgot(e: Event) {
        e.preventDefault();
        await goto(appLink('/forgot-password'));
    }

    // Generate random example password
    const examplePassword = genPassword(12);
</script>

<div>
    {#if passwordResetSuccess === null}
        <!-- Password reset not completed, ask for password until it is good. -->
        <div>
            <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
                <div>
                    <h1 class="page__title">{m.setNewPassword()}</h1>
                    <!-- This is mostly copy-pasted from the registration page -->
                </div>
                <div>
                    <label for="password">
                        {m.password()} *
                    </label>
                    <input
                        id="password"
                        class={[(passwordMissingChar || passwordNotSame) && 'error', 'input__text']}
                        bind:value={passwordResetData.newPassword}
                        name="password"
                        type="password"
                        data-testid="password"
                        placeholder={m.examplePlaceholderRequired({ value: examplePassword })}
                        minlength={minInputLengths.password}
                        maxlength={maxInputLengths.password}
                        required
                    />
                    {#if passwordMissingChar}
                        <p class="text__error" data-testid="error">{m.missingCharacter()}</p>
                    {/if}
                </div>
                <div>
                    <input
                        id="repeatPassword"
                        class={[passwordNotSame && 'error', 'input__text']}
                        bind:value={repeatPassword}
                        name="repeatPassword"
                        type="password"
                        data-testid="repeatPassword"
                        placeholder={m.requiredPlaceholder({ value: m.repeatPassword() })}
                        minlength={minInputLengths.password}
                        maxlength={maxInputLengths.password}
                        required
                    />
                    {#if passwordNotSame}
                        <p class="text__error" data-testid="error">{m.passwordsNotTheSame()}</p>
                    {/if}
                </div>
                <button type="submit">{m.save()}</button>
            </form>
        </div>
    {:else if passwordResetSuccess!}
        <!-- Password reset completed and successful. -->
        <form class="completed">
            <div>
                <h1 class="page__title">{m.resetPasswordSuccessTitle()}</h1>
                <p>{m.resetPasswordSuccessBody()}</p>
            </div>

            <button type="submit" class="cursor-pointer rounded px-3 py-1" onclick={gotoLogin}
                >{m.login()}</button
            >
        </form>
    {:else}
        <!-- Password reset completed, but with an invalid code. -->
        <form class="completed">
            <div>
                <h1 class="page__title">{m.resetPasswordFailureTitle()}</h1>
                <p>{m.resetPasswordFailureBody()}</p>
            </div>

            <div class="mb-0 flex flex-row gap-1 *:w-1/2">
                <button
                    type="submit"
                    class="mb- cursor-pointer rounded px-3 py-1"
                    onclick={gotoLogin}>{m.login()}</button
                >
                <button type="submit" class="cursor-pointer rounded px-3 py-1" onclick={gotoForgot}
                    >{m.retry()}</button
                >
            </div>
        </form>
    {/if}
</div>
