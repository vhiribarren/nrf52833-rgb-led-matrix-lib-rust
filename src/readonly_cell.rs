// https://stackoverflow.com/questions/58953892/is-there-a-lightweight-alternative-for-a-mutex-in-embedded-rust-when-a-value-wil

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// A cell that can be written to once. After that, the cell is readonly and will panic if written to again.
/// Getting the value will panic if it has not already been set. Try 'try_get(_ref)' to see if it has already been set.
///
/// The cell can be used in embedded environments where a variable is initialized once, but later only written to.
/// This can be used in interrupts as well as it implements Sync.
///
/// Usage:
/// ```rust
/// static MY_VAR: DynamicReadOnlyCell<u32> = DynamicReadOnlyCell::new();
///
/// fn main() {
///     initialize();
///     calculate();
/// }
///
/// fn initialize() {
///     // ...
///     MY_VAR.set(42);
///     // ...
/// }
///
/// fn calculate() {
///     let my_var = MY_VAR.get(); // Will be 42
///     // ...
/// }
/// ```
pub struct DynamicReadOnlyCell<T: Sized> {
    data: UnsafeCell<Option<T>>,
    is_populated: AtomicBool,
}

impl<T: Sized> DynamicReadOnlyCell<T> {
    /// Creates a new unpopulated cell
    pub const fn new() -> Self {
        DynamicReadOnlyCell {
            data: UnsafeCell::new(None),
            is_populated: AtomicBool::new(false),
        }
    }
    /// Creates a new cell that is already populated
    pub const fn from(data: T) -> Self {
        DynamicReadOnlyCell {
            data: UnsafeCell::new(Some(data)),
            is_populated: AtomicBool::new(true),
        }
    }

    /// Populates the cell with data.
    /// Panics if the cell is already populated.
    pub fn set(&self, data: T) {
        cortex_m::interrupt::free(|_| {
            if self.is_populated.load(Ordering::Acquire) {
                panic!("Trying to set when the cell is already populated");
            }
            unsafe {
                *self.data.get() = Some(data);
            }

            self.is_populated.store(true, Ordering::Release);
        });
    }

    /// Gets a reference to the data from the cell.
    /// Panics if the cell is not yet populated.
    #[inline(always)]
    pub fn get_ref(&self) -> &T {
        if let Some(data) = self.try_get_ref() {
            data
        } else {
            panic!("Trying to get when the cell hasn't been populated yet");
        }
    }

    /// Gets a reference to the data from the cell.
    /// Returns Some(T) if the cell is populated.
    /// Returns None if the cell is not populated.
    #[inline(always)]
    pub fn try_get_ref(&self) -> Option<&T> {
        if !self.is_populated.load(Ordering::Acquire) {
            None
        } else {
            Some(unsafe { self.data.get().as_ref().unwrap().as_ref().unwrap() })
        }
    }
}

impl<T: Sized + Copy> DynamicReadOnlyCell<T> {
    /// Gets a copy of the data from the cell.
    /// Panics if the cell is not yet populated.
    #[inline(always)]
    pub fn get(&self) -> T {
        *self.get_ref()
    }

    /// Gets a copy of the data from the cell.
    /// Returns Some(T) if the cell is populated.
    /// Returns None if the cell is not populated.
    #[inline(always)]
    pub fn try_get(&self) -> Option<T> {
        self.try_get_ref().cloned()
    }
}

unsafe impl<T: Sized> Sync for DynamicReadOnlyCell<T> {}
