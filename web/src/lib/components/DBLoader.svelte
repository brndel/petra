<script lang="ts">
    import { loadDB } from "$lib/db";
    import { onMount } from "svelte";

    export let data: ("current_user" | "month_index" | "user")[] = ["current_user"];

    let ready = false;

    onMount(async () => {
        for(const d of data) {
            await loadDB(d);
        }
        ready = true;
    });
</script>

<h1>
    {#if ready}
        Hey
    {:else}
        ...
    {/if}
</h1>
