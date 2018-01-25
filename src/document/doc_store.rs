use super::vm_value::{to_value, Value};
use UnQLite;
use error::{Result, Wrap};
use ffi::{unqlite_array_add_strkey_elem, unqlite_compile, unqlite_compile_file, unqlite_value,
          unqlite_value_bool, unqlite_value_double, unqlite_value_null, unqlite_value_string,
          unqlite_vm, unqlite_vm_config, unqlite_vm_dump, unqlite_vm_exec,
          unqlite_vm_extract_variable, unqlite_vm_new_array, unqlite_vm_new_scalar,
          unqlite_vm_release, unqlite_vm_release_value, unqlite_vm_reset, unqlite_value_int64};
use ffi::constants::{UNQLITE_OK, UNQLITE_VM_CONFIG_ARGV_ENTRY, UNQLITE_VM_CONFIG_CREATE_VAR,
                     UNQLITE_VM_CONFIG_ENV_ATTR, UNQLITE_VM_CONFIG_ERR_REPORT,
                     UNQLITE_VM_CONFIG_EXEC_VALUE, UNQLITE_VM_CONFIG_EXTRACT_OUTPUT,
                     UNQLITE_VM_CONFIG_IMPORT_PATH, UNQLITE_VM_CONFIG_OUTPUT,
                     UNQLITE_VM_CONFIG_RECURSION_DEPTH, UNQLITE_VM_OUTPUT_LENGTH};
use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::{null, null_mut, Shared};
use std::rc::Rc;
use std::slice;
use std::sync::mpsc;

/// Jx9 script compiler Interface.
///
/// The Document store interface to UnQLite which is used to store JSON docs
/// (i.e. Objects, Arrays, Strings, etc.) in the database is powered by the
/// [Jx9 programming language](https://unqlite.org/jx9.html).
pub trait Jx9 {
    /// Compile a Jx9 script to a bytecode program.
    fn compile<T: AsRef<[u8]>>(&self, jx9: T) -> Result<UnQLiteVm>;

    /// Compile a Jx9 script file to a bytecode program.
    fn compile_file<P: AsRef<str>>(&self, filename: P) -> Result<UnQLiteVm>;
}

/// Jx9 script compiler Interface.
impl Jx9 for UnQLite {
    /// Compile a Jx9 script to a bytecode program.
    fn compile<T: AsRef<[u8]>>(&self, jx9: T) -> Result<UnQLiteVm> {
        let mut vm: *mut unqlite_vm = null_mut();
        let jx9 = jx9.as_ref();
        wrap_raw!(self, compile, jx9.as_ptr() as _, jx9.len() as _, &mut vm)
            .map(|_| UnQLiteVm::new(vm))
    }

    /// Compile a Jx9 script file to a bytecode program.
    fn compile_file<P: AsRef<str>>(&self, filename: P) -> Result<UnQLiteVm> {
        let mut vm: *mut unqlite_vm = null_mut();
        let filename = CString::new(filename.as_ref())?;
        wrap_raw!(self, compile_file, filename.as_ptr(), &mut vm).map(|_| UnQLiteVm::new(vm))
    }
}

/// UnQLite Virtual Machine Object
///
/// Wrapper for native [`unqlite_vm`](https://unqlite.org/c_api_object.html#unqlite_vm) structure,
/// related [functions](https://unqlite.org/c_api_func.html)
/// and [configuration](https://unqlite.org/c_api/unqlite_vm_config.html).
pub struct UnQLiteVm {
    native: Shared<unqlite_vm>,
    executed: bool,
    output: Option<RefCell<Sender>>,
    names: Vec<Rc<CString>>,
}

/// Virtual Machine Object configuration and execution
impl UnQLiteVm {
    fn new(vm: *mut unqlite_vm) -> Self {
        UnQLiteVm {
            native: unsafe { Shared::new_unchecked(vm) },
            executed: false,
            output: None,
            names: Vec::new(),
        }
    }

    /// Execute a compiled Jx9 program.
    pub fn exec(&mut self) -> Result<Option<Value>> {
        self.exec_void()?;

        let ptr: *mut unqlite_value = null_mut();
        wrap_raw!(self, vm_config, UNQLITE_VM_CONFIG_EXEC_VALUE, &ptr)?;
        Ok(unsafe { to_value(ptr) })
    }

    /// Execute a compiled Jx9 program. Jx9 script return value ignored.
    pub fn exec_void(&mut self) -> Result<()> {
        if self.executed {
            self.reset()?
        }
        wrap_raw!(self, vm_exec).map(|_| self.executed = true)
    }

