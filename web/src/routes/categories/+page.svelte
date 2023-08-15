<script lang="ts">
    import { apiPost } from "$lib/api";
    import Headerbar from "$lib/components/Headerbar.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import type { CategoryGroup } from "$lib/data_types";
    import { categoryArray, categoryGroupArray } from "$lib/db/category";

    let selectedGroup: CategoryGroup | undefined = undefined;

    type NewCategory = {
        name: string;
        icon: string;
        group: CategoryGroup | undefined;
    };

    function createNewCategory(group: CategoryGroup | undefined) {
        newCategory = {
            name: "",
            icon: "",
            group: group,
        };
    }

    let newCategory: NewCategory | undefined = undefined;

    function addNewCategory() {
        if (newCategory === undefined) {
            return;
        }

        let method =
            newCategory.group !== undefined ? "category" : "category_group";

        apiPost(method, {
            name: newCategory.name,
            icon: newCategory.icon,
            groupId: newCategory.group?.id,
        }).then(() => {
            newCategory = undefined;
            location.reload();
        });
    }
</script>

<Headerbar loader_data={["current_user", "category_group", "category"]} />

{#if newCategory === undefined}
    <div class="col side scroll">
        <div class="col spacer scroll">
            {#each $categoryGroupArray as group}
                <button class="row card" on:click={() => (selectedGroup = group)}>
                    <Icon icon={group.icon} />
                    {group.name}
                </button>
            {/each}
        </div>
        <div class="divider"></div>
        <button on:click={() => createNewCategory(undefined)}>
            <Icon icon="add" />
            Neue Gruppe
        </button>
    </div>
{/if}

{#if newCategory !== undefined}
    <div class="col main center">
        <div class="card col stretch">
            <label for="name">Name</label>
            <input type="text" id="name" bind:value={newCategory.name} />
            <label for="icon"><a href="https://fonts.google.com/icons?icon.set=Material+Icons" target="_blank">Icon</a></label>
            <input type="text" id="icon" bind:value={newCategory.icon} />
            <Icon icon={newCategory.icon} />
            {#if newCategory.group !== undefined}
                <div class="row center card light">
                    <Icon icon={newCategory.group.icon} />
                    <span>{newCategory.group.name}</span>
                </div>
            {/if}
        </div>
        <div class="row">
            <button on:click={() => (newCategory = undefined)}>
                <Icon icon="cancel" />
                Abbrechen
            </button>
            <button
                class="primary"
                on:click={addNewCategory}
                disabled={newCategory.name.length == 0 ||
                    newCategory.icon.length == 0}
            >
                <Icon icon="add" />
                Hinzufügen
            </button>
        </div>
    </div>
{:else if selectedGroup !== undefined}
    <div class="col main scroll">
        <div class="col scroll spacer">
            {#each $categoryArray as category}
                {#if category.group === selectedGroup}
                    <div class="row card">
                        <Icon icon={category.icon} />
                        {category.name}
                    </div>
                {/if}
            {/each}
        </div>
        <div class="divider" />
        <button on:click={() => createNewCategory(selectedGroup)}>
            <Icon icon="add" />
            Neue Kategorie
        </button>
    </div>
{/if}
