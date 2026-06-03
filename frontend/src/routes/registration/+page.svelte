<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { ParaglideMessage } from '@inlang/paraglide-js-svelte';
    import { getLocale } from '$lib/paraglide/runtime.js';
    import { addUser, type RegisterData } from '$lib/api-wrapper/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import {
        checkSamePassword,
        checkLowercase,
        checkUppercase,
        checkNumber,
        checkSpecialCharacter
    } from '$lib/passwords';
    import type { Snippet } from 'svelte';

    // Page information
    let formData: RegisterData = $state({
        data: {
            name: '',
            nickname: '',
            aboutMe: '',
            emailVisible: false,
            language: getLocale()
        },
        email: '',
        password: ''
    });
    let repeatPassword = $state('');
    let agreedToTOS = $state(false);
    let registrationPending = $state(false);
    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let usernameAlreadyExists: boolean = $state(false);
    let emailAlreadyExists: boolean = $state(false);
    let formCompleted: boolean = $derived(
        formData.email !== '' &&
            formData.password !== '' &&
            formData.data.name !== '' &&
            repeatPassword !== '' &&
            agreedToTOS
    );

    const minInputLengths = {
        name: 4,
        nickname: 2,
        email: 0,
        password: 8
    };

    const maxInputLengths = {
        name: 32,
        nickname: 32,
        email: 128,
        password: 72
    };

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        passwordMissingChar = false;
        passwordNotSame = false;
        emailAlreadyExists = false;
        usernameAlreadyExists = false;

        if (
            !checkLowercase(formData.password) ||
            !checkUppercase(formData.password) ||
            !checkNumber(formData.password) ||
            !checkSpecialCharacter(formData.password)
        ) {
            passwordMissingChar = true;
            return;
        }
        if (!checkSamePassword(formData.password, repeatPassword)) {
            passwordNotSame = true;
            return;
        }

        let addResponse = await addUser(formData);
        if (addResponse.type === 'Pending') {
            emailAlreadyExists = false;
            usernameAlreadyExists = false;
            registrationPending = true;
        } else if (addResponse.type === 'EmailAlreadyUsed') {
            emailAlreadyExists = true;
        } else if (addResponse.type === 'UsernameAlreadyUsed') {
            usernameAlreadyExists = true;
        } else {
            throw new Error('ERROR: Unexpected output from API.');
        }
    }

    async function gotoLogin(e: Event) {
        e.preventDefault();
        await goto(appLink('/login'));
    }

    // Generate random example password
    const examplePassword = genPassword(12);

    function genPassword(length: number): string {
        const lowerChars = 'abcdefghijklmnopqrstuvwxyz';
        const upperChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
        const numChars = '0123456789';
        const specialChars = '?!@#$%&*()+=.,<>{}[]|_-';
        const alphaChars = lowerChars + upperChars;
        const allChars = lowerChars + upperChars + numChars + specialChars;

        const randomIndex = Math.floor(Math.random() * alphaChars.length);
        let password = alphaChars[randomIndex];

        for (let i = 1; i < length; i++) {
            const randomIndex = Math.floor(Math.random() * allChars.length);
            password += allChars[randomIndex];
        }

        return password;
    }
</script>

