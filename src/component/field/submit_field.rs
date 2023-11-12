use leptos::*;

use super::field::{FieldReset, FieldUnchaged};

#[component]
pub fn SubmitField<
    F: FieldReset + FieldUnchaged + Copy + 'static,
    S: Fn() + Copy + 'static,
    D: Fn() + Copy + 'static,
>(
    field: F,
    on_submit: S,
    on_delete: D,
    #[prop(optional)] is_new: bool,
) -> impl IntoView {
    let delete_confirm = RwSignal::new(false);

    let cancel_button_state = move || {
        if delete_confirm.get() {
            return CancelButtonState::CancelDelete;
        }

        if field.is_unchanged() {
            return CancelButtonState::Delete;
        }

        return CancelButtonState::Cancel;
    };

    let submit_button_state = move || {
        if delete_confirm.get() {
            return SubmitButtonState::ConfirmDelete;
        }

        SubmitButtonState::Submit {
            disabled: field.is_unchanged(),
        }
    };

    view! {
        <div class="button-bar">
            {if !is_new {
                Some({move || match cancel_button_state() {
                        CancelButtonState::Cancel => view! {
                            <button on:click=move |_| field.reset()>
                                "Cancel"
                            </button>
                        }.into_view(),
                        CancelButtonState::CancelDelete => view! {
                            <button on:click=move |_| delete_confirm.set(false)>
                                "Cancel"
                            </button>
                        }.into_view(),
                        CancelButtonState::Delete => view! {
                            <button class="error" on:click=move |_| delete_confirm.set(true)>
                                "Delete"
                            </button>
                        }.into_view(),
                }})
            } else {
                None
            }}

            {move || match submit_button_state() {
                SubmitButtonState::Submit {disabled} => view! {
                    <button
                        class="primary"
                        on:click=move |_| on_submit()
                        disabled=disabled
                    >{if is_new {"Create"} else {"Update"}}</button>
                }.into_view(),
                SubmitButtonState::ConfirmDelete => view! {
                    <button
                        class="error"
                        on:click=move |_| on_delete()
                    >"Confirm"</button>
                }.into_view()
            }}
        </div>
    }
}

#[derive(PartialEq)]
enum CancelButtonState {
    Cancel,
    CancelDelete,
    Delete,
}

#[derive(PartialEq)]
enum SubmitButtonState {
    Submit { disabled: bool },
    ConfirmDelete,
}
