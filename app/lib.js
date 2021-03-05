const sbffi = require('sbffi')

const libpath = {
    prefix: ['darwin', 'linux'].indexOf(process.platform) > -1 ? 'lib' : '',
    extension: ['darwin', 'linux'].indexOf(process.platform) > -1 ? '.so' : '.dll',
    name: 'passguard'
};

const call_api = async (method_name) => {
    const libpath = libpath.prefix + libpath.name + libpath.extension
    const call = sbffi.getNativeFunction(libpath, 'entrypoint', 'int32_t', ['char*'])
    await call(method_name)
}

export default call_api