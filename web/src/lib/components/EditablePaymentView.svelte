<script lang="ts">
  import type { Category, EditablePayment, User } from "$lib/data_types";
  import { categoriesArray, currentUser, usersArray } from "$lib/db";
  import { formatDate } from "$lib/text";
  import MoneySpan from "./MoneySpan.svelte";
  import SelectionMenu from "./SelectionMenu.svelte";
  import {
    calculateRepay,
    paymentAmountHasError,
    paymentHasError,
    paymentNameHasError,
    paymentUsersHasError,
  } from "$lib/util";
  import Icon from "./Icon.svelte";

  export let payment: EditablePayment;

  export let highlightError: boolean = false;

  export let onChange: (() => void) | undefined = undefined;

  function toggleCategory(category: Category) {
    if (payment.categories.includes(category)) {
      payment.categories = payment.categories.filter((v) => v !== category);
    } else {
      payment.categories = payment.categories.concat(category);
    }

    if(onChange !== undefined) {
      onChange();
    }
  }

  function toggleUser(user: User) {
    if (payment.users.includes(user)) {
      payment.users = payment.users.filter((v) => v !== user);
    } else {
      payment.users = payment.users.concat(user);
    }

    if(onChange !== undefined) {
      onChange();
    }
  }

  $: repayAmount = calculateRepay(payment, $currentUser);

  function onAmountValueChange(
    event: Event & { currentTarget: EventTarget & HTMLInputElement }
  ) {
    let value = event.currentTarget.valueAsNumber;
    if (Number.isNaN(value)) {
      payment.amount = 0;
      return;
    }

    value = Math.floor(value * 100);

    payment.amount = value;
    payment = payment;

    if(onChange !== undefined) {
      onChange();
    }
  }
</script>

<div class="payment row stretch">
  {#if highlightError}
    {#if paymentHasError(payment)}
      <div class="icon error">
        <Icon icon="sentiment_extremely_dissatisfied" />
      </div>
    {:else}
      <div class="icon">
        <Icon icon="sentiment_satisfied" />
      </div>
    {/if}
  {/if}
  <div class="col space">
    <input
      type="text"
      bind:value={payment.name}
      class={paymentNameHasError(payment) ? "error" : ""}
    />
    {#if payment.importInfo !== undefined}
      <span class="name-raw">{payment.importInfo.nameRaw}</span>
    {/if}
    <span>{formatDate(payment.date)}</span>
  </div>
  <div class="start">
    <SelectionMenu
      options={$categoriesArray}
      values={payment.categories}
      toggle={toggleCategory}
    />
  </div>
  <div class="spacer" />
  <div class="col stretch">
    <div class="row">
      <MoneySpan amount={payment.amount} />
      {#if payment.importInfo === undefined}
        <input
          class="payment-input {paymentAmountHasError(payment) ? 'error' : ''}"
          type="number"
          on:input={onAmountValueChange}
          step="0.01"
        />
      {/if}
    </div>
    <div class="row space">
      <MoneySpan amount={repayAmount} role="repay" />
      <SelectionMenu
        options={$usersArray}
        values={payment.users}
        toggle={toggleUser}
        class={paymentUsersHasError(payment) ? "error" : ""}
      />
    </div>
  </div>
</div>

<style lang="scss">
  .payment {
    background-color: var(--surface);
    border-radius: var(--small);
    padding: var(--small);
  }

  .payment-input {
    width: 128px;
  }

  .icon {
    background-color: var(--positive);
    padding: var(--small);
    border-radius: var(--small);
    display: flex;
    align-items: center;

    &.error {
      background-color: var(--error);
    }
  }

  .start {
    align-self: flex-start;
  }

  /* .col,
  .row {
    justify-content: space-between;
    align-items: center;
  } */
</style>
