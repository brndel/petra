import type { Payment, User } from "./data_types";


export function calculateRepay(payment: { amount: number; users: User[] }, user: User | null | undefined): number {
  if (payment.users.length === 0 || user === undefined || user === null) {
    return 0;
  }

  let repay = payment.amount;
  let amountPerUser = payment.amount / payment.users.length;

  if (payment.users.includes(user)) {
    repay -= amountPerUser;
  }

  return -repay;
}

export function paymentHasError(payment: {name: string, amount: number, users: User[]}): boolean {
  return paymentAmountHasError(payment) || paymentUsersHasError(payment) || paymentNameHasError(payment);
}

export function paymentNameHasError(payment: {name: string}): boolean {
  return payment.name.trim().length === 0;
}

export function paymentAmountHasError(payment: {amount: number}): boolean {
  return payment.amount === 0;
}


export function paymentUsersHasError(payment: {users: User[]}): boolean {
  return payment.users.length === 0;
}