<script lang="ts">
    import type { User } from "$lib/data_types";
    import UserView from "./UserView.svelte";

    export let users: User[];
</script>

<div class="player-stack">
    {#each users as user}
        <div class="user-holder">
            <UserView {user} />
        </div>
    {/each}
</div>

<style>
    .player-stack {
        display: flex;
        flex-direction: row;
        padding: 0px calc(var(--user-size) / 4);
    }

    .user-holder {
        display: flex;
        width: calc(var(--user-size) / 2);
        align-items: center;
        justify-content: center;
    }
</style>
