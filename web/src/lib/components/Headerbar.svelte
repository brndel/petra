<script lang="ts">
    import { currentUser, type LoaderMethod } from "$lib/db";
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

    {#if user !== undefined && user !== null}
        <UserView {user} />
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
