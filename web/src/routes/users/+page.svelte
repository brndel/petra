<script lang="ts">
    import { apiPost } from "$lib/api";
    import Headerbar from "$lib/components/Headerbar.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import UserView from "$lib/components/UserView.svelte";
    import { currentUser, userArray } from "$lib/db/user";

    type NewUserData = {
        displayName: string;
        name: string;
        password: string;
        passwordRepeat: string | undefined;
    };

    function openUserMenu() {
        userData = {
            displayName: "",
            name: "",
            password: "",
            passwordRepeat: "",
        };
    }

    let userData: NewUserData | undefined = undefined;

    function addUser() {
        console.log(userData);
        if (userData === undefined) {
            return;
        }

        console.log(userData);

        userData.passwordRepeat = undefined;

        apiPost("user", userData).then(() => {
            location.reload();
        });
    }
</script>

<Headerbar loader_data={["current_user", "user"]} />

<div class="col side stretch scroll">
    {#each $userArray as user}
        <div class="card row center">
            <UserView userName={user.userName} />
            <span>{user.name}</span>
            {#if user === $currentUser}
                <div class="spacer" />
                <Icon icon="person" tooltip="Du" tooltipLocation="left"/>
            {/if}
        </div>
    {/each}
    <div class="spacer" />
    <button class="card row center" on:click={openUserMenu}>
        <Icon icon="add"/>
        Nutzer hinzufügen
    </button>
</div>

{#if userData !== undefined}
    <div class="col main center">
        <div class="card col stretch">
            <label for="displayname">Anzeigename</label>
            <input
                type="text"
                id="displayname"
                bind:value={userData.displayName}
            />
            <label for="username">Nutzername</label>
            <input type="text" id="username" bind:value={userData.name} />
            <label for="password">Passwort</label>
            <input
                type="password"
                id="password"
                bind:value={userData.password}
            />
            <label for="password-repeat">Passwort wiederholen</label>
            <input
                type="password"
                id="password-repeat"
                bind:value={userData.passwordRepeat}
            />
        </div>
        <button
        class="primary"
            disabled={userData.password.length < 3 || userData.password !== userData.passwordRepeat}
            on:click={addUser}
        >
            <Icon icon="add" />
            Hinzufügen
        </button>
    </div>
{/if}