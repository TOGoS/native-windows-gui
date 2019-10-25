/// All the events that can be dispatched by the built-in controls of native-windows-gui
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Event {
    /// Undefined / not implemented event
    Unknown,

    /// When a button is clicked. Similar to a MouseUp event
    OnButtonClick,

    /// When a button is clicked twice rapidly
    OnButtonDoubleClick,

    /// When a label is clicked
    OnLabelClick,

    /// When a label is clicked twice rapidly
    OnLabelDoubleClick,

    /// When a ImageFrame is clicked
    OnImageFrameClick,

    /// When a ImageFrame is clicked twice rapidly
    OnImageFrameDoubleClick,

    /// When a TextInput value is changed
    OnTextInput,

    /// When the list of a combobox is closed
    OnComboBoxClosed,

    /// When the list of a combobox is about to be visible
    OnComboBoxDropdown,

    /// When the current selection of the combobox was changed
    OnComboxBoxSelection,

    /// When the date select dropdown is expanded
    OnDatePickerDropdown,

    /// When the date select dropdown is closed
    OnDatePickerClosed,

    /// When the value of the date select is changed
    OnDatePickerChanged,

    /// When an item on a list box is clicked twice
    OnListBoxDoubleClick,

    /// When an item on a list box is selected
    OnListBoxSelect,

    // The select tab of a TabsContainer changed
    TabsContainerChanged,

    // The select tab of a TabsContainer is about to be changed
    TabsContainerChanging,

    // When the user click on a menu item
    OnMenuItemClick,

    /// When a timer delay is elapsed
    OnTimerTick,

    /// When a notice is... noticed
    OnNotice,

    /// When a user click on the X button of a window
    OnWindowClose,

    /// When most control receive keyboard focus
    OnFocus,

    /// When most control lose keyboard focus
    OnFocusLost
}
