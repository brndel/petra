<script lang="ts">
    import type { Rule } from "$lib/data_types";

    export let rule: Rule;
</script>

<div class="rule">
    <span>{rule.name}</span>
</div>

<style>
    .rule {
        background-color: var(--surface);
        border-radius: var(--small);
        padding: var(--small);
    }
</style>
