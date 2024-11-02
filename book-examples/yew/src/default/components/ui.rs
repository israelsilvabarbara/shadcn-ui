// In actual projects this module would contain the copied components, but this example uses the local workspace packages.

#[cfg(feature = "button")]
pub use shadcn_ui_yew_button::default as button;
#[cfg(feature = "skeleton")]
pub use shadcn_ui_yew_skeleton::default as skeleton;