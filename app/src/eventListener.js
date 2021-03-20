const { ipcMain } = require('electron')
const { entrypoint } = require('./lib')

ipcMain.on('signin', (event, arg) => {
    event.reply('signin-response', entrypoint({
        methodName: 'signin',
        param1: arg.username,
        param2: arg.password
    }))
})

ipcMain.on('signup', (event, arg) => {
    event.reply('signup-response', entrypoint({
        methodName: 'createUser',
        param1: arg.username,
        param2: arg.password
    }))
})

ipcMain.on('signout', (event) => {
    event.reply('signout-response', entrypoint({ methodName: 'signout' }))
})

ipcMain.on('decrypt-password', (event, arg) => {
    event.reply('decrypt-password-response', entrypoint({
        methodName: 'decrypt',
        param1: arg.password,
    }))
})

ipcMain.on('get-passwords', (event, arg) => {
    event.reply('get-passwords-response', entrypoint({
        methodName: 'getPasswords'
    }))
})

ipcMain.on('add-password', (event, arg) => {
    event.reply('add-password-response', entrypoint({
        methodName: 'createPassword',
        param1: arg.url,
        param2: arg.password
    }))
})

ipcMain.on('remove-password', (event, arg) => {
    event.reply('remove-password-response', entrypoint({
        methodName: 'deletePassword',
        param1: arg.id
    }))
})