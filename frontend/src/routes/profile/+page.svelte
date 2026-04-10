<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { locales } from '$lib/paraglide/runtime.js';
    import { getOwnUserData, getUserInfo, updateUser, type UserGetData } from '$lib/backend/users';
    import { m } from '$lib/paraglide/messages.js';

    let error: null | Error = $state(null);
    let userDataPromise: Promise<UserGetData> = $state(getOwnUserData());

    async function handleSubmit(event: SubmitEvent, userData: UserGetData) {
        event.preventDefault();
        const userInfo = getUserInfo();
        if (userInfo === null) {
            error = new Error('You are not logged in!');
        } else {
            try {
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
            } catch (e) {
                error = e as Error;
            }
        }
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
                <button class="rounded-sm bg-emerald-500 p-1">{m.confirmChanges()}</button>
                <p class="text-red-600">{error?.message}</p>
            </form>
        </div>
    </div>
{:catch error}
    <p style="color:red">{error.message}</p>
{/await}
