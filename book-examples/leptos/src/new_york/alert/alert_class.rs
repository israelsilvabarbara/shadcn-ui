use leptos::prelude::*;
use lucide_leptos::Terminal;

use crate::new_york::components::ui::alert::{
    Alert,
    AlertDescription,
    AlertTitle,
};

#[component]
pub fn AlertClass() -> impl IntoView {
    view! {
        <Alert class="bg-slate-800 w-1/2">
            <Terminal attr:class="h-4 w-4" />
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
