import type { EditablePayment } from "$lib/data_types";
import { writable, type Readable } from "svelte/store";

let editablePaymentsWrite = writable<EditablePayment[]>([]);

export let editablePaymentArray = editablePaymentsWrite as Readable<EditablePayment[]>;


export function addEditablePayment() {
    let payment: EditablePayment = {
        name: "",
        amount: 0,
        date: new Date(),
        users: [],
        categories: [],
        rule: undefined,
        importInfo: undefined,
    }
    editablePaymentsWrite.update((payments) => payments.concat(payment))
}

export function insertEditablePayment(payment: EditablePayment, after: EditablePayment) {
    editablePaymentsWrite.update((payments) => {
        let index = payments.indexOf(after);
        payments.splice(index+1, 0, payment);
        return payments;
    })
}

export function setEditablePayments(payments: EditablePayment[]) {
    editablePaymentsWrite.set(payments);
}

export function removeEditablePayment(payment: EditablePayment) {
    editablePaymentsWrite.update((payments) => payments.filter((p) => p !== payment));
}