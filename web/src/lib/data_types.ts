export type User = {
    id: number,
    name: string,
    displayName: string,
}

export type CategoryGroup = {
    id: number,
    name: string,
    icon: string,
}

export type Category = {
    id: number,
    name: string,
    icon: string,
    group: CategoryGroup
}

export type Payment = {
    id: number,
    name: string,
    amount: number,
    repayAmount: number,
    timestamp: Date,
    owner: User,
    users: User[],
    categories: Category[]
}



export type MonthData = {
    // "2023-12"
    month: string,
    positive: number,
    negative: number,
    repay: number
}