<div>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">{m.register()}</h1>
        </div>
        <div>
            <label for="name">
                {m.name()} *
            </label>
            <input
                id="name"
                class={[usernameAlreadyExists && 'error', 'input__text']}
                bind:value={formData.data.name}
                name="username"
                type="text"
                data-testid="name"
                placeholder={m.examplePlaceholderRequired({ value: m.exampleName() })}
                minlength={minInputLengths.name}
                maxlength={maxInputLengths.name}
                autocomplete="username"
                required
                pattern="[a-zA-Z0-9._\-]*"
                title={m.allowedRegex()}
            />
            {#if usernameAlreadyExists}
                <p class="text__error" data-testid="error">{m.usernameAlreadyExists()}</p>
            {/if}
        </div>
        <div>
            <label for="nickname">
                {m.nickname()}
            </label>
            <input
                id="nickname"
                class="input__text"
                bind:value={formData.data.nickname}
                name="name"
                type="text"
                placeholder={m.examplePlaceholder({ value: m.exampleNickname() })}
                minlength={minInputLengths.nickname}
                maxlength={maxInputLengths.nickname}
                autocomplete="name"
            />
        </div>
        <div>
            <label for="email">
                {m.email()} *
            </label>
            <input
                id="email"
                class={[emailAlreadyExists && 'error', 'input__text']}
                bind:value={formData.email}
                name="email"
                type="email"
                data-testid="email"
                placeholder={m.examplePlaceholderRequired({ value: m.emailPlaceholder() })}
                minlength={minInputLengths.email}
                maxlength={maxInputLengths.email}
                autocomplete="email"
                required
            />
            {#if emailAlreadyExists}
                <p class="text__error" data-testid="error">{m.emailAlreadyExists()}</p>
            {/if}
        </div>
        <div>
            <label for="password">
                {m.password()} *
            </label>
            <input
                id="password"
                class={[(passwordMissingChar || passwordNotSame) && 'error', 'input__text']}
                bind:value={formData.password}
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
            {#if formData.password}
                <input
                    id="name"
                    class={[usernameAlreadyExists && 'error', 'input__text']}
                    bind:value={formData.name}
                    name="name"
                    type="text"
                    data-testid="name"
                    placeholder={m.examplePlaceholderRequired({ value: m.exampleName() })}
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    autocomplete="name"
                    required
                />
                {#if usernameAlreadyExists}
                    <p class="text__error" data-testid="error">{m.usernameAlreadyExists()}</p>
                {/if}
            </div>
            <div>
                <label for="nickname">
                    {m.nickname()}
                </label>
                <input
                    id="nickname"
                    class="input__text"
                    bind:value={formData.nickname}
                    name="nickname"
                    type="text"
                    placeholder={m.examplePlaceholder({ value: m.exampleNickname() })}
                    minlength={minInputLengths.nickname}
                    maxlength={maxInputLengths.nickname}
                />
            </div>
            <div>
                <label for="email">
                    {m.email()} *
                </label>
                <input
                    id="email"
                    class={[emailAlreadyExists && 'error', 'input__text']}
                    bind:value={formData.email}
                    name="email"
                    type="email"
                    data-testid="email"
                    placeholder={m.examplePlaceholderRequired({ value: m.emailPlaceholder() })}
                    minlength={minInputLengths.email}
                    maxlength={maxInputLengths.email}
                    autocomplete="email"
                    required
                />
                {#if emailAlreadyExists}
                    <p class="text__error" data-testid="error">{m.emailAlreadyExists()}</p>
                {/if}
            </div>
            <div>
                <label for="password">
                    {m.password()} *
                </label>
                <input
                    id="password"
                    class={[(passwordMissingChar || passwordNotSame) && 'error', 'input__text']}
                    bind:value={formData.password}
                    name="password"
                    type="password"
                    autocomplete="new-password"
                    data-testid="password"
                    placeholder={m.examplePlaceholderRequired({ value: examplePassword })}
                    minlength={minInputLengths.password}
                    maxlength={maxInputLengths.password}
                    required
                />
            {/if}
            {#if passwordNotSame}
                <p class="text__error" data-testid="error">{m.passwordsNotTheSame()}</p>
            {/if}
        </div>
        <div>
            <div class="checkbox__container">
                <input
                    id="emailVisible"
                    class="input__checkbox"
                    bind:checked={formData.data.emailVisible}
                    name="emailVisible"
                    type="checkbox"
                />
                <label for="emailVisible">
                    {m.emailVisible()}
                </label>
                <div class="info">
                    <sup>🛈</sup>
                    <div class="info__content">{m.emailVisibleInfo()}</div>
                </div>
            </div>
            <button>{m.register()}</button>
        </form>
    {:else}
        <form class="completed">
            <div>
                <h1 class="page__title">{m.pendingRegistrationTitle()}</h1>
                <p>
                    <ParaglideMessage
                        message={m.pendingRegistrationBody}
                        inputs={{ mail: formData.email }}
                    >
                        {#snippet link({ children }: { children?: Snippet })}
                            <a class="text__link" href="mailto:{formData.email}">
                                {@render children?.()}
                            </a>
                        {/snippet}
                    </ParaglideMessage>
                </p>
            </div>

            <button class="cursor-pointer rounded px-3 py-1" onclick={gotoLogin}>{m.login()}</button
            >
        </form>
    {/if}
</div>
