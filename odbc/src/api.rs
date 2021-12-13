use crate::{
    handles::{
        Connection, ConnectionState, Env, EnvState, MongoHandle, Statement, StatementState,
    },
    util::set_handle_state
};
use odbc_sys::{
    BulkOperation, CDataType, Char, CompletionType, ConnectionAttribute, Desc, DriverConnectOption,
    EnvironmentAttribute, FetchOrientation, HDbc, HDesc, HEnv, HStmt, HWnd, Handle, HandleType,
    InfoType, Integer, Len, Nullability, ParamType, Pointer, RetCode, SmallInt, SqlDataType,
    SqlReturn, StatementAttribute, ULen, USmallInt, WChar,
};
use std::{
    sync::RwLock,
    ptr,
    cmp::min
};

pub const UNIMPLEMENTED_FUNC: &str = "HYC00";

#[no_mangle]
pub extern "C" fn SQLAllocHandle(
    handle_type: HandleType,
    input_handle: Handle,
    output_handle: *mut Handle,
) -> SqlReturn {
    match sql_alloc_handle(handle_type, input_handle as *mut _, output_handle) {
        Ok(_) => SqlReturn::SUCCESS,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

fn sql_alloc_handle(
    handle_type: HandleType,
    input_handle: *mut MongoHandle,
    output_handle: *mut Handle,
) -> Result<(), ()> {
    match handle_type {
        HandleType::Env => {
            let env = RwLock::new(Env::with_state(EnvState::Allocated));
            let mh = Box::new(MongoHandle::Env(env));
            unsafe {
                *output_handle = Box::into_raw(mh) as *mut _;
            }
            Ok(())
        }
        HandleType::Dbc => {
            // input handle cannot be NULL
            if input_handle.is_null() {
                return Err(());
            }
            // input handle must be an Env
            let env = unsafe { (*input_handle).as_env().ok_or(())? };
            let conn = RwLock::new(Connection::with_state(
                input_handle,
                ConnectionState::Allocated,
            ));
            let mut env_contents = (*env).write().unwrap();
            let mh = Box::new(MongoHandle::Connection(conn));
            let mh_ptr = Box::into_raw(mh);
            env_contents.connections.insert(mh_ptr);
            env_contents.state = EnvState::ConnectionAllocated;
            unsafe { *output_handle = mh_ptr as *mut _ }
            Ok(())
        }
        HandleType::Stmt => {
            // input handle cannot be NULL
            if input_handle.is_null() {
                return Err(());
            }
            // input handle must be an Connection
            let conn = unsafe { (*input_handle).as_connection().ok_or(())? };
            let stmt = RwLock::new(Statement::with_state(
                input_handle,
                StatementState::Allocated,
            ));
            let mut conn_contents = (*conn).write().unwrap();
            let mh = Box::new(MongoHandle::Statement(stmt));
            let mh_ptr = Box::into_raw(mh);
            conn_contents.statements.insert(mh_ptr);
            conn_contents.state = ConnectionState::StatementAllocated;
            unsafe { *output_handle = mh_ptr as *mut _ }
            Ok(())
        }
        HandleType::Desc => {
            unimplemented!();
        }
    }
}

#[no_mangle]
pub extern "C" fn SQLBindCol(
    _hstmt: HStmt,
    _col_number: USmallInt,
    _target_type: CDataType,
    _target_value: Pointer,
    _buffer_length: Len,
    _length_or_indicatior: *mut Len,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLBindParameter(
    hstmt: HStmt,
    _parameter_number: USmallInt,
    _input_output_type: ParamType,
    _value_type: CDataType,
    _parmeter_type: SqlDataType,
    _column_size: ULen,
    _decimal_digits: SmallInt,
    _parameter_value_ptr: Pointer,
    _buffer_length: Len,
    _str_len_or_ind_ptr: *mut Len,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        hstmt as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLBindParameter is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLBrowseConnect(
    _connection_handle: HDbc,
    _in_connection_string: *const Char,
    _string_length: SmallInt,
    _out_connection_string: *mut Char,
    _buffer_length: SmallInt,
    _out_buffer_length: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLBrowseConnectW(
    _connection_handle: HDbc,
    _in_connection_string: *const WChar,
    _string_length: SmallInt,
    _out_connection_string: *mut WChar,
    _buffer_length: SmallInt,
    _out_buffer_length: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLBulkOperations(
    statement_handle: HStmt,
    _operation: BulkOperation,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLBulkOperations is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLCancel(_statement_handle: HStmt) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLCancelHandle(_handle_type: HandleType, _handle: Handle) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLCloseCursor(_statement_handle: HStmt) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColAttribute(
    _statement_handle: HStmt,
    _column_number: USmallInt,
    _field_identifier: Desc,
    _character_attribute_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
    _numeric_attribute_ptr: *mut Len,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColAttributeW(
    _statement_handle: HStmt,
    _column_number: USmallInt,
    _field_identifier: Desc,
    _character_attribute_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
    _numeric_attribute_ptr: *mut Len,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColumnPrivileges(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _table_name: *const Char,
    _table_name_length: SmallInt,
    _column_name: *const Char,
    _column_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColumnPrivilegesW(
    _statement_handle: HStmt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _table_name: *const WChar,
    _table_name_length: SmallInt,
    _column_name: *const WChar,
    _column_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColumns(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _table_name: *const Char,
    _table_name_length: SmallInt,
    _column_name: *const Char,
    _column_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLColumnsW(
    _statement_handle: HStmt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _table_name: *const WChar,
    _table_name_length: SmallInt,
    _column_name: *const WChar,
    _column_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLCompleteAsync(
    handle_type: HandleType,
    handle: Handle,
    _async_ret_code_ptr: *mut RetCode,
) -> SqlReturn {
    match set_handle_state(
        handle_type,
        handle,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLCompleteAsync is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLConnect(
    _connection_handle: HDbc,
    _server_name: *const Char,
    _name_length_1: SmallInt,
    _user_name: *const Char,
    _name_length_2: SmallInt,
    _authentication: *const Char,
    _name_length_3: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLConnectW(
    _connection_handle: HDbc,
    _server_name: *const WChar,
    _name_length_1: SmallInt,
    _user_name: *const WChar,
    _name_length_2: SmallInt,
    _authentication: *const WChar,
    _name_length_3: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLCopyDesc(source_desc_handle: HDesc, target_desc_handle: HDesc) -> SqlReturn {
    let error_message = "SQLCopyDesc is unimplemented";
    match set_handle_state(HandleType::Desc, source_desc_handle as *mut _, UNIMPLEMENTED_FUNC.clone(), error_message) {
        Ok(_) => {
            match set_handle_state(HandleType::Desc, target_desc_handle as *mut _, UNIMPLEMENTED_FUNC.clone(), error_message) {
                Ok(_) => SqlReturn::ERROR,
                Err(_) => SqlReturn::INVALID_HANDLE
            }
        },
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDataSources(
    environment_handle: HEnv,
    _direction: FetchOrientation,
    _server_name: *mut Char,
    _buffer_length_1: SmallInt,
    _name_length_1: *mut SmallInt,
    _description: *mut Char,
    _buffer_length_2: SmallInt,
    _name_length_2: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Env,
        environment_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDataSources is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDataSourcesW(
    environment_handle: HEnv,
    _direction: FetchOrientation,
    _server_name: *mut WChar,
    _buffer_length_1: SmallInt,
    _name_length_1: *mut SmallInt,
    _description: *mut WChar,
    _buffer_length_2: SmallInt,
    _name_length_2: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Env,
        environment_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDataSourcesW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDescribeCol(
    _hstmt: HStmt,
    _col_number: USmallInt,
    _col_name: *mut Char,
    _buffer_length: SmallInt,
    _name_length: *mut SmallInt,
    _data_type: *mut SqlDataType,
    _col_size: *mut ULen,
    _decimal_digits: *mut SmallInt,
    _nullable: *mut Nullability,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLDescribeColW(
    _hstmt: HStmt,
    _col_number: USmallInt,
    _col_name: *mut WChar,
    _buffer_length: SmallInt,
    _name_length: *mut SmallInt,
    _data_type: *mut SqlDataType,
    _col_size: *mut ULen,
    _decimal_digits: *mut SmallInt,
    _nullable: *mut Nullability,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLDescribeParam(
    statement_handle: HStmt,
    _parameter_number: USmallInt,
    _data_type_ptr: *mut SqlDataType,
    _parameter_size_ptr: *mut ULen,
    _decimal_digits_ptr: *mut SmallInt,
    _nullable_ptr: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDescribeParam is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDisconnect(_connection_handle: HDbc) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLDriverConnect(
    connection_handle: HDbc,
    _window_handle: HWnd,
    _in_connection_string: *const Char,
    _string_length_1: SmallInt,
    _out_connection_string: *mut Char,
    _buffer_length: SmallInt,
    _string_length_2: *mut SmallInt,
    _drive_completion: DriverConnectOption,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Dbc,
        connection_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDriverConnect is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDriverConnectW(
    connection_handle: HDbc,
    _window_handle: HWnd,
    _in_connection_string: *const WChar,
    _string_length_1: SmallInt,
    _out_connection_string: *mut WChar,
    _buffer_length: SmallInt,
    _string_length_2: *mut SmallInt,
    _driver_completion: DriverConnectOption,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Dbc,
        connection_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDriverConnectW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDrivers(
    henv: HEnv,
    _direction: FetchOrientation,
    _driver_desc: *mut Char,
    _driver_desc_max: SmallInt,
    _out_driver_desc: *mut SmallInt,
    _driver_attributes: *mut Char,
    _drvr_attr_max: SmallInt,
    _out_drvr_attr: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Env,
        henv as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDrivers is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLDriversW(
    henv: HEnv,
    _direction: FetchOrientation,
    _driver_desc: *mut WChar,
    _driver_desc_max: SmallInt,
    _out_driver_desc: *mut SmallInt,
    _driver_attributes: *mut WChar,
    _drvr_attr_max: SmallInt,
    _out_drvr_attr: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Env,
        henv as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLDriversW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLEndTran(
    handle_type: HandleType,
    handle: Handle,
    _completion_type: CompletionType,
) -> SqlReturn {
    match set_handle_state(
        handle_type,
        handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLEndTran is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLExecDirect(
    statement_handle: HStmt,
    _statement_text: *const Char,
    _text_length: Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLExecDirect is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLExecDirectW(
    statement_handle: HStmt,
    _statement_text: *const WChar,
    _text_length: Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLExecDirectW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLExecute(statement_handle: HStmt) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLExecute is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLFetch(_statement_handle: HStmt) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLFetchScroll(
    _statement_handle: HStmt,
    _fetch_orientation: FetchOrientation,
    _fetch_offset: Len,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLForeignKeys(
    _statement_handle: HStmt,
    _pk_catalog_name: *const Char,
    _pk_catalog_name_length: SmallInt,
    _pk_schema_name: *const Char,
    _pk_schema_name_length: SmallInt,
    _pk_table_name: *const Char,
    _pk_table_name_length: SmallInt,
    _fk_catalog_name: *const Char,
    _fk_catalog_name_length: SmallInt,
    _fk_schema_name: *const Char,
    _fk_schema_name_length: SmallInt,
    _fk_table_name: *const Char,
    _fk_table_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLForeignKeysW(
    _statement_handle: HStmt,
    _pk_catalog_name: *const WChar,
    _pk_catalog_name_length: SmallInt,
    _pk_schema_name: *const WChar,
    _pk_schema_name_length: SmallInt,
    _pk_table_name: *const WChar,
    _pk_table_name_length: SmallInt,
    _fk_catalog_name: *const WChar,
    _fk_catalog_name_length: SmallInt,
    _fk_schema_name: *const WChar,
    _fk_schema_name_length: SmallInt,
    _fk_table_name: *const WChar,
    _fk_table_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLFreeHandle(handle_type: HandleType, handle: Handle) -> SqlReturn {
    match sql_free_handle(handle_type, handle as *mut _) {
        Ok(_) => SqlReturn::SUCCESS,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

fn sql_free_handle(handle_type: HandleType, handle: *mut MongoHandle) -> Result<(), ()> {
    match handle_type {
        // By making Boxes to the types and letting them go out of
        // scope, they will be dropped.
        HandleType::Env => {
            let _ = unsafe { (*handle).as_env().ok_or(())? };
        }
        HandleType::Dbc => {
            let conn = unsafe { (*handle).as_connection().ok_or(())? };
            let mut env_contents = unsafe {
                (*conn.write().unwrap().env)
                    .as_env()
                    .ok_or(())?
                    .write()
                    .unwrap()
            };
            env_contents.connections.remove(&handle);
            if env_contents.connections.is_empty() {
                env_contents.state = EnvState::Allocated;
            }
        }
        HandleType::Stmt => {
            let stmt = unsafe { (*handle).as_statement().ok_or(())? };
            // Actually reading this value would make ASAN fail, but this
            // is what the ODBC standard expects.
            let mut conn_contents = unsafe {
                (*stmt.write().unwrap().connection)
                    .as_connection()
                    .ok_or(())?
                    .write()
                    .unwrap()
            };
            conn_contents.statements.remove(&handle);
            if conn_contents.statements.is_empty() {
                conn_contents.state = ConnectionState::Connected;
            }
        }
        HandleType::Desc => {
            unimplemented!();
        }
    }
    // create the Box at the end to ensure Drop only occurs when there are no errors due
    // to incorrect handle type.
    let _ = unsafe { Box::from_raw(handle) };
    Ok(())
}

#[no_mangle]
pub extern "C" fn SQLFreeStmt(_statement_handle: HStmt, _option: SmallInt) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetConnectAttr(
    _connection_handle: HDbc,
    _attribute: ConnectionAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetConnectAttrW(
    _connection_handle: HDbc,
    _attribute: ConnectionAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetCursorName(
    _statement_handle: HStmt,
    _cursor_name: *mut Char,
    _buffer_length: SmallInt,
    _name_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetCursorNameW(
    _statement_handle: HStmt,
    _cursor_name: *mut WChar,
    _buffer_length: SmallInt,
    _name_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetData(
    _statement_handle: HStmt,
    _col_or_param_num: USmallInt,
    _target_type: CDataType,
    _target_value_ptr: Pointer,
    _buffer_length: Len,
    _str_len_or_ind_ptr: *mut Len,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetDescField(
    descriptor_handle: HDesc,
    _record_number: SmallInt,
    _field_identifier: SmallInt,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        descriptor_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLGetDescField is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLGetDescFieldW(
    descriptor_handle: HDesc,
    _record_number: SmallInt,
    _field_identifier: SmallInt,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        descriptor_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLGetDescFieldW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLGetDescRec(
    descriptor_handle: HDesc,
    _record_number: SmallInt,
    _name: *mut Char,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
    _type_ptr: *mut SmallInt,
    _sub_type_ptr: *mut SmallInt,
    _length_ptr: *mut Len,
    _precision_ptr: *mut SmallInt,
    _scale_ptr: *mut SmallInt,
    _nullable_ptr: *mut Nullability,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        descriptor_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLGetDescRec is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLGetDescRecW(
    descriptor_handle: HDesc,
    _record_number: SmallInt,
    _name: *mut WChar,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
    _type_ptr: *mut SmallInt,
    _sub_type_ptr: *mut SmallInt,
    _length_ptr: *mut Len,
    _precision_ptr: *mut SmallInt,
    _scale_ptr: *mut SmallInt,
    _nullable_ptr: *mut Nullability,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        descriptor_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLGetDescRecW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLGetDiagField(
    _handle_type: HandleType,
    _handle: Handle,
    _record_rumber: SmallInt,
    _diag_identifier: SmallInt,
    _diag_info_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetDiagFieldW(
    _handle_type: HandleType,
    _handle: Handle,
    _record_rumber: SmallInt,
    _diag_identifier: SmallInt,
    _diag_info_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

fn set_sql_state(mut sql_state: String, output_ptr: *mut Char) -> () {
    unsafe {
        let state = std::mem::transmute::<*mut u8, *mut Char>(sql_state.as_mut_ptr());
        std::ptr::copy_nonoverlapping(state,output_ptr, 5)
    }
}

fn set_error_message(mut error_message: String, output_ptr: *mut Char, buffer_len: usize, text_length_ptr: *mut SmallInt) -> () {
    // TODO: make note abt how i wanted these to be macros but was getting weird issues when transmuting between types (byte slices were changing)
    unsafe {
        let msg = std::mem::transmute::<*mut u8, *mut Char>(error_message.as_mut_ptr());
        let num_chars =  min(error_message.len(),buffer_len);
        *text_length_ptr = num_chars as SmallInt;
        ptr::copy_nonoverlapping(msg, output_ptr, num_chars);
    }
}

#[no_mangle]
pub extern "C" fn SQLGetDiagRec(
    handle_type: HandleType,
    handle: Handle,
    rec_number: SmallInt,
    state: *mut Char,
    native_error_ptr: *mut Integer,
    message_text: *mut Char,
    buffer_length: SmallInt,
    text_length_ptr: *mut SmallInt,
) -> SqlReturn {
    if rec_number < 1 || buffer_length < 0 {
        return SqlReturn::ERROR
    }
    let mongo_handle = handle as *mut MongoHandle;
    let rec_number = (rec_number - 1) as usize; // subtract one because vecs are zero-indexed
    // TODO: unsafe { ptr::copy_nonoverlapping(0 as *mut Integer, native_error_ptr, 1) };
    match handle_type {
        HandleType::Env => {
            let env = unsafe { (*mongo_handle).as_env().unwrap() }; // TODO: error handling
            let env_contents = (*env).read().unwrap();
            match env_contents.sql_states.get(rec_number) {
                Some(sql_state) => set_sql_state(sql_state.clone(), state),
                None => return SqlReturn::NO_DATA
            }
            match env_contents.error_messages.get(rec_number) {
                Some(error_message) => set_error_message(error_message.clone(), message_text, buffer_length as usize, text_length_ptr),
                None => return SqlReturn::NO_DATA
            }
        },
        HandleType::Dbc => {
            let dbc = unsafe { (*mongo_handle).as_connection().unwrap() }; // TODO: error handling
            let dbc_contents = (*dbc).read().unwrap();
            match dbc_contents.sql_states.get(rec_number) {
                Some(sql_state) => set_sql_state(sql_state.clone(), state),
                None => return SqlReturn::NO_DATA
            }

            match dbc_contents.error_messages.get(rec_number) {
                // TODO: rm clones
                Some(error_message) => set_error_message(error_message.clone(), message_text, buffer_length as usize, text_length_ptr),
                None => return SqlReturn::NO_DATA
            }
        },
        HandleType::Stmt => {
            let stmt = unsafe { (*mongo_handle).as_statement().unwrap() }; // TODO: error handling
            let stmt_contents = (*stmt).read().unwrap();

            match stmt_contents.sql_states.get(rec_number) {
                Some(sql_state) => set_sql_state(sql_state.clone(), state),
                None => return SqlReturn::NO_DATA
            }

            match stmt_contents.error_messages.get(rec_number) {
                Some(error_message) => set_error_message(error_message.clone(), message_text, buffer_length as usize, text_length_ptr),
                None => return SqlReturn::NO_DATA
            }
        },
        HandleType::Desc => {
            let desc = unsafe { (*mongo_handle).as_descriptor().unwrap() }; // TODO: error handling
            let desc_contents = (*desc).read().unwrap();
            match desc_contents.sql_states.get(rec_number) {
                Some(sql_state) => set_sql_state(sql_state.clone(), state),
                None => return SqlReturn::NO_DATA
            }

            match desc_contents.error_messages.get(rec_number) {
                Some(error_message) => set_error_message(error_message.clone(), message_text, buffer_length as usize, text_length_ptr),
                None => return SqlReturn::NO_DATA
            }
        }
    }
    SqlReturn::SUCCESS
}

#[no_mangle]
pub extern "C" fn SQLGetDiagRecW(
    _handle_type: HandleType,
    _handle: Handle,
    _record_rumber: SmallInt,
    _state: *mut WChar,
    _native_error_ptr: *mut Integer,
    _message_text: *mut WChar,
    _buffer_length: SmallInt,
    _text_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetEnvAttr(
    _environment_handle: HEnv,
    _attribute: EnvironmentAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetEnvAttrW(
    _environment_handle: HEnv,
    _attribute: EnvironmentAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetInfo(
    _connection_handle: HDbc,
    _info_type: InfoType,
    _info_value_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetInfoW(
    _connection_handle: HDbc,
    _info_type: InfoType,
    _info_value_ptr: Pointer,
    _buffer_length: SmallInt,
    _string_length_ptr: *mut SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetStmtAttr(
    _handle: HStmt,
    _attribute: StatementAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetStmtAttrW(
    _handle: HStmt,
    _attribute: StatementAttribute,
    _value_ptr: Pointer,
    _buffer_length: Integer,
    _string_length_ptr: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLGetTypeInfo(_handle: HStmt, _data_type: SqlDataType) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLMoreResults(_handle: HStmt) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLNativeSql(
    _connection_handle: HDbc,
    _in_statement_text: *const Char,
    _in_statement_len: Integer,
    _out_statement_text: *mut Char,
    _buffer_len: Integer,
    _out_statement_len: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLNativeSqlW(
    _connection_handle: HDbc,
    _in_statement_text: *const WChar,
    _in_statement_len: Integer,
    _out_statement_text: *mut WChar,
    _buffer_len: Integer,
    _out_statement_len: *mut Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLNumParams(
    statement_handle: HStmt,
    _param_count_ptr: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLNumParams is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLNumResultCols(
    statement_handle: HStmt,
    _column_count_ptr: *mut SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLNumResultCols is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLParamData(hstmt: HStmt, _value_ptr_ptr: *mut Pointer) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        hstmt as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLParamData is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLPrepare(
    hstmt: HStmt,
    _statement_text: *const Char,
    _text_length: Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        hstmt as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLPrepare is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLPrepareW(
    hstmt: HStmt,
    _statement_text: *const WChar,
    _text_length: Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        hstmt as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLPrepareW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLPrimaryKeys(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _table_name: *const Char,
    _table_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLPrimaryKeysW(
    _statement_handle: HStmt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _table_name: *const WChar,
    _table_name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLProcedureColumns(
    statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _proc_name: *const Char,
    _proc_name_length: SmallInt,
    _column_name: *const Char,
    _column_name_length: SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLProcedureColumns is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLProcedureColumnsW(
    statement_handle: HStmt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _proc_name: *const WChar,
    _proc_name_length: SmallInt,
    _column_name: *const WChar,
    _column_name_length: SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLProcedureColumnsW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLProcedures(
    statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _proc_name: *const Char,
    _proc_name_length: SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLProcedures is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLProceduresW(
    statement_handle: HStmt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _proc_name: *const WChar,
    _proc_name_length: SmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLProceduresW is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLPutData(
    statement_handle: HStmt,
    _data_ptr: Pointer,
    _str_len_or_ind_ptr: Len,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLPutData is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLRowCount(_statement_handle: HStmt, _row_count_ptr: *mut Len) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetConnectAttr(
    _hdbc: HDbc,
    _attr: ConnectionAttribute,
    _value: Pointer,
    _str_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetConnectAttrW(
    _hdbc: HDbc,
    _attr: ConnectionAttribute,
    _value: Pointer,
    _str_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetCursorName(
    _statement_handle: HStmt,
    _cursor_name: *const Char,
    _name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetCursorNameW(
    _statement_handle: HStmt,
    _cursor_name: *const WChar,
    _name_length: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetDescField(
    desc_handle: HDesc,
    _rec_number: SmallInt,
    _field_identifier: SmallInt,
    _value_ptr: Pointer,
    _buffer_length: Integer,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        desc_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLSetDescField is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLSetDescRec(
    desc_handle: HDesc,
    _rec_number: SmallInt,
    _desc_type: SmallInt,
    _desc_sub_type: SmallInt,
    _length: Len,
    _precision: SmallInt,
    _scale: SmallInt,
    _data_ptr: Pointer,
    _string_length_ptr: *const Len,
    _indicator_ptr: *const Len,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Desc,
        desc_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLSetDescRec is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLSetPos(
    statement_handle: HStmt,
    _row_number: ULen,
    _operation: USmallInt,
    _lock_type: USmallInt,
) -> SqlReturn {
    match set_handle_state(
        HandleType::Stmt,
        statement_handle as *mut _,
        UNIMPLEMENTED_FUNC.clone(),
        "SQLSetPos is unimplemented"
    ) {
        Ok(_) => SqlReturn::ERROR,
        Err(_) => SqlReturn::INVALID_HANDLE,
    }
}

#[no_mangle]
pub extern "C" fn SQLSetEnvAttr(
    _environment_handle: HEnv,
    _attribute: EnvironmentAttribute,
    _value: Pointer,
    _string_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetEnvAttrW(
    _environment_handle: HEnv,
    _attribute: EnvironmentAttribute,
    _value: Pointer,
    _string_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetStmtAttr(
    _hstmt: HStmt,
    _attr: StatementAttribute,
    _value: Pointer,
    _str_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSetStmtAttrW(
    _hstmt: HStmt,
    _attr: StatementAttribute,
    _value: Pointer,
    _str_length: Integer,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSpecialColumns(
    _statement_handle: HStmt,
    _identifier_type: SmallInt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _table_name: *const Char,
    _table_name_length: SmallInt,
    _scope: SmallInt,
    _nullable: Nullability,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLSpecialColumnsW(
    _statement_handle: HStmt,
    _identifier_type: SmallInt,
    _catalog_name: *const WChar,
    _catalog_name_length: SmallInt,
    _schema_name: *const WChar,
    _schema_name_length: SmallInt,
    _table_name: *const WChar,
    _table_name_length: SmallInt,
    _scope: SmallInt,
    _nullable: Nullability,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLStatistics(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _catalog_name_length: SmallInt,
    _schema_name: *const Char,
    _schema_name_length: SmallInt,
    _table_name: *const Char,
    _table_name_length: SmallInt,
    _unique: SmallInt,
    _reserved: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLTablePrivileges(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _name_length_1: SmallInt,
    _schema_name: *const Char,
    _name_length_2: SmallInt,
    _table_name: *const Char,
    _name_length_3: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLTablesPrivilegesW(
    _statement_handle: HStmt,
    _catalog_name: *const WChar,
    _name_length_1: SmallInt,
    _schema_name: *const WChar,
    _name_length_2: SmallInt,
    _table_name: *const WChar,
    _name_length_3: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLTables(
    _statement_handle: HStmt,
    _catalog_name: *const Char,
    _name_length_1: SmallInt,
    _schema_name: *const Char,
    _name_length_2: SmallInt,
    _table_name: *const Char,
    _name_length_3: SmallInt,
    _table_type: *const Char,
    _name_length_4: SmallInt,
) -> SqlReturn {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn SQLTablesW(
    _statement_handle: HStmt,
    _catalog_name: *const WChar,
    _name_length_1: SmallInt,
    _schema_name: *const WChar,
    _name_length_2: SmallInt,
    _table_name: *const WChar,
    _name_length_3: SmallInt,
    _table_type: *const WChar,
    _name_length_4: SmallInt,
) -> SqlReturn {
    unimplemented!()
}
