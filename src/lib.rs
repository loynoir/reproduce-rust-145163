use {
    core::{mem::MaybeUninit, ptr::null_mut},
    napi_sys::{
        PropertyAttributes::enumerable, Status::napi_ok, napi_callback_info, napi_create_uint32, napi_define_properties, napi_env,
        napi_get_cb_info, napi_property_descriptor, napi_throw_error, napi_value,
    },
};

#[unsafe(no_mangle)]
extern "C" fn reproduce(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut argc = [2];
    let args = MaybeUninit::<[napi_value; 2]>::uninit();

    let mut args = unsafe { args.assume_init() };
    let status = unsafe { napi_get_cb_info(env, info, argc.as_mut_ptr(), args.as_mut_ptr(), null_mut(), null_mut()) };

    if status == napi_ok {
        eprintln!("[reproduce] status == napi_ok, should not error");
    } else {
        eprintln!("[reproduce] status != napi_ok, should be error");
    }

    if status != napi_ok {
        unsafe {
            napi_throw_error(env, null_mut(), c"[reproduce] Error: status != napi_ok".as_ptr());
        }
        return null_mut();
    }

    let result = MaybeUninit::<napi_value>::uninit();

    let mut result: napi_value = unsafe { result.assume_init() };
    let status = unsafe { napi_create_uint32(env, 42, &mut result) };
    if status != napi_ok {
        panic!();
    }

    result
}

#[unsafe(no_mangle)]
extern "C" fn workaround(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut argc = [2];
    let args = MaybeUninit::<[napi_value; 2]>::uninit();

    let mut args = unsafe { args.assume_init() };
    let status = unsafe { napi_get_cb_info(env, info, argc.as_mut_ptr(), args.as_mut_ptr(), null_mut(), null_mut()) };

    if status == napi_ok {
        eprintln!("[workaround] status == napi_ok, should not error");
    } else {
        eprintln!("[workaround] status != napi_ok, should be error");
    }

    if status != napi_ok {
        unsafe {
            napi_throw_error(env, null_mut(), c"[workaround] Error: status != napi_ok".as_ptr());
        }
        return null_mut();
    }

    let result = MaybeUninit::<[napi_value; 1]>::uninit();

    let mut result: [napi_value; 1] = unsafe { result.assume_init() };
    let status = unsafe { napi_create_uint32(env, 42, result.as_mut_ptr()) };

    if status != napi_ok {
        panic!();
    }

    result[0]
}

fn init(env: napi_env, exports: napi_value) -> napi_value {
    let properties: [napi_property_descriptor; 2] = [
        napi_property_descriptor {
            utf8name: c"reproduce".as_ptr(),
            name: null_mut(),

            method: Some(reproduce),
            getter: None,
            setter: None,
            value: null_mut(),
            attributes: enumerable,
            data: null_mut(),
        },
        napi_property_descriptor {
            utf8name: c"workaround".as_ptr(),
            name: null_mut(),

            method: Some(workaround),
            getter: None,
            setter: None,
            value: null_mut(),
            attributes: enumerable,
            data: null_mut(),
        },
    ];

    let status = unsafe { napi_define_properties(env, exports, properties.len(), properties.as_ptr()) };
    assert_eq!(status, napi_ok);

    exports
}

#[unsafe(no_mangle)]
extern "C" fn node_api_module_get_api_version_v1() -> i32 {
    8
}

#[unsafe(no_mangle)]
extern "C" fn napi_register_module_v1(env: napi_env, exports: napi_value) -> napi_value {
    init(env, exports)
}
