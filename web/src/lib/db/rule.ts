import { get, readable, writable } from "svelte/store";
import { multiSubscribe2, rawToMapId, type RawData } from "./helpers";
import type { EditablePayment, Rule } from "$lib/data_types";
import { categoryMap } from "./category";
import { currentUser, userArray } from "./user";

export let rawRules = writable<RawData[]>(undefined);

export let ruleMap = readable<Map<number, Rule> | undefined>(undefined, (set) => {
    multiSubscribe2([rawRules, categoryMap], set, ([rawRules, categoryMap]) => {
        return rawToMapId(rawRules, (raw) => {
            let data: Rule = {
                id: raw.id,
                name: raw.name,
                categories: raw.categories.map((id: number) => categoryMap.get(id)),
                keywords: raw.keywords,
                shareRule: raw.shared
            };

            return data;
        });
    });
});

export let ruleArray = readable<Rule[]>([], (set) => {
    ruleMap.subscribe((ruleMap) => {
        if (ruleMap === undefined) {
            set([]);
            return;
        }

        set(Array.from(ruleMap.values()));
    });
});

export function applyRule(payment: EditablePayment) {
    let name = payment.importInfo?.nameRaw;
    if (name === undefined) {
        return;
    }

    let nameLower = name.toLowerCase();

    let owner = get(currentUser);
    if (owner === undefined || owner === null) {
        return;
    }

    let rules = get(ruleArray);
    let users = get(userArray);

    for (const rule of rules) {
        for (const keyword of rule.keywords) {
            if (nameLower.includes(keyword)) {
                payment.rule = rule;
                payment.categories = [...rule.categories]
                switch (rule.shareRule) {
                    case true: payment.users = [...users]; break;
                    case false: payment.users = [owner]; break;
                    case null: payment.users = []; break;
                }
                return;
            }
        }
    }
}