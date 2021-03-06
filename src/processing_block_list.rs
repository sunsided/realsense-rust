//! Defines the iterable list of processing blocks.

use crate::{
    common::*,
    error::{ErrorChecker, Result as RsResult},
    processing_block::{marker as processing_block_marker, ProcessingBlock},
};

/// The iterable list of [ProcessingBlock](ProcessingBlock)s.
#[derive(Debug)]
pub struct ProcessingBlockList {
    ptr: NonNull<realsense_sys::rs2_processing_block_list>,
}

impl ProcessingBlockList {
    /// Retrieves the [ProcessingBlock](ProcessingBlock) instance at index.
    pub fn get(&mut self, index: usize) -> RsResult<ProcessingBlock<processing_block_marker::Any>> {
        let block = unsafe {
            let mut checker = ErrorChecker::new();
            let ptr = realsense_sys::rs2_get_processing_block(
                self.ptr.as_ptr(),
                index as c_int,
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            ProcessingBlock::new_from_ptr(NonNull::new(ptr).unwrap())?
        };
        Ok(block)
    }

    /// Returns the length of list.
    pub fn len(&mut self) -> RsResult<usize> {
        unsafe {
            let mut checker = ErrorChecker::new();
            let val = realsense_sys::rs2_get_recommended_processing_blocks_count(
                self.ptr.as_ptr(),
                checker.inner_mut_ptr(),
            );
            checker.check()?;
            Ok(val as usize)
        }
    }

    /// Checks if the list is empty.
    pub fn is_empty(&mut self) -> RsResult<bool> {
        Ok(self.len()? == 0)
    }

    /// Converts to iterator type.
    pub fn try_into_iter(mut self) -> RsResult<ProcessingBlockListIntoIter> {
        let len = self.len()?;
        let ptr = unsafe { self.take() };
        let iter = ProcessingBlockListIntoIter { len, index: 0, ptr };
        Ok(iter)
    }

    pub(crate) unsafe fn take(self) -> NonNull<realsense_sys::rs2_processing_block_list> {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    pub(crate) unsafe fn from_ptr(ptr: NonNull<realsense_sys::rs2_processing_block_list>) -> Self {
        Self { ptr }
    }
}

impl IntoIterator for ProcessingBlockList {
    type Item = RsResult<ProcessingBlock<processing_block_marker::Any>>;
    type IntoIter = ProcessingBlockListIntoIter;

    /// The method calls [ProcessingBlockList::try_into_iter](ProcessingBlockList::try_into_iter).
    ///
    /// # Panics
    /// It panics if [ProcessingBlockList::try_into_iter](ProcessingBlockList::try_into_iter) returns [Err](Result::Err).
    fn into_iter(self) -> Self::IntoIter {
        self.try_into_iter().unwrap()
    }
}

impl Drop for ProcessingBlockList {
    fn drop(&mut self) {
        unsafe {
            realsense_sys::rs2_delete_recommended_processing_blocks(self.ptr.as_ptr());
        }
    }
}

/// The iterator type returned by [ProcessingBlockList::try_into_iter](ProcessingBlockList::try_into_iter).
pub struct ProcessingBlockListIntoIter {
    len: usize,
    index: usize,
    ptr: NonNull<realsense_sys::rs2_processing_block_list>,
}

impl Iterator for ProcessingBlockListIntoIter {
    type Item = RsResult<ProcessingBlock<processing_block_marker::Any>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let result = unsafe {
                let mut checker = ErrorChecker::new();
                let ptr = realsense_sys::rs2_get_processing_block(
                    self.ptr.as_ptr(),
                    self.index as c_int,
                    checker.inner_mut_ptr(),
                );
                match checker.check() {
                    Ok(()) => ProcessingBlock::new_from_ptr(NonNull::new(ptr).unwrap()),
                    Err(err) => return Some(Err(err)),
                }
            };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl FusedIterator for ProcessingBlockListIntoIter {}

impl Drop for ProcessingBlockListIntoIter {
    fn drop(&mut self) {
        unsafe {
            realsense_sys::rs2_delete_recommended_processing_blocks(self.ptr.as_ptr());
        }
    }
}