    /// Reset a UnQLite virtual machine to its initial state.
    fn reset(&mut self) -> Result<()> {
        wrap_raw!(self, vm_reset).map(|_| self.executed = false)
    }

    /// Dump Jx9 virtual machine instructions to `stdout`.
    pub fn dump(&self) -> Result<()> {
        wrap_raw!(
            self,
            vm_dump,
            Some(callback_to_stdout),
            null_mut::<c_void>()
        )
    }

    /// Redirect VM output to `std::sync::mpsc::Sender<Vec<u8>>` and
    /// return corresponding `Receiver`.
    ///
    /// Should be called before `exec()` method.
    pub fn output_to_channel(&mut self) -> Result<mpsc::Receiver<Vec<u8>>> {
        let (sender, receiver) = mpsc::channel();
        self.output = Some(RefCell::new(sender));
        let sender_ptr = self.output.as_ref().unwrap().as_ptr();

        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_OUTPUT,
            callback_to_channel as OutputCallback,
            sender_ptr
        ).map(|_| receiver)
    }

    /// Redirect VM output to `stdout`.
    /// Should be called before `exec()` method.
    pub fn output_to_stdout(&self) -> Result<()> {
        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_OUTPUT,
            callback_to_stdout as OutputCallback,
            null_mut::<c_void>()
        )
    }

    /// If the host application did not install a VM output consumer (`output_to_*` functions),
    /// then the Virtual Machine will automatically redirect its output to an internal buffer.
    /// Should be called after `exec()` method.
    pub fn extract_output(&self) -> Result<&[u8]> {
        let ptr: *const c_void = null();
        let mut len: u32 = 0;

        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_EXTRACT_OUTPUT,
            &ptr,
            &mut len
        ).map(|_| unsafe {
            slice::from_raw_parts(ptr as *const u8, len as usize)
        })
    }

    /// Return the total number of bytes that have been outputted by the Virtual Machine
    /// during program execution.
    ///
    /// Should be called after `exec()` method.
    pub fn output_length(&self) -> Result<u32> {
        let mut len: u32 = 0;
        wrap_raw!(self, vm_config, UNQLITE_VM_OUTPUT_LENGTH, &mut len)?;
        Ok(len)
    }

    /// Add a path to the import search directories for using in include or import Jx9 constructs.
    /// Should be called before `exec()` method.
    pub fn import_path<T: AsRef<str>>(&self, path: T) -> Result<()> {
        let path = CString::new(path.as_ref())?;
        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_IMPORT_PATH,
            path.as_ptr()
        )
    }

    /// All Jx9 run-time errors will be reported to the VM output.
    /// Should be called before `exec()` method.
    pub fn report_errors_to_output(&self) -> Result<()> {
        wrap_raw!(self, vm_config, UNQLITE_VM_CONFIG_ERR_REPORT)
    }

    /// Set a recursion limit to the running script. That is, a function may not call itself
    /// (recurse) more than this limit.
    /// If this limit is reached then the virtual machine abort the call and null is returned to the
    /// caller instead of the function return value.
    ///
    /// Should be called before `exec()` method.
    pub fn recursion_depth(&self, max_depth: i32) -> Result<()> {
        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_RECURSION_DEPTH,
            max_depth
        )
    }

    /// Populate the `$argv[n]` predefined Jx9 variable. First call of this method setups
    /// `$argv[0]`, second `$argv[1]`...
    ///
    /// Should be called before `exec()` method.
    pub fn add_argument<T: AsRef<[u8]>>(&self, arg: T) -> Result<()> {
        let arg = CString::new(arg.as_ref())?;
        wrap_raw!(self, vm_config, UNQLITE_VM_CONFIG_ARGV_ENTRY, arg.as_ptr())
    }

    /// Manually populate the $_ENV predefined JSON object which hold environments variable.
    /// Should be called before `exec()` method.
    pub fn add_env_attr<K, V>(&self, key: K, value: V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        let key = CString::new(key.as_ref())?;
        let value = value.as_ref();
        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_ENV_ATTR,
            key.as_ptr(),
            value.as_ptr(),
            value.len() as i32
        )
    }

    fn new_variable(&self, value: Value) -> Result<RawValue> {
        let raw: *mut unqlite_value = unsafe {
            if value.is_scalar() {
                unqlite_vm_new_scalar(self.as_raw_mut_ptr())
            } else {
                unqlite_vm_new_array(self.as_raw_mut_ptr())
            }
        };

        match value {
            Value::Null => wrap!(value_null, raw)?,
            Value::Int(x) => wrap!(value_int64, raw, x)?,
            Value::Bool(x) => wrap!(value_bool, raw, if x { 1 } else { 0 })?,
            Value::Real(x) => wrap!(value_double, raw, x)?,
            Value::String(x) => wrap!(
                value_string,
                raw,
                CString::new(x.as_bytes()).unwrap().as_ptr(),
                x.len() as i32
            )?,
            Value::Array(array) => for x in array {
                let raw_elem = self.new_variable(x)?;
                wrap!(array_add_strkey_elem, raw, null_mut(), raw_elem.as_ptr())?;
            },
            Value::Object(map) => for (key, value) in map {
                let raw_key = CString::new(key.as_bytes()).unwrap();
                let raw_value = self.new_variable(value)?;
                wrap!(
                    array_add_strkey_elem,
                    raw,
                    raw_key.as_ptr(),
                    raw_value.as_ptr()
                )?;
            },
        }

        Ok(RawValue::new(unsafe { self.as_raw_mut_ptr() }, raw))
    }

    // Register a foreign variable within the running Jx9 script.
    // This option is useful if you want to pass information from the outside environment
    // to the target Jx9 script.
    pub fn add_variable<T: Into<Vec<u8>>>(&mut self, name: T, value: Value) -> Result<()> {
        let value = self.new_variable(value)?;
        let name = Rc::new(CString::new(name).unwrap());
        self.names.push(Rc::clone(&name));
        wrap_raw!(
            self,
            vm_config,
            UNQLITE_VM_CONFIG_CREATE_VAR,
            name.as_ptr(),
            value.as_ptr()
        )
    }

    /// Extract the content of a variable declared inside your Jx9 script.
    /// Should be called before `exec()` method.
    pub fn extract_variable<T: AsRef<[u8]>>(&self, name: T) -> Option<Value> {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            to_value(unqlite_vm_extract_variable(
                self.as_raw_mut_ptr(),
                name.as_ptr(),
            ))
        }
    }

    unsafe fn as_raw_mut_ptr(&self) -> *mut unqlite_vm {
        self.native.as_ptr()
    }
}

