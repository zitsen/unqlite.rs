<a name="v1.4.2"></a>
## v1.4.2 (2019-05-21)


#### Bug Fixes

* **tests:**  fix locked error in some multi-threading cases ([b40cc5ac](b40cc5ac))



<a name="v1.4.1"></a>
### v1.4.1 (2018-05-16)


#### Features

* **UnQLiteVm:**  Add Clone trait to UnQLiteVm ([53b977a4](53b977a4))



<a name="v1.4.0"></a>
## v1.4.0 (2018-01-26)


#### Bug Fixes

* **util:**  warn use to not use `load_mmaped_file` ([ded1052e](ded1052e))

#### Features

*   deprecate unqlite-sys and reexport `ffi` and `vars` module ([86747446](86747446))



<a name="v1.3.2"></a>
## v1.3.2 (2017-12-20)


#### Bug Fixes

*   implement `Send` and `Sync` for `Error` ([c3c93972](c3c93972))



<a name="v1.3.1"></a>
## v1.3.1 (2017-08-28)


#### Bug Fixes

*   fix error as Unique/Shared syntax changed ([1ce81552](1ce81552), closes [#5](5))



<a name="v1.3.0"></a>
## v1.3.0 (2017-06-28)


#### Features

* **api:**  document store interface (json via jx9) ([276eb35c](276eb35c))



<a name="v1.2.2"></a>
## v1.2.2 (2017-05-24)


#### Bug Fixes

*   fix in case of Shared.as_mut_ptr deprecation ([5a90c035](5a90c035))



<a name="v1.2.1"></a>
## v1.2.1 (2016-05-25)


#### Bug Fixes

* **cursor:**  fix delete error when reach the last ([9664f93d](9664f93d))



<a name="v1.2.0"></a>
## v1.2.0 (2016-05-13)


#### Features

* **UnQLite:**  use official name as struct name ([65b647e0](65b647e0))

#### Bug Fixes

* **clippy:**  fix some clippy warnings ([c877df4a](c877df4a))



<a name="v1.1.1"></a>
## v1.1.1 (2016-05-13)


#### Bug Fixes

* **unqlite-sys:**  update dependency to v1.0.0 ([9a0a05e2](9a0a05e2))



<a name="v1.1.0"></a>
## v1.1.0 (2016-05-12)


#### Features

* **cursor:**  add key_callback/value_callback method to `Entry` ([817ac5da](817ac5da))

#### Bug Fixes

*   set OpenMode as private ([d5270acd](d5270acd))
* **CI:**  fix unstable bug in travis-cargo ([be2c016d](be2c016d))



<a name="v1.0.0"></a>
## v1.0.0 (2016-05-12)




