import React, { useState, useEffect } from 'react'
import { List, Paper, TextField, Button, ButtonGroup, DialogTitle, Dialog, DialogContent, DialogActions, DialogContentText } from '@material-ui/core'
import PasswordRow from './PasswordRow'
import AddIcon from '@material-ui/icons/Add'
import CloseIcon from '@material-ui/icons/Close'
import { ipcRenderer } from 'electron'
import Profile from './Profile'

const AddNewPasswordDialog = (props) => {
    const [open, setOpen] = useState(true)
    const [url, setUrl] = useState("")
    const [password, setPassword] = useState("")
    const [error, setError] = useState(undefined)
    const { onClose } = props

    useEffect(() => {
        ipcRenderer.on('add-password-response', (_, arg) => {
            if (arg) {
                setOpen(false)
                onClose()
            } else {
                setError("cannot create the password")
            }
        })

        return () => {
            ipcRenderer.removeAllListeners('add-password-response')
        }
    }, [])

    const addPassword = () => {
        ipcRenderer.send('add-password', {
            url,
            password
        })
    }

    return (
        <Dialog onClose={() => { setOpen(false); onClose() }} open={open}>
            <DialogTitle>Add a new password</DialogTitle>
            <DialogContent>
                <DialogContentText>
                    Enter the URL and password to add a new password in your list.
                    {error && <><br /><b style={{ color: 'red' }}>Error: {error}</b></>}
                </DialogContentText>
                <TextField fullWidth required label="URL" autoFocus margin="dense" onChange={(e) => setUrl(e.target.value)} />
                <TextField fullWidth required label="Password" type="password" margin="dense" onChange={(e) => setPassword(e.target.value)} />
            </DialogContent>
            <DialogActions>
                <Button color="secondary" onClick={() => { setOpen(false); onClose() }}>
                    Close <CloseIcon />
                </Button>
                <Button color="primary" onClick={() => addPassword()}>
                    Add <AddIcon />
                </Button>
            </DialogActions>
        </Dialog>
    )
}

const PasswordDialog = (props) => {
    const [open, setOpen] = useState(true)
    const { onClose, url, password } = props

    return (
        <Dialog onClose={() => { setOpen(false); onClose() }} open={open}>
            <DialogTitle>The password is...</DialogTitle>
            <DialogContent>
                <DialogContentText>
                    <b style={{ color: 'red' }}>{password}</b>
                </DialogContentText>
            </DialogContent>
            <DialogActions>
                <Button color="secondary" onClick={() => { setOpen(false); onClose() }}>
                    Close <CloseIcon />
                </Button>
            </DialogActions>
        </Dialog>
    )
}

const PasswordTable = () => {
    const [passwords, setPasswords] = useState([])
    const [showNewPasswordDialog, setShowNewPasswordDialog] = useState(false)
    const [showVisiblePassword, setShowVisiblePassword] = useState(undefined)

    useEffect(() => {
        ipcRenderer.send('get-passwords')

        ipcRenderer.on('get-passwords-response', (_, arg) => {
            setPasswords(arg)
        })

        ipcRenderer.on('decrypt-password-response', (_, arg) => {
            const response = arg
            setShowVisiblePassword(response)
        })

        ipcRenderer.on('remove-password-response', (_, arg) => {
            if (arg) {
                ipcRenderer.send('get-passwords')
            }
        })

        return () => {
            ipcRenderer.removeAllListeners('decrypt-password-response')
            ipcRenderer.removeAllListeners('get-passwords-response')
        }
    }, [])

    useEffect(() => {
        ipcRenderer.send('get-passwords')
    }, [showNewPasswordDialog])

    const createPassword = () => {
        setShowNewPasswordDialog(true)
    }

    const visibilityChanged = (id, value) => {
        const password = passwords.find((password) => password.id === id)
        if (password && value) {
            ipcRenderer.send('decrypt-password', { password: password.password })
        }
    }

    return (
        <Paper elevation={3}>
            <Profile />
            <div style={{ backgroundColor: 'white', width: '100%' }}>
                <List>
                    {passwords.map((value, index) => (
                        <PasswordRow
                            key={index}
                            id={value.id}
                            url={value.url}
                            password={value.password}
                            onVisibilityChanged={visibilityChanged} />
                    ))}
                </List>
                <ButtonGroup disableElevation style={{ height: '48px' }} fullWidth variant="contained">
                    <Button color="primary" onClick={() => createPassword()}>
                        <AddIcon />
                    </Button>
                </ButtonGroup>
                {showNewPasswordDialog && <AddNewPasswordDialog onClose={() => setShowNewPasswordDialog(false)} />}
                {showVisiblePassword && <PasswordDialog onClose={() => setShowVisiblePassword(undefined)} password={showVisiblePassword} />}
            </div>
        </Paper>
    )
}

export default PasswordTable