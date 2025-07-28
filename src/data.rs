#![deny(unsafe_op_in_unsafe_fn)]

use std::{ffi::c_void, ptr::NonNull};

use crate::{
    c_arc::CArc,
    c_box::{CBox, FnFree, Free},
    error::{Rav1dError, Rav1dResult},
    include::{
        common::validate::validate_input,
        dav1d::{common::Rav1dDataProps, data::Rav1dData},
    },
    send_sync_non_null::SendSyncNonNull,
};

impl From<CArc<[u8]>> for Rav1dData {
    fn from(data: CArc<[u8]>) -> Self {
        let size = data.len();
        Self {
            data: Some(data),
            m: Rav1dDataProps {
                size,
                ..Default::default()
            },
        }
    }
}

impl Rav1dData {
    pub fn create(size: usize) -> Rav1dResult<Self> {
        let data = CArc::zeroed_slice(size)?;
        Ok(data.into())
    }

    /// # Safety
    ///
    /// See [`CBox::from_c`]'s safety for `data`, `free_callback`, `cookie`.
    pub unsafe fn wrap(
        data: NonNull<[u8]>,
        free_callback: Option<FnFree>,
        cookie: Option<SendSyncNonNull<c_void>>,
    ) -> Rav1dResult<Self> {
        let free = validate_input!(free_callback.ok_or(Rav1dError::InvalidArgument))?;
        let free = Free { free, cookie };
        // SAFETY: Preconditions delegate to `CBox::from_c`'s safety.
        let data = unsafe { CBox::from_c(data, free) };
        let data = CArc::wrap(data)?;
        Ok(data.into())
    }

    /// # Safety
    ///
    /// See [`CBox::from_c`]'s safety for `user_data`, `free_callback`, `cookie`.
    pub unsafe fn wrap_user_data(
        &mut self,
        user_data: NonNull<u8>,
        free_callback: Option<FnFree>,
        cookie: Option<SendSyncNonNull<c_void>>,
    ) -> Rav1dResult {
        let free = validate_input!(free_callback.ok_or(Rav1dError::InvalidArgument))?;
        let free = Free { free, cookie };
        // SAFETY: Preconditions delegate to `CBox::from_c`'s safety.
        let user_data = unsafe { CBox::from_c(user_data, free) };
        let user_data = CArc::wrap(user_data)?;
        self.m.user_data = Some(user_data);
        Ok(())
    }
}

impl AsRef<[u8]> for Rav1dData {
    fn as_ref(&self) -> &[u8] {
        match &self.data {
            Some(data) => data.as_ref(),
            None => &[],
        }
    }
}
