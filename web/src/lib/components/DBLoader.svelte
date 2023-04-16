<script lang="ts">
    import { loadDB, type LoaderMethod } from "$lib/db";
    import { onMount } from "svelte";

    export let data: LoaderMethod[] = ["current_user"];

    let ready = false;

    onMount(async () => {
        for (const d of data) {
            await loadDB(d);
        }
        ready = true;
    });
</script>

<h1>
    {#if ready}
        <a href="/"> Petra </a>
    {:else}
        ...
    {/if}
</h1>

<style>
    a {
        color: var(--surface-on);
        text-decoration: none;
    }
</style>
