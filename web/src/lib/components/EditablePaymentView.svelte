<script lang="ts">
  import { DateInput } from "date-picker-svelte";
  import type { Category, EditablePayment, User } from "$lib/data_types";
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
  import { currentUser, userArray } from "$lib/db/user";
  import { categoryArray } from "$lib/db/category";
  import { addEditablePayment, insertEditablePayment, removeEditablePayment } from "$lib/db/editablePayment";

  export let payment: EditablePayment;

  export let highlightError: boolean = false;

  export let onChange: (() => void) | undefined = undefined;

  function toggleCategory(category: Category) {
    if (payment.categories.includes(category)) {
      payment.categories = payment.categories.filter((v) => v !== category);
    } else {
      payment.categories = payment.categories.concat(category);
    }

    if (onChange !== undefined) {
      onChange();
    }
  }

  function toggleUser(user: User) {
    if (payment.users.includes(user)) {
      payment.users = payment.users.filter((v) => v !== user);
    } else {
      payment.users = payment.users.concat(user);
    }

    if (onChange !== undefined) {
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

    if (onChange !== undefined) {
      onChange();
    }
  }

  function onActionSelect(action: { name: string; icon: string }) {
    switch (action.icon) {
      case "delete":
        removeEditablePayment(payment);
        break;
      case "cut":
        let splitAmount = window.prompt("Wie viel soll abgespalten werden?");
        if (splitAmount === null) {
          return;
        }
        splitAmount = splitAmount.replace(",", ".");

        let amount = parseFloat(splitAmount);
        amount = Math.abs(Math.floor(amount * 100));

        amount *= Math.sign(payment.amount);

        if(amount - payment.amount <= 0) {
          window.alert(`Das ist zu viel ${Math.abs(amount/100)} (max ${Math.abs(payment.amount/100)})`);
          return;
        }

        payment.amount -= amount;

        let newPayment: EditablePayment = {
            name: payment.name,
            amount: amount,
            date: payment.date,
            users: [],
            categories: [],
            importInfo: payment.importInfo
        }

        insertEditablePayment(newPayment, payment);
        break;
    }
  }
</script>

<div class="payment row stretch">
  <div class="col">
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
    <SelectionMenu
      options={[
        { name: "Löschen", icon: "delete" },
        { name: "Aufteilen", icon: "cut" },
      ]}
      values={[{ name: "", icon: "more_vert" }]}
      toggle={onActionSelect}
    />
  </div>
  <div class="col space">
    <input
      type="text"
      bind:value={payment.name}
      class={paymentNameHasError(payment) ? "error" : ""}
    />
    {#if payment.importInfo !== undefined}
      <span class="name-raw">{payment.importInfo.nameRaw}</span>
      <span>{formatDate(payment.date)}</span>
    {:else}
      <DateInput bind:value={payment.date} />
    {/if}
    <!-- {#if payment.importInfo !== undefined}
      <span>{payment.importInfo.refHash}</span>
    {/if} -->
  </div>
  <div class="row start">
    <SelectionMenu
      options={$categoryArray}
      values={payment.categories}
      toggle={toggleCategory}
    />
    {#if payment.rule !== undefined}
      <div class="rule-tag row">
        <Icon icon="smart_toy" tooltip="Regel {payment.rule.name}" />
        {payment.rule.name}
      </div>
    {/if}
  </div>
  <div class="spacer" />
  <div class="col stretch space">
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
        menuPosition="left"
        options={$userArray}
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

  .rule-tag {
    background-color: var(--primary);
    border-radius: var(--small);
    padding: var(--small);
  }

  .start {
    align-self: flex-start;
  }
</style>
