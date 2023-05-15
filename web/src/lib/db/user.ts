import type { User } from "$lib/data_types";
import { rawToMapId, type RawData, multiSubscribe2 } from "$lib/db/helpers";
import { readable, writable } from "svelte/store";
import { loadDB } from "./loader";

export let rawUsers = writable<RawData[]>(undefined);
export let rawCurrentUser = writable<RawData | null>(undefined);

export let userMap = readable<Map<number, User> | undefined>(undefined, (set) => {
    rawUsers.subscribe((rawUsers) => {
        set(
            rawToMapId(rawUsers, (raw) => {
                let data: User = {
                    id: raw.id,
                    userName: raw.name,
                    name: raw.display_name,
                    icon: "person"
                }
                return data;
            })
        );
    });
});

export let userArray = readable<User[]>([], (set) => {
    userMap.subscribe((users) => {
        if (users === undefined) {
            set([]);
            return;
        }

        set(Array.from(users.values()))
    })
});


export let currentUser = readable<User | null | undefined>(undefined, (set) => {
    multiSubscribe2([rawCurrentUser, userMap], set, ([rawCurrentUser, userMap]) => {
        if (rawCurrentUser == null) {
            return null;
        }
        return userMap.get(rawCurrentUser["user"]);
    })
});
