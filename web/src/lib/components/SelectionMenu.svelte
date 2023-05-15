<script lang="ts">
  import UserView from "./UserView.svelte";

  import Icon from "./Icon.svelte";
  import UserStack from "./UserStack.svelte";

  type Group = { name: string; icon: string };
  type T = $$Generic<{
    name: string;
    icon: string;
    userName?: string;
    group?: { name: string; icon: string };
  }>;

  export let options: T[];
  export let values: T[];

  let className = "";
  export {className as class};

  export let toggle: (value: T) => void;

  export let menuPosition: "left" | "right" = "right";

  let hovered = false;
  let searchbarElement: HTMLInputElement | null | undefined;

  let filterQuery = "";

  $: sortedValues = values.sort((a, b) => options.indexOf(a) - options.indexOf(b));

  $: selectedIcon =
    values.length === 0 ? "question_mark" : sortedValues.map((v) => v.icon).join("");

  $: selectedUsernames = createUsernameList(sortedValues);

  function createUsernameList(values: T[]): string[] | undefined {
    if (values.length === 0) {
      return undefined;
    }

    let usernames: string[] = [];

    for (const v of values) {
      if (v.userName !== undefined) {
        usernames.push(v.userName);
      } else {
        return undefined;
      }
    }

    return usernames;
  }

  function focusSearchbar() {
    if (searchbarElement) {
      searchbarElement.focus();
    }
  }

  function groupOptions(options: T[]): { group?: Group; options: T[] }[] {
    let groups: { group?: Group; options: T[] }[] = [];

    for (const option of options) {
      let groupEntry = groups.find((value) => value.group === option.group);
      if (groupEntry === undefined) {
        groupEntry = { group: option.group, options: [] };
        groups.push(groupEntry);
      } else {
      }
      groupEntry.options.push(option);
    }

    return groups;
  }

  $: groupedOptions = groupOptions(options);

  function filterOptions(
    options: {
      group?: Group;
      options: T[];
    }[],
    filter: string
  ): { group?: Group; options: T[] }[] {
    let groups: { group?: Group; options: T[] }[] = [];

    for (const group of options) {
      let newOptions: T[] | undefined = undefined;
      for (const option of group.options) {
        if (option.name.toLowerCase().includes(filter.toLowerCase())) {
          if (newOptions === undefined) {
            newOptions = [];
            groups.push({
              group: group.group,
              options: newOptions,
            });
          }
          newOptions.push(option);
        }
      }
    }

    if (groups.length == 1 && groups[0].options.length == 1) {
      filterFocusedOption = groups[0].options[0];
    } else {
      filterFocusedOption = undefined;
    }

    return groups;
  }

  $: filteredGroupOptions = filterOptions(groupedOptions, filterQuery);

  let filterFocusedOption: T | undefined = undefined;

  function onSearchbarKeypress(event: KeyboardEvent) {
    if (event.key === "Enter") {
      if (filterFocusedOption !== undefined) {
        toggle(filterFocusedOption);
        filterQuery = "";
      }
    }
  }
</script>

<div
  class="menu-button {className}"
  on:mouseenter={() => {
    hovered = true;
    filterQuery = "";
  }}
  on:mouseleave={() => {
    hovered = false;
  }}
>
  <button on:click={focusSearchbar}>
    {#if selectedUsernames !== undefined}
      <UserStack userNames={selectedUsernames} />
    {:else}
      <Icon icon={selectedIcon} />
    {/if}
  </button>
  {#if hovered}
    <div class="menu {menuPosition}">
      <input
        type="text"
        bind:value={filterQuery}
        class="searchbar"
        bind:this={searchbarElement}
        on:keypress={onSearchbarKeypress}
      />
      <div class="option-container">
        {#each filteredGroupOptions as groupOption}
          {#if groupOption.group !== undefined}
            <div class="group row space">
              <Icon icon={groupOption.group.icon} />
              {groupOption.group.name}
            </div>
          {/if}
          {#each groupOption.options as option}
            {#if option.name.toLowerCase().includes(filterQuery.toLowerCase())}
              <button
                class="option row space
                {values.includes(option) ? 'selected' : ''}
                {filterFocusedOption === option ? 'focused' : ''}
                "
                on:click={() => toggle(option)}
              >
                <span>
                  {option.name}
                </span>
                <div class="icon">
                  {#if option.userName !== undefined}
                    <UserView userName={option.userName} />
                  {:else}
                    <Icon icon={option.icon} />
                  {/if}
                </div>
              </button>
            {/if}
          {/each}
        {/each}
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .menu-button {
    --user-size: 24px;
    position: relative;
    background-color: var(--surface-light);
    color: var(--surface-on);
    border-radius: var(--small);
  }

  .menu-button > button {
    padding: var(--small);
    border: none;
    color: var(--surface-on);
    background-color: transparent;
    cursor: pointer;
  }

  .menu {
    z-index: 1;
    position: absolute;
    top: 0%;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    background-color: var(--surface-light);
    border-radius: var(--small);
    box-shadow: var(--button-shadow);
    
    &.left {
      right: calc(100% + var(--small));
    }

    &.right {
      left: calc(100% + var(--small));
    }
  }

  .menu::before {
    content: "";
    z-index: -1;
    /* background-color: rebeccapurple; */
    position: absolute;
    top: -16px;
    left: -16px;
    width: calc(100% + 2 * 16px);
    height: calc(100% + 2 * 16px);
  }

  .option-container {
    max-height: 256px;
    overflow: scroll;
    display: flex;
    flex-direction: column;
  }

  .option,
  .group {
    user-select: none;
    height: 42px;
    padding: var(--small);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .option {
    border: none;
    cursor: pointer;
    background-color: transparent;
    color: var(--surface-on);
    font-weight: bold;
  }

  .icon {
    padding: 4px;
  }

  .option.selected .icon {
    background-color: var(--primary);
    color: var(--primary-on);
    border-radius: 4px;
  }

  .option.focused span::after {
    content: " Enter";
    background-color: #5e5e5e;
    color: #3b3b3b;
    padding: 4px;
    margin-left: 8px;
    border-radius: 4px;
  }

  .option:hover {
    background-color: #ffffff13;
  }

  .group {
    position: sticky;
    top: 0px;
    z-index: 1;
    height: 32px;
    background-color: var(--surface);
  }

  .searchbar {
    width: calc(256px - var(--small) * 4);
    margin: var(--small);
  }
</style>
