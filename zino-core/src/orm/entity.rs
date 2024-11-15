use crate::{
    error::Error,
    model::{Column, Model},
};
use std::str::FromStr;

/// An interface for the model entity.
pub trait Entity: Model {
    /// The column type.
    type Column: AsRef<str> + FromStr<Err: Into<Error>> + Into<Column<'static>>;

    /// The primary key column.
    const PRIMARY_KEY: Self::Column;
}