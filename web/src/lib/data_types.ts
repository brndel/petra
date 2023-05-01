export type User = {
    id: number,
    name: string,
    userName: string,
    icon: string,
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

export type Rule = {
    id: number,
    name: string,
    paymentRename: string,
    categories: Category[]
    shareRule: boolean | null,
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

export type EditablePayment = {
    name: string,
    amount: number,
    date: Date,
    users: User[],
    categories: Category[],
    rule: Rule | null,

    importInfo: EditablePaymentImportInfo | undefined,
}

export type EditablePaymentImportInfo = {
    nameRaw: string,
    refHash: string,
}

export type MonthData = {
    // "2023-12"
    month: string,
    positive: number,
    negative: number,
    repay: number
}

export type TinkImportInfo = {
    pending: number,
    listed: number
}