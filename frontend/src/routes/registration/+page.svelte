<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { ParaglideMessage } from '@inlang/paraglide-js-svelte';
    import { getLocale } from '$lib/paraglide/runtime.js';
    import { addUser, type RegisterData } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';

    // Page information
    let formData: RegisterData = $state({
        email: '',
        password: '',
        name: '',
        nickname: '',
        aboutMe: '',
        emailVisible: false,
        language: getLocale()
    });
    let repeatPassword = $state('');
    let agreedToTOS = $state(false);
    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let emailAlreadyExists: boolean = $state(false);
    let formCompleted: boolean = $derived(
        formData.email !== '' &&
            formData.password !== '' &&
            formData.name !== '' &&
            repeatPassword !== '' &&
            agreedToTOS
    );

    const minInputLengths = {
        name: 2,
        nickname: 2,
        email: 0,
        password: 8
    };

    const maxInputLengths = {
        name: 128,
        nickname: 128,
        email: 256,
        password: 72
    };

    // Validate input
    function checkSamePassword(): boolean {
        return repeatPassword == formData.password;
    }

    function checkLowercase(): boolean {
        let passwordChecker = formData.password.toUpperCase();
        return passwordChecker != formData.password;
    }

    function checkUppercase(): boolean {
        let passwordChecker = formData.password.toLowerCase();
        return passwordChecker != formData.password;
    }

    function checkNumber(): boolean {
        let regex = /\d/;
        return regex.test(formData.password);
    }

    function checkSpecialCharacter(): boolean {
        let regex = /[?!@#$%^&*()+=.,<>{}[\]:;"'|~`_/\\-]/;
        return regex.test(formData.password);
    }

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        passwordMissingChar = false;
        passwordNotSame = false;
        emailAlreadyExists = false;

        if (!checkLowercase() || !checkUppercase() || !checkNumber() || !checkSpecialCharacter()) {
            passwordMissingChar = true;
            return;
        }
        if (!checkSamePassword()) {
            passwordNotSame = true;
            return;
        }

        let addResponse = await addUser(formData);
        if (addResponse.type === 'Success') {
            emailAlreadyExists = false;
            await goto(appLink('/login'));
        } else if (addResponse.type === 'EmailAlreadyUsed') {
            emailAlreadyExists = true;
        } else {
            throw new Error('ERROR: Unexpected output from API.');
        }
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
    <h1 class="page__title">{m.register()}</h1>

    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <label for="name">
                {m.name()} *
            </label>
            <input
                id="name"
                class="input__text"
                bind:value={formData.name}
                name="name"
                type="text"
                data-testid="name"
                placeholder={m.examplePlaceholder() + m.exampleName() + m.requiredPlaceholder()}
                minlength={minInputLengths.name}
                maxlength={maxInputLengths.name}
                autocomplete="name"
                required
            />
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
                placeholder={m.examplePlaceholder() + 'Johnny'}
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
                placeholder={m.examplePlaceholder() +
                    m.emailPlaceholder() +
                    m.requiredPlaceholder()}
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
                placeholder={m.examplePlaceholder() + examplePassword + m.requiredPlaceholder()}
                minlength={minInputLengths.password}
                maxlength={maxInputLengths.password}
                required
            />
            {#if passwordMissingChar}
                <p class="text__error" data-testid="error">{m.missingCharacter()}</p>
            {/if}
            {#if formData.password}
                <input
                    id="repeatPassword"
                    class={[passwordNotSame && 'error', 'input__text']}
                    bind:value={repeatPassword}
                    name="repeat password"
                    type="password"
                    data-testid="repeatPassword"
                    placeholder={m.repeatPassword() + m.requiredPlaceholder()}
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
                    bind:checked={formData.emailVisible}
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
            <div class="checkbox__container">
                <input
                    id="tos"
                    class="input__checkbox"
                    bind:checked={agreedToTOS}
                    name="tos"
                    type="checkbox"
                    data-testid="agreeToTOS"
                    required
                />
                <label for="tos">
                    <ParaglideMessage message={m.agreeToTOS} inputs={{}}>
                        {#snippet link({ children, options })}
                            <a class="text__link" target="_blank" href={options.to}>
                                {@render children?.()}
                            </a>
                        {/snippet}
                    </ParaglideMessage> *
                </label>
            </div>
        </div>
        <button>{m.register()}</button>
    </form>
</div>
