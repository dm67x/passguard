const { ipcMain } = require('electron')
const { entrypoint } = require('./lib')

ipcMain.on('signin', (event, arg) => {
    event.reply('signin-response', entrypoint({
        method: 'signin',
        params: [arg.username, arg.password]
    }))
})

ipcMain.on('signup', (event, arg) => {
    event.reply('signup-response', entrypoint({
        method: 'createUser',
        params: [arg.username, arg.password]
    }))
})

ipcMain.on('signout', (event) => {
    event.reply('signout-response', entrypoint({ method: 'signout' }))
})

ipcMain.on('decrypt-password', (event, arg) => {
    event.reply('decrypt-password-response', entrypoint({
        method: 'decrypt',
        params: [arg.password]
    }))
})

ipcMain.on('get-passwords', (event, arg) => {
    event.reply('get-passwords-response', entrypoint({
        method: 'getPasswords'
    }))
})

ipcMain.on('add-password', (event, arg) => {
    event.reply('add-password-response', entrypoint({
        method: 'createPassword',
        params: [arg.url, arg.password]
    }))
})

ipcMain.on('remove-password', (event, arg) => {
    event.reply('remove-password-response', entrypoint({
        method: 'deletePassword',
        params: [arg.id]
    }))
})