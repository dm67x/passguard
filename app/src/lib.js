const ffi = require('ffi-napi')
const ref = require('ref-napi')

let lib

const noop = () => { }
let init = () => {
    init = noop
    const libname = (process.platform !== 'darwin') ? 'passguard_api' : 'passguard_api.dll'
    lib = ffi.Library(libname, {
        'entrypoint': [ref.types.CString, [ref.types.CString]]
    })
}

export const entrypoint = (parameters) => {
    init()
    let { method, params } = parameters
    params = {
        method,
        params: params || []
    }
    return JSON.parse(lib.entrypoint(JSON.stringify(params)))
}