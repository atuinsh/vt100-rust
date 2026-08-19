/// Represents the state of an in-progress capture of formatted terminal
/// output.
///
/// This type is needed if you're capturing terminal data in a streaming
/// fashion using [`RowContents::write_formatted`] in the [`on_scroll`]
/// callback.
///
/// [`on_scroll`]: crate::Callbacks::on_scroll
#[derive(Debug, Default, Clone)]
pub struct CaptureState {
    pub(crate) prev_pos: Option<crate::grid::Pos>,
    pub(crate) prev_attrs: Option<crate::attrs::Attrs>,
    pub(crate) row: u16,
    pub(crate) wrapping: bool,
}

impl CaptureState {
    /// Creates a new [`CaptureState`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Represents the contents of a row in the terminal.
pub struct RowContents<'a>(pub(crate) &'a crate::row::Row);

impl RowContents<'_> {
    /// Writes the contents of the row to the provided buffer.
    pub fn write_formatted(
        &self,
        buffer: &mut String,
        state: &mut CaptureState,
    ) {
        let (prev_pos, prev_attrs) = self.0.write_contents_formatted(
            buffer,
            0,
            u16::MAX,
            state.row,
            state.wrapping,
            state.prev_pos,
            state.prev_attrs,
        );
        state.prev_pos = Some(prev_pos);
        state.prev_attrs = Some(prev_attrs);
        state.row += 1;
        state.wrapping = self.0.wrapped();
    }
}
