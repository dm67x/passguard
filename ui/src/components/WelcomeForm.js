import React, { useEffect, useState } from 'react'
import TextField from '@material-ui/core/TextField'
import Grid from '@material-ui/core/Grid'
import Typography from '@material-ui/core/Typography'
import Button from '@material-ui/core/Button'
import ButtonGroup from '@material-ui/core/ButtonGroup'
import { ipcRenderer } from 'electron'
import { useHistory } from 'react-router-dom'

const WelcomeForm = () => {
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")
    const [error, setError] = useState(false)
    let history = useHistory()

    useEffect(() => {
        ipcRenderer.on('signin-response', (_, arg) => {
            arg ? history.push('/passwords') : setError(true)
        })

        ipcRenderer.on('signup-response', (_, arg) => {
            arg.username ? history.push('/passwords') : setError(true)
        })

        return function cleanup() {
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
                backgroundColor: 'white',
                padding: '20px',
                boxShadow: '0px 0px 10px rgba(0, 0, 0, 0.5)'
            }}>
            <Grid
                container
                spacing={3}
                direction="row"
                alignItems="center"
                justify="center">
                <Grid item xs={12}>
                    <Typography variant="h4" align="center">
                        Welcome
                    </Typography>
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
        </div>
    )
}

export default WelcomeForm