<script lang="ts">
    import type { Payment } from "$lib/data_types";
    import { formatDate } from "$lib/text";
    import Icon from "./Icon.svelte";
    import MoneySpan from "./MoneySpan.svelte";
    import UserStack from "./UserStack.svelte";
    import UserView from "./UserView.svelte";

    export let payment: Payment;

    $: showUsers = !(
        payment.users.length == 1 && payment.users[0] == payment.owner
    );
</script>

<div class="payment row">
    <div class="col start">
        <span>
            {payment.name}
            &#x2219;
            {formatDate(payment.timestamp)}
        </span>
        <div class="row center">
            <UserView userName={payment.owner.userName} />
            {#if showUsers}
                <Icon icon="arrow_right" />
                <UserStack userNames={payment.users.map((v) => v.userName)} />
            {/if}
        </div>
    </div>
    <div class="row center categories">
        {#each payment.categories as category}
            <Icon icon={category.icon} tooltip={category.name} />
        {/each}
    </div>
    <div class="col end">
        <MoneySpan amount={payment.amount + payment.repayAmount} role="auto" />
        {#if showUsers}
        <MoneySpan amount={payment.repayAmount} role="repay" />
            <MoneySpan amount={payment.amount} role="neutral" />
        {/if}
    </div>
</div>

<style lang="scss">
    .payment {
        background-color: var(--surface);
        border-radius: var(--small);
        padding: var(--small);

        & > * {
            flex: 1;
        }
    }

    .col,
    .row {
        justify-content: space-between;
    }

    .row.categories {
        justify-content: start;
    }
</style>
