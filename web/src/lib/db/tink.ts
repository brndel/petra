import { writable, type Readable, readable } from "svelte/store";
import type { RawData } from "./helpers";
import type { EditablePayment, TinkImportInfo } from "$lib/data_types";
import { setEditablePayments } from "./editablePayment";
import { loadDB } from "./loader";
import { applyRule } from "./rule";

export let rawTinkToken = writable<RawData | null>(undefined);
export let rawTinkPayments = writable<RawData>(undefined);

let tinkImportInfoWrite = writable<TinkImportInfo | undefined>(undefined);

export let tinkImportInfo = tinkImportInfoWrite as Readable<TinkImportInfo | undefined>;

rawTinkPayments.subscribe((rawTinkPayments) => {
    if (rawTinkPayments === undefined) {
        return;
    }

    tinkImportInfoWrite.set({
        new: rawTinkPayments.new.length,
        listed: rawTinkPayments.listed,
        pending: rawTinkPayments.pending,
    });

    let payments: EditablePayment[] = [];
    for (const rawPayment of rawTinkPayments.new) {
        let payment: EditablePayment = {
            name: rawPayment.name,
            amount: rawPayment.amount,
            date: new Date(rawPayment.date),
            users: [],
            categories: [],
            rule: undefined,
            importInfo: {
                nameRaw: rawPayment.name_raw,
                refHash: rawPayment.ref_hash
            }
        }

        applyRule(payment);

        payments.push(payment);
    }

    setEditablePayments(payments);
});


export let tinkTokenTimeout = readable<Date | null | undefined>(undefined, (set) => {
    rawTinkToken.subscribe((rawTinkToken) => {
        // console.log("rawTinkToken", rawTinkToken);

        if (rawTinkToken === null || rawTinkToken === undefined) {
            set(rawTinkToken);
        } else {
            let timestamp = rawTinkToken.expires_timestamp;
            set(new Date(timestamp));
        }
    })
});

export async function loadTinkPayments(month: string) {
    await loadDB("tink/payment", { "month": month });
}
