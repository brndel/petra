import { readable, writable } from "svelte/store";
import { rawToMapId, type RawData, multiSubscribe2 } from "./helpers";
import type { Category, CategoryGroup } from "$lib/data_types";

export let rawCategoryGroups = writable<RawData[]>(undefined);
export let rawCategories = writable<RawData[]>(undefined);

let categoryGroupMap = readable<Map<number, CategoryGroup> | undefined>(undefined, (set) => {
    rawCategoryGroups.subscribe((rawCategoryGroups) => {
        if (rawCategoryGroups === undefined) {
            set(undefined);
            return;
        }
        set(
            rawToMapId(rawCategoryGroups, (raw) => {
                let data: CategoryGroup = {
                    id: raw.id,
                    name: raw.name,
                    icon: raw.icon,
                }
                return data;
            })
        );
    });
});

export let categoryMap = readable<Map<number, Category> | undefined>(undefined, (set) => {
    multiSubscribe2([rawCategories, categoryGroupMap], set, ([rawCategories, categoryGroupMap]) => {
        let map = rawToMapId(rawCategories, (raw) => {
            let data: Category = {
                id: raw.id,
                name: raw.name,
                icon: raw.icon,
                group: categoryGroupMap.get(raw.group_id)!
            };
            return data;
        });

        return map;
    });
});

export let categoryArray = readable<Category[]>([], (set) => {
    categoryMap.subscribe((categoryMap) => {
        if (categoryMap === undefined) {
            set([]);
            return;
        }

        set(Array.from(categoryMap.values()))
    })
});