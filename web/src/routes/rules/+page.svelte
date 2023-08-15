<script lang="ts">
    import Headerbar from "$lib/components/Headerbar.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import RuleView from "$lib/components/RuleView.svelte";
    import type { Rule } from "$lib/data_types";
    import { ruleArray } from "$lib/db/rule";
    import { space } from "svelte/internal";

    let selectedRule: Rule | undefined;
</script>

<Headerbar
    loader_data={["current_user", "rule", "category_group", "category"]}
/>
<div class="col side scroll">
    {#each $ruleArray as rule}
        <button class="row card" on:click={() => (selectedRule = rule)}>
            {rule.name}
        </button>
    {/each}
</div>

{#if selectedRule !== undefined}
    <div class="main col">
        <h1>{selectedRule.name}</h1>
        <div class="divider" />
        <h2>Schlüsselwörter</h2>
        <div class="row">
            {#each selectedRule.keywords as keyword}
                <div class="tag">{keyword}</div>
            {/each}
        </div>
        <h2>Kategorien</h2>
        <div class="row">
            {#each selectedRule.categories as category}
                <div class="row tag">
                    <Icon icon={category.icon} />
                    {category.name}
                </div>
            {/each}
        </div>
        <h2>Geteilt</h2>
        <div class="row">
            <div class="tag">
                {#if selectedRule.shareRule === true}
                Ja
                {:else if selectedRule.shareRule === false}
                Nein
                {:else}
                Vielleicht
                {/if}
            </div>
        </div>
    </div>
{/if}

<style lang="scss">
    .tag {
        background-color: var(--surface-light);
        color: var(--surface-on);
        padding: var(--small);
        border-radius: var(--small);
    }
</style>
