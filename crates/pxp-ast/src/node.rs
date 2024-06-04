use std::any::Any;

use pxp_span::Span;
use pxp_syntax::comments::CommentGroup;

/// A generic node trait that all AST nodes implement.
/// This trait is used to provide a common interface for all nodes in the AST.
pub trait Node: Any {
    /// Get a string representation of the node's name.
    fn name(&self) -> &'static str;

    /// Get a list of the node's children.
    fn children(&self) -> Vec<&dyn Node>;

    /// Get the span of the node.
    fn span(&self) -> Span;

    /// Get the comments associated with the node.
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }
}
