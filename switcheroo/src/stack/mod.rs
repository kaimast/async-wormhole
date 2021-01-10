//! Different stack implementations (currently only contains a 8 Mb stack).

mod eight_mb;
mod one_mb;
pub use eight_mb::EightMbStack;
pub use one_mb::OneMbStack;

/// An implementation of this trait will be accepted by a [generator](struct.Generator.html) as a
/// valid Stack. Most of the functions provided here are straightforward except for
/// [deallocation](trait.Stack.html#tymethod.deallocation), this is a Windows only construct.
///
/// Windows reserves a few pages above the stack top, so if a stack overflow exception is triggered
/// the handler has still enough of stack to process it. The name comes from the fact that it
/// points to the top most address of the memory area designated to the stack and will be used as a
/// pointer when freeing/deallocating the stack.
pub trait Stack: Sized {
    /// Returns a new stack.
    fn new() -> Result<Self, std::io::Error>;

    /// Returns a pointer to the bottom of the stack.
    fn bottom(&self) -> *mut usize;

    /// Returns a pointer to the top of the stack.
    fn top(&self) -> *mut usize;

    /// Returns a pointer to the deallocation stack (a Windows construct).
    fn deallocation(&self) -> *mut usize;

    /// Get the size of the stack in bytes
    fn size(&self) -> usize;

    /// Fill contents of with the contents of another stack
    /// Note: this assumes stacks are growing in the same direction
    unsafe fn fill(&mut self, bottom: *const usize, top: *const usize) {
        let (src_base, dst_base, offset) = if bottom < top {
            let offset = top.offset_from(bottom);
            (bottom, self.bottom(), offset as usize)
        } else if top < bottom {
            let offset = bottom.offset_from(top);
            (top, self.top(), offset as usize)
        } else {
            panic!("Stack is empty");
        };

        if offset > self.size() {
            panic!("Source stack is greater than self");
        }

        std::ptr::copy(src_base, dst_base, offset);
    }
}
