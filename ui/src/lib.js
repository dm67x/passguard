const ffi = require('ffi-napi')
const ref = require('ref-napi')
const StructType = require('ref-struct-di')(ref)

const Parameters = StructType({
    method_name: ref.types.CString,
    param1: ref.types.CString,
    param2: ref.types.CString
})

const parametersPtr = ref.refType(Parameters)
let lib

const noop = () => { }
let init = () => {
    init = noop
    const libname = (process.platform !== 'darwin') ? 'passguard_api' : 'passguard_api.dll'
    lib = ffi.Library(libname, {
        'entrypoint': [ref.types.CString, [parametersPtr]]
    })
}

export const call = (params) => {
    init()
    const { methodName, param1, param2 } = params
    const parameters = new Parameters({
        method_name: methodName || '',
        param1: param1 || '',
        param2: param2 || ''
    })
    return JSON.parse(lib.entrypoint(parameters.ref()))
}