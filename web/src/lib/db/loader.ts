import { apiGet } from "$lib/api";
import type { Writable } from "svelte/store";
import { rawTinkPayments, rawTinkToken } from "./tink";
import { rawMonths } from "./month";
import { rawPayments } from "./payment";
import { rawCurrentUser, rawUsers } from "./user";
import { rawCategoryGroups, rawCategories } from "./category";
import { rawRules } from "./rule";

export type LoaderMethod = "current_user" | "user" | "category_group" | "category" | "month_index" | "payment" | "tink/token" | "tink/payment" | "rule";

function getSetter(method: LoaderMethod): Writable<any> {
    switch (method) {
        case "current_user": return rawCurrentUser;
        case "user": return rawUsers;
        case "category_group": return rawCategoryGroups;
        case "category": return rawCategories;
        case "rule": return rawRules;
        case "month_index": return rawMonths;
        case "payment": return rawPayments;
        case "tink/token": return rawTinkToken;
        case "tink/payment": return rawTinkPayments;
    }
}

export async function loadDB(method: LoaderMethod, args?: {}) {
    let setter = getSetter(method);
    await loadRawData(method, setter, args);
}

async function loadRawData(method: LoaderMethod, setter: Writable<any>, args?: {}) {
    let rawData = await apiGet(method, args);
    setter.set(rawData);
}