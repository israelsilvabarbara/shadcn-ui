
use leptos::{ev::InputEvent, ev::FocusEvent,prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_style::Style;
use tailwind_fuse::*;



#[component]
pub fn Input(
    // Global attributes
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] autocapitalize: MaybeProp<String>,
    #[prop(into, optional)] autofocus: bool,
    #[prop(into, optional)] spellcheck: MaybeProp<String>,
    // Input attributes
    #[prop(into, optional)] accept: MaybeProp<String>,
    #[prop(into, optional)] alt: MaybeProp<String>,
    #[prop(into, optional)] autocomplete: MaybeProp<String>,
    #[prop(into, optional)] capture: MaybeProp<String>,
    #[prop(into, optional)] checked: bool,
    #[prop(into, optional)] dirname: MaybeProp<String>,
    #[prop(into, optional)] disabled: bool,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] formaction: MaybeProp<String>,
    #[prop(into, optional)] formenctype: MaybeProp<String>,
    #[prop(into, optional)] formmethod: MaybeProp<String>,
    #[prop(into, optional)] formnovalidate: bool,
    #[prop(into, optional)] formtarget: MaybeProp<String>,
    #[prop(into, optional)] height: MaybeProp<String>,
    #[prop(into, optional)] list: MaybeProp<String>,
    #[prop(into, optional)] max: MaybeProp<String>,
    #[prop(into, optional)] maxlength: MaybeProp<String>,
    #[prop(into, optional)] min: MaybeProp<String>,
    #[prop(into, optional)] minlength: MaybeProp<String>,
    #[prop(into, optional)] multiple: bool,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] pattern: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] popovertarget: MaybeProp<String>,
    #[prop(into, optional)] popovertargetaction: MaybeProp<String>,
    #[prop(into, optional)] readonly: bool,
    #[prop(into, optional)] required: bool, 
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] step: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] width: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<String>,
    // Event handlers
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_change: Option<Callback<InputEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_input: Option<Callback<InputEvent>>,
    // Node reference
    node_ref: AnyNodeRef,

) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref

            autocapitalize=move || autocapitalize.get()
            autofocus=autofocus
            class=move || tw_merge!(
                "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
                class.get()
            )
            id=move || id.get()
            spellcheck=move || spellcheck.get()
            style=style

            accept=move || accept.get()
            alt=move || alt.get()
            autocomplete=move || autocomplete.get()
            capture=move || capture.get()
            checked=checked
            disabled=disabled
            form=move || form.get()
            formaction=move || formaction.get()
            formenctype=move || formenctype.get()
            formmethod=move || formmethod.get()
            formnovalidate=formnovalidate
            formtarget=move || formtarget.get()
            height=move || height.get()
            list=move || list.get()
            max=move || max.get()
            maxlength=move || maxlength.get()
            min=move || min.get()
            minlength=move || minlength.get()
            multiple=multiple
            name=move || name.get()
            pattern=move || pattern.get()
            placeholder=move || placeholder.get()
            popovertarget=move || popovertarget.get()
            popovertargetaction=move || popovertargetaction.get()
            readonly=readonly
            required=required
            src=move || src.get()
            step=move || step.get()
            r#type=move || r#type.get()
            value=move || value.get()
            width=move || width.get()

            on:blur= on_blur
            on:change= on_change
            on:focus= on_focus
            on:input= on_input
        />
    }
}
