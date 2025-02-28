use crate::{api::errors::ODBCError, handles::definitions::*, SQLGetDiagRecW};
use odbc_sys::{HandleType, SqlReturn};
use std::sync::RwLock;

const UNIMPLEMENTED_FUNC: &str = "HYC00\0";

#[test]
fn simple() {
    fn validate_diag_rec(handle_type: HandleType, handle: *mut MongoHandle) {
        const ERROR_MESSAGE: &str = "[MongoDB][API] The feature SQLDrivers is not implemented\0";

        // Initialize buffers
        let sql_state = &mut [0u16; 6] as *mut _;
        // Note: len(ERROR_MESSAGE) = 57
        let message_text = &mut [0u16; 57] as *mut _;
        let text_length_ptr = &mut 0;
        let native_err_ptr = &mut 0;

        unsafe { (*handle).add_diag_info(ODBCError::Unimplemented("SQLDrivers")) }
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLGetDiagRecW(
                handle_type,
                handle as *mut _,
                1,
                sql_state,
                native_err_ptr,
                message_text,
                60, // Some number >= 57
                text_length_ptr,
            )
        );
        assert_eq!(UNIMPLEMENTED_FUNC, unsafe {
            String::from_utf16(&*(sql_state as *const [u16; 6])).unwrap()
        });
        assert_eq!(ERROR_MESSAGE, unsafe {
            String::from_utf16(&*(message_text as *const [u16; 57])).unwrap()
        });
        // Exclude the number of characters required for the null terminator
        assert_eq!(56, *text_length_ptr);
        assert_eq!(0, *native_err_ptr);
    }

    let env_handle: *mut _ =
        &mut MongoHandle::Env(RwLock::new(Env::with_state(EnvState::Allocated)));
    validate_diag_rec(HandleType::Env, env_handle);

    let conn_handle: *mut _ = &mut MongoHandle::Connection(RwLock::new(Connection::with_state(
        env_handle,
        ConnectionState::Allocated,
    )));
    validate_diag_rec(HandleType::Dbc, conn_handle);

    let stmt_handle: *mut _ = &mut MongoHandle::Statement(RwLock::new(Statement::with_state(
        std::ptr::null_mut(),
        StatementState::Allocated,
    )));
    validate_diag_rec(HandleType::Stmt, stmt_handle);
}

#[test]
fn error_message() {
    let env_handle: *mut _ =
        &mut MongoHandle::Env(RwLock::new(Env::with_state(EnvState::Allocated)));

    // Initialize buffers
    let sql_state = &mut [0u16; 6] as *mut _;
    let message_text = &mut [0u16; 57] as *mut _;
    let text_length_ptr = &mut 0;
    let native_err_ptr = &mut 0;

    unsafe { (*env_handle).add_diag_info(ODBCError::Unimplemented("SQLDrivers")) }
    // Buffer is too small to hold the entire error message and the null terminator
    // (0 < length < 57)
    assert_eq!(
        SqlReturn::SUCCESS_WITH_INFO,
        SQLGetDiagRecW(
            HandleType::Env,
            env_handle as *mut _,
            1,
            sql_state,
            native_err_ptr,
            message_text,
            15,
            text_length_ptr
        )
    );
    assert_eq!(
        "[MongoDB][API]\0",
        String::from_utf16(unsafe { &*(message_text as *const [u16; 15]) }).unwrap()
    );
    // Error message string where some characters are composed of more than one byte.
    // 1 < RecNumber =< number of diagnostic records.
    unsafe { (*env_handle).add_diag_info(ODBCError::Unimplemented("SQLDriv✐𑜲")) }
    assert_eq!(
        SqlReturn::SUCCESS,
        SQLGetDiagRecW(
            HandleType::Env,
            env_handle as *mut _,
            2,
            sql_state,
            native_err_ptr,
            message_text,
            57,
            text_length_ptr
        )
    );
    assert_eq!(
        "[MongoDB][API] The feature SQLDriv✐𑜲 is not implemented\0",
        String::from_utf16(unsafe { &*(message_text as *const [u16; 57]) }).unwrap()
    );
}

#[test]
fn invalid_ops() {
    let env_handle: *mut _ =
        &mut MongoHandle::Env(RwLock::new(Env::with_state(EnvState::Allocated)));

    // Initialize buffers
    let sql_state = &mut [0u16; 6] as *mut _;
    let message_text = &mut [0u16; 57] as *mut _;
    let text_length_ptr = &mut 0;
    let native_err_ptr = &mut 0;

    unsafe { (*env_handle).add_diag_info(ODBCError::Unimplemented("SQLDrivers")) }
    // Buffer length < 0
    assert_eq!(
        SqlReturn::ERROR,
        SQLGetDiagRecW(
            HandleType::Env,
            env_handle as *mut _,
            1,
            sql_state,
            native_err_ptr,
            message_text,
            -1,
            text_length_ptr
        )
    );
    // Record number <= 0
    assert_eq!(
        SqlReturn::ERROR,
        SQLGetDiagRecW(
            HandleType::Env,
            env_handle as *mut _,
            0,
            sql_state,
            native_err_ptr,
            message_text,
            57,
            text_length_ptr
        )
    );
    // Record number > number of diagnostic records
    assert_eq!(
        SqlReturn::NO_DATA,
        SQLGetDiagRecW(
            HandleType::Env,
            env_handle as *mut _,
            3,
            sql_state,
            native_err_ptr,
            message_text,
            5,
            text_length_ptr
        )
    );
}