unsafe impl Send for UnQLiteVm {}
unsafe impl Sync for UnQLiteVm {}

impl Drop for UnQLiteVm {
    /// Destroy a unQLite virtual machine.
    fn drop(&mut self) {
        unsafe {
            unqlite_vm_release(self.as_raw_mut_ptr());
        }
    }
}

type Sender = mpsc::Sender<Vec<u8>>;
type OutputCallback = extern "C" fn(*const c_void, u32, *mut c_void) -> i32;

/// VM output consumer callback.
/// Should return `UNQLITE_ABORT` or `UNQLITE_OK`.
/// If `UNQLITE_ABORT` returned then the Jx9 program will be terminated at this point.
extern "C" fn callback_to_channel(data: *const c_void, len: u32, sender: *mut c_void) -> i32 {
    let slice: &[u8] = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    let sender: &Sender = unsafe { &*(sender as *mut Sender) };

    let mut msg = Vec::with_capacity(len as usize);
    msg.extend_from_slice(slice);

    #[allow(match_same_arms)]
    match sender.send(msg) {
        Ok(_) => UNQLITE_OK,

        // This error means that the channel receiver is disconnected and
        // data will never be received.
        // So just continue the Jx9 program.
        Err(_) => UNQLITE_OK,
    }
}

extern "C" fn callback_to_stdout(data: *const c_void, len: u32, _: *mut c_void) -> i32 {
    let slice = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    print!("{}", String::from_utf8_lossy(slice));
    UNQLITE_OK
}

// Raliasable wrapper for raw unqlite_value. Used for creating vaiables for Jx9 script.
struct RawValue {
    host: *mut unqlite_vm,
    raw: *mut unqlite_value,
}

impl RawValue {
    pub fn new(host: *mut unqlite_vm, raw: *mut unqlite_value) -> Self {
        RawValue {
            host: host,
            raw: raw,
        }
    }

    pub fn as_ptr(&self) -> *mut unqlite_value {
        self.raw
    }
}

impl Drop for RawValue {
    fn drop(&mut self) {
        unsafe {
            unqlite_vm_release_value(self.host, self.raw);
        }
    }
}
