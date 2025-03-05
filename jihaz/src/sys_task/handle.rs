
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::backend::sys_task_handle as backend;
use crate::sys_task::thread_util;
use super::clipboard::Clipboard;
use super::error::Error;

/// A top level object.
///
/// This can be thought of as a reference and it can be safely cloned.
#[derive(Clone)]
pub struct SystemTaskHandle {
    pub(crate) backend_task_handle: backend::SystemTaskHandle,
}

/// Used to ensure only one SystemTaskHandle instance is ever created.
static HANDLE_CREATED: AtomicBool = AtomicBool::new(false);

thread_local! {
    /// A reference object to the current `SystemTaskHandle`, if any.
    static GLOBAL_APP: RefCell<Option<SystemTaskHandle>> = RefCell::new(None);
}

impl SystemTaskHandle {
    /// Create a new `SystemTaskHandle`. When app run loop finishes, the `SystemTaskHandle` should call finish to clear references.
    ///
    /// # Errors
    ///
    /// Errors if an `SystemTaskHandle` has already been created.
    ///
    /// This may change in the future. See [druid#771] for discussion.
    pub fn activate() -> Result<SystemTaskHandle, Error> {
        HANDLE_CREATED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Error::SystemTaskHandleAlreadyExists)?;
        let backend_task_handle = backend::SystemTaskHandle::activate()?;
        let app = SystemTaskHandle { backend_task_handle };
        GLOBAL_APP.with(|global_app| {
            *global_app.borrow_mut() = Some(app.clone());
        });
        Ok(app)
    }

    /// Get the current globally active `SystemTaskHandle`.
    ///
    /// A globally active `SystemTaskHandle` exists
    /// after [`new`] is called and until [`run`] returns.
    ///
    /// # Panics
    ///
    /// Panics if there is no globally active `SystemTaskHandle`.
    /// For a non-panicking function use [`try_global`].
    ///
    /// This function will also panic if called from a non-main thread.
    ///
    /// [`new`]: #method.new
    /// [`run`]: #method.run
    /// [`try_global`]: #method.try_global
    #[inline]
    pub fn global() -> SystemTaskHandle {
        // Main thread assertion takes place in try_global()
        SystemTaskHandle::try_global().expect("There is no globally active SystemTaskHandle")
    }

    /// Get the current globally active `SystemTaskHandle`.
    ///
    /// A globally active `SystemTaskHandle` exists
    /// after [`new`] is called and until [`run`] returns.
    ///
    /// # Panics
    ///
    /// Panics if called from a non-main thread.
    ///
    /// [`new`]: #method.new
    /// [`run`]: #method.run
    pub fn try_global() -> Option<SystemTaskHandle> {
        thread_util::assert_main_thread_or_main_unclaimed();
        GLOBAL_APP.with(|global_app| global_app.borrow().clone())
    }

    /// Finish the SystemTaskHandle and clear the references as the app has exited.
    pub fn finished(self) {

        // Run the platform task handle
        self.backend_task_handle.finished();

        // This sys_task_handle is no longer active, so clear the global reference
        GLOBAL_APP.with(|global_app| {
            *global_app.borrow_mut() = None;
        });
        // // .. and release the main thread
        // thread_util::release_main_thread();
        // .. and mark as done so a new sequence can start
        HANDLE_CREATED
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
            .expect("SystemTaskHandle marked as not created while still running.");
    }

    /// Quit the `SystemTaskHandle`.
    ///
    /// This will cause [`run`] to return control back to the calling function.
    ///
    /// [`run`]: #method.run
    pub fn quit(&self) {
        self.backend_task_handle.quit()
    }

    /// Returns a handle to the system clipboard.
    pub fn clipboard(&self) -> Clipboard {
        self.backend_task_handle.clipboard().into()
    }
}