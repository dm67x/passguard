const { app, BrowserWindow, ipcMain } = require('electron')
const path = require('path')
const lib = require('./lib')

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) { // eslint-disable-line global-require
  app.quit();
}

const createWindow = () => {
  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
      enableRemoteModule: true,
    }
  });

  // and load the index.html of the app.
  mainWindow.loadURL(MAIN_WINDOW_WEBPACK_ENTRY);

  // Open the DevTools.
  //mainWindow.webContents.openDevTools();
};

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', createWindow);

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and import them here.
ipcMain.on('signin', (event, arg) => {
  event.reply('signin-response', lib.call({
    methodName: 'signin',
    param1: arg.username,
    param2: arg.password
  }))
})

ipcMain.on('signup', (event, arg) => {
  event.reply('signup-response', lib.call({
    methodName: 'createUser',
    param1: arg.username,
    param2: arg.password
  }))
})

ipcMain.on('signout', (event) => {
  event.reply('signout-response', lib.call({ methodName: 'signout' }))
})

ipcMain.on('decrypt-password', (event, arg) => {
  event.reply('decrypt-password-response', lib.call({
    methodName: 'decrypt',
    param1: arg.password,
  }))
})

ipcMain.on('get-passwords', (event, arg) => {
  event.reply('get-passwords-response', lib.call({
    methodName: 'getPasswords'
  }))
})

ipcMain.on('add-password', (event, arg) => {
  event.reply('add-password-response', lib.call({
    methodName: 'createPassword',
    param1: arg.url,
    param2: arg.password
  }))
})

ipcMain.on('remove-password', (event, arg) => {
  event.reply('remove-password-response', lib.call({
    methodName: 'deletePassword',
    param1: arg.id
  }))
})