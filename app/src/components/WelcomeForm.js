import React, { useEffect, useState } from 'react'
import TextField from '@material-ui/core/TextField'
import { Grid, Button, ButtonGroup, Paper } from '@material-ui/core'
import { ipcRenderer } from 'electron'
import { useHistory } from 'react-router-dom'
import logo from '../images/icon.png'

const WelcomeForm = () => {
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")
    const [error, setError] = useState(false)
    let history = useHistory()

    useEffect(() => {
        ipcRenderer.on('signin-response', (_, arg) => {
            arg?.error ? setError(true) : history.push('/passwords')
        })

        ipcRenderer.on('signup-response', (_, arg) => {
            arg?.error ? setError(true) : history.push('/passwords')
        })

        return () => {
            ipcRenderer.removeAllListeners('signin-response')
            ipcRenderer.removeAllListeners('signup-response')
            setError(false)
        }
    }, [])

    const signin = () => {
        ipcRenderer.send('signin', {
            username,
            password
        })
    }

    const signup = () => {
        ipcRenderer.send('signup', {
            username,
            password
        })
    }

    return (
        <div
            style={{
                position: 'absolute',
                left: '50%',
                top: '50%',
                transform: 'translate(-50%, -50%)',
                boxShadow: '0px 0px 5px rgba(0, 0, 0, 0.3)',
            }}>
            <Paper elevation={3} style={{ padding: '20px' }}>
                <Grid
                    container
                    spacing={3}
                    direction="row"
                    alignItems="center"
                    justify="center">
                    <Grid item xs={12} align="center">
                        <img src={logo} />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField fullWidth error={error} required label="Username" onChange={e => { setUsername(e.target.value) }} />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField fullWidth error={error} required type="password" label="Password" onChange={e => { setPassword(e.target.value) }} />
                    </Grid>
                    <Grid item xs={12}>
                        <ButtonGroup disableElevation style={{ height: '48px' }} fullWidth variant="contained">
                            <Button color="primary" onClick={() => signin()}>Signin</Button>
                            <Button color="secondary" onClick={() => signup()}>Signup</Button>
                        </ButtonGroup>
                    </Grid>
                </Grid>
            </Paper>
        </div>
    )
}

export default WelcomeForm