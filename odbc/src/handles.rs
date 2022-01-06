use crate::util::*;
use std::{collections::HashSet, sync::RwLock};

use odbc_sys::{Integer, SmallInt, SqlReturn, WChar};
use std::fmt::{Display, Formatter};

pub const VENDOR_IDENTIFIER: &str = "MongoDB";

#[derive(Debug)]
pub enum MongoHandle {
    Env(RwLock<Env>),
    Connection(RwLock<Connection>),
    Statement(RwLock<Statement>),
    #[allow(dead_code)]
    Descriptor(RwLock<Descriptor>),
}

impl MongoHandle {
    pub fn as_env(&self) -> Option<&RwLock<Env>> {
        match self {
            MongoHandle::Env(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_connection(&self) -> Option<&RwLock<Connection>> {
        match self {
            MongoHandle::Connection(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_statement(&self) -> Option<&RwLock<Statement>> {
        match self {
            MongoHandle::Statement(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_descriptor(&self) -> Option<&RwLock<Descriptor>> {
        match self {
            MongoHandle::Descriptor(d) => Some(d),
            _ => None,
        }
    }

    /// add_diag_info appends a new ODBCError object to the `errors` field.
    pub fn add_diag_info(&mut self, error: ODBCError) -> Result<(), ()> {
        match self {
            MongoHandle::Env(e) => {
                let mut env_contents = (*e).write().unwrap();
                env_contents.errors.push(error);
            }
            MongoHandle::Connection(c) => {
                let mut dbc_contents = (*c).write().unwrap();
                dbc_contents.errors.push(error);
            }
            MongoHandle::Statement(s) => {
                let mut stmt_contents = (*s).write().unwrap();
                stmt_contents.errors.push(error);
            }
            MongoHandle::Descriptor(d) => {
                let mut desc_contents = (*d).write().unwrap();
                desc_contents.errors.push(error);
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum SQLState {
    HYC00,
}

impl Display for SQLState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLState::HYC00 => write!(f, "HYC00"),
        }
    }
}

#[derive(Debug)]
pub enum ODBCError {
    Unimplemented(String),
}

impl ODBCError {
    pub fn get_sql_state(&self) -> SQLState {
        match self {
            ODBCError::Unimplemented(_) => SQLState::HYC00,
        }
    }
    pub fn get_error_message(&self) -> String {
        match self {
            ODBCError::Unimplemented(fn_name) => format!("[{}][API]{}", VENDOR_IDENTIFIER, fn_name),
        }
    }
    pub fn get_native_err_code(&self) -> i32 {
        match self {
            ODBCError::Unimplemented(_) => 0,
        }
    }
    pub fn get_diag_rec(
        &self,
        state: *mut WChar,
        message_text: *mut WChar,
        buffer_length: SmallInt,
        text_length_ptr: *mut SmallInt,
        native_error_ptr: *mut Integer,
    ) -> SqlReturn {
        unsafe { *native_error_ptr = self.get_native_err_code() };
        set_sql_state(self.get_sql_state(), state);
        set_error_message(
            self.get_error_message(),
            message_text,
            buffer_length as usize,
            text_length_ptr,
        )
    }
}

#[derive(Debug)]
pub struct Env {
    // attributes for this Env. We box the attributes so that the MongoHandle type
    // remains fairly small regardless of underlying handle type.
    pub _attributes: Box<EnvAttributes>,
    // state of this Env
    pub state: EnvState,
    pub connections: HashSet<*mut MongoHandle>,
    pub errors: Vec<ODBCError>,
}

impl Env {
    pub fn with_state(state: EnvState) -> Self {
        Self {
            _attributes: Box::new(EnvAttributes::default()),
            state,
            connections: HashSet::new(),
            errors: vec![],
        }
    }
}

#[derive(Debug)]
pub struct EnvAttributes {
    pub odbc_ver: Integer,
}

impl Default for EnvAttributes {
    fn default() -> Self {
        Self { odbc_ver: 3 }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnvState {
    Allocated,
    ConnectionAllocated,
}

#[derive(Debug)]
pub struct Connection {
    // type of this handle for runtime checking purposes.
    // Pointer to the Env from which
    // this Connection was allocated
    pub env: *mut MongoHandle,
    // all the possible Connection settings
    pub _attributes: Box<ConnectionAttributes>,
    // state of this connection
    pub state: ConnectionState,
    // MongoDB Client for issuing commands
    // pub client: Option<MongoClient>,
    // all Statements allocated from this Connection
    pub statements: HashSet<*mut MongoHandle>,
    pub errors: Vec<ODBCError>,
}

#[derive(Debug, Default)]
pub struct ConnectionAttributes {
    pub current_db: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Allocated,
    _ConnectionFunctionNeedsDataEnv,
    Connected,
    StatementAllocated,
    _TransactionInProgress,
}

impl Connection {
    pub fn with_state(env: *mut MongoHandle, state: ConnectionState) -> Self {
        Self {
            env,
            _attributes: Box::new(ConnectionAttributes::default()),
            state,
            statements: HashSet::new(),
            errors: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub connection: *mut MongoHandle,
    pub _attributes: Box<StatementAttributes>,
    pub state: StatementState,
    //pub cursor: Option<Box<Peekable<Cursor>>>,
    pub errors: Vec<ODBCError>,
}

#[derive(Debug, Default)]
pub struct StatementAttributes {}

#[derive(Debug, PartialEq, Eq)]
pub enum StatementState {
    Allocated,
    _Prepared,
    _PreparedHasResultSet,
    _ExecutedNoResultSet,
    _ExecutedHasResultSet,
    _CursorFetchSet,
    _CursorExtendedFetchSet,
    _FunctionNeedsDataNoParam,
    _FunctionNeedsDataNoPut,
    _FunctionNeedsDataPutCalled,
    _Executing,
    _AsyncCancelled,
}

impl Statement {
    pub fn with_state(connection: *mut MongoHandle, state: StatementState) -> Self {
        Self {
            connection,
            _attributes: Box::new(StatementAttributes::default()),
            state,
            errors: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct Descriptor {
    pub errors: Vec<ODBCError>,
}
