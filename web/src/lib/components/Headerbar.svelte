<script lang="ts">
    import type { LoaderMethod } from "$lib/db/loader";
    import { currentUser } from "$lib/db/user";
    import DbLoader from "./DBLoader.svelte";
    import Icon from "./Icon.svelte";
    import UserView from "./UserView.svelte";

    export let loader_data: LoaderMethod[];

    $: user = $currentUser;
</script>

<header class="row center">
    <DbLoader data={loader_data} />
    <div class="spacer" />

    <a href="add"><Icon icon="add" tooltip="Neu" tooltipLocation="bottom"/></a>
    <a href="users"><Icon icon="group" tooltip="Benutzer" tooltipLocation="bottom"/></a>

    {#if user !== undefined && user !== null}
        <UserView userName={user.userName} />
    {/if}
</header>

<style>
    header {
        background-color: var(--surface);
        color: var(--surface-on);
        padding: var(--small);
    }


    a {
        background-color: var(--button-light);
        color: var(--button-on);
        text-decoration: none;
        border-radius: 50%;
    }
</style>
