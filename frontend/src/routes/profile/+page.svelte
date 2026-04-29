<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { locales } from '$lib/paraglide/runtime.js';
    import { getOwnUserData, getUserInfo, updateUser, type UserGetData } from '$lib/backend/users';
    import { m } from '$lib/paraglide/messages.js';
    import { redirectLogin } from '$lib/backend';
    import { page } from '$app/state';
    import { onMount } from 'svelte';

    let userDataPromise = $state(getOwnUserData());

    let inputsChanged = $state({
        emailChanged: false,
        nameChanged: false,
        nicknameChanged: false,
        aboutMeChanged: false,
        languageChanged: false,
        emailVisibleChanged: false
    });

    let formChanged: boolean = $derived(
        inputsChanged.emailChanged ||
        inputsChanged.nameChanged ||
        inputsChanged.nicknameChanged ||
        inputsChanged.aboutMeChanged ||
        inputsChanged.languageChanged ||
        inputsChanged.emailVisibleChanged
    );

    let languageOpen = $state(false);

    const minInputLengths = {
        email: 0,
        name: 2,
        nickname: 2,
        aboutMe: 0
    };

    const maxInputLengths = {
        email: 256,
        name: 128,
        nickname: 128,
        aboutMe: 160
    };

    function resetInputStates() {
        inputsChanged.emailChanged = false;
        inputsChanged.nameChanged = false;
        inputsChanged.nicknameChanged = false;
        inputsChanged.aboutMeChanged = false;
        inputsChanged.languageChanged = false;
        inputsChanged.emailVisibleChanged = false;
    }

    async function handleSubmit(event: SubmitEvent, userData: UserGetData) {
        event.preventDefault();

        if (!formChanged) {
            return;
        }

        const userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }

        resetInputStates();

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
</script>

<div>
    {#await userDataPromise}
        <p>{m.loading()}</p>
    {:then userData}
        <h1 class="page__title">{m.profile()}</h1>
        <form
            class={[formChanged && 'completed']}
            onsubmit={(event) => handleSubmit(event, userData)}
        >
            <h2 class="page__heading">{m.general()}</h2>
            <div>
                <label for="email">{m.email()}</label>
                <input
                    id="email"
                    class={[!inputsChanged.emailChanged && 'inactive', 'input__text']}
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
            </div>
            <div>
                <label for="name">{m.name()}</label>
                <input
                    id="name"
                    class={[!inputsChanged.nameChanged && 'inactive', 'input__text']}
                    bind:value={userData.name}
                    name="name"
                    type="text"
                    data-testid="name"
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    oninput={() => {
                        inputsChanged.nameChanged = true;
                    }}
                    required
                />
            </div>
            <div>
                <label for="nickname">{m.nickname()}</label>
                <input
                    id="nickname"
                    class={[!inputsChanged.nicknameChanged && 'inactive', 'input__text']}
                    bind:value={userData.nickname}
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
            <div>
                <label for="aboutMe">{m.aboutMe()}</label>
                <textarea
                    id="aboutMe"
                    class={[!inputsChanged.aboutMeChanged && 'inactive', 'input__long_text']}
                    bind:value={userData.aboutMe}
                    name="aboutMe"
                    placeholder={m.optional()}
                    minlength={minInputLengths.aboutMe}
                    maxlength={maxInputLengths.aboutMe}
                    rows="4"
                    data-testid="about-me"
                    oninput={() => {
                        inputsChanged.aboutMeChanged = true;
                    }}
                ></textarea>
            </div>
            <div>
                <label for="language">{m.language()}</label>
                <select
                    id="language"
                    class={[
                        !inputsChanged.languageChanged && 'inactive',
                        languageOpen && 'open',
                        'input__select'
                    ]}
                    bind:value={userData.language}
                    oninput={() => {
                        inputsChanged.languageChanged = true;
                        languageOpen = false;
                    }}
                    onblur={() => {
                        languageOpen = false;
                    }}
                    onmousedown={() => {
                        languageOpen = !languageOpen;
                    }}
                >
                    {#each locales.filter((l) => l !== userData.language) as value (value)}
                        <option class="input__select_option last:rounded-md">
                            {value}
                        </option>
                    {/each}
                    <option class="hidden">{userData.language}</option>
                </select>
            </div>
            <div class="checkbox__container">
                <input
                    id="emailVisible"
                    class={[!inputsChanged.emailVisibleChanged && 'inactive', 'input__checkbox']}
                    bind:checked={userData.emailVisible}
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
            <div class="mb-0 flex flex-row gap-1 *:w-1/2">
                <button data-testid="submit">{m.confirmChanges()}</button>
                <button
                    onclick={(e) => {
                        e.preventDefault();
                        resetInputStates();
                        userDataPromise = getOwnUserData();
                    }}>{m.cancel()}</button
                >
            </div>
        </form>
    {/await}
</div>